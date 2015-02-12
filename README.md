# vergen
Includes 3 functions for use in version strings in ```lib.rs/main.rs``` at compile time via a custom cargo build script.

Note:  This currently only works for projects using git building on Linux or msys2.  I haven't tested on Mac OS/X, but it will work if the date command exists.  I plan to expand the functionality to additional platforms eventually.

```rust
pub fn now() -> &'static str {
   // Output of 'date --rc-3339=ns'
}

pub fn sha() -> &'static str {
   // Output of 'git rev-parse HEAD'
}

pub fn semver() -> &'static str {
   // output of 'git describe'
   // this works best if you tag your releases 'vX.X.X'
   // and create a new tag on master after a release 'vX.X.Y-pre'
}
```

## Status
[![Build Status](https://travis-ci.org/rustyhorde/vergen.svg?branch=0.0.1)](https://travis-ci.org/rustyhorde/vergen)

## Basic Usage
#### Cargo.toml
```toml
[package]
#
build = "build.rs"
#
[build-dependencies]
vergen = "*"
```
#### build.rs
```rust
use vergen::vergen;

fn main() {
    vergen();
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
