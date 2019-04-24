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
        },
        {
            Name = "win32-msvc",
            DefaultOnHost = "windows",
            Tools = { "msvc" },
            Env = {
                CXXOPTS = { "/EHsc" },
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
    
