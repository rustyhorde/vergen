#!/usr/bin/env fish
cargo matrix -c nightly clippy --all-targets -- -Dwarnings
