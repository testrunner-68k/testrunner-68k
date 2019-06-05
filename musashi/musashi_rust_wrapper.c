
#include <setjmp.h>
#include <stddef.h>
#include "musashi_rust_wrapper.h"

extern RustM68KReadResult rust_m68k_read_memory_8(void* context, uint32_t address);
extern RustM68KReadResult rust_m68k_read_memory_16(void* context, uint32_t address);
extern RustM68KReadResult rust_m68k_read_memory_32(void* context, uint32_t address);

extern RustM68KWriteResult rust_m68k_write_memory_8(void* context, uint32_t address, uint32_t value);
extern RustM68KWriteResult rust_m68k_write_memory_16(void* context, uint32_t address, uint32_t value);
extern RustM68KWriteResult rust_m68k_write_memory_32(void* context, uint32_t address, uint32_t value);

extern RustM68KInstructionHookResult rust_m68k_instruction_hook(void* context);


extern int m68k_execute(int num_cycles);
extern void m68k_pulse_reset(void);


static void* s_context = NULL;
static jmp_buf s_abort_execution;

uint32_t m68k_read_memory_8(uint32_t address)
{
    RustM68KReadResult result = rust_m68k_read_memory_8(s_context, address);
    if (!result.success)
        longjmp(s_abort_execution, 1);
    return result.value;
}

uint32_t m68k_read_memory_16(uint32_t address)
{
    RustM68KReadResult result = rust_m68k_read_memory_16(s_context, address);
    if (!result.success)
        longjmp(s_abort_execution, 1);
    return result.value;
}

uint32_t m68k_read_memory_32(uint32_t address)
{
    RustM68KReadResult result = rust_m68k_read_memory_32(s_context, address);
    if (!result.success)
        longjmp(s_abort_execution, 1);
    return result.value;
}

void m68k_write_memory_8(uint32_t address, uint32_t value)
{
    RustM68KWriteResult result = rust_m68k_write_memory_8(s_context, address, value);
    if (!result.success)
        longjmp(s_abort_execution, 1);
}

uint32_t m68k_write_memory_16(uint32_t address, uint32_t value)
{
    RustM68KWriteResult result = rust_m68k_write_memory_16(s_context, address, value);
    if (!result.success)
        longjmp(s_abort_execution, 1);
}

uint32_t m68k_write_memory_32(uint32_t address, uint32_t value)
{
    RustM68KWriteResult result = rust_m68k_write_memory_32(s_context, address, value);
    if (!result.success)
        longjmp(s_abort_execution, 1);
}

void m68k_instruction_hook()
{
    RustM68KInstructionHookResult result = rust_m68k_instruction_hook(s_context);
    if (!result.success)
        longjmp(s_abort_execution, 1);
}

void wrapped_m68k_pulse_reset(void* context)
{
    s_context = context;

    if (setjmp(s_abort_execution) == 0)
        m68k_pulse_reset();

    s_context = NULL;
}

int wrapped_m68k_execute(void* context, int num_cycles)
{
    s_context = context;

    if (setjmp(s_abort_execution) == 0)
    {
        int cycles_used = m68k_execute(num_cycles);

        s_context = NULL;
        return cycles_used;
    }
    else
    {
        s_context = NULL;
        return 0;
    }
}
