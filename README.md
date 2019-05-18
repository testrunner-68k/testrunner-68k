
# Test runner for Amiga/680x0 code [![Build Status](https://travis-ci.com/Kalmalyzer/testrunner-68k.svg?branch=master)](https://travis-ci.com/Kalmalyzer/testrunner-68k)

## INSTALLATION

### Linux (Ubuntu)

```bash
add-apt-repository 'deb [trusted=yes] https://testrunner-68k-apt.s3-eu-west-1.amazonaws.com stable main'
apt-get update
apt-get install testrunner-68k
```

### Windows

- Download and run the latest Windows installer from [the GitHub Releases page](https://github.com/Kalmalyzer/testrunner-68k/releases)

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

### Build & test

Windows (Command Prompt):
* Ensure the 64-bit `cl.exe` is available on the command line; run `build-scripts\vcvars64_vs2017.bat` if necessary
* `powershell build-scripts\windows-build.ps1`

Linux:
* `./build-scripts/linux-build.sh`

# Legal

testrunner-68k is licensed under the MIT license.

testrunner-68k makes use of Musashi, by Karl Stenerud. See [the License and Copyright section of Musashi's readme file - it is a BSD-style license](musashi/readme.txt).

