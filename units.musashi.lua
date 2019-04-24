Program {
   Name = "m68kmake",
   Pass = "musashi_code_generator",
	Sources = { "musashi/m68kmake.c" },
}

Default "m68kmake"

DefRule {
	Name = "run_m68kmake",
   Pass = "musashi_code_generation",
	Command = "$(OBJECTDIR)$(SEP)m68kmake$(PROGSUFFIX) $(OBJECTDIR)/_musashi_generated/ $(<)",
	ImplicitInputs = { "$(OBJECTDIR)$(SEP)m68kmake$(PROGSUFFIX)" },

	Blueprint = {
		InputFile = { Required = true, Type = "string", Help = "Input filename", },
		OutputFolder = { Required = true, Type = "string", Help = "Output folder", },
	},

	Setup = function (env, data)
		return {
         InputFiles    = { data.InputFile },
         OutputFolder  = { data.OutputFolder },
         OutputFiles   = {
            data.OutputFolder .. "/m68kops.c",
            data.OutputFolder .. "/m68kopac.c",
            data.OutputFolder .. "/m68kopdm.c",
            data.OutputFolder .. "/m68kopnz.c",
         }
		}
	end,
}

StaticLibrary {
   Name = "musashi",
   Pass = "musashi_library",
   Sources = { "musashi/m68kcpu.c",
      "musashi/m68kdasm.c",
      run_m68kmake {
         InputFile = "musashi/m68k_in.c",
         OutputFolder = "$(OBJECTDIR)/_musashi_generated",
      }
   },
   Includes = {
      "musashi",
      "$(OBJECTDIR)/_musashi_generated",
   },

   Defines = {
      "M68K_COMPILE_FOR_MAME=2",    -- Do not compile for MAME; also, do not use settings from m68kconf.h

      -- Turn ON if you want to use the following M68K variants
      "M68K_EMULATE_010=OPT_OFF",
      "M68K_EMULATE_EC020=OPT_OFF",
      "M68K_EMULATE_020=OPT_OFF",

      -- If ON, the CPU will call m68k_read_immediate_xx() for immediate addressing
      -- and m68k_read_pcrelative_xx() for PC-relative addressing.
      -- If off, all read requests from the CPU will be redirected to m68k_read_xx()
      "M68K_SEPARATE_READS=OPT_OFF",

      -- If ON, the CPU will call m68k_write_32_pd() when it executes move.l with a
      -- predecrement destination EA mode instead of m68k_write_32().
      -- To simulate real 68k behavior, m68k_write_32_pd() must first write the high
      -- word to [address+2], and then write the low word to [address].
      "M68K_SIMULATE_PD_WRITES=OPT_OFF",
     
      -- If ON, CPU will call the interrupt acknowledge callback when it services an
      -- interrupt.
      -- If off, all interrupts will be autovectored and all interrupt requests will
      -- auto-clear when the interrupt is serviced.
      "M68K_EMULATE_INT_ACK=OPT_OFF",
      --#define M68K_INT_ACK_CALLBACK(A)    your_int_ack_handler_function(A)

      -- If ON, CPU will call the breakpoint acknowledge callback when it encounters
      -- a breakpoint instruction and it is running a 68010+.
      "M68K_EMULATE_BKPT_ACK=OPT_OFF",
      --#define M68K_BKPT_ACK_CALLBACK()    your_bkpt_ack_handler_function()
     
      -- If ON, the CPU will monitor the trace flags and take trace exceptions
      "M68K_EMULATE_TRACE=OPT_OFF",

      -- If ON, CPU will call the output reset callback when it encounters a reset
      -- instruction.
      "M68K_EMULATE_RESET=OPT_OFF",
      --#define M68K_RESET_CALLBACK()       your_reset_handler_function()

      -- If ON, CPU will call the set fc callback on every memory access to
      -- differentiate between user/supervisor, program/data access like a real
      -- 68000 would.  This should be enabled and the callback should be set if you
      -- want to properly emulate the m68010 or higher. (moves uses function codes
      -- to read/write data from different address spaces)
      "M68K_EMULATE_FC=OPT_OFF",
      --#define M68K_SET_FC_CALLBACK(A)     your_set_fc_handler_function(A)
     
      -- If ON, CPU will call the pc changed callback when it changes the PC by a
      -- large value.  This allows host programs to be nicer when it comes to
      -- fetching immediate data and instructions on a banked memory system.
      "M68K_MONITOR_PC=OPT_OFF",
      --#define M68K_SET_PC_CALLBACK(A)     your_pc_changed_handler_function(A)
     
      -- If ON, CPU will call the instruction hook callback before every
      -- instruction.
      "M68K_INSTRUCTION_HOOK=OPT_OFF",
      --#define M68K_INSTRUCTION_CALLBACK() your_instruction_hook_function()

      -- If ON, the CPU will emulate the 4-byte prefetch queue of a real 68000
      "M68K_EMULATE_PREFETCH=OPT_OFF",
      
      -- If ON, the CPU will generate address error exceptions if it tries to
      -- access a word or longword at an odd address.
      -- NOTE: This is only emulated properly for 68000 mode.
      "M68K_EMULATE_ADDRESS_ERROR=OPT_OFF",

      -- Turn ON to enable logging of illegal instruction calls.
      -- M68K_LOG_FILEHANDLE must be #defined to a stdio file stream.
      -- Turn on M68K_LOG_1010_1111 to log all 1010 and 1111 calls.
      "M68K_LOG_ENABLE=OPT_OFF",
      "M68K_LOG_1010_1111=OPT_OFF",
      --#define M68K_LOG_FILEHANDLE         some_file_handle

      -- If ON, the enulation core will use 64-bit integers to speed up some
      -- operations.
      "M68K_USE_64_BIT=OPT_OFF",

      -- Set to your compiler's static inline keyword to enable it, or
      -- set it to blank to disable it.
      -- If you define INLINE in the makefile, it will override this value.
      -- NOTE: not enabling inline functions will SEVERELY slow down emulation.
      { "INLINE=\"static __inline\""; Config = { "win32-msvc" } },
      { "INLINE=\"static inline\""; Config = { "macosx-clang", "linux-gcc" } },
     
   }
}

Default "musashi"
