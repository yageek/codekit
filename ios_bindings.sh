#!/bin/sh
LIBRARY_NAME=libcodekit_core

## See https://developer.apple.com/forums/thread/711294 for information related to platforms

## Run cbindgen
echo "Building headers ⚙️"
cd codekit-core
cbindgen --config cbindgen.toml  --output ../bindings/${LIBRARY_NAME}/include/CodeKit-Core.h

## iOS Build | Debug
if [  "$1" == "release" ]
then
    echo "Building release"
    cargo build --release --target=aarch64-apple-ios --features ffi-interface
    # iOS Simulator
    cargo build --release --target=aarch64-apple-ios-sim --features ffi-interface
    # macOS
    cargo build --release --target=aarch64-apple-darwin --features ffi-interface
    cargo build --release --target=x86_64-apple-darwin --features ffi-interface
    
    mkdir -p target/universal-apple-darwin/release
    lipo -create -output ./target/universal-apple-darwin/release/${LIBRARY_NAME}.a ./target/x86_64-apple-darwin/release/${LIBRARY_NAME}.a ./target/aarch64-apple-darwin/release/${LIBRARY_NAME}.a

    # Create Apple Silicon based version
    xcodebuild -create-xcframework -output ../bindings/${LIBRARY_NAME}/lib/Apple/release/CodeKitCore.xcframework \
            -library ./target/aarch64-apple-ios/release/${LIBRARY_NAME}.a \
            -headers ../bindings/libcodekit_core/include \
            -library ./target/aarch64-apple-ios-sim/release/${LIBRARY_NAME}.a \
            -headers ../bindings/libcodekit_core/include \
            -library ./target/universal-apple-darwin/release/${LIBRARY_NAME}.a \
            -headers ../bindings/libcodekit_core/include
else
    echo "Building debug"
    
    cargo build --target=aarch64-apple-ios --features ffi-interface

    cargo build --target=aarch64-apple-ios-sim --features ffi-interface

    cargo build --target=aarch64-apple-darwin --features ffi-interface
    cargo build --target=x86_64-apple-darwin --features ffi-interface
    
    # Now we need to use lipo to merge the two libraries for macOS
    mkdir -p target/universal-apple-darwin/debug
    lipo -create -output ./target/universal-apple-darwin/debug/${LIBRARY_NAME}.a ./target/x86_64-apple-darwin/debug/${LIBRARY_NAME}.a ./target/aarch64-apple-darwin/debug/${LIBRARY_NAME}.a
    # Apple Silicon
    xcodebuild -create-xcframework -output ../bindings/${LIBRARY_NAME}/lib/Apple/debug/CodeKitCore.xcframework \
                -library ./target/aarch64-apple-ios/debug/${LIBRARY_NAME}.a \
                -headers ../bindings/libcodekit_core/include \
                -library ./target/aarch64-apple-ios-sim/debug/${LIBRARY_NAME}.a \
                -headers ../bindings/libcodekit_core/include \
                -library ./target/universal-apple-darwin/debug/${LIBRARY_NAME}.a \
                -headers ../bindings/libcodekit_core/include
fi
