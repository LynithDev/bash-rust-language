#!/usr/bin/env bash
RUSTFLAGS="-Zlocation-detail=none -Zfmt-debug=none"
TARGET=""

if [ ! -z "$1" ]; then
    TARGET="--target $1"
fi

cargo +nightly build \
    $TARGET \
    -Z build-std=std,panic_abort \
    -Z build-std-features="optimize_for_size" \
    -Z build-std-features=panic_immediate_abort \
    --profile release-size