
# Test runner for Amiga/680x0 code [![Build Status](https://travis-ci.com/Kalmalyzer/testrunner-68k.svg?branch=master)](https://travis-ci.com/Kalmalyzer/testrunner-68k)

testrunner-68k is a test runner for 680x0 code: Compile/assemble your test code into an Amiga executable,
and use testrunner-68k to run the test code. The results will be printed in an easy-to-read format.

testrunner-68k includes a modified version of Musashi. It emulates a 68000 CPU. No FPU support.
No machine-specific hardware support. No OS support.

## INSTALLATION

### Linux (Ubuntu)

```bash
echo "deb https://testrunner-68k-apt.s3-eu-west-1.amazonaws.com stable main" | sudo tee /etc/apt/sources.list.d/testrunner-68k.list
wget https://testrunner-68k-apt.s3-eu-west-1.amazonaws.com/Release.key -O - | sudo apt-key add -
sudo apt-get update
sudo apt-get install testrunner-68k
```

### Windows

- Download and run the latest Windows installer from [the GitHub Releases page](https://github.com/Kalmalyzer/testrunner-68k/releases)

## HOW TO USE

* Write test code, as a number of "test_" prefixed labels/functions. See [the example repository](https://github.com/Kalmalyzer/testrunner-68k-example) for inspiration.
* Assemble/compile the test code into an Amiga executable with symbols present.
* Run the tests by doing: `testrunner-68k <executable>`

## IDEAS FOR THE FUTURE

* Logging? Printf macro + emulator hook?
* Assertions? Printf macro + emulator hook?
* Performance monitoring/constraints? Begin/end macro + emulator hook?
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

