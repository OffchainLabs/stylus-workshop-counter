#!/bin/bash

MYDIR=$(realpath "$(dirname "$0")")
cd "$MYDIR" || exit
TEST_NODE_DIR="$MYDIR/../../nitro-testnode"
cd ../..

# Clone nitro-testnode
git clone --recurse-submodules https://github.com/OffchainLabs/nitro-testnode.git
cd ./nitro-testnode || exit
# `release` branch.
git checkout 8cb6b84e31909157d431e7e4af9fb83799443e00 || exit

# Initialize test node
./test-node.bash --no-run --init || exit

# Start with detached mode
./test-node.bash --detach
