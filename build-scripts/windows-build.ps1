
param([string] $BuildId = "local")

# Build Musashi in debug & release configurations
tundra2 win32-msvc-debug-default win32-msvc-release-default

# Build & run testrunner-68k tests
cargo test

# Build testrunner-68k executable in debug config
cargo build

# Build testrunner-68k executable in release config
cargo build --release

# Include testrunner-68k executable with deploy
if (Test-Path deploy) { rd -recurse deploy }
md deploy
7z a "deploy\testrunner-68k-$($BuildId)-windows-binaries.zip" .\target\release\testrunner-68k.exe

