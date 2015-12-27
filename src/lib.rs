//! Defines the `vergen` function.
//!
//! `vergen` when used in conjunction with the
//! [build script support](http://doc.crates.io/build-script.html) from
//! cargo, generates a file in OUT_DIR (defined by cargo) with three functions
//! defined (now, sha, and semver).  This file can then be use with include!
//! to pull the functions into your source for use.
//!
//! # Example Cargo.toml
//! ```toml
//! [package]
//! build = "build.rs"
//!
//! [build-dependencies]
//! vergen = "*"
//! ```
//!
//! # Example build.rs
//! ```ignore
//! // build.rs
//! extern crate vergen;
//!
//! use vergen::vergen;
//!
//! fn main() {
//!     vergen();
//! }
//! ```
//!
//! # Example Usage
//! ```ignore
//! extern crate vergen;
//!
//! include!(concat!(env!("OUT_DIR"), "/version.rs"));
//!
//! fn main() {
//!     version();
//! }
//!
//! // Example version function
//! fn version() -> String {
//!    format!("{} {} blah {}", now(), sha(), semver())
//! }
//! ```
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(feature="clippy", deny(clippy, clippy_pedantic))]
#![deny(missing_docs)]
extern crate time;
#[macro_use]
extern crate blastfig;

use std::env;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

bitflags!(
    /// Output Functions Bitflags
    flags OutputFns: u32 {
        /// Generate the now fn.
        const NOW         = 0x00000001,
        /// Generate the short_now fn.
        const SHORT_NOW   = 0x00000010,
        /// Generate the sha fn.
        const SHA         = 0x00000100,
        /// Generate the short_sha fn.
        const SHORT_SHA   = 0x00001000,
        /// Generate the commit_date fn.
        const COMMIT_DATE = 0x00010000,
        /// Generate the target fn.
        const TARGET      = 0x00100000,
        /// Generate the semver fn.
        const SEMVER      = 0x01000000,
    }
);

#[derive(Debug, Default)]
/// An error generated by the vergen function.
pub struct VergenError {
    desc: String,
    detail: String,
}

/// Implemented as 'self.desc: self.detail'.
impl fmt::Display for VergenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.desc, self.detail)
    }
}

impl VergenError {
    /// Create a VergenError struct from the given description and detail.
    pub fn new<T>(desc: &str, detail: T) -> VergenError
        where T: fmt::Debug
    {
        VergenError {
            desc: desc.to_owned(),
            detail: format!("{:?}", detail),
        }
    }
}

impl From<std::env::VarError> for VergenError {
    fn from(e: std::env::VarError) -> VergenError {
        VergenError::new("VarError", e)
    }
}

impl From<std::io::Error> for VergenError {
    fn from(e: std::io::Error) -> VergenError {
        VergenError::new("IOError", e)
    }
}

fn gen_now_fn() -> String {
    let mut now_fn = String::from("/// Generate a timestamp representing now (UTC) in RFC3339 \
                                   format.\n");
    now_fn.push_str("pub fn now() -> &'static str {\n");

    let now = time::now_utc();
    let now_str = format!("{}", now.rfc3339());

    now_fn.push_str("    \"");
    now_fn.push_str(&now_str[..]);
    now_fn.push_str("\"\n");
    now_fn.push_str("}\n\n");

    now_fn
}

fn gen_short_now_fn() -> String {
    let mut now_fn = String::from("/// Generate a timstamp string representing now (UTC).\n");
    now_fn.push_str("pub fn short_now() -> &'static str {\n");

    let now = time::now_utc();
    let now_str = match time::strftime("%F", &now) {
        Ok(n) => n,
        Err(e) => format!("{:?}", e),
    };

    now_fn.push_str("    \"");
    now_fn.push_str(&now_str[..]);
    now_fn.push_str("\"\n");
    now_fn.push_str("}\n\n");

    now_fn
}

fn gen_sha_fn() -> String {
    let mut sha_fn = String::from("/// Generate a SHA string\n");
    sha_fn.push_str("pub fn sha() -> &'static str {\n");
    sha_fn.push_str("    \"");

    let mut sha_cmd = Command::new("git");
    sha_cmd.args(&["rev-parse", "HEAD"]);

    match sha_cmd.output() {
        Ok(o) => {
            let po = String::from_utf8_lossy(&o.stdout[..]);
            sha_fn.push_str(po.trim());
        }
        Err(_) => {
            sha_fn.push_str("UNKNOWN");
        }
    };

    sha_fn.push_str("\"\n");
    sha_fn.push_str("}\n\n");

    sha_fn
}

fn gen_short_sha_fn() -> String {
    let mut sha_fn = String::from("/// Generate a short SHA string\n");
    sha_fn.push_str("pub fn short_sha() -> &'static str {\n");
    sha_fn.push_str("    \"");

    let mut sha_cmd = Command::new("git");
    sha_cmd.args(&["rev-parse", "--short", "HEAD"]);

    match sha_cmd.output() {
        Ok(o) => {
            let po = String::from_utf8_lossy(&o.stdout[..]);
            sha_fn.push_str(po.trim());
        }
        Err(_) => {
            sha_fn.push_str("UNKNOWN");
        }
    }

    sha_fn.push_str("\"\n");
    sha_fn.push_str("}\n\n");

    sha_fn
}

