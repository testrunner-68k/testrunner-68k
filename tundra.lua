Build {

    Configs = {
        {
            Name = "macosx-clang",
            DefaultOnHost = "macosx",
            Tools = { "clang-osx" },
        },
        {
            Name = "linux-gcc",
            DefaultOnHost = "linux",
            Tools = { "gcc" },
            Env = {
				CCOPTS = {
					"-fPIC",
                    { "-O0", "-g"; Config = "*-*-debug" },
                    { "-O3"; Config = "*-*-release" },
                }
            },
        },
        {
            Name = "win32-msvc",
            DefaultOnHost = "windows",
            Tools = { "msvc" },
            Env = {
				GENERATE_PDB = "1",
				CCOPTS = {
					"/FS",
					{ "/Od"; Config = "*-*-debug" },
					{ "/O2"; Config = "*-*-release" },
				}
            },
        },
    },

    Passes = {
        musashi_code_generator = { Name = "Musashi Code Generator", BuildOrder = 1 },
        musashi_code_generation = { Name = "Musashi Code Generation", BuildOrder = 2 },
        musashi_library = { Name = "Musashi Library", BuildOrder = 3 },
    },

    Units = "units.musashi.lua",
}
    
