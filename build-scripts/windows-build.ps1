
Param(
    [Parameter(Mandatory=$false)] [String] $Version
)

# Inject build ID into Cargo.toml, if it has been specified
if ($Version -ne "")
{
    (Get-Content Cargo.toml -ErrorAction Stop) -replace '^version *= *".*"$', "version = `"${Version}`"" | Out-File -Encoding UTF8 Cargo.toml -ErrorAction Stop
}

# Build Musashi in debug & release configurations
& tundra2 win32-msvc-debug-default win32-msvc-release-default
if ($LASTEXITCODE -ne 0) { throw "Building Musashi failed with exit code $LASTEXITCODE" }

# Build & run testrunner-68k tests
& cargo test
if ($LASTEXITCODE -ne 0) { throw "Building/running testrunner-68k tests failed with exit code $LASTEXITCODE" }

# Build testrunner-68k executable in debug config
& cargo build
if ($LASTEXITCODE -ne 0) { throw "Building testrunner-68k in debug configuration failed with exit code $LASTEXITCODE" }

# Build testrunner-68k executable in release config
& cargo build --release
if ($LASTEXITCODE -ne 0) { throw "Building testrunner-68k in release configuration failed with exit code $LASTEXITCODE" }

if ($Version -ne "")
{
    if (Test-Path deploy) { Remove-Item -recurse deploy -ErrorAction Stop }
    mkdir deploy -ErrorAction Stop

    # Package up testrunner-68k windows binaries for deploy
    & 7z a deploy\testrunner-68k-${Version}-windows-binaries.zip .\target\release\testrunner-68k.exe
    if ($LASTEXITCODE -ne 0) { throw "Creating windows binaries zip archive failed with exit code $LASTEXITCODE" }

    # Create windows installer, and move to deploy
    & cargo wix
    Copy-Item target\wix\testrunner-68k-${Version}-x86_64.msi deploy -ErrorAction Stop
}
