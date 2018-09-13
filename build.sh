#!/usr/bin/env bash

usage() { echo "Usage: $0 arch: arm|arm64|x86 operation:(cargo operation) .. cargo flags" 1>&2; exit 1; }

ARCH=${1? "Missing architecture"}
OP=${2? "Missing cargo operation"}
REST="${@:3}"

export PKG_CONFIG_ALLOW_CROSS=1

if [ ! -d "${ANDROID_SDK_ROOT-}" ]; then
    ANDROID_SDK_ROOT=/usr/local/share/android-sdk
fi
if [ ! -d "${ANDROID_HOME-}" ]; then
    ANDROID_HOME="$ANDROID_SDK_ROOT"
fi
if [ ! -d "${ANDROID_NDK_HOME-}" ]; then
    ANDROID_NDK_HOME="$ANDROID_HOME/ndk-bundle"
fi

if [ "$ARCH" = "arm" ]; then
    TRIPLE="arm-linux-androideabi"
fi

if [ "$ARCH" = "arm64" ]; then
    TRIPLE="aarch64-linux-android"
fi

if [ "$ARCH" = "x86" ]; then
    TRIPLE="i686-linux-android"
fi

if [ "$ARCH" = "x86_64" ]; then
    TRIPLE="x86_64-linux-android"
fi



export OPENSSL_DIR="/home/semtexzv/openssl/built"
export OPENSSL_LIB_DIR="${OPENSSL_DIR}/lib"
export OPENSSL_INCLUDE_DIR="${OPENSSL_DIR}/include"

export PATH="$(pwd)/target/ndk/$ARCH/bin:$PATH"
export TARGET_CC="$TRIPLE-clang"
export TARGET_CXX="$TRIPLE-clang++"
export TARGET_CFLAGS="-Wno-macro-redefined"
cargo $OP --manifest-path ./platform/android/Cargo.toml --target "$TRIPLE"