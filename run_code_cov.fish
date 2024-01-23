#!/usr/bin/env fish
cargo llvm-cov clean --workspace; and \
cargo llvm-cov -p vergen-lib -F unstable --no-report; and \
cargo llvm-cov -p vergen-lib -F unstable,build,cargo,git,rustc,si --no-report; and \
cargo llvm-cov -p vergen -F unstable --no-report; and \
cargo llvm-cov -p vergen -F unstable,build,cargo,rustc,si --no-report; and \
cargo llvm-cov -p vergen-git2 -F unstable --no-report; and \
cargo llvm-cov -p vergen-git2 -F unstable,build,cargo,rustc,si --no-report; and \
cargo llvm-cov -p vergen-gitcl -F unstable --no-report; and \
cargo llvm-cov -p vergen-gitcl -F unstable,build,cargo,rustc,si --no-report; and \
cargo llvm-cov -p vergen-gix -F unstable --no-report; and \
cargo llvm-cov -p vergen-gix -F unstable,build,cargo,rustc,si --no-report; and \
cargo llvm-cov -p vergen-pretty -F __vergen_test --no-report; and \
cargo llvm-cov -p vergen-pretty -F __vergen_test,color,header,serde,trace --no-report; and \
cargo llvm-cov report --lcov --output-path lcov.info; and \
cargo llvm-cov report --html
