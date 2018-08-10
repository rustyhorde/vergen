# vergen
## Version
[![Crates.io](https://img.shields.io/crates/v/vergen.svg)](https://crates.io/crates/vergen)
[![Build
Status](https://travis-ci.org/rustyhorde/vergen.svg?branch=master)](https://travis-ci.org/rustyhorde/vergen)

**NOTE**: Version 2.x.x is compatible with Version 1.x.x, but introduces a completely new way to use the
constants without having to use the `include!` macro.

**NOTE**: Version 1.x.x is a breaking change from the 0.1.0 series.  This crate no longer generates functions
to display the build time information, but rather generates constants.  See below for more detail.

## Basic Usage
`vergen`, when used in conjunction with the [Build Scripts] support in
cargo, can either

[Build Scripts]: https://doc.rust-lang.org/cargo/reference/build-scripts.html

1. Generate environment variables to use with the `env!` macro.  See the
documentation for `VergenKey` for the environment variables names.
2. Generate a file in `OUT_DIR` (defined by cargo) with up to 8 build time
constants.  This file can then be used with the `include!` macro to pull the
constants into your source for use.

## 2.x.x
### Example Cargo.toml
```toml
[package]
#..
build = "build.rs"

[dependencies]
#..

[build-dependencies]
vergen = "2"
```

### Example `build.rs`
```rust
extern crate vergen;

use vergen::{ConstantsFlags, Result, Vergen};

fn main() {
    gen_constants().expect("Unable to generate vergen constants!");
}

fn gen_constants() -> Result<()> {
    let vergen = Vergen::new(ConstantsFlags::all())?;

    for (k, v) in vergen.build_info() {
        println!("cargo:rustc-env={}={}", k.name(), v);
    }

    Ok(())
}
```

### Use constants in your code
```rust
fn my_fn() {
    println!("Build Timestamp: {}", env!("VERGEN_BUILD_TIMESTAMP"));
}
```

## 1.x.x
### Example `build.rs`
```rust
extern crate vergen;

use vergen::{ConstantsFlags, Result, vergen};

fn main() {
    let mut flags = ConstantsFlags::all();
    flags.toggle(ConstantsFlags::BUILD_TIMESTAMP);
    vergen(flags).expect("Unable to generate constants!");
}
```

### Example `version.rs`
```rust
/// Compile Time (UTC)
pub const VERGEN_BUILD_TIMESTAMP: &str = "2018-08-09T15:15:57.282334589+00:00";

/// Compile Time - Short (UTC)
pub const VERGEN_BUILD_DATE: &str = "2018-08-09";

/// Commit SHA
pub const VERGEN_SHA: &str = "75b390dc6c05a6a4aa2791cc7b3934591803bc22";

/// Commit SHA - Short
pub const VERGEN_SHA_SHORT: &str = "75b390d";

/// Commit Date
pub const VERGEN_COMMIT_DATE: &str = "'2018-08-08'";

/// Target Triple
pub const VERGEN_TARGET_TRIPLE: &str = "x86_64-unknown-linux-gnu";

/// Semver
pub const VERGEN_SEMVER: &str = "v0.1.0-pre.0";

/// Semver (Lightweight)
pub const VERGEN_SEMVER_LIGHTWEIGHT: &str = "v0.1.0-pre.0";
```

### Include the constants in your code (Version 1.x.x only)
```rust
include!(concat!(env!("OUT_DIR"), "/version.rs"));

format!("{} {} blah {}", VERGEN_COMMIT_DATE, VERGEN_SHA, VERGEN_SEMVER)
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
