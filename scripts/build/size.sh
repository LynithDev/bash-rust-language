#!/usr/bin/env bash
RUSTFLAGS="-Zlocation-detail=none -Zfmt-debug=none"
cargo +nightly build \
    $1 \
    -Z build-std=std,panic_abort \
    -Z build-std-features="optimize_for_size" \
    -Z build-std-features=panic_immediate_abort \
    --profile release-size