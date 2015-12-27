# vergen
## Version
[![Crates.io](https://img.shields.io/crates/v/vergen.svg)](https://crates.io/crates/vergen)
[![Build
Status](https://travis-ci.org/rustyhorde/vergen.svg?branch=0.1.0)](https://travis-ci.org/rustyhorde/vergen)

## Basic Usage
The following code is optionally generated in the Cargo OUT_DIR.
```rust
pub fn now() -> &'static str {
   // RFC3339 formatted string representing now (UTC)
}

pub fn short_now() -> &'static str {
   // Short string representing now (UTC)
}

pub fn sha() -> &'static str {
   // Output of 'git rev-parse HEAD'
}

pub fn short_sha() -> &'static str {
   // Output of 'git rev-parse --short HEAD'
}

pub fn commit_date() -> &'static str {
   // Output of 'git log --pretty=format:"%ad" -n1 --date=short'
}

pub fn target() -> &'static str {
   // env::var("TARGET")
}

pub fn semver() -> &'static str {
   // output of 'git describe'
   // this works best if you tag your releases 'vX.X.X'
   // and create a new tag on master after a release 'vX.X.Y-pre'
}
```

#### Cargo.toml
```toml
[package]
#
build = "build.rs"
#
[build-dependencies]
vergen = "~0.1.0"
```
#### build.rs
```rust
use vergen::vergen;

fn main() {
    // Turn on all flags to start.
    let mut flags = OutputFns::all();
    // Toggle output fns you don't want generated.
    flags.toggle(NOW);
    // Generate the version.rs file in the Cargo OUT_DIR.
    assert!(vergen(flags).is_ok());
}
```
#### lib.rs/main.rs
```rust
include!(concat!(env!("OUT_DIR"), "/version.rs"));

// The following is an exmaple.  You could use now(), sha(), and semver() however you want.
fn version() -> String {
    format!("{} {} {}", now(), sha(), semver())
    // 2015-02-11 15:35:30.991638113-05:00 b8acdc17bbf0d9928f08b15cba6d3b659770a624 rh v0.0.1-pre-21-gb8acdc1
}
```
