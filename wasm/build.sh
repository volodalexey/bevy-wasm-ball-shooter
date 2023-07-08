#!/bin/sh

set -ex
# initial wasm build
cargo build --release --target wasm32-unknown-unknown
# prepare js module that loads wasm
mkdir -p target/wasm32-unknown-unknown/bindgen
wasm-bindgen --no-typescript --out-name wasm_bindgen_core --out-dir target/wasm32-unknown-unknown/bindgen/ --target web target/wasm32-unknown-unknown/release/bevy-wasm-ball-shooter.wasm

mkdir -p dist
# remove all files in dist folder except .git folder
find dist -mindepth 1 -not \( -path '*/.git' \) -not \( -path '*/.git/*' \) -exec rm -rf {} +
cp -r assets dist/assets
cp target/wasm32-unknown-unknown/bindgen/wasm_bindgen_core_bg.wasm dist/
cp target/wasm32-unknown-unknown/bindgen/wasm_bindgen_core.js dist/
cp wasm/index.html dist/
cp wasm/styles.css dist/
cp wasm/*.js dist/
ls -la dist
