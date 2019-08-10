
Param(
    [Parameter(Mandatory=$false)] [String] $Version
)

try {

    & $PSScriptRoot\Enable-Tls12.ps1

    Write-Host "********************** INSTALLING BUILD TOOLS **********************"

    & $PSScriptRoot\windows-install-build-tools.ps1

    Write-Host "******************** INSTALLING PACKAGING TOOLS ********************"

    & $PSScriptRoot\windows-install-package-tools.ps1

    Write-Host "*********************** BUILDING & PACKAGING ***********************"

    & $PSScriptRoot\windows-build.ps1 $Version

    Write-Host "******************************* DONE *******************************"
} catch {
    # Convert statement-terminating errors to script-terminating errors
    throw
}