#!/bin/sh
set -x
export LIBRARY_NAME=libcodekit_core
export JNILIBS_DIR=../bindings/${LIBRARY_NAME}/android/jniLibs
cd codekit-core
# Arm 64
cargo build --lib -p codekit-core --features android --target aarch64-linux-android
cp ./target/aarch64-linux-android/debug/${LIBRARY_NAME}.so ${JNILIBS_DIR}/arm64-v8a/${LIBRARY_NAME}.so

# Armv7
cargo build --lib -p codekit-core --features android  --target armv7-linux-androideabi
cp ./target/armv7-linux-androideabi/debug/${LIBRARY_NAME}.so ${JNILIBS_DIR}/armeabi-v7a/${LIBRARY_NAME}.so

# # x86
cargo build --lib -p codekit-core --features android  --target i686-linux-android
cp ./target/i686-linux-android/debug/${LIBRARY_NAME}.so ${JNILIBS_DIR}/x86/${LIBRARY_NAME}.so

# x86_64
cargo build --lib -p codekit-core --features android --target x86_64-linux-android
cp ./target/x86_64-linux-android/debug/${LIBRARY_NAME}.so ${JNILIBS_DIR}/x86_64/${LIBRARY_NAME}.so


## Copy libraries to android
cd ..
 cp -R bindings/${LIBRARY_NAME}/android/jniLibs/ bindings/CodeKitAndroid/app/codekit/src/main/jniLibs  