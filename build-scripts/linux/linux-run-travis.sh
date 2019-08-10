#!/bin/bash

set -e

VERSION="$1"

./linux-install.sh
./linux-build.sh "$VERSION"
