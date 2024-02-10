#!/usr/bin/env fish
cargo llvm-cov clean --workspace; and \
cargo matrix -c llvm-cov -p vergen-lib --manifest-path vergen-lib/Cargo.toml llvm-cov --no-report; and \
cargo matrix -c nightly -p vergen --manifest-path vergen/Cargo.toml  llvm-cov --no-report; and \
cargo matrix -c nightly -p vergenl-git2 --manifest-path vergen-git2/Cargo.toml  llvm-cov --no-report; and \
cargo matrix -c nightly -p vergen-gitcl --manifest-path vergen-gitcl/Cargo.toml  llvm-cov --no-report; and \
cargo matrix -c nightly -p vergen-gix --manifest-path vergen-gix/Cargo.toml  llvm-cov --no-report; and \
cargo matrix -c nightly -p vergen-pretty --manifest-path vergen-pretty/Cargo.toml  llvm-cov --no-report; and \
cargo matrix -c nightly-empty -p vergen-pretty --manifest-path vergen-pretty/Cargo.toml  llvm-cov --no-report; and \
cargo llvm-cov report --lcov --output-path lcov.info; and \
cargo llvm-cov report --html
