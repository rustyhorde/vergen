# vergen
Generate build, git, and rustc related 'cargo:rustc-env' instructions via 'build.rs' for use in your code via the env! macro

## Current Release
[![Crates.io](https://img.shields.io/crates/v/vergen.svg)](https://crates.io/crates/vergen)
[![Crates.io](https://img.shields.io/crates/l/vergen.svg)](https://crates.io/crates/vergen)
[![Crates.io](https://img.shields.io/crates/d/vergen.svg)](https://crates.io/crates/vergen)

## Release 4 Breaking Changes
* The main entry point for use has changed from `generate_cargo_keys` to `gen`
* There are now 4 features that allow you to control what instructions can be generated (`build`, `cargo`, `git`, and `rustc`).
    * The `build` feature enables the `VERGEN_BUILD_*` instructions.
    * The `git` feature enables the `VERGEN_GIT_*` instructions and the `cargo:rerun-if-changed` instructions.
    * The `rustc` feature enables the `VERGEN_RUSTC_*` instructions.
    * The `cargo` feature enables the `VERGEN_CARGO_*` instructions.
    * By default, all features are enabled.
    * You can build with all features disabled, which basically make the `gen` function a no-op.
    * You can still use `ConstantsFlags` to toggle individual cargo instructions.
* The generated instructions have been normalized.  Therefore, you may need to update what env variable you are referring to in code.  I've included a list below of the full set of instructions that can be generated for reference.

```text, no_run
cargo:rustc-env=VERGEN_BUILD_DATE=2021-02-12
cargo:rustc-env=VERGEN_BUILD_TIMESTAMP=2021-02-12T01:54:15.134750+00:00
cargo:rustc-env=VERGEN_GIT_BRANCH=feature/git2
cargo:rustc-env=VERGEN_GIT_COMMIT_DATE=2021-02-11T20:05:53-05:00
cargo:rustc-env=VERGEN_GIT_SEMVER=v3.2.0-86-g95fc0f5
cargo:rustc-env=VERGEN_GIT_SEMVER_LIGHTWEIGHT=blah-33-g95fc0f5
cargo:rustc-env=VERGEN_GIT_SHA=95fc0f5d066710f16e0c23ce3239d6e040abca0d
cargo:rustc-env=VERGEN_GIT_SHA_SHORT=95fc0f5
cargo:rustc-env=VERGEN_RUSTC_CHANNEL=nightly
cargo:rustc-env=VERGEN_RUSTC_COMMIT_DATE=2021-02-10
cargo:rustc-env=VERGEN_RUSTC_COMMIT_HASH=07194ffcd25b0871ce560b9f702e52db27ac9f77
cargo:rustc-env=VERGEN_RUSTC_HOST_TRIPLE=x86_64-apple-darwin
cargo:rustc-env=VERGEN_RUSTC_LLVM_VERSION=11.0
cargo:rustc-env=VERGEN_RUSTC_SEMVER=1.52.0-nightly
cargo:rustc-env=VERGEN_CARGO_TARGET_TRIPLE=x86_64-unknown-linux-gnu
cargo:rustc-env=VERGEN_CARGO_PROFILE=debug
cargo:rustc-env=VERGEN_CARGO_FEATURES=git,build
cargo:rerun-if-changed=/Users/yoda/projects/rust-lang/vergen/.git/HEAD
cargo:rerun-if-changed=/Users/yoda/projects/rust-lang/vergen/.git/refs/heads/feature/git2
```

* Under the hood, the `Command`s used for git have been removed in lieu of using the `git2` library directly.
* `git2` is also used to determine the `HEAD` path and the path that it refers to for the `cargo:rerun-if-changed` instructions.  This is more reliable then the manual method that was in place before.
* I've migrated the CI stuff from Travis to GitHub Actions.

## Build Status
![CI](https://github.com/rustyhorde/vergen/workflows/CI/badge.svg)

## Code Coverage
[![codecov](https://codecov.io/gh/rustyhorde/vergen/branch/master/graph/badge.svg?token=cBXro7o2UN)](https://codecov.io/gh/rustyhorde/vergen)

## Documentation
[![docs.rs](https://docs.rs/vergen/badge.svg)](https://docs.rs/vergen)

## Example Usage
See the documentation at [docs.rs](https://docs.rs/vergen/3.2.0/vergen/#cargo-key-build-script-output) for example usage

## Contributing
See the documentation at [CONTRIBUTING.md](CONTRIBUTING.md)

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
