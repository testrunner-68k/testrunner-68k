
Param(
    [Parameter(Mandatory=$false)] [String] $Version
)

try {

    & $PSScriptRoot\Enable-Tls12.ps1

    & $PSScriptRoot\windows-install-build-tools.ps1
    & $PSScriptRoot\windows-install-package-tools.ps1
    & $PSScriptRoot\windows-build.ps1 $Version

} catch {
    # Convert statement-terminating errors to script-terminating errors
    throw
}