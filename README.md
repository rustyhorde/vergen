# vergen
Generate build, git, and rustc related 'cargo:rustc-env' instructions via 'build.rs' for use in your code via the env! macro

## Current Release
[![docs.rs](https://docs.rs/vergen/badge.svg)](https://docs.rs/vergen)
[![Crates.io](https://img.shields.io/crates/v/vergen.svg)](https://crates.io/crates/vergen)
[![Crates.io](https://img.shields.io/crates/l/vergen.svg)](https://crates.io/crates/vergen)
[![Crates.io](https://img.shields.io/crates/d/vergen.svg)](https://crates.io/crates/vergen)
[![codecov](https://codecov.io/gh/rustyhorde/vergen/branch/master/graph/badge.svg?token=cBXro7o2UN)](https://codecov.io/gh/rustyhorde/vergen)
![CI](https://github.com/rustyhorde/vergen/actions/workflows/main.yml/badge.svg)

## Example Usage
See the documentation at [docs.rs](https://docs.rs/vergen) for example usage

## Release 4 Breaking Changes
* The main entry point for use has changed from `generate_cargo_keys` to `vergen`
* There are now 4 features that allow you to control what instructions can be generated (`build`, `cargo`, `git`, and `rustc`).
    * The `build` feature enables the `VERGEN_BUILD_*` instructions.
    * The `git` feature enables the `VERGEN_GIT_*` instructions and the `cargo:rerun-if-changed` instructions.
    * The `rustc` feature enables the `VERGEN_RUSTC_*` instructions.
    * The `cargo` feature enables the `VERGEN_CARGO_*` instructions.
    * By default, all features are enabled.
    * You can build with all features disabled, which basically make the `vergen` function a no-op.
    * You can still use `ConstantsFlags` with the `gen` function to toggle individual cargo instructions, but it has been deprecated.
* The generated instructions have been normalized.  Therefore, you may need to update what env variable you are referring to in code.  I've included a list below of the full set of instructions that can be generated for reference.

```text, no_run
cargo:rustc-env=VERGEN_BUILD_TIMESTAMP=2021-02-25T23:28:39.493201+00:00
cargo:rustc-env=VERGEN_BUILD_SEMVER=4.1.0
cargo:rustc-env=VERGEN_GIT_BRANCH=feature/datetime-toggles
cargo:rustc-env=VERGEN_GIT_COMMIT_TIMESTAMP=2021-02-24T20:55:21+00:00
cargo:rustc-env=VERGEN_GIT_SEMVER=4.1.0-2-gf49246c
cargo:rustc-env=VERGEN_GIT_SHA=f49246ce334567bff9f950bfd0f3078184a2738a
cargo:rustc-env=VERGEN_RUSTC_CHANNEL=nightly
cargo:rustc-env=VERGEN_RUSTC_COMMIT_DATE=2021-02-24
cargo:rustc-env=VERGEN_RUSTC_COMMIT_HASH=a8486b64b0c87dabd045453b6c81500015d122d6
cargo:rustc-env=VERGEN_RUSTC_HOST_TRIPLE=x86_64-apple-darwin
cargo:rustc-env=VERGEN_RUSTC_LLVM_VERSION=11.0
cargo:rustc-env=VERGEN_RUSTC_SEMVER=1.52.0-nightly
cargo:rustc-env=VERGEN_CARGO_TARGET_TRIPLE=x86_64-unknown-linux-gnu
cargo:rustc-env=VERGEN_CARGO_PROFILE=debug
cargo:rustc-env=VERGEN_CARGO_FEATURES=git,build
cargo:rerun-if-changed=/Users/kon8116/projects/rust-lang/vergen/.git/HEAD
cargo:rerun-if-changed=/Users/kon8116/projects/rust-lang/vergen/.git/refs/heads/feature/datetime-toggles
```

* Under the hood, the `Command`s used for git have been removed in lieu of using the `git2` library directly.
* `git2` is also used to determine the `HEAD` path and the path that it refers to for the `cargo:rerun-if-changed` instructions.  This is more reliable then the manual method that was in place before.
* I've migrated the CI stuff from Travis to GitHub Actions.

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
