#!/bin/bash

VERSION="$1"

# Inject build ID into Cargo.toml, if it exists
if [[ -n "$VERSION" ]]; then
    sed -i -e "s/^version *= *\".*\"$/version = \"${VERSION}\"/" Cargo.toml || exit 1
fi

# Build Musashi
tundra2 linux-gcc-debug-default linux-gcc-release-default || exit 1

# Build & run testrunner-68k tests
cargo test || exit 1

# Build testrunner-68k executable in debug config
cargo build || exit 1

# Build testrunner-68k executable in release config
cargo build --release || exit 1

if [[ -n "$VERSION" ]]; then
    # Package up application as .deb package
    cargo deb --no-build || exit 1
    mkdir -p deploy || exit 1
    cp target/debian/testrunner-68k_${VERSION}_amd64.deb deploy/ || exit 1
fi
