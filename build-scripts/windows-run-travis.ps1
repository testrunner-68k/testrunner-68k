
Param(
    [Parameter(Mandatory=$false)] [String] $Version
)

& $PSScriptRoot\windows-install-build-tools.ps1
& $PSScriptRoot\windows-install-package-tools.ps1
& $PSScriptRoot\windows-build.ps1 $Version
