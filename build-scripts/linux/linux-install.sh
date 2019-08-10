#!/bin/bash

set -e

# Ensure that Tundra is installed
if ! command -v tundra2 > /dev/null
then
    # Build Tundra from source & install
    git clone --recursive https://github.com/deplinenoise/tundra.git
    cd tundra
    make
    sudo make install
    cd ..
fi

# Install cargo 'deb' subcommand
# If the subcommand is already installed, uninstall first; this ensures that the latest version is used
#if [[ -n `cargo install --list | grep "cargo-deb"` ]]; then cargo uninstall cargo-deb; fi
cargo install --list
cargo install cargo-deb
