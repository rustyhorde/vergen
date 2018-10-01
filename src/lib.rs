// Copyright (c) 2016, 2018 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Generate build time information for use within a project.
//!
//! `vergen`, when used in conjunction with the [Build Scripts] support in
//! cargo, can either
//!
//! 1. Generate environment variables to use with the `env!` macro.  See the
//! documentation for `VergenKey` for the environment variables names.
//! 2. Generate a file in `OUT_DIR` (defined by cargo) with up to 8 build time
//! constants.  This file can then be used with the `include!` macro to pull the
//! constants into your source for use.
//!
//! [Build Scripts]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
//!
//! # 2.x.x
//! ## Example Cargo.toml
//! ```toml
//! [package]
//! #..
//! build = "build.rs"
//!
//! [dependencies]
//! #..
//!
//! [build-dependencies]
//! vergen = "2"
//! ```
//!
//! ## Example `build.rs`
//!
//! ```
//! extern crate vergen;
//!
//! use vergen::{ConstantsFlags, Result, Vergen};
//!
//! fn main() {
//!     gen_constants().expect("Unable to generate vergen constants!");
//! }
//!
//! fn gen_constants() -> Result<()> {
//!     let vergen = Vergen::new(ConstantsFlags::all())?;
//!
//!     for (k, v) in vergen.build_info() {
//!         println!("cargo:rustc-env={}={}", k.name(), v);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Use constants in your code
//!
//! ```ignore
//! fn my_fn() {
//!     println!("Build Timestamp: {}", env!("VERGEN_BUILD_TIMESTAMP"));
//! }
//! ```
//!
//! # 1.x.x
//! ## Example `build.rs`
//! ```
//! extern crate vergen;
//!
//! # use std::env;
//! use vergen::{ConstantsFlags, Result, vergen};
//!
//! fn main() {
//! #   env::set_var("OUT_DIR", "target");
//!     let mut flags = ConstantsFlags::all();
//!     flags.toggle(ConstantsFlags::BUILD_TIMESTAMP);
//!     vergen(flags).expect("Unable to generate constants!");
//! }
//! ```
//!
//! ## Example `version.rs`
//! ```
//! /// Compile Time (UTC)
//! pub const VERGEN_BUILD_TIMESTAMP: &str = "2018-08-09T15:15:57.282334589+00:00";
//!
//! /// Compile Time - Short (UTC)
//! pub const VERGEN_BUILD_DATE: &str = "2018-08-09";
//!
//! /// Commit SHA
//! pub const VERGEN_SHA: &str = "75b390dc6c05a6a4aa2791cc7b3934591803bc22";
//!
//! /// Commit SHA - Short
//! pub const VERGEN_SHA_SHORT: &str = "75b390d";
//!
//! /// Commit Date
//! pub const VERGEN_COMMIT_DATE: &str = "2018-08-08";
//!
//! /// Target Triple
//! pub const VERGEN_TARGET_TRIPLE: &str = "x86_64-unknown-linux-gnu";
//!
//! /// Semver
//! pub const VERGEN_SEMVER: &str = "v0.1.0-pre.0";
//!
//! /// Semver (Lightweight)
//! pub const VERGEN_SEMVER_LIGHTWEIGHT: &str = "v0.1.0-pre.0";
//! ```
//!
//! ## Include the constants in your code (Version 1.x.x only)
//! ```ignore
//! include!(concat!(env!("OUT_DIR"), "/version.rs"));
//!
//! format!("{} {} blah {}", VERGEN_BUILD_TIMESTAMP, VERGEN_SHA, VERGEN_SEMVER)
//! ```
#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications
)]
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate getset;
#[cfg(test)]
#[macro_use]
extern crate lazy_static;

extern crate chrono;
#[cfg(test)]
extern crate regex;

mod codegen;
mod constants;
mod envvar;
mod error;

pub use codegen::vergen;
pub use constants::ConstantsFlags;
pub use envvar::{Vergen, VergenKey};
pub use error::Result;
