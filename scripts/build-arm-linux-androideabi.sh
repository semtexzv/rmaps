#!/usr/bin/env bash

export OPENSSL_DIR
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

export PATH="${ANDROID_NDK_HOME}/standalone/arm/bin:$PATH"
export TARGET_CC=arm-linux-androideabi-clang
export TARGET_CXX=arm-linux-androideabi-clang++
cargo build --manifest-path ./platform/android/Cargo.toml --target armv7-linux-androideabi