@echo off

:start
call :GetVS16xDir
if "%VS16xDIR%"=="" goto error_no_VS16xDIR

call "%VS16xDIR%\VC\Auxiliary\Build\vcvars64.bat"

exit /B 0

REM -----------------------------------------------------------------------
:error_no_VS16xDIR
echo ERROR: Cannot determine the location of the VS2019 installation folder.
exit /B 1


REM -----------------------------------------------------------------------
:GetVS16xDir
set VS16xDIR=

REM Use vswhere.exe (included with VS2017 and newer) to locate a VS2019 installation directory.
REM Ideally this query should specify which VS components must be installed - the X64 C++ compiler, and perhaps something more
REM reference: https://github.com/microsoft/vswhere/wiki/Find-VC

for /f "usebackq tokens=*" %%i in (`"%ProgramFiles(x86)%\Microsoft Visual Studio\Installer\vswhere.exe" -version ^[16.0^,17.0^) -property installationPath`) do (
	set VS16xDir=%%i
)

if "%VS16xDir%"=="" exit /B 1

exit /B 0

:end