@echo off

REM Adds WiX Toolset 3.x binary folder to path, so that candle.exe & light.exe can be invoked

set WIX_INSTALL_DIR=
for /f "tokens=1,2*" %%i in ('reg query "HKLM\SOFTWARE\Microsoft\Windows Installer XML\3.x" /reg:32') DO (
	if [%%j]==[REG_SZ] (
 		set WIX_INSTALL_DIR=%%k
	)
)

if "[%WIX_INSTALL_DIR%]" == "[]" (
	echo WiX 3.x installation folder not found
	EXIT /B 1
)

echo Adding %WIX_INSTALL_DIR%bin to path

path %path%;%WIX_INSTALL_DIR%bin

EXIT /B 0