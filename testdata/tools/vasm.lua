module(..., package.seeall)

-- This is a hacked up version of the tundra generic-asm.lua module
-- that will also add implicit dependencies on our build tools so we
-- can build the assembler/linker for the host automatically from a fresh tree.

local path     = require "tundra.path"
local util     = require "tundra.util"
local boot     = require "tundra.boot"
local scanner  = require "tundra.scanner"
local depgraph = require "tundra.depgraph"
local native   = require "tundra.native"

local default_keywords = { "include" }
local default_bin_keywords = { "incbin" }

local function get_asm_scanner(env, fn)
  local function test_bool(name, default)
    val = env:get(name, default)
    if val == "yes" or val == "true" or val == "1" then
      return 1
    else
      return 0
    end
  end
  local function new_scanner()
    local paths = util.map(env:get_list("ASMINCPATH"), function (v) return env:interpolate(v) end)
    local data = {
      Paths = paths,
      Keywords = env:get_list("ASMINC_KEYWORDS", default_keywords),
      KeywordsNoFollow = env:get_list("ASMINC_BINARY_KEYWORDS", default_bin_keywords),
      RequireWhitespace = test_bool("ASMINC_REQUIRE_WHITESPACE", "yes"),
      UseSeparators = test_bool("ASMINC_USE_SEPARATORS", "yes"),
      BareMeansSystem = test_bool("ASMINC_BARE_MEANS_SYSTEM", "no"),
    }
    return scanner.make_generic_scanner(data)
  end
  return env:memoize("ASMINCPATH", "_asm_scanner", new_scanner)
end

-- Register implicit make functions for assembly files.
-- These functions are called to transform source files in unit lists into
-- object files. This function is registered as a setup function so it will be
-- run after user modifications to the environment, but before nodes are
-- processed. This way users can override the extension lists.
local function newage_asm_setup(env)
  local _assemble = function(env, pass, fn)
    local object_fn = path.make_object_filename(env, fn, '$(OBJECTSUFFIX)')

    -- Hack
    local implicit_inputs = {
      env:interpolate('$(VASM)'),
      env:interpolate('$(VLINK)'),
    }

    return depgraph.make_node {
      Env            = env,
      Label          = 'Asm $(@)',
      Pass           = pass,
      Action         = "$(ASMCOM)",
      InputFiles     = { fn },
      ImplicitInputs = implicit_inputs,
      OutputFiles    = { object_fn },
      Scanner        = get_asm_scanner(env, fn),
    }
  end

  for _, ext in ipairs(env:get_list("ASM_EXTS")) do
    env:register_implicit_make_fn(ext, _assemble)
  end
end

function apply(_outer_env, options)

  _outer_env:add_setup_function(newage_asm_setup)

  local resp = '$(<)'
  -- Use response files for object lists on windows, because cmd.exe
  --  note: currently commented out, because when building on windows, the response files
  --  seem to not get created, or in the wrong place
  --
  --if native.host_platform == "windows" then
  --  resp = "@RESPONSE!@!$(<:P\n)"
  --end

  _outer_env:set_many {
    ["ASM_EXTS"]           = { ".s", ".asm" },
    ["ASMINCPATH"]         = {},
    ["ASMDEFS"]            = "",
    ["ASMDEFS_DEBUG"]      = "",
    ["ASMDEFS_PRODUCTION"] = "",
    ["ASMDEFS_RELEASE"]    = "",
    ["ASMOPTS"]            = "",
    ["ASMOPTS_DEBUG"]      = "",
    ["ASMOPTS_PRODUCTION"] = "",
    ["ASMOPTS_RELEASE"]    = "",

    ["NATIVE_SUFFIXES"]    = { ".s", ".asm", ".a", ".o" },
    ["OBJECTSUFFIX"]       = ".o",
    ["LIBPREFIX"]          = "",
    ["LIBSUFFIX"]          = ".a",
    ["SHLIBLINKSUFFIX"]    = "",
    ["PROGPREFIX"]         = "",
    ["PROGSUFFIX"]         = "",

    ["ASMCOM"]             = "$(VASM) -o $(@) -quiet -Fhunk $(ASMDEFS:p-D ) $(ASMINCPATH:p-I ) $(ASMOPTS) $(<)",
    ["LIB"]                = "$(VLINK)",
    ["LIBCOM"]             = "$(LIB) $(LIBOPTS) -o $(@) $(<)",
    ["VLINKOPTS"]          = "",
    ["PROGCOM"]            = "$(VLINK) $(PROGOPTS) -o $(@) " .. resp,
  }
end


