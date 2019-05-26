#!/usr/bin/env bash

has() {
    type -p $1 > /dev/null
}

throw() {
    echo $1 1>&2
    exit 1
}

if ! has cc; then
    throw "A c/c++ compiler is required, install clang or gcc!"
fi

# Check dependencies
if ! has rustup; then
    throw "Rustup must be installed manaully. Go to https://rustup.rs/ and use rustup to install it."
fi

activeToolchain=`rustup show active-toolchain`
if [[ $activeToolchain != nightly-* ]]; then
    rustup default nightly
    if [ $? != 0 ]; then
        rustup update nightly
        if [ $? != 0 ]; then
            throw "Failed to install nightly rust"
        fi
        rustup default nightly
        if [ $? != 0 ]; then
            throw "Failed to enable nightly rust"
        fi
    fi
fi

if ! has cargo-xbuild; then
    cargo install cargo-xbuild
fi

if ! has bootimage; then
    cargo install bootimage --version "^0.5.0"
fi