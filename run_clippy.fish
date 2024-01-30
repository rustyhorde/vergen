#!/usr/bin/env fish
cargo clippy -p test_util --all-targets -- -Dwarnings; and \
cargo clippy -p vergen --all-targets --features build,cargo,rustc,si -- -Dwarnings; and \
cargo clippy -p vergen-gix --all-targets --features build,cargo,rustc,si -- -Dwarnings; and \
cargo clippy -p vergen-git2 --all-targets --features build,cargo,rustc,si -- -Dwarnings; and \
cargo clippy -p vergen-gitcl --all-targets --features build,cargo,rustc,si -- -Dwarnings; and \
cargo clippy -p vergen-lib --all-targets --features build,cargo,git,rustc,si -- -Dwarnings; and \
cargo clippy -p vergen-pretty --all-targets --features color,header,trace -- -Dwarnings
