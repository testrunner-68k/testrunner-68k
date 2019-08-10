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
if [[ -z `cargo install --list | grep "cargo-deb"` ]]
then
    cargo install cargo-deb
fi
