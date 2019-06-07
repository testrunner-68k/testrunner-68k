
#include <setjmp.h>
#include <stddef.h>
#include "musashi_rust_wrapper.h"

extern RustM68KReadResult rust_m68k_read_memory_8(void* execution_context, uint32_t address);
extern RustM68KReadResult rust_m68k_read_memory_16(void* execution_context, uint32_t address);
extern RustM68KReadResult rust_m68k_read_memory_32(void* execution_context, uint32_t address);

extern RustM68KWriteResult rust_m68k_write_memory_8(void* execution_context, uint32_t address, uint32_t value);
extern RustM68KWriteResult rust_m68k_write_memory_16(void* execution_context, uint32_t address, uint32_t value);
extern RustM68KWriteResult rust_m68k_write_memory_32(void* execution_context, uint32_t address, uint32_t value);

extern RustM68KInstructionHookResult rust_m68k_instruction_hook(void* execution_context);
extern RustM68KInstructionHookResult rust_m68k_exception_illegal_hook(void* execution_context);


extern int m68k_execute(int num_cycles);
extern void m68k_pulse_reset(void);
extern void m68ki_exception_illegal_default(void);

static void* s_execution_context = NULL;
static jmp_buf s_abort_execution;

uint32_t m68k_read_memory_8(uint32_t address)
{
    RustM68KReadResult result = rust_m68k_read_memory_8(s_execution_context, address);
    if (!result.continue_simulation)
        longjmp(s_abort_execution, 1);
    return result.value;
}

uint32_t m68k_read_memory_16(uint32_t address)
{
    RustM68KReadResult result = rust_m68k_read_memory_16(s_execution_context, address);
    if (!result.continue_simulation)
        longjmp(s_abort_execution, 1);
    return result.value;
}

uint32_t m68k_read_memory_32(uint32_t address)
{
    RustM68KReadResult result = rust_m68k_read_memory_32(s_execution_context, address);
    if (!result.continue_simulation)
        longjmp(s_abort_execution, 1);
    return result.value;
}

void m68k_write_memory_8(uint32_t address, uint32_t value)
{
    RustM68KWriteResult result = rust_m68k_write_memory_8(s_execution_context, address, value);
    if (!result.continue_simulation)
        longjmp(s_abort_execution, 1);
}

void m68k_write_memory_16(uint32_t address, uint32_t value)
{
    RustM68KWriteResult result = rust_m68k_write_memory_16(s_execution_context, address, value);
    if (!result.continue_simulation)
        longjmp(s_abort_execution, 1);
}

void m68k_write_memory_32(uint32_t address, uint32_t value)
{
    RustM68KWriteResult result = rust_m68k_write_memory_32(s_execution_context, address, value);
    if (!result.continue_simulation)
        longjmp(s_abort_execution, 1);
}

void m68k_instruction_hook()
{
    RustM68KInstructionHookResult result = rust_m68k_instruction_hook(s_execution_context);
    if (!result.continue_simulation)
        longjmp(s_abort_execution, 1);
}

void m68k_exception_illegal_custom()
{
    RustM68KInstructionHookResult result = rust_m68k_exception_illegal_hook(s_execution_context);
    if (!result.continue_simulation)
        longjmp(s_abort_execution, 1);
    m68ki_exception_illegal_default();
}

void wrapped_m68k_pulse_reset(void* execution_context)
{
    s_execution_context = execution_context;

    if (setjmp(s_abort_execution) == 0)
        m68k_pulse_reset();

    s_execution_context = NULL;
}

int wrapped_m68k_execute(void* execution_context, int num_cycles)
{
    s_execution_context = execution_context;

    if (setjmp(s_abort_execution) == 0)
    {
        int cycles_used = m68k_execute(num_cycles);

        s_execution_context = NULL;
        return cycles_used;
    }
    else
    {
        s_execution_context = NULL;
        return 0;
    }
}
