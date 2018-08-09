// Copyright (c) 2016, 2018 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Defines the `vergen` function.
//!
//! `vergen`, when used in conjunction with the
//! [Build Scripts] support in
//! cargo, generates a file in `OUT_DIR` (defined by cargo) with up to 7 build
//! time constants.  This file can then be use with `include!` to pull the
//! constants into your source for use.
//!
//! [Build Scripts]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
//!
//! # Example Cargo.toml
//! ```toml
//! [package]
//! #..
//! build = "build.rs"
//!
//! [dependencies]
//! #..
//!
//! [build-dependencies]
//! vergen = "0"
//! ```
//!
//! # Example `build.rs`
//! ```
//! extern crate vergen;
//!
//! # use std::env;
//! use vergen::{ConstantsFlags, COMPILE_TIME, Result, vergen};
//!
//! fn main() {
//! #   env::set_var("OUT_DIR", "target");
//!     let mut flags = ConstantsFlags::all();
//!     flags.toggle(COMPILE_TIME);
//!     vergen(flags).expect("Unable to generate constants!");
//! }
//! ```
//!
//! # Example `version.rs` (All Flags Enabled)
//! ```
//! /// Compile Time (UTC)
//! const COMPILE_TIME: &str = "2018-08-09T15:15:57.282334589+00:00";
//!
//! /// Compile Time - Short (UTC)
//! const COMPILE_TIME_SHORT: &str = "2018-08-09";
//!
//! /// Commit SHA
//! const SHA: &str = "75b390dc6c05a6a4aa2791cc7b3934591803bc22";
//!
//! /// Commit SHA - Short
//! const SHA_SHORT: &str = "75b390d";
//!
//! /// Commit Date
//! const COMMIT_DATE: &str = "'2018-08-08'";
//!
//! /// Target Triple
//! const TARGET_TRIPLE: &str = "x86_64-unknown-linux-gnu";
//!
//! /// Semver
//! const SEMVER: &str = "v0.1.0-pre.0";
//! ```
//!
//! # Include the constants in your code
//! ```ignore
//! include!(concat!(env!("OUT_DIR"), "/version.rs"));
//!
//! format!("{} {} blah {}", COMMIT_TIME, SHA, SEMVER)
//! # }
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

extern crate chrono;

mod error;

pub use error::Result;

use chrono::{DateTime, Utc};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

bitflags!(
    /// Constants Flags
    ///
    /// Use these to toggle off the generation of constants you won't use.
    ///
    /// ```
    /// # extern crate vergen;
    /// #
    /// # use vergen::*;
    /// #
    /// # fn foo() {
    /// let mut flags = ConstantsFlags::all();
    /// flags.toggle(SHA_SHORT);
    /// flags.toggle(COMMIT_DATE);
    ///
    /// assert_eq!(
    ///   flags,
    ///   COMPILE_TIME &
    ///   COMPILE_TIME_SHORT &
    ///   SHA &
    ///   TARGET_TRIPLE &
    ///   SEMVER
    /// )
    /// # }
    /// ```
    pub struct ConstantsFlags: u32 {
        /// Generate the compile time constant.
        ///
        /// "2018-08-09T15:15:57.282334589+00:00"
        const COMPILE_TIME       = 0x0000_0001;
        /// Generate the compile date constant.
        ///
        /// "2018-08-09"
        const COMPILE_TIME_SHORT = 0x0000_0010;
        /// Generate the SHA constant.
        ///
        /// "75b390dc6c05a6a4aa2791cc7b3934591803bc22"
        const SHA                = 0x0000_0100;
        /// Generate the short SHA constant.
        ///
        /// "75b390d"
        const SHA_SHORT          = 0x0000_1000;
        /// Generate the commit date constant.
        ///
        /// "2018-08-08"
        const COMMIT_DATE        = 0x0001_0000;
        /// Generate the target triple constant.
        ///
        /// "x86_64-unknown-linux-gnu"
        const TARGET_TRIPLE      = 0x0010_0000;
        /// Generate the semver constant.
        ///
        /// This defaults to the output of `git describe`.  If that output is
        /// empty, the the `CARGO_PKG_VERSION` environment variable is used.
        ///
        /// "v0.1.0-pre.0"
        const SEMVER             = 0x0100_0000;
    }
);

const CONST_PREFIX: &str = "pub const ";
const CONST_TYPE: &str = ": &str = ";
const COMPILE_TIME_NAME: &str = "COMPILE_TIME";
const COMPILE_TIME_COMMENT: &str = "/// Compile Time (UTC)";
const COMPILE_TIME_SHORT_NAME: &str = "COMPILE_TIME_SHORT";
const COMPILE_TIME_SHORT_COMMENT: &str = "/// Compile Time - Short (UTC)";
const SHA_NAME: &str = "SHA";
const SHA_COMMENT: &str = "/// Commit SHA";
const SHA_SHORT_NAME: &str = "SHA_SHORT";
const SHA_SHORT_COMMENT: &str = "/// Commit SHA - Short";
const COMMIT_DATE_NAME: &str = "COMMIT_DATE";
const COMMIT_DATE_COMMENT: &str = "/// Commit Date";
const TARGET_TRIPLE_NAME: &str = "TARGET_TRIPLE";
const TARGET_TRIPLE_COMMENT: &str = "/// Target Triple";
const SEMVER_NAME: &str = "SEMVER";
const SEMVER_COMMENT: &str = "/// Semver";

