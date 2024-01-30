# vergen - A suite of libraries for generating cargo instructions from a build script
### `vergen`, `vergen-git2`, `vergen-gitcl`, `vergen-gix`
When used in conjunction with cargo [build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html) can emit the following [output]((https://doc.rust-lang.org/cargo/reference/build-scripts.html#outputs-of-the-build-script)):

- Will emit [`cargo:rustc-env=VAR=VALUE`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-envvarvalue)
for each feature you have enabled.  These can be referenced with the [`env!`](https://doc.rust-lang.org/std/macro.env.html) or [`option_env!`](https://doc.rust-lang.org/std/macro.option_env.html) macro in your code.
- If using one of the git enabled libraries, will emit [`cargo:rerun-if-changed=.git/HEAD`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed).  This is done to ensure any git instructions are regenerated when commits are made.
- If using one of the git enabled libraries, will emit [`cargo:rerun-if-changed=.git/<path_to_ref>`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed).  This is done to ensure any git instructions are regenerated when commits are made.
- Can emit [`cargo:warning`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargo-warning) outputs if the
[`fail_on_error`](EmitBuilder::fail_on_error) feature is not enabled and the requested variable is defaulted through error or
the [`idempotent`](EmitBuilder::idempotent) flag.
- Will emit [`cargo:rerun-if-changed=build.rs`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed)
to rerun instruction emission if the `build.rs` file changed.
- Will emit [`cargo:rerun-if-env-changed=VERGEN_IDEMPOTENT`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed)
to rerun instruction emission if the `VERGEN_IDEMPOTENT` environment variable has changed.
- Will emit [`cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed)
to rerun instruction emission if the `SOURCE_DATE_EPOCH` environment variable has changed.

## Current Release
### vergen
[![docs.rs](https://docs.rs/vergen/badge.svg)](https://docs.rs/vergen)
[![Crates.io](https://img.shields.io/crates/v/vergen.svg)](https://crates.io/crates/vergen)
[![Crates.io](https://img.shields.io/crates/l/vergen.svg)](https://crates.io/crates/vergen)
[![Crates.io](https://img.shields.io/crates/d/vergen.svg)](https://crates.io/crates/vergen)
[![codecov](https://codecov.io/gh/rustyhorde/vergen/branch/master/graph/badge.svg?token=cBXro7o2UN)](https://codecov.io/gh/rustyhorde/vergen)
[![CI](https://github.com/rustyhorde/vergen/actions/workflows/main.yml/badge.svg)](https://github.com/rustyhorde/vergen/actions)
[![sponsor](https://img.shields.io/github/sponsors/crazysacx?logo=github-sponsors)](https://github.com/sponsors/CraZySacX)

## ⚠️ Notes on version 9 ⚠️
With version 9 comes the introduction of 3 new libraries, `vergen-git2`, `vergen-gitcl`, and `vergen-gix`.  Along with this change, the git features has been removed from the base `vergen` library.   The 3 new libraries are intended to be drop in replacements for
`vergen` when you need to generate git based cargo build script instructions.   `vergen` now contains the `build`, `cargo`, `rustc`, and `sysinfo` feature implementations.   These features are re-exported by the new libraries allowing you to configure the
output as you have previously.

Why?  This was done to resolve issues with [Cargo feature unification](https://doc.rust-lang.org/cargo/reference/features.html#mutually-exclusive-features) and mutually exclusive features.  Previous version of `vergen` had 3 mutually exclusive features (`git2`, `gitcl`, and `gix`).  Feature unification could cause compilation issues if you had included a dependency that also used `vergen` but had configured a different git feature.  Splitting the git backends into separate libraries helps alleviate this issue.

Version 9 also introduces the `AddCustomEntries` trait.  Implementing this trait allows you to include your own custom Cargo instructions, using `vergen` as the engine to generate them. See the [`AddCustomEntries`](https://docs.rs/vergen/latest/vergen/) docs for more information.

## MSRV
The current minimum supported rust version is 1.70.0

## Example Usage
See the documentation at [docs.rs](https://docs.rs/vergen/latest/vergen/) for example usage

## Contributing
See the documentation at [CONTRIBUTING.md](CONTRIBUTING.md)

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
