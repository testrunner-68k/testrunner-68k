
:start
@call :GetVS150Dir
@if "%VS150DIR%"=="" goto error_no_VS150DIR

@call "%VS150DIR%VC\Auxiliary\Build\vcvarsall.bat" amd64

@exit /B 0

@REM -----------------------------------------------------------------------
:error_no_VS150DIR
@echo ERROR: Cannot determine the location of the VS2017 installation folder.
@exit /B 1


@REM -----------------------------------------------------------------------
:GetVS150Dir
@set VS150DIR=
@call :GetVS2017DirHelper32 HKLM > nul 2>&1
@if errorlevel 1 call :GetVS150DirHelper32 HKCU > nul 2>&1
@if errorlevel 1 call :GetVS150DirHelper64  HKLM > nul 2>&1
@if errorlevel 1 call :GetVS150DirHelper64  HKCU > nul 2>&1
@exit /B 0

:GetVS150DirHelper32
@for /F "tokens=1,2*" %%i in ('reg query "%1\SOFTWARE\Microsoft\VisualStudio\SxS\VS7" /v "15.0"') DO (
	@if "%%i"=="15.0" (
		@SET VS150DIR=%%k
	)
)
@if "%VS150DIR%"=="" exit /B 1
@exit /B 0

:GetVS150DirHelper64
@for /F "tokens=1,2*" %%i in ('reg query "%1\SOFTWARE\Wow6432Node\Microsoft\VisualStudio\SxS\VS7" /v "15.0"') DO (
	@if "%%i"=="15.0" (
		@SET VS150DIR=%%k
	)
)
@if "%VS150DIR%"=="" exit /B 1
@exit /B 0


:end