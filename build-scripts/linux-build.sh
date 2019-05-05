#!/bin/sh

# Build Musashi
tundra2 linux-gcc-debug-default linux-gcc-release-default

# Build & run testrunner-68k tests
cargo test

# Build testrunner-68k executable in debug config
cargo build

# Build testrunner-68k executable in release config
cargo build --release
