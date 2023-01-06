#!/usr/bin/env fish
cargo fmt --all -- --check; and \
cargo clippy -p vergen --all-targets --features build,cargo,git,gitcl,rustc,si -- -Dwarnings; and \
cargo clippy -p vergen --all-targets --features build,cargo,git,git2,rustc,si -- -Dwarnings; and \
cargo clippy -p vergen --all-targets --features build,cargo,git,gix,rustc,si -- -Dwarnings; and \
cargo clippy -p vergen-fmt --all-targets --all-features -- -Dwarnings; and \
cargo build-all-features; and \
cargo test-all-features; and \
cargo test -p vergen -F build,cargo,git,gitcl,rustc,si; and \
cargo test -p vergen -F build,cargo,git,git2,rustc,si; and \
cargo test -p vergen -F build,cargo,git,gix,rustc,si; and \
cargo test -p vergen-fmt -F __vergen_test; and \
cargo test -p vergen-fmt -F __vergen_test,color; and \
cargo test -p vergen-fmt -F __vergen_test,trace; and \
cargo test -p vergen-fmt -F __vergen_test,color,trace; and \
cargo doc -p vergen -F build,cargo,git,gitcl,rustc,si; and \
cargo doc -p vergen-fmt -F color,trace
