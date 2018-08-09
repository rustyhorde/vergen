// Copyright (c) 2016, 2018 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Defines the `vergen` function.
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
//! vergen = "2"
//! ```
//!
//! # Example `build.rs` (Version 2.x.x)
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
//! # Example `build.rs` (Version 1.x.x)
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
//! # Example `version.rs` (Version 1.x.x only)
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
//! pub const VERGEN_COMMIT_DATE: &str = "'2018-08-08'";
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
//! # Include the constants in your code (Version 1.x.x only)
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
#[macro_use]
extern crate getset;

extern crate chrono;

mod error;

pub use error::Result;

use chrono::Utc;
use std::collections::HashMap;
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
    /// # use vergen::ConstantsFlags;
    /// #
    /// # fn foo() {
    /// let mut flags = ConstantsFlags::all();
    /// flags.toggle(ConstantsFlags::SHA_SHORT);
    /// flags.toggle(ConstantsFlags::COMMIT_DATE);
    ///
    /// assert_eq!(
    ///   flags,
    ///   ConstantsFlags::BUILD_TIMESTAMP &
    ///   ConstantsFlags::BUILD_DATE &
    ///   ConstantsFlags::SHA &
    ///   ConstantsFlags::TARGET_TRIPLE &
    ///   ConstantsFlags::SEMVER &
    ///   ConstantsFlags::SEMVER_LIGHTWEIGHT
    /// )
    /// # }
    /// ```
    pub struct ConstantsFlags: u32 {
        /// Generate the build timestamp constant.
        ///
        /// "2018-08-09T15:15:57.282334589+00:00"
        const BUILD_TIMESTAMP    = 0x0000_0001;
        /// Generate the build date constant.
        ///
        /// "2018-08-09"
        const BUILD_DATE         = 0x0000_0010;
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
        /// Generate the semver constant, including lightweight tags.
        ///
        /// This defaults to the output of `git describe`.  If that output is
        /// empty, the the `CARGO_PKG_VERSION` environment variable is used.
        ///
        /// "v0.1.0-pre.0"
        const SEMVER_LIGHTWEIGHT = 0x0200_0000;
    }
);

const CONST_PREFIX: &str = "pub const ";
const CONST_TYPE: &str = ": &str = ";
const BUILD_TIMESTAMP_NAME: &str = "VERGEN_BUILD_TIMESTAMP";
const BUILD_TIMESTAMP_COMMENT: &str = "/// Build Timestamp (UTC)";
const BUILD_DATE_NAME: &str = "VERGEN_BUILD_DATE";
const BUILD_DATE_COMMENT: &str = "/// Compile Time - Short (UTC)";
const SHA_NAME: &str = "VERGEN_SHA";
const SHA_COMMENT: &str = "/// Commit SHA";
const SHA_SHORT_NAME: &str = "VERGEN_SHA_SHORT";
const SHA_SHORT_COMMENT: &str = "/// Commit SHA - Short";
const COMMIT_DATE_NAME: &str = "VERGEN_COMMIT_DATE";
const COMMIT_DATE_COMMENT: &str = "/// Commit Date";
const TARGET_TRIPLE_NAME: &str = "VERGEN_TARGET_TRIPLE";
const TARGET_TRIPLE_COMMENT: &str = "/// Target Triple";
const SEMVER_NAME: &str = "VERGEN_SEMVER";
const SEMVER_COMMENT: &str = "/// Semver";
const SEMVER_TAGS_NAME: &str = "VERGEN_SEMVER_LIGHTWEIGHT";
const SEMVER_TAGS_COMMENT: &str = "/// Semver (Lightweight)";

/// `vergen` build information keys.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum VergenKey {
    /// The build timestamp. (VERGEN_BUILD_TIMESTAMP)
    BuildTimestamp,
    /// The build date. (VERGEN_BUILD_DATE)
    BuildDate,
    /// The latest commit SHA. (VERGEN_SHA)
    Sha,
    /// The latest commit short SHA. (VERGEN_SHA_SHORT)
    ShortSha,
    /// The commit date. (VERGEN_COMMIT_DATE).
    CommitDate,
    /// The target triple. (VERGEN_TARGET_TRIPLE)
    TargetTriple,
    /// The semver version from the last git tag. (VERGEN_SEMVER)
    Semver,
    /// The semver version from the last git tag, including lightweight.
    /// (VERGEN_SEMVER_LIGHTWEIGHT)
    SemverLightweight,
}

impl VergenKey {
    /// Get the comment string for the given key.
    pub fn comment(self) -> &'static str {
        match self {
            VergenKey::BuildTimestamp => BUILD_TIMESTAMP_COMMENT,
            VergenKey::BuildDate => BUILD_DATE_COMMENT,
            VergenKey::Sha => SHA_COMMENT,
            VergenKey::ShortSha => SHA_SHORT_COMMENT,
            VergenKey::CommitDate => COMMIT_DATE_COMMENT,
            VergenKey::TargetTriple => TARGET_TRIPLE_COMMENT,
            VergenKey::Semver => SEMVER_COMMENT,
            VergenKey::SemverLightweight => SEMVER_TAGS_COMMENT,
        }
    }

    /// Get the name for the given key.
    pub fn name(self) -> &'static str {
        match self {
            VergenKey::BuildTimestamp => BUILD_TIMESTAMP_NAME,
            VergenKey::BuildDate => BUILD_DATE_NAME,
            VergenKey::Sha => SHA_NAME,
            VergenKey::ShortSha => SHA_SHORT_NAME,
            VergenKey::CommitDate => COMMIT_DATE_NAME,
            VergenKey::TargetTriple => TARGET_TRIPLE_NAME,
            VergenKey::Semver => SEMVER_NAME,
            VergenKey::SemverLightweight => SEMVER_TAGS_NAME,
        }
    }
}

