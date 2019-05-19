#!/bin/bash

VERSION="$1"

# Inject build ID into Cargo.toml, if it exists
if [[ -n "$VERSION" ]]; then
    sed -i -e "s/^version *= *\".*\"$/version = \"${VERSION}\"/" Cargo.toml
fi

# Build Musashi
tundra2 linux-gcc-debug-default linux-gcc-release-default

# Build & run testrunner-68k tests
cargo test

# Build testrunner-68k executable in debug config
cargo build

# Build testrunner-68k executable in release config
cargo build --release

if [[ -n "$VERSION" ]]; then
    # Package up application as .deb package
    cargo deb --no-build
    mkdir -p deploy
    cp target/debian/testrunner-68k_${VERSION}_amd64.deb deploy/
fi
