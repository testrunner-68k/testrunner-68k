
#Requires -RunAsAdministrator

# Install cargo 'wix' subcommand
# TODO: Check whether or not we really need to update to the latest version on Travis machines
# If the subcommand is already installed, uninstall first; this ensures that the latest version is used
if ((& cargo install --list) -like '*cargo-wix*') {
   & cargo uninstall cargo-wix
   if ($LASTEXITCODE -ne 0) { throw "Running Cargo uninstaller for cargo-wix failed with exit code $LASTEXITCODE" }
}

if (!((& cargo install --list) -like '*cargo-wix*')) {
    if ($LASTEXITCODE -ne 0) { throw "Searching for cargo-wix failed with exit code ${LASTEXITCODE}" }
    & cargo install cargo-wix
    if ($LASTEXITCODE -ne 0) { throw "Running Cargo installer for cargo-wix failed with exit code ${LASTEXITCODE}" }
}

# Install .NET 3.5 Framework (needed by WiX Toolset)
if (Get-Command Install-WindowsFeature -ErrorAction SilentlyContinue) {
    # Install for Windows Server 2012R2
    if ((Get-WindowsFeature -Name Net-Framework-Core -ErrorAction Stop).InstallState -ne "Installed") {
        Install-WindowsFeature Net-Framework-Core -ErrorAction Stop
    }
} elseif (Get-Command Add-WindowsCapability -ErrorAction SilentlyContinue) {
    # Install for Windows 10 / Windows Server 2016
    if ((Get-WindowsCapability -Online -Name NetFx3~~~~ -ErrorAction Stop).State -ne "Installed") {
        Add-WindowsCapability -Online -Name NetFx3~~~~ -ErrorAction Stop
    }
}

# Install WiX Toolset
& cinst -y wixtoolset
if ($LASTEXITCODE -ne 0) { throw "Installing WiX Toolset failed with exit code $LASTEXITCODE" }

# Add WiX Toolset binary path to current install path (as the installer does not refresh the current shell's env vars)
& $PSScriptRoot\Invoke-CmdScript.ps1 "${PSScriptRoot}\set_environment_vars_for_wix_3.x.bat"

# Ensure 7-Zip is installed
if (!(Get-Command 7z.exe -ErrorAction SilentlyContinue)) {
    throw "7z.exe not found. Please install 7-Zip from https://www.7-zip.org"
}
