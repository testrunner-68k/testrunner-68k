
#Requires -RunAsAdministrator

try {

    # Install Tundra
    Write-Host "Ensuring that Tundra is installed..."
    if (!(Get-Command tundra2.exe -ErrorAction SilentlyContinue)) {
    try {
            # Fetch Tundra installer
            Write-Host "Downloading Tundra installer..."
            Invoke-WebRequest -Uri https://github.com/deplinenoise/tundra/releases/download/v2.09/Tundra-Setup.exe -OutFile Tundra-Setup.exe -ErrorAction Stop

            # Run Tundra installer
            Write-Host "Running Tundra installer..."
            $tundraExitCode = (Start-Process .\Tundra-Setup.exe -ArgumentList "/S" -Wait -PassThru -ErrorAction Stop).ExitCode
            if ($tundraExitCode -ne 0) { throw "Running Tundra installer failed with exit code ${tundraExitCode}" }

            # Add Tundra binary path to current install path (as the installer does not refresh the current shell's env vars)
            # TODO: use Update-SessionEnvironment instead of manually adding to the path
            Write-Host "Adding Tundra binary path to system PATH..."
            Set-Item ENV:Path ((Get-Item ENV:Path).Value + ";C:\Program Files\Tundra 2.0\bin")
        } finally {
            # Ensure Tundra-Setup.exe is removed at end of installation step
            if (Test-Path "Tundra-Setup.exe" -ErrorAction SilentlyContinue) {
                Write-Host "Removing Tundra installer executable..."
                Remove-Item "Tundra-Setup.exe"
            }
        }
    }

    # Setup VC environment variables. We assume that VS 2019 is already installed on the machine.
    Write-Host "Ensuring that VS 2019 is installed..."
    & $PSScriptRoot\Invoke-CmdScript.ps1 "${PSScriptRoot}\vcvars64_vs2019.bat"
    # Ensure Visual Studio command-line tools are available on the path
    if (!(Get-Command cl.exe -ErrorAction SilentlyContinue)) {
        throw "cl.exe not found. Please ensure Visual Studio has been correctly installed."
    }

    # Ensure LLVM/Clang is installed
    Write-Host "Ensuring that LLVM/Clang is installed..."
    if (!(Get-Command clang.exe -ErrorAction SilentlyContinue)) {
        throw "clang.exe not found. Please install LLVM/Clang from http://releases.llvm.org/download.html"
    }

    # Ensure Rust is installed
    Write-Host "Ensuring that Rust is installed..."
    if (!(Get-Command rustc.exe -ErrorAction SilentlyContinue)) {
        throw "rustc.exe not found. Please install Rust from https://www.rust-lang.org/tools/install"
    }

} catch {
    # Convert statement-terminating errors to script-terminating errors
    throw
}
