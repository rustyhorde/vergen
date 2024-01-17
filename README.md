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
- Will emit [`cargo:rerun-if-env-changed=VERGEN_IDEMPOTENT`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed)
to rerun instruction emission if the `VERGEN_IDEMPOTENT` environment variable has changed.
- Will emit [`cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed)
to rerun instruction emission if the `SOURCE_DATE_EPOCH` environment variable has changed.

## Current Release
[![docs.rs](https://docs.rs/vergen/badge.svg)](https://docs.rs/vergen)
[![Crates.io](https://img.shields.io/crates/v/vergen.svg)](https://crates.io/crates/vergen)
[![Crates.io](https://img.shields.io/crates/l/vergen.svg)](https://crates.io/crates/vergen)
[![Crates.io](https://img.shields.io/crates/d/vergen.svg)](https://crates.io/crates/vergen)
[![codecov](https://codecov.io/gh/rustyhorde/vergen/branch/master/graph/badge.svg?token=cBXro7o2UN)](https://codecov.io/gh/rustyhorde/vergen)
[![CI](https://github.com/rustyhorde/vergen/actions/workflows/main.yml/badge.svg)](https://github.com/rustyhorde/vergen/actions)
[![sponsor](https://img.shields.io/github/sponsors/crazysacx?logo=github-sponsors)](https://github.com/sponsors/CraZySacX)

## MSRV
The current minimum supported rust version is 1.70.0

## Example Usage
See the documentation at [docs.rs](https://docs.rs/vergen/latest/vergen/) for example usage

## `Cargo` feature unification for `vergen` versions prior to 8.3.0
When a dependency is used by multiple packages, Cargo will [use the union](https://doc.rust-lang.org/cargo/reference/features.html#feature-unification) of all features enabled on that dependency when building it.  Prior to version **8.3.0**, `vergen` had a set of mutually exclusive features `gitcl`, `git2`, and `gitoxide` to enable to specific git backend you wished to use.  If your crate has a dependency on another crate that uses `vergen`, your crate may fail to compile if you select a different `git` backend then the crate you depend on.  For example, your crate depends on `fancy-lib`.   

#### fancy-lib `Cargo.toml`
```toml
[build-dependencies]
vergen = { version = "8.2.10", features = ["git","gitcl"] }
```

#### your crate `Cargo.toml`
```toml
[dependencies]
fancy-lib = "0.1.0"

[build-dependencies]
vergen = { version = "8.2.10", features = ["git","gitoxide"] }
```

Your crate will fail to compile because `cargo` unifies this to
```toml
vergen = { version = "8.2.10", features = ["git","gitcl","gitoxide"] }
```
and prior to **8.3.0** `vergen` will not compile with both `gitcl` and `gitoxide` as features.

As a workaround, you can use `cargo tree -f "{p} {f}" | grep vergen` to determine the feature list cargo has set for `vergen`.  If
a `git` backend has already been determined you will be able to use that without declaring those features in your dependency list.  This is not perfect as this leaves you at the mercy of your dependency and the git feature they selected, but it's a workaround until version 9 comes out.

#### fancy-lib `Cargo.toml`
```toml
[build-dependencies]
vergen = { version = "8.2.10", features = ["git","gitcl"] }
```

#### your crate `Cargo.toml`
```toml
[dependencies]
fancy-lib = "0.1.0"

[build-dependencies]
vergen = "8.2.10"
```
#### Unified
```toml
vergen = { version = "8.2.10", features = ["git","gitcl"] }
```
## `Cargo` feature unification for `vergen` versions 8.3.0 and beyond
`vergen` will accept `gitcl`,`git2`, and `gitoxide` as features.  If more than one of them is included, `vergen` will select `gitcl` before `git2` and `git2` before `gitoxide`.

## Notes about the optional `git2` dependency
`git2` picked up some [security related features](https://github.blog/2022-04-12-git-security-vulnerability-announced/).  In docker environments especially, this requires a `safe.directory` configuration.   There are a couple methods for achieving this.
1.  If you control the build, you can add `git config --global --add safe.directory /workspace` to the build file.
2.  If you do not control the docker build, you can add `git config --global --add safe.directory /workspace &&` before the actual command you are running when using docker run.
3.  If you do not control the docker build, you can mount a `.gitconfig` file at `/root` that includes the `safe.directory` configuration.  I use this method myself when building static binaries with clux/muslrust.

````docker run -v cargo-cache:/root/.cargo/registry -v (pwd):/volume -v ~/.gitconfig:/root/.gitconfig:ro --rm -t clux/muslrust:stable cargo build --release````

See https://github.com/rustyhorde/vergen/pull/126 for more discussion on the topic.   If the solutions above do not work for your usecase, you can pin your `vergen` version to 7.4.3, with the caveat you will be exposed to the issues described at the link above.

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
