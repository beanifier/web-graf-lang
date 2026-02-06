#!/bin/bash
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --out-dir web --target web target/wasm32-unknown-unknown/release/web_graf_lang.wasm
