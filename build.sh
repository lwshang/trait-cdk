#!/usr/bin/env bash
set -e

cargo build --target wasm32-unknown-unknown \
    --release \
    --package "$1"

# ic-wasm "$example_root/target/wasm32-unknown-unknown/release/$package.wasm" \
#     -o "$example_root/target/wasm32-unknown-unknown/release/$package.wasm" \
#     metadata candid:service -v public -f $did_file

# ic-wasm "$example_root/target/wasm32-unknown-unknown/release/$package.wasm" \
#     -o "$example_root/target/wasm32-unknown-unknown/release/$package-opt.wasm" \
#     shrink
