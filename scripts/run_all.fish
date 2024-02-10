#!/usr/bin/env fish
cargo fmt --all -- --check; and \
scripts/run_clippy.fish; and \
scripts/run_build.fish; and \
scripts/run_test.fish; and \
scripts/run_code_cov.fish; and \
scripts/run_docs.fish

