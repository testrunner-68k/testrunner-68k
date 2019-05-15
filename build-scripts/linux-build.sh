#!/bin/bash

BUILD_ID="$1"

# Inject build ID into Cargo.toml, if it exists
if [[ -n "$BUILD_ID" ]]; then
    sed -i -e "s/^version *= *\"0.0.0-localbuild\"$/version = \"0.0.0-build${BUILD_ID}\"/" Cargo.toml
fi

# Build Musashi
tundra2 linux-gcc-debug-default linux-gcc-release-default

# Build & run testrunner-68k tests
cargo test

# Build testrunner-68k executable in debug config
cargo build

# Build testrunner-68k executable in release config
cargo build --release
