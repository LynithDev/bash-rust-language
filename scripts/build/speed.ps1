$env:RUSTFLAGS="-Zlocation-detail=none -Zfmt-debug=none -C target-cpu=native"

cargo +nightly build \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort \
    --profile release-speed