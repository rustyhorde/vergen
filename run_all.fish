#!/usr/bin/env fish
cargo fmt --all -- --check; and \
cargo clippy -p test_util --all-targets -- -Dwarnings; and \
cargo clippy -p vergen --all-targets --features build,cargo,rustc,si -- -Dwarnings; and \
cargo clippy -p vergen-gix --all-targets --features build,cargo,rustc,si -- -Dwarnings; and \
cargo clippy -p vergen-lib --all-targets --features build,cargo,git,rustc,si -- -Dwarnings; and \
cargo clippy -p vergen-pretty --all-targets --features color,header,trace -- -Dwarnings; and \
cargo build-all-features; and \
cargo test-all-features; and \
cargo test -p vergen-pretty -F __vergen_test; and \
cargo test -p vergen-pretty -F __vergen_test,color; and \
# cargo test -p vergen-pretty -F __vergen_test,header; and \
cargo test -p vergen-pretty -F __vergen_test,trace; and \
cargo test -p vergen-pretty -F __vergen_test,serde; and \
# cargo test -p vergen-pretty -F __vergen_test,color,header; and \
cargo test -p vergen-pretty -F __vergen_test,color,serde; and \
cargo test -p vergen-pretty -F __vergen_test,color,trace; and \
# cargo test -p vergen-pretty -F __vergen_test,header,serde; and \
# cargo test -p vergen-pretty -F __vergen_test,header,trace; and \
cargo test -p vergen-pretty -F __vergen_test,serde,trace; and \
# cargo test -p vergen-pretty -F __vergen_test,color,header,serde; and \
# cargo test -p vergen-pretty -F __vergen_test,color,header,trace; and \
# cargo test -p vergen-pretty -F __vergen_test,header,serde,trace; and \
# cargo test -p vergen-pretty -F __vergen_test,color,header,serde,trace; and \
cargo doc -p vergen -F build,cargo,rustc,si; and \
cargo doc -p vergen-gix -F build,cargo,rustc,si; and \
cargo doc -p vergen-pretty -F color,header,trace; and \
cargo doc -p vergen-lib -F build,cargo,git,rustc,si; and \
cargo doc -p test_util -F repo
