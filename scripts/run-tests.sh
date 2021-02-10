#!/bin/bash
set -ev

cargo clean

if [ "${TRAVIS_RUST_VERSION}" = "stable" ]; then
    cargo build
    cargo test  --all-features
elif [ "${TRAVIS_RUST_VERSION}" = "beta" ]; then
    cargo build --features beta
    cargo test
else
    cargo build --features nightly
    cargo test
fi
