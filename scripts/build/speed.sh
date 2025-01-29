#!/usr/bin/env bash
RUSTFLAGS="-Zlocation-detail=none -Zfmt-debug=none"
TARGET=""

if [ -n $1 ]; then
    TARGET="--target $1"
fi

if [ "$CPU_NATIVE" == "true" ]; then
    RUSTFLAGS="$RUSTFLAGS -C target-cpu=native"
fi

cargo +nightly build \
    $TARGET \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort \
    --profile release-speed