#!/usr/bin/env bash
set -e

cargo build --target wasm32-unknown-unknown \
    --release \
    --package "$1"
