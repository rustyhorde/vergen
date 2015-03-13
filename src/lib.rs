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
#![feature(core,io,path,staged_api)]
#![staged_api]
extern crate time;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Command;

fn gen_now_fn() -> String {
    let mut now_fn = "pub fn now() -> &'static str {\n".to_string();

    let mut now = Command::new("date");
    now.arg("--rfc-3339=ns");

    match now.output() {
        Ok(o) => {
            let po = String::from_utf8_lossy(o.stdout.as_slice());
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
            let po = String::from_utf8_lossy(o.stdout.as_slice());
            sha_fn.push_str("    \"");
            sha_fn.push_str(po.trim());
            sha_fn.push_str("\"\n");
            sha_fn.push_str("}\n\n");
        },
        Err(e) => panic!("failed to execute process: {}", e),
    };

    sha_fn
}

fn gen_semver_fn() -> String {
    let mut semver_fn = "pub fn semver() -> &'static str {\n".to_string();

    let mut branch_cmd = Command::new("git");
    branch_cmd.args(&["describe"]);

    match branch_cmd.output() {
        Ok(o) => {
            let po = String::from_utf8_lossy(o.stdout.as_slice());
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
/// # sha
/// ```rust
/// fn sha() -> &'static str {
///     // Output of the system cmd 'git rev-parse HEAD'
///     "002735cb66437b96cee2a948fcdfc79d9bf96c94"
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
pub fn vergen() {
    let dst = PathBuf::new(&env::var_os("OUT_DIR").unwrap());
    let mut f = File::create(&dst.join("version.rs")).unwrap();
    f.write_all(gen_now_fn().as_bytes()).unwrap();
    f.write_all(gen_sha_fn().as_bytes()).unwrap();
    f.write_all(gen_semver_fn().as_bytes()).unwrap();
}

#[cfg(test)]
mod test {
    use std::env;
    use std::fs::PathExt;
    use super::vergen;

    #[test]
    #[unstable(feature = "vergen")]
    fn test_vergen() {
        let tmp = env::temp_dir();
        env::set_var("OUT_DIR",&tmp);
        vergen();
        assert!(&tmp.join("version.rs").exists());
    }
}
