#!/bin/sh
cd codekit-core
cargo build --target wasm32-unknown-unknown --features wasm-interface
wasm-pack build --target web --out-dir ../bindings/codekit-js --out-name codekit -- --features wasm-interface