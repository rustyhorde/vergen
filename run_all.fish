#!/usr/bin/env fish
cargo fmt --all -- --check; and \
cargo clippy --all-targets --features build,cargo,git,gitcl,rustc,si -- -Dwarnings; and \
cargo clippy --all-targets --features build,cargo,git,git2,rustc,si -- -Dwarnings; and \
cargo clippy --all-targets --features build,cargo,git,gix,rustc,si -- -Dwarnings; and \
cargo build-all-features; and \
cargo test-all-features; and \
cargo test -F build,cargo,git,gitcl,rustc,si; and \
cargo test -F build,cargo,git,git2,rustc,si; and \
cargo test -F build,cargo,git,gix,rustc,si; and \
cargo doc -F build,cargo,git,gitcl,rustc,si
