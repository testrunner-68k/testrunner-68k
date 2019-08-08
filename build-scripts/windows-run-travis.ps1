
Param(
    [Parameter(Mandatory=$false)] [String] $Version
)

# Print each line of PowerShell script before it is executed
Set-PSDebug -Trace 1

& $PSScriptRoot\Enable-Tls12.ps1

& $PSScriptRoot\windows-install-build-tools.ps1
& $PSScriptRoot\windows-install-package-tools.ps1
& $PSScriptRoot\windows-build.ps1 $Version
