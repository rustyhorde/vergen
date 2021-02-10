#!/bin/bash
set -ev

if [ "${TRAVIS_RUST_VERSION}" = "stable" ]; then
    cargo build
    cargo test
elif [ "${TRAVIS_RUST_VERSION}" = "beta" ]; then
    cargo build --features beta
    cargo test --features beta
else
    cargo build --features nightly
    cargo test --features nightly
fi
