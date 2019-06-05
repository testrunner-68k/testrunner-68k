
#include <stddef.h>
#include "musashi_rust_wrapper.h"

extern RustM68KReadResult rust_m68k_read_memory_8(void* context, uint32_t address);
extern RustM68KReadResult rust_m68k_read_memory_16(void* context, uint32_t address);
extern RustM68KReadResult rust_m68k_read_memory_32(void* context, uint32_t address);

extern RustM68KWriteResult rust_m68k_write_memory_8(void* context, uint32_t address, uint32_t value);
extern RustM68KWriteResult rust_m68k_write_memory_16(void* context, uint32_t address, uint32_t value);
extern RustM68KWriteResult rust_m68k_write_memory_32(void* context, uint32_t address, uint32_t value);


extern int m68k_execute(int num_cycles);
extern void m68k_pulse_reset(void);


static void* s_context = NULL;


uint32_t m68k_read_memory_8(uint32_t address)
{
    RustM68KReadResult result = rust_m68k_read_memory_8(s_context, address);
    return result.value;
}

uint32_t m68k_read_memory_16(uint32_t address)
{
    RustM68KReadResult result = rust_m68k_read_memory_16(s_context, address);
    return result.value;
}

uint32_t m68k_read_memory_32(uint32_t address)
{
    RustM68KReadResult result = rust_m68k_read_memory_32(s_context, address);
    return result.value;
}

void m68k_write_memory_8(uint32_t address, uint32_t value)
{
    RustM68KWriteResult result = rust_m68k_write_memory_8(s_context, address, value);
}

uint32_t m68k_write_memory_16(uint32_t address, uint32_t value)
{
    RustM68KWriteResult result = rust_m68k_write_memory_16(s_context, address, value);
}

uint32_t m68k_write_memory_32(uint32_t address, uint32_t value)
{
    RustM68KWriteResult result = rust_m68k_write_memory_32(s_context, address, value);
}


void wrapped_m68k_pulse_reset(void* context)
{
    s_context = context;
    m68k_pulse_reset();
    s_context = NULL;
}

int wrapped_m68k_execute(void* context, int num_cycles)
{
    s_context = context;
    int cycles_used = m68k_execute(num_cycles);
    s_context = NULL;
    return cycles_used;
}
