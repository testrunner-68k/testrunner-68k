
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
