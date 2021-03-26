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

## Release 5 Breaking Changes
* The main entry point for use has changed from `gen` to `vergen`
* The old `ConstantsFlags` has been removed in lieu of `Config`.

## Environment Variables
A full list of environment variables that can be generated are listed in the following table.

| Variable | Sample |
| -------  | ------ |
| `VERGEN_BUILD_DATE` | 2021-02-25 |
| `VERGEN_BUILD_TIME` | 23:28:39.493201 |
| `VERGEN_BUILD_TIMESTAMP` | 2021-02-25T23:28:39.493201+00:00 |
| `VERGEN_BUILD_SEMVER` | 5.0.0 |
| `VERGEN_GIT_BRANCH` | feature/fun |
| `VERGEN_GIT_COMMIT_DATE` | 2021-02-24 |
| `VERGEN_GIT_COMMIT_TIME` | 20:55:21 |
| `VERGEN_GIT_COMMIT_TIMESTAMP` | 2021-02-24T20:55:21+00:00 |
| `VERGEN_GIT_SEMVER` | 5.0.0-2-gf49246c |
| `VERGEN_GIT_SHA` | f49246ce334567bff9f950bfd0f3078184a2738a |
| `VERGEN_RUSTC_CHANNEL` | nightly |
| `VERGEN_RUSTC_COMMIT_DATE` | 2021-02-24 |
| `VERGEN_RUSTC_COMMIT_HASH` | a8486b64b0c87dabd045453b6c81500015d122d6 |
| `VERGEN_RUSTC_HOST_TRIPLE` | x86_64-apple-darwin |
| `VERGEN_RUSTC_LLVM_VERSION` | 11.0 |
| `VERGEN_RUSTC_SEMVER` | 1.52.0-nightly |
| `VERGEN_CARGO_FEATURES` | git,build |
| `VERGEN_CARGO_PROFILE` | debug |
| `VERGEN_CARGO_TARGET_TRIPLE` | x86_64-unknown-linux-gnu |
| `VERGEN_SYSINFO_NAME` | Darwin |
| `VERGEN_SYSINFO_OS_VERSION` | MacOS 10.15.7 Catalina |
| `VERGEN_SYSINFO_USER` | Yoda |
| `VERGEN_SYSINFO_TOTAL_MEMORY` | 16 GB |
| `VERGEN_SYSINFO_CPU_VENDOR` | Intel(R) Core(TM) i7-7820HQ CPU @ 2.90GHz |
| `VERGEN_SYSINFO_CPU_CORE_COUNT` | 4 |

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
