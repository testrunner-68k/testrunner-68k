
#include <stdint.h>
#include <stdbool.h>

typedef struct 
{
    bool continue_simulation;
    uint32_t value;
} RustM68KReadResult;

typedef struct 
{
    bool continue_simulation;
} RustM68KWriteResult;

typedef struct 
{
    bool continue_simulation;
} RustM68KInstructionHookResult;

void wrapped_m68k_pulse_reset(void* execution_context);
int wrapped_m68k_execute(void* execution_context, int num_cycles);
