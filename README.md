# vergen - Emit cargo instructions from a build script
`vergen`, when used in conjunction with cargo [build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html#outputs-of-the-build-script) can emit the following:

- Will emit [`cargo:rustc-env=VAR=VALUE`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-envvarvalue)
for each feature you have enabled.  These can be referenced with the [`env!`](https://doc.rust-lang.org/std/macro.env.html) macro in your code.
- Will emit [`cargo:rerun-if-changed=.git/HEAD`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed)
if the git feature is enabled.  This is done to ensure any git instructions are regenerated when commits are made.
- Will emit [`cargo:rerun-if-changed=.git/<path_to_ref>`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed)
if the git feature is enabled.  This is done to ensure any git instructions are regenerated when commits are made.
- Can emit [`cargo:warning`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargo-warning) outputs if the
[`fail_on_error`](EmitBuilder::fail_on_error) feature is not enabled and the requested variable is defaulted through error or
the [`idempotent`](EmitBuilder::idempotent) flag.
- Will emit [`cargo:rerun-if-changed=build.rs`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed)
to rerun instruction emission if the `build.rs` file changed.
- Will emit [`cargo:rurun-if-env-changed=VERGEN_IDEMPOTENT`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed)
to rerun instruction emission if the `VERGEN_IDEMPOTENT` environment variable has changed.
- Will emit [`cargo:rurun-if-env-changed=SOURCE_DATE_EPOCH`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed)
to rerun instruction emission if the `SOURCE_DATE_EPOCH` environment variable has changed.

## Current Release
[![docs.rs](https://docs.rs/vergen/badge.svg)](https://docs.rs/vergen)
[![Crates.io](https://img.shields.io/crates/v/vergen.svg)](https://crates.io/crates/vergen)
[![Crates.io](https://img.shields.io/crates/l/vergen.svg)](https://crates.io/crates/vergen)
[![Crates.io](https://img.shields.io/crates/d/vergen.svg)](https://crates.io/crates/vergen)
[![codecov](https://codecov.io/gh/rustyhorde/vergen/branch/master/graph/badge.svg?token=cBXro7o2UN)](https://codecov.io/gh/rustyhorde/vergen)
[![CI](https://github.com/rustyhorde/vergen/actions/workflows/main.yml/badge.svg)](https://github.com/rustyhorde/vergen/actions)
[![sponsor](https://img.shields.io/github/sponsors/crazysacx?logo=github-sponsors)](https://github.com/sponsors/CraZySacX)

## Sponsors
Special thanks to the sponsors of this project
* [tryretool](https://github.com/tryretool)

# ⚠️ This documention is for the version 8 beta ⚠️
If you wish to refer to version 7 you can find that branch [`here`](https://github.com/rustyhorde/vergen/tree/legacy/v7)

## MSRV
The current minimum supported rust version is 1.64.0 for non-Windows platforms
The current minimum supported rust version is 1.64.0 for Windows platforms

## Example Usage
See the documentation at [docs.rs](https://docs.rs/vergen/8.0.0-beta.0/vergen/index.html#usage) for example usage

## Notes about the optional `git2 0.15` dependency
This update to git2 picked up some [security related features](https://github.blog/2022-04-12-git-security-vulnerability-announced/).  In docker environments especially, this requires a `safe.directory` configuration.   There are a couple methods for achieving this.
1.  If you control the build, you can add `git config --global --add safe.directory /workspace` to the build file.
2.  If you do not control the docker build, you can add `git config --global --add safe.directory /workspace &&` before the actual command you are running when using docker run.
3.  If you do not control the docker build, you can mount a `.gitconfig` file at `/root` that includes the `safe.directory` configuration.  I use this method myself when building static binaries with clux/muslrust.

````docker run -v cargo-cache:/root/.cargo/registry -v (pwd):/volume -v ~/.gitconfig:/root/.gitconfig:ro --rm -t clux/muslrust:stable cargo build --release````

See https://github.com/rustyhorde/vergen/pull/126 for more discussion on the topic.   If the solutions above do not work for your usecase, you can pin your `vergen` version to 7.4.3.   Feel free to open issues about this.   If it comes up enough, I could support a version of `vergen` with the older `git2` dependency.

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
