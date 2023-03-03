#!/bin/sh
LIBRARY_NAME=libcodekit_core
## Run cbindgen
echo "Building headers ⚙️"
cd codekit-core
cargo clean
cbindgen --config cbindgen.toml  --output ../bindings/${LIBRARY_NAME}/include/CodeKit-Core.h


## iOS Build | Debug
cargo build --target=aarch64-apple-ios
cp ./target/aarch64-apple-ios/debug/${LIBRARY_NAME}.a ../bindings/${LIBRARY_NAME}/lib/iOS/debug/aarch64-apple-ios/${LIBRARY_NAME}.a

cargo build --target=aarch64-apple-ios-sim
cp ./target/aarch64-apple-ios-sim/debug/${LIBRARY_NAME}.a ../bindings/${LIBRARY_NAME}/lib/iOS/debug/aarch64-apple-ios-sim/${LIBRARY_NAME}.a
