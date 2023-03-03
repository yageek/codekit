#/bin/sh
 ## Reference: https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html
export NDK_HOME=$HOME/Library/Android/sdk/ndk/21.1.6352462/
mkdir NDK
$NDK_HOME/build/tools/make_standalone_toolchain.py --api 26 --arch arm64 --install-dir NDK/arm64
$NDK_HOME/build/tools/make_standalone_toolchain.py --api 26 --arch arm --install-dir NDK/arm
$NDK_HOME/build/tools/make_standalone_toolchain.py --api 26 --arch x86 --install-dir NDK/x86
$NDK_HOME/build/tools/make_standalone_toolchain.py --api 26 --arch x86_64 --install-dir NDK/x86_64