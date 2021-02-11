#!/bin/bash
set -ev

cargo clean

if [ "${TRAVIS_RUST_VERSION}" = "stable" ]; then
    cargo build
    cargo test
    cargo test --no-default-features --features rustc
    cargo test --no-default-features --features rustc,build
    cargo test --no-default-features --features rustc,git
elif [ "${TRAVIS_RUST_VERSION}" = "beta" ]; then
    cargo build
    cargo test
    cargo test --no-default-features --features rustc
    cargo test --no-default-features --features rustc,build
    cargo test --no-default-features --features rustc,git
else
    cargo build
    cargo test
    cargo test --no-default-features --features rustc
    cargo test --no-default-features --features rustc,build
    cargo test --no-default-features --features rustc,git
fi