/// Build time information struct.
///
/// # Example `build.rs`
///
/// ```
/// extern crate vergen;
///
/// use vergen::{ConstantsFlags, Result, Vergen};
///
/// fn main() {
///     gen_constants().expect("Unable to generate vergen constants!");
/// }
///
/// fn gen_constants() -> Result<()> {
///     let vergen = Vergen::new(ConstantsFlags::all())?;
///
///     for (k, v) in vergen.build_info() {
///         println!("cargo:rustc-env={}={}", k.name(), v);
///     }
///
///     Ok(())
/// }
/// ```
#[derive(Clone, Debug, Default, Getters, Eq, PartialEq)]
pub struct Vergen {
    /// The build information map.
    #[get = "pub"]
    build_info: HashMap<VergenKey, String>,
}

impl Vergen {
    /// Create a `Vergen` stuct to use in `build.rs`.
    pub fn new(flags: ConstantsFlags) -> Result<Self> {
        let mut vergen = Self::default();
        let mut build_info = HashMap::new();
        let now = Utc::now();

        if flags.contains(ConstantsFlags::BUILD_TIMESTAMP) {
            build_info.insert(VergenKey::BuildTimestamp, now.to_rfc3339());
        }

        if flags.contains(ConstantsFlags::BUILD_DATE) {
            build_info.insert(VergenKey::BuildDate, now.format("%Y-%m-%d").to_string());
        }

        if flags.contains(ConstantsFlags::SHA) {
            let sha = run_command(Command::new("git").args(&["rev-parse", "HEAD"]));
            build_info.insert(VergenKey::Sha, sha);
        }

        if flags.contains(ConstantsFlags::SHA_SHORT) {
            let sha = run_command(Command::new("git").args(&["rev-parse", "--short", "HEAD"]));
            build_info.insert(VergenKey::ShortSha, sha);
        }

        if flags.contains(ConstantsFlags::COMMIT_DATE) {
            let commit_date = run_command(Command::new("git").args(&[
                "log",
                "--pretty=format:'%ad'",
                "-n1",
                "--date=short",
            ]));
            build_info.insert(VergenKey::CommitDate, commit_date);
        }

        if flags.contains(ConstantsFlags::TARGET_TRIPLE) {
            let target_triple = env::var("TARGET").unwrap_or_else(|_| "UNKNOWN".to_string());
            build_info.insert(VergenKey::TargetTriple, target_triple);
        }

        if flags.contains(ConstantsFlags::SEMVER) {
            let describe = run_command(Command::new("git").args(&["describe"]));

            let semver = if describe.is_empty() {
                env::var("CARGO_PKG_VERSION")?
            } else {
                describe
            };
            build_info.insert(VergenKey::Semver, semver);
        }

        if flags.contains(ConstantsFlags::SEMVER_LIGHTWEIGHT) {
            let describe = run_command(Command::new("git").args(&["describe", "--tags"]));

            let semver = if describe.is_empty() {
                env::var("CARGO_PKG_VERSION")?
            } else {
                describe
            };
            build_info.insert(VergenKey::SemverLightweight, semver);
        }

        vergen.build_info = build_info;
        Ok(vergen)
    }
}

fn gen_const<W: Write>(f: &mut W, comment: &str, name: &str, value: &str) -> Result<()> {
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

/// Create a `version.rs` file in `OUT_DIR`, and write up to 7 constants into
/// it.
///
/// # Example build.rs
/// ```
/// # extern crate vergen;
/// #
/// # use std::env;
/// # use vergen::{ConstantsFlags, Result, vergen};
/// #
/// fn main() {
/// #   env::set_var("OUT_DIR", "target");
///     let mut flags = ConstantsFlags::all();
///     flags.toggle(ConstantsFlags::BUILD_TIMESTAMP);
///     vergen(flags).expect("Unable to generate constants!");
/// }
/// ```
///
/// # Example Output (All Flags Enabled)
/// ```
/// /// Build Timestamp (UTC)
/// pub const VERGEN_BUILD_TIMESTAMP: &str = "2018-08-09T15:15:57.282334589+00:00";
///
/// /// Build Date - Short (UTC)
/// pub const VERGEN_BUILD_DATE: &str = "2018-08-09";
///
/// /// Commit SHA
/// pub const VERGEN_SHA: &str = "75b390dc6c05a6a4aa2791cc7b3934591803bc22";
///
/// /// Commit SHA - Short
/// pub const VERGEN_SHA_SHORT: &str = "75b390d";
///
/// /// Commit Date
/// pub const VERGEN_COMMIT_DATE: &str = "'2018-08-08'";
///
/// /// Target Triple
/// pub const VERGEN_TARGET_TRIPLE: &str = "x86_64-unknown-linux-gnu";
///
/// /// Semver
/// pub const VERGEN_SEMVER: &str = "v0.1.0-pre.0";
///
/// /// Semver (Lightweight)
/// pub const VERGEN_SEMVER_LIGHTWEIGHT: &str = "v0.1.0-pre.0";
/// ```
pub fn vergen(flags: ConstantsFlags) -> Result<()> {
    let dst = PathBuf::from(env::var("OUT_DIR")?);
    let mut f = File::create(&dst.join("version.rs"))?;
    let vergen = Vergen::new(flags)?;

    for (k, v) in vergen.build_info() {
        gen_const(&mut f, k.comment(), k.name(), v)?;
        writeln!(f)?;
    }

    Ok(())
}
