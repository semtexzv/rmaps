#!/usr/bin/env bash

set -eu

if [ ! -d "${ANDROID_SDK_ROOT-}" ]; then
    ANDROID_SDK_ROOT=/usr/local/share/android-sdk
fi
if [ ! -d "${ANDROID_HOME-}" ]; then
    ANDROID_HOME="$ANDROID_SDK_ROOT"
fi
if [ ! -d "${ANDROID_NDK_HOME-}" ]; then
    ANDROID_NDK_HOME="$ANDROID_HOME/ndk-bundle"
fi
MAKER="${ANDROID_NDK_HOME}/build/tools/make_standalone_toolchain.py"

if [ -x "$MAKER" ]; then
    echo 'Creating standalone NDK...'
else
    printf '\033[91;1mPlease install Android NDK!\033[0m\n\n'
    printf '  $ sdkmanager ndk-bundle\n\n'
    printf "\033[33;1mnote\033[0m: file \033[34;4m$MAKER\033[0m not found.\n"
    printf 'If you have installed the NDK in non-standard location, please define the \033[1m$ANDROID_NDK_HOME\033[0m variable.\n'
    exit 1
fi

DIR=$(pwd)
create_ndk() {
    echo "($1)..."
    mkdir -p $DIR/target/ndk/$1
    "$MAKER" --api "$2" --arch "$1" --force --install-dir "${DIR}/target/ndk/$1"
}

create_ndk arm64 21
create_ndk arm 14
create_ndk x86 14


echo 'Updating cargo-config.toml...'

mkdir -p .cargo
sed 's|$ROOT|'"${DIR}/target"'|g' ./scripts/cargo-config.toml.template > .cargo/config