fn gen_const(f: &mut File, comment: &str, name: &str, value: &str) -> Result<()> {
    writeln!(
        f,
        "{}\n{}{}{}\"{}\";",
        comment, CONST_PREFIX, name, CONST_TYPE, value
    )?;
    Ok(())
}

fn run_command(command: &mut Command) -> String {
    let raw_output = if let Ok(o) = command.output() {
        String::from_utf8_lossy(&o.stdout).into_owned()
    } else {
        "Unknown".to_string()
    };
    raw_output.trim().to_string()
}

fn gen_compile_time(f: &mut File, now: DateTime<Utc>) -> Result<()> {
    gen_const(
        f,
        COMPILE_TIME_COMMENT,
        COMPILE_TIME_NAME,
        &now.to_rfc3339().to_string(),
    )
}

fn gen_compile_time_short(f: &mut File, now: DateTime<Utc>) -> Result<()> {
    gen_const(
        f,
        COMPILE_TIME_SHORT_COMMENT,
        COMPILE_TIME_SHORT_NAME,
        &now.format("%Y-%m-%d").to_string(),
    )
}

fn gen_sha(f: &mut File) -> Result<()> {
    let sha = run_command(Command::new("git").args(&["rev-parse", "HEAD"]));
    gen_const(f, SHA_COMMENT, SHA_NAME, &sha)
}

fn gen_short_sha(f: &mut File) -> Result<()> {
    let sha = run_command(Command::new("git").args(&["rev-parse", "--short", "HEAD"]));
    gen_const(f, SHA_SHORT_COMMENT, SHA_SHORT_NAME, &sha)
}

fn gen_commit_date(f: &mut File) -> Result<()> {
    let commit_date = run_command(Command::new("git").args(&[
        "log",
        "--pretty=format:'%ad'",
        "-n1",
        "--date=short",
    ]));
    gen_const(f, COMMIT_DATE_COMMENT, COMMIT_DATE_NAME, &commit_date)
}

fn gen_target(f: &mut File) -> Result<()> {
    gen_const(
        f,
        TARGET_TRIPLE_COMMENT,
        TARGET_TRIPLE_NAME,
        &env::var("TARGET").unwrap_or_else(|_| "UNKNOWN".to_string()),
    )
}

fn gen_semver(f: &mut File) -> Result<()> {
    let describe = run_command(Command::new("git").args(&["describe"]));

    let semver = if describe.is_empty() {
        env::var("CARGO_PKG_VERSION")?
    } else {
        describe
    };
    gen_const(f, SEMVER_COMMENT, SEMVER_NAME, &semver)
}

/// Create a `version.rs` file in `OUT_DIR`, and write up to 7 constants into
/// it.
///
/// # Example build.rs
/// ```
/// # extern crate vergen;
/// #
/// # use std::env;
/// # use vergen::{ConstantsFlags, COMPILE_TIME, Result, vergen};
/// #
/// fn main() {
/// #   env::set_var("OUT_DIR", "target");
///     let mut flags = ConstantsFlags::all();
///     flags.toggle(COMPILE_TIME);
///     vergen(flags).expect("Unable to generate constants!");
/// }
/// ```
///
/// # Example Output (All Flags Enabled)
/// ```
/// /// Compile Time (UTC)
/// const COMPILE_TIME: &str = "2018-08-09T15:15:57.282334589+00:00";
///
/// /// Compile Time - Short (UTC)
/// const COMPILE_TIME_SHORT: &str = "2018-08-09";
///
/// /// Commit SHA
/// const SHA: &str = "75b390dc6c05a6a4aa2791cc7b3934591803bc22";
///
/// /// Commit SHA - Short
/// const SHA_SHORT: &str = "75b390d";
///
/// /// Commit Date
/// const COMMIT_DATE: &str = "'2018-08-08'";
///
/// /// Target Triple
/// const TARGET_TRIPLE: &str = "x86_64-unknown-linux-gnu";
///
/// /// Semver
/// const SEMVER: &str = "v0.1.0-pre.0";
/// ```
pub fn vergen(flags: ConstantsFlags) -> Result<()> {
    let dst = PathBuf::from(env::var("OUT_DIR")?);
    let mut f = File::create(&dst.join("version.rs"))?;
    let now = Utc::now();
    let mut first = true;

    if flags.contains(COMPILE_TIME) {
        gen_compile_time(&mut f, now)?;
        first = false
    }

    if flags.contains(COMPILE_TIME_SHORT) {
        if !first {
            writeln!(f);
        }
        gen_compile_time_short(&mut f, now)?;
        first = false;
    }

    if flags.contains(SHA) {
        if !first {
            writeln!(f);
        }
        gen_sha(&mut f)?;
        first = false;
    }

    if flags.contains(SHA_SHORT) {
        if !first {
            writeln!(f);
        }
        gen_short_sha(&mut f)?;
        first = false;
    }

    if flags.contains(COMMIT_DATE) {
        if !first {
            writeln!(f);
        }
        gen_commit_date(&mut f)?;
        first = false;
    }

    if flags.contains(TARGET_TRIPLE) {
        if !first {
            writeln!(f);
        }
        gen_target(&mut f)?;
        first = false;
    }

    if flags.contains(SEMVER) {
        if !first {
            writeln!(f);
        }
        gen_semver(&mut f)?;
    }

    Ok(())
}
