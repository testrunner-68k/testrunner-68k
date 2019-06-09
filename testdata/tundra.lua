
local common = {
	Env = {
		VASM = "vasm",
		VLINK = "vlink",
--		ASM_IMPLICIT_DEPS = { "$(VASM)" },
--		ASMINCPATH = { ".", "$(VBCC)/include/sdk", "$(OBJECTDIR)" },
	},

	Defines = {
		{ "NDEBUG"; Config = "*-*-release" },
	},
}

local amiga = {
	Inherit = common,
	Env = {

		ASMOPTS = {
			"-m68000",
			"-phxass",
			"-ignore-mult-inc",
			"-spaces",
			{
				"-linedebug" ; Config = "debug"
			},
--			"-nowarn=62", -- imported symbol <x> was not refrenced
		},
		LIBOPTS = {
			"-r",
		},
		PROGOPTS = { "-bamigahunk" },
	},
}

Build {
	Configs = {
		Config {
			Name = "amiga-crossosx",
			Virtual = true,
			Inherit = amiga,
			Env = {
				HOSTPLATFORM = "Darwin",
			},
			SupportedHosts = { "macosx" },
			Tools = { "tools.vasm" },
		},

		Config {
			Name = "amiga-crosslinux",
			Virtual = true,
			Inherit = amiga,
			Env = {
				HOSTPLATFORM = "linux",
			},
			SupportedHosts = { "linux" },
			Tools = { "tools.vasm" },
		},

		Config {
			Name = "amiga-crosswin",
			Virtual = true,
			Inherit = amiga,
			Env = {
				HOSTPLATFORM = "win32",
			},
			SupportedHosts = { "windows" },
			Tools = { "tools.vasm" },
		},


		Config {
			Name = "amiga-mac",
			SubConfigs = {
				host = "macosx-clang",
				target = "amiga-crossosx",
			},

			SupportedHosts = { "macosx" },
			DefaultSubConfig = "target",
		},

		Config {
			Name = "amiga-linux",
			SubConfigs = {
				host = "linux-gcc",
				target = "amiga-crosslinux",
			},

			SupportedHosts = { "linux" },
			DefaultSubConfig = "target",
		},

		Config {
			Name = "amiga-win",
			SubConfigs = {
				host = "win32-msvc",
				target = "amiga-crosswin",
			},

			SupportedHosts = { "windows" },
			DefaultSubConfig = "target",
		},

		-- Native platforms

		Config {
			Name = "win32-msvc",
--			Inherit = win32,
--			Tools = { "msvc" },
			DefaultOnHost = "windows",
			SupportedHosts = { "windows"},
		},

		Config {
			Name = "macosx-clang",
--			Inherit = macosx,
--			Tools = { "clang-osx" },
			DefaultOnHost = "macosx",
			SupportedHosts = { "macosx" },
		},

		Config {
			Name = "linux-gcc",
--			Inherit = linux,
--			Tools = { "gcc"	},
			DefaultOnHost = "linux",
			SupportedHosts = { "linux" },
		},
	},

    Units = "units.tests.lua",

	ScriptDirs = {
		"."
	}
}
    