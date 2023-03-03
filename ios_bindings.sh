#!/bin/sh
LIBRARY_NAME=libcodekit_core

## Run cbindgen
echo "Building headers ⚙️"
cd codekit-core
cbindgen --config cbindgen.toml  --output ../bindings/${LIBRARY_NAME}/include/CodeKit-Core.h


## iOS Build | Debug
if [  "$1" == "release" ]
then
    echo "Building release"
    cargo build --release --target=aarch64-apple-ios
    cp ./target/aarch64-apple-ios/release/${LIBRARY_NAME}.a ../bindings/${LIBRARY_NAME}/lib/iOS/release/aarch64-apple-ios/${LIBRARY_NAME}.a

    cargo build --release --target=x86_64-apple-ios
    cp ./target/x86_64-apple-ios/release/${LIBRARY_NAME}.a ../bindings/${LIBRARY_NAME}/lib/iOS/release/x86_64-apple-ios/${LIBRARY_NAME}.a

    cargo build --release --target=aarch64-apple-ios-sim
    cp ./target/aarch64-apple-ios-sim/release/${LIBRARY_NAME}.a ../bindings/${LIBRARY_NAME}/lib/iOS/release/aarch64-apple-ios-sim/${LIBRARY_NAME}.a

    # Create Apple Silicon based version
    xcodebuild -create-xcframework -output ../bindings/${LIBRARY_NAME}/lib/iOS/release/AppleSilicon/CodeKitCore.xcframework \
            -library ./target/aarch64-apple-ios/release/${LIBRARY_NAME}.a \
            -headers ../bindings/libcodekit_core/include \
            -library ./target/aarch64-apple-ios-sim/release/${LIBRARY_NAME}.a \
            -headers ../bindings/libcodekit_core/include

    # Create Intel based version
    xcodebuild -create-xcframework -output ../bindings/${LIBRARY_NAME}/lib/iOS/release/Intel/CodeKitCore.xcframework \
        -library ./target/aarch64-apple-ios/release/${LIBRARY_NAME}.a \
        -headers ../bindings/libcodekit_core/include \
        -library ./target/x86_64-apple-ios/release/${LIBRARY_NAME}.a \
        -headers ../bindings/libcodekit_core/include
else
    echo "Building debug"
    cargo build --target=aarch64-apple-ios
    cp ./target/aarch64-apple-ios/debug/${LIBRARY_NAME}.a ../bindings/${LIBRARY_NAME}/lib/iOS/debug/aarch64-apple-ios/${LIBRARY_NAME}.a

    cargo build --target=x86_64-apple-ios
    cp ./target/x86_64-apple-ios/debug/${LIBRARY_NAME}.a ../bindings/${LIBRARY_NAME}/lib/iOS/debug/x86_64-apple-ios/${LIBRARY_NAME}.a

    cargo build --target=aarch64-apple-ios-sim
    cp ./target/aarch64-apple-ios-sim/debug/${LIBRARY_NAME}.a ../bindings/${LIBRARY_NAME}/lib/iOS/debug/aarch64-apple-ios-sim/${LIBRARY_NAME}.a

    # Apple Silicon
    xcodebuild -create-xcframework -output ../bindings/${LIBRARY_NAME}/lib/iOS/debug/AppleSilicon/CodeKitCore.xcframework \
                -library ./target/aarch64-apple-ios/debug/${LIBRARY_NAME}.a \
                -headers ../bindings/libcodekit_core/include \
                -library ./target/aarch64-apple-ios-sim/debug/${LIBRARY_NAME}.a \
                -headers ../bindings/libcodekit_core/include

    # Intel
    xcodebuild -create-xcframework -output ../bindings/${LIBRARY_NAME}/lib/iOS/debug/Intel/CodeKitCore.xcframework \
                -library ./target/aarch64-apple-ios/debug/${LIBRARY_NAME}.a \
                -headers ../bindings/libcodekit_core/include \
                -library ./target/x86_64-apple-ios/debug/${LIBRARY_NAME}.a \
                -headers ../bindings/libcodekit_core/include
fi
