
#Requires -RunAsAdministrator

# Install Tundra
if (!(Get-Command tundra2.exe -ErrorAction SilentlyContinue)) {
    try {
        # Fetch Tundra installer
        Invoke-WebRequest -Uri https://github.com/deplinenoise/tundra/releases/download/v2.09/Tundra-Setup.exe -OutFile Tundra-Setup.exe -ErrorAction Stop

        # Run Tundra installer
        $tundraExitCode = (Start-Process .\Tundra-Setup.exe -ArgumentList "/S" -Wait -PassThru -ErrorAction Stop).ExitCode
        if ($tundraExitCode -ne 0) { throw "Running Tundra installer failed with exit code ${tundraExitCode}" }

        # Add Tundra binary path to current install path (as the installer does not refresh the current shell's env vars)
        # TODO: use Update-SessionEnvironment instead of manually adding to the path
        Set-Item ENV:Path ((Get-Item ENV:Path).Value + ";C:\Program Files\Tundra 2.0\bin")

    } finally {
        # Ensure Tundra-Setup.exe is removed at end of installation step
        if (Test-Path "Tundra-Setup.exe" -ErrorAction SilentlyContinue) {
            Remove-Item "Tundra-Setup.exe" -ErrorAction Stop
        }
    }
}

# Setup VC environment variables. We assume that VS 2017 is already installed on the machine.
& $PSScriptRoot\Invoke-CmdScript.ps1 "${PSScriptRoot}\vcvars64_vs2017.bat"
# Ensure Visual Studio command-line tools are available on the path
if (!(Get-Command cl.exe -ErrorAction SilentlyContinue)) {
    throw "cl.exe not found. Please ensure Visual Studio has been correctly installed."
}

# Ensure LLVM/Clang is installed
if (!(Get-Command clang.exe -ErrorAction SilentlyContinue)) {
    throw "clang.exe not found. Please install LLVM/Clang from http://releases.llvm.org/download.html"
}

# Ensure Rust is installed
if (!(Get-Command rustc.exe -ErrorAction SilentlyContinue)) {
    throw "rustc.exe not found. Please install Rust from https://www.rust-lang.org/tools/install"
}
