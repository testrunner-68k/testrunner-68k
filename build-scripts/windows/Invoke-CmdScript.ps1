
# Invoke an executable or a batch script, and apply resulting environment changes to the PowerShell environment

param(
    [String] $scriptName
)

$cmdLine = """$scriptName"" $args & set"
$environment = & cmd.exe /c $cmdLine

$environment | Select-String '^([^=]*)=(.*)$' | ForEach-Object {
    $varName = $_.Matches[0].Groups[1].Value
    $varValue = $_.Matches[0].Groups[2].Value
    Set-Item Env:$varName $varValue
}
