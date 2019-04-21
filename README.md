
## HOW TO USE

runtests \<executable\>

* Load executable
* Scan symbol table for "test_" prefixed names
* For each test:
	- initialize 68k core
	- inject code into 68k core
	- call "test_" prefix location
	- wait (timeout in cycles?)
	- check return code to determine pass/fail
	- capture exceptions -> fail
* Output results go either as text-output or as XML output (NUnit/JUnit/XUnit/...)

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
