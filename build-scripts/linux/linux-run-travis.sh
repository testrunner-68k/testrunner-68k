#!/bin/bash

set -e

VERSION="$1"

SCRIPT_DIRECTORY="$(dirname -- "$0")"

${SCRIPT_DIRECTORY}/linux-install.sh
${SCRIPT_DIRECTORY}/linux-build.sh "$VERSION"
