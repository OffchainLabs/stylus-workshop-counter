#!/bin/bash

# Build wasm32-unknown-unknown binary
cargo build --release --target wasm32-unknown-unknown

# Run tests
export RPC_URL=http://localhost:8547
cargo test --locked --test "integration_tests"