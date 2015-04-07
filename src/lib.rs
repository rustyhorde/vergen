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
//! # extern crate vergen; fn main() {
//! // Other stuff
//! include!(concat!(env!("OUT_DIR"), "/version.rs"));
//!
//! // Example version function
//! fn version() -> String {
//!    format!("{} {} blah {}", now(), sha(), semver())
//! }
//! # }
//! ```
extern crate time;
#[macro_use] extern crate bitflags;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Command;

bitflags!(
    flags Flags: u32 {
        const NOW         = 0x00000001,
        const SHORT_NOW   = 0x00000010,
        const SHA         = 0x00000100,
        const SHORT_SHA   = 0x00001000,
        const COMMIT_DATE = 0x00010000,
        const TARGET      = 0x00100000,
        const SEMVER      = 0x01000000,
    }
);

fn gen_now_fn() -> String {
    let mut now_fn = "pub fn now() -> &'static str {\n".to_string();

    let mut now = Command::new("date");
    now.arg("--rfc-3339=ns");

    match now.output() {
        Ok(o) => {
            let po = String::from_utf8_lossy(&o.stdout[..]);
            now_fn.push_str("    \"");
            now_fn.push_str(po.trim());
            now_fn.push_str("\"\n");
            now_fn.push_str("}\n\n");
        },
        Err(e) => panic!("failed to execute process: {}", e),
    }

    now_fn
}

fn gen_short_now_fn() -> String {
    let mut now_fn = "pub fn short_now() -> &'static str {\n".to_string();

    let mut now = Command::new("date");
    now.arg("--rfc-3339=date");

    match now.output() {
        Ok(o) => {
            let po = String::from_utf8_lossy(&o.stdout[..]);
            now_fn.push_str("    \"");
            now_fn.push_str(po.trim());
            now_fn.push_str("\"\n");
            now_fn.push_str("}\n\n");
        },
        Err(e) => panic!("failed to execute process: {}", e),
    }

    now_fn
}

fn gen_sha_fn() -> String {
    let mut sha_fn = "pub fn sha() -> &'static str {\n".to_string();

    let mut sha_cmd = Command::new("git");
    sha_cmd.args(&["rev-parse", "HEAD"]);

    match sha_cmd.output() {
        Ok(o) => {
            let po = String::from_utf8_lossy(&o.stdout[..]);
            sha_fn.push_str("    \"");
            sha_fn.push_str(po.trim());
            sha_fn.push_str("\"\n");
            sha_fn.push_str("}\n\n");
        },
        Err(e) => panic!("failed to execute process: {}", e),
    };

    sha_fn
}

fn gen_short_sha_fn() -> String {
    let mut sha_fn = "pub fn short_sha() -> &'static str {\n".to_string();

    let mut sha_cmd = Command::new("git");
    sha_cmd.args(&["rev-parse", "--short", "HEAD"]);

    match sha_cmd.output() {
        Ok(o) => {
            let po = String::from_utf8_lossy(&o.stdout[..]);
            sha_fn.push_str("    \"");
            sha_fn.push_str(po.trim());
            sha_fn.push_str("\"\n");
            sha_fn.push_str("}\n\n");
        },
        Err(e) => panic!("failed to execute process: {}", e),
    };

    sha_fn
}

fn gen_commit_date_fn() -> String {
    let mut commit_date_fn = "pub fn commit_date() -> &'static str {\n".to_string();

    let mut log_cmd = Command::new("git");
    log_cmd.args(&["log", "--pretty=format:\"%ad\"", "-n1", "--date=short"]);

    match log_cmd.output() {
        Ok(o) => {
            let po = String::from_utf8_lossy(&o.stdout[..]);
            commit_date_fn.push_str("    ");
            commit_date_fn.push_str(po.trim());
            commit_date_fn.push_str("\n");
            commit_date_fn.push_str("}\n\n");
        },
        Err(e) => panic!("failed to execute process: {}", e),
    };

    commit_date_fn
}

fn gen_target_fn() -> String {
    let mut target_fn = "pub fn target() -> &'static str {\n".to_string();

    target_fn.push_str("    \"");
    target_fn.push_str(&env::var("TARGET").unwrap()[..]);
    target_fn.push_str("\"\n");
    target_fn.push_str("}\n\n");

    target_fn
}

fn gen_semver_fn() -> String {
    let mut semver_fn = "pub fn semver() -> &'static str {\n".to_string();

    let mut branch_cmd = Command::new("git");
    branch_cmd.args(&["describe"]);

    match branch_cmd.output() {
        Ok(o) => {
            let po = String::from_utf8_lossy(&o.stdout[..]);
            semver_fn.push_str("    \"");
            semver_fn.push_str(po.trim());
            semver_fn.push_str("\"\n");
            semver_fn.push_str("}\n");
        },
        Err(e) => panic!("failed to execute process: {}", e),
    };

    semver_fn
}

/// Create the `version.rs` file in OUT_DIR, and write three functions into it.
///
/// # now
/// ```rust
/// fn now() -> &'static str {
///     // Output of the system cmd 'date '--rfc-3339=ns'
///     "2015-02-13 11:24:23.613994142-0500"
/// }
/// ```
///
/// # short_now
/// ```rust
/// fn short_now() -> &'static str {
///     // Output of the system cmd 'date '--rfc-3339=date'
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
pub fn vergen(flags: Flags) {
    let dst = PathBuf::from(&env::var_os("OUT_DIR").unwrap());
    let mut f = File::create(&dst.join("version.rs")).unwrap();

    if flags.contains(NOW) {
        f.write_all(gen_now_fn().as_bytes()).unwrap();
    }

    if flags.contains(SHORT_NOW) {
        f.write_all(gen_short_now_fn().as_bytes()).unwrap();
    }

    if flags.contains(SHA) {
        f.write_all(gen_sha_fn().as_bytes()).unwrap();
    }

    if flags.contains(SHORT_SHA) {
        f.write_all(gen_short_sha_fn().as_bytes()).unwrap();
    }

    if flags.contains(COMMIT_DATE) {
        f.write_all(gen_commit_date_fn().as_bytes()).unwrap();
    }

    if flags.contains(TARGET) {
        f.write_all(gen_target_fn().as_bytes()).unwrap();
    }

    if flags.contains(SEMVER) {
        f.write_all(gen_semver_fn().as_bytes()).unwrap();
    }
}
