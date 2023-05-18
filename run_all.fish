#!/usr/bin/env fish
cargo fmt --all -- --check; and \
cargo clippy -p vergen --all-targets --features build,cargo,git,gitcl,rustc,si -- -Dwarnings; and \
cargo clippy -p vergen --all-targets --features build,cargo,git,git2,rustc,si -- -Dwarnings; and \
cargo clippy -p vergen --all-targets --features build,cargo,git,gitoxide,rustc,si -- -Dwarnings; and \
cargo clippy -p vergen-pretty --all-targets --features color,header,trace -- -Dwarnings; and \
cargo build-all-features; and \
cargo test-all-features; and \
cargo test -p vergen -F build,cargo,git,gitcl,rustc,si; and \
cargo test -p vergen -F build,cargo,git,git2,rustc,si; and \
cargo test -p vergen -F build,cargo,git,gitoxide,rustc,si; and \
cargo test -p vergen-pretty -F __vergen_test; and \
cargo test -p vergen-pretty -F __vergen_test,color; and \
cargo test -p vergen-pretty -F __vergen_test,header; and \
cargo test -p vergen-pretty -F __vergen_test,trace; and \
cargo test -p vergen-pretty -F __vergen_test,serde; and \
cargo test -p vergen-pretty -F __vergen_test,color,header; and \
cargo test -p vergen-pretty -F __vergen_test,color,serde; and \
cargo test -p vergen-pretty -F __vergen_test,color,trace; and \
cargo test -p vergen-pretty -F __vergen_test,header,serde; and \
cargo test -p vergen-pretty -F __vergen_test,header,trace; and \
cargo test -p vergen-pretty -F __vergen_test,serde,trace; and \
cargo test -p vergen-pretty -F __vergen_test,color,header,serde; and \
cargo test -p vergen-pretty -F __vergen_test,color,header,trace; and \
cargo test -p vergen-pretty -F __vergen_test,header,serde,trace; and \
cargo test -p vergen-pretty -F __vergen_test,color,header,serde,trace; and \
cargo doc -p vergen -F build,cargo,git,gitcl,rustc,si; and \
cargo doc -p vergen-pretty -F color,header,trace
