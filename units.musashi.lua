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
      "musashi/musashi_rust_wrapper.c",
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
      -- Set to your compiler's static inline keyword to enable it, or
      -- set it to blank to disable it.
      -- If you define INLINE in the makefile, it will override this value.
      -- NOTE: not enabling inline functions will SEVERELY slow down emulation.
      { "INLINE=\"static __inline\""; Config = { "win32-msvc" } },
      { "INLINE=\"static inline\""; Config = { "macosx-clang", "linux-gcc" } },
     
   }
}

Default "musashi"