fn gen_commit_date_fn() -> String {
    let mut commit_date_fn = String::from("/// Generate the commit date string\n");
    commit_date_fn.push_str("pub fn commit_date() -> &'static str {\n");
    commit_date_fn.push_str("    \"");

    let mut log_cmd = Command::new("git");
    log_cmd.args(&["log", "--pretty=format:'%ad'", "-n1", "--date=short"]);

    match log_cmd.output() {
        Ok(o) => {
            let po = String::from_utf8_lossy(&o.stdout[..]);

            if po.trim().is_empty() {
                commit_date_fn.push_str("");
            } else {
                commit_date_fn.push_str(po.trim().trim_matches('\''));
            }
        }
        Err(_) => {
            commit_date_fn.push_str("UNKNOWN");
        }
    }

    commit_date_fn.push_str("\"\n");
    commit_date_fn.push_str("}\n\n");

    commit_date_fn
}

fn gen_target_fn() -> String {
    let mut target_fn = String::from("/// Generate the target triple string\n");

    let target = &(env::var("TARGET").unwrap_or("UNKNOWN".to_owned()))[..];

    target_fn.push_str("pub fn target() -> &'static str {\n");
    target_fn.push_str("    \"");
    target_fn.push_str(target);
    target_fn.push_str("\"\n");
    target_fn.push_str("}\n\n");

    target_fn
}

fn gen_semver_fn() -> String {
    let mut semver_fn = String::from("/// Generate a semver string\n");
    semver_fn.push_str("pub fn semver() -> &'static str {\n");
    semver_fn.push_str("    \"");

    let mut branch_cmd = Command::new("git");
    branch_cmd.args(&["describe"]);

    match branch_cmd.output() {
        Ok(o) => {
            let po = String::from_utf8_lossy(&o.stdout[..]);
            semver_fn.push_str(po.trim());
        }
        Err(_) => {
            semver_fn.push_str("UNKNOWN");
        }
    }

    semver_fn.push_str("\"\n");
    semver_fn.push_str("}\n");

    semver_fn
}

/// Create the `version.rs` file in OUT_DIR, and write three functions into it.
///
/// # now
/// ```rust
/// fn now() -> &'static str {
///     // RFC3339 formatted string representing now (UTC)
///     "2015-02-13 11:24:23.613994142-0500"
/// }
/// ```
///
/// # short_now
/// ```rust
/// fn short_now() -> &'static str {
///     // Short string representing now (UTC)
///     "2015-04-07"
/// }
/// ```
///
/// # sha
/// ```rust
/// fn sha() -> &'static str {
///     // Output of the system cmd 'git rev-parse HEAD'
///     "002735cb66437b96cee2a948fcdfc79d9bf96c94"
/// }
/// ```
///
/// # short_sha
/// ```rust
/// fn short_sha() -> &'static str {
///     // Output of the system cmd 'git rev-parse --short HEAD'
///     "002735c"
/// }
/// ```
///
/// # commit_date
/// ```rust
/// fn commit_date() -> &'static str {
///     // Output of the system cmd
///     // 'git log --pretty=format:"%ad" -n1 --date=short'
///     "2015-04-07"
/// }
/// ```
///
/// # target
/// ```rust
/// fn target() -> &'static str {
///     // env::var("TARGET"), set by cargo
///     "x86_64-unknown-linux-gnu"
/// }
/// ```
///
/// # semver
/// ```rust
/// fn semver() -> &'static str {
///     // Output of the system cmd 'git describe'
///     // Note this works best if you create a tag
///     // at each version bump named 'vX.X.X-pre'
///     // and a tag at release named 'vX.X.X'
///     "v0.0.1-pre-24-g002735c"
/// }
/// ```
pub fn vergen(flags: OutputFns) -> Result<(), VergenError> {
    let out = try!(env::var("OUT_DIR"));
    let dst = PathBuf::from(out);
    let mut f = try!(File::create(&dst.join("version.rs")));

    if flags.contains(NOW) {
        try!(f.write_all(gen_now_fn().as_bytes()));
    }

    if flags.contains(SHORT_NOW) {
        try!(f.write_all(gen_short_now_fn().as_bytes()));
    }

    if flags.contains(SHA) {
        try!(f.write_all(gen_sha_fn().as_bytes()));
    }

    if flags.contains(SHORT_SHA) {
        try!(f.write_all(gen_short_sha_fn().as_bytes()));
    }

    if flags.contains(COMMIT_DATE) {
        try!(f.write_all(gen_commit_date_fn().as_bytes()));
    }

    if flags.contains(TARGET) {
        try!(f.write_all(gen_target_fn().as_bytes()));
    }

    if flags.contains(SEMVER) {
        try!(f.write_all(gen_semver_fn().as_bytes()));
    }

    Ok(())
}
