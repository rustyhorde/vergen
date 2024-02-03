#!/usr/bin/env fish
cargo fmt --all -- --check; and \
./run_clippy.fish; and \
cargo build-all-features; and \
cargo test-all-features; and \
./run_code_cov.fish; and \
./run_docs.fish

