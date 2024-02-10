#!/usr/bin/env fish
cargo fmt --all -- --check; and \
cargo clippy -p vergen-pretty --all-targets --features color,header,trace -- -Dwarnings; and \
cd vergen-pretty; and \
cargo build-all-features -p vergen-pretty; and \
cargo test-all-features -p vergen-pretty; and \
cd ..; and \
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
cargo doc -p vergen-pretty -F color,header,trace
