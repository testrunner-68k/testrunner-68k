
#include <stdint.h>
#include <stdbool.h>

typedef struct 
{
    bool success;
    uint32_t value;
} RustM68KReadResult;

typedef struct 
{
    bool success;
} RustM68KWriteResult;

void wrapped_m68k_pulse_reset(void* context);
int wrapped_m68k_execute(void* context, int num_cycles);
