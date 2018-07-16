#!/usr/bin/env bash

usage() { echo "Usage: $0 arch: arm|arm64|x86 operation:(cargo operation) .. cargo flags" 1>&2; exit 1; }

ARCH=${1? "Missing architecture"}
OP=${2? "Missing cargo operation"}
REST="${@:3}"

PKG_CONFIG_ALLOW_CROSS=1

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




PATH="${ANDROID_NDK_HOME}/standalone/$ARCH/bin:$PATH"
TARGET_CC="$TRIPLE-clang"
TARGET_CXX="$TRIPLE-clang"
cargo $OP --manifest-path ./platform/android/Cargo.toml --target "$TRIPLE"