
# Build Musashi in debug & release configurations
tundra2 win32-msvc-debug-default win32-msvc-release-default
if ($LASTEXITCODE -ne 0) { throw "Building Musashi failed with exit code $LASTEXITCODE" }

# Build & run testrunner-68k tests
cargo test
if ($LASTEXITCODE -ne 0) { throw "Building/running testrunner-68k tests failed with exit code $LASTEXITCODE" }

# Build testrunner-68k executable in debug config
cargo build
if ($LASTEXITCODE -ne 0) { throw "Building testrunner-68k in debug configuration failed with exit code $LASTEXITCODE" }

# Build testrunner-68k executable in release config
cargo build --release
if ($LASTEXITCODE -ne 0) { throw "Building testrunner-68k in release configuration failed with exit code $LASTEXITCODE" }

# Include testrunner-68k executable with deploy
if (Test-Path deploy) { rd -recurse deploy }
md deploy
copy target\release\testrunner-68k.exe deploy
