
# Test runner for Amiga/680x0 code [![Build Status](https://travis-ci.com/Kalmalyzer/testrunner-68k.svg?branch=master)](https://travis-ci.com/Kalmalyzer/testrunner-68k)

## HOW TO USE

* Write test code, as a number of "test_" prefixed labels/functions.
* Assemble/compile the test code into an Amiga executable with symbols present.
* Run the tests by doing: `testrunner-68k <executable>`

## OPEN QUESTIONS

* Logging? Printf macro + emulator hook?
* Assertions? Printf macro + emulator hook?
* Performance monitoring/constraints? Begin/end macro + emulator hook?

## IDEAS FOR THE FUTURE

* Select machine configuration - either per test-suite or per-test
* Software environment - raw vs OS/kickstart booted
	* stdout/stderr capture
	* file serving
	* floppy disk mounting

## DEVELOPING

### Requirements

* Install [Tundra 2.0](https://github.com/deplinenoise/tundra)
* Install [MSVC 2017](https://visualstudio.microsoft.com/vs/older-downloads/) (Windows) or GCC (Linux)
* Install [LLVM/Clang](http://releases.llvm.org/download.html) (Windows) or via apt-get (Linux)
* Install [Rust](https://www.rust-lang.org/tools/install)

### Build executable

* On Windows: ensure the 64-bit `cl.exe` is available on the command line
* `tundra2`
* `cargo test`
* `cargo build`
