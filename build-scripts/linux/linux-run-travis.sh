#!/bin/bash

set -e

VERSION="$1"

SCRIPT_DIRECTORY="$(dirname -- "$0")"

echo "**************** INSTALLING BUILD & PACKAGING TOOLS ****************"

${SCRIPT_DIRECTORY}/linux-install.sh

echo "*********************** BUILDING & PACKAGING ***********************"

${SCRIPT_DIRECTORY}/linux-build.sh "$VERSION"

echo "******************************* DONE *******************************"
