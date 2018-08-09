# vergen
## Version
[![Crates.io](https://img.shields.io/crates/v/vergen.svg)](https://crates.io/crates/vergen)
[![Build
Status](https://travis-ci.org/rustyhorde/vergen.svg?branch=master)](https://travis-ci.org/rustyhorde/vergen)

## Basic Usage
The following code is optionally generated in the Cargo `OUT_DIR` in `version.rs`.
```rust
/// Compile Time (UTC)
pub const COMPILE_TIME: &str = "2018-08-09T15:18:02.918056182+00:00";

/// Compile Time - Short (UTC)
pub const COMPILE_TIME_SHORT: &str = "2018-08-09";

/// Commit SHA
pub const SHA: &str = "75b390dc6c05a6a4aa2791cc7b3934591803bc22";

/// Commit SHA - Short
pub const SHA_SHORT: &str = "75b390d";

/// Commit Date
pub const COMMIT_DATE: &str = "'2018-08-08'";

/// Target Triple
pub const TARGET_TRIPLE: &str = "x86_64-unknown-linux-gnu";

/// Semver
pub const SEMVER: &str = "v0.1.0-pre.0";
```

#### Cargo.toml
```toml
[package]
build = "build.rs"

[build-dependencies]
vergen = "0.2"
```
#### build.rs
```rust
extern crate vergen;

use vergen::{vergen, ConstantsFlags, COMPILE_TIME};

fn main() {
    // Turn on all flags to start.
    let mut flags = OutputFns::all();
    // Toggle the flags for the constants you don't want generated.
    flags.toggle(COMPILE_TIME);
    // Generate the version.rs file in the Cargo OUT_DIR.
    assert!(vergen(flags).is_ok());
}
```
#### lib.rs/main.rs
```rust
include!(concat!(env!("OUT_DIR"), "/version.rs"));

// The following is an exmaple.  You could use now(), sha(), and semver() however you want.
fn version() -> String {
    format!("{} {} {}", COMPILE_TIME, SHA, SEMVER)
    // 2015-02-11 15:35:30.991638113-05:00 b8acdc17bbf0d9928f08b15cba6d3b659770a624 rh v0.0.1-pre-21-gb8acdc1
}
```

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
