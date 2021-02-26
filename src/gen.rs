// Copyright (c) 2016, 2018, 2021 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `vergen` cargo instruction generation

use crate::{
    config::{Config, Instructions, VergenKey},
    constants::ConstantsFlags,
    error::Result,
};
use std::{
    io::{self, Write},
    path::Path,
};

/// Generate the `cargo:` instructions
///
/// # Errors
///
/// Any generated errors will be wrapped in [`vergen::Error`](crate::error::Error)
///
/// # Usage
///
/// ```
/// # use vergen::{ConstantsFlags, Error, gen};
/// #
/// # fn main() -> std::result::Result<(), Error> {
/// // Generate the default 'cargo:' instruction output
/// gen(ConstantsFlags::all())?;
/// #   Ok(())
/// # }
/// ```
#[cfg(not(feature = "git"))]
#[deprecated(since = "4.2.0", note = "Please use the `vergen` function instead")]
pub fn gen(flags: ConstantsFlags) -> Result<()> {
    // This is here to help with type inference
    let no_repo: Option<&'static str> = None;
    config_from_flags(flags, no_repo, &mut io::stdout())
}

/// Generate the `cargo:` instructions
///
/// # Errors
///
/// Any generated errors will be wrapped in [`vergen::Error`](crate::error::Error)
///
/// # Usage
///
/// ```
/// # use vergen::{Config, Error, vergen};
/// #
/// # fn main() -> std::result::Result<(), Error> {
/// // Generate the default 'cargo:' instruction output
/// vergen(Config::default())?;
/// #   Ok(())
/// # }
/// ```
#[cfg(not(feature = "git"))]
pub fn vergen(config: crate::Config) -> Result<()> {
    // This is here to help with type inference
    let no_repo: Option<&'static str> = None;
    config_from_instructions(config, no_repo, &mut io::stdout())
}

/// Generate the `cargo:` instructions
///
/// # Errors
///
/// Any generated errors will be wrapped in [`vergen::Error`](crate::error::Error)
///
/// # Usage
///
/// ```
/// # use vergen::{ConstantsFlags, Error, gen};
/// #
/// # fn main() -> std::result::Result<(), Error> {
/// // Generate the default 'cargo:' instruction output
/// gen(ConstantsFlags::all())?;
/// #   Ok(())
/// # }
/// ```
#[cfg(feature = "git")]
#[deprecated(since = "4.2.0", note = "Please use the `vergen` function instead")]
pub fn gen(flags: ConstantsFlags) -> Result<()> {
    config_from_flags(flags, Some("."), &mut io::stdout())
}

/// Generate the `cargo:` instructions
///
/// # Errors
///
/// Any generated errors will be wrapped in [`vergen::Error`](crate::error::Error)
///
/// # Usage
///
/// ```
/// # use vergen::{Config, Error, vergen};
/// #
/// # fn main() -> std::result::Result<(), Error> {
/// // Generate the default 'cargo:' instruction output
/// vergen(Config::default())?;
/// #   Ok(())
/// # }
/// ```
#[cfg(feature = "git")]
pub fn vergen(config: crate::Config) -> Result<()> {
    config_from_instructions(config, Some("."), &mut io::stdout())
}

fn config_from_flags<T, U>(flags: ConstantsFlags, repo: Option<U>, stdout: &mut T) -> Result<()>
where
    T: Write,
    U: AsRef<Path>,
{
    output_cargo_instructions(&Config::build(flags, repo)?, stdout)
}

fn config_from_instructions<T, U>(
    instructions: Instructions,
    repo: Option<U>,
    stdout: &mut T,
) -> Result<()>
where
    T: Write,
    U: AsRef<Path>,
{
    output_cargo_instructions(&instructions.config(repo)?, stdout)
}

fn output_cargo_instructions<T>(config: &Config, stdout: &mut T) -> Result<()>
where
    T: Write,
{
    // Generate the 'cargo:' instruction output
    for (k, v) in config.cfg_map().iter().filter_map(some_vals) {
        writeln!(stdout, "cargo:rustc-env={}={}", k.name(), v)?;
    }

    // Add the HEAD path to cargo:rerun-if-changed
    if let Some(head_path) = config.head_path() {
        writeln!(stdout, "cargo:rerun-if-changed={}", head_path.display())?;
    }

    // Add the resolved ref path to cargo:rerun-if-changed
    if let Some(ref_path) = config.ref_path() {
        writeln!(stdout, "cargo:rerun-if-changed={}", ref_path.display())?;
    }

    Ok(())
}

fn some_vals<'a>(tuple: (&'a VergenKey, &'a Option<String>)) -> Option<(&VergenKey, &String)> {
    if tuple.1.is_some() {
        Some((tuple.0, tuple.1.as_ref().unwrap()))
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    #[allow(deprecated)]
    use super::gen;
    use super::{config_from_flags, vergen};
    use crate::{
        config::Instructions,
        constants::ConstantsFlags,
        error::Result,
        testutils::{setup, teardown},
    };
    use lazy_static::lazy_static;
    use regex::Regex;
    use std::{io, path::PathBuf};

    lazy_static! {
        static ref VBD_REGEX: Regex = Regex::new(r".*VERGEN_BUILD_DATE.*").unwrap();
    }

    #[cfg(feature = "build")]
    lazy_static! {
        static ref DATE_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_BUILD_DATE=\d{4}-\d{2}-\d{2}"#;
        static ref TIMESTAMP_RE_STR: &'static str = r#"cargo:rustc-env=VERGEN_BUILD_TIMESTAMP=([0-9]+)-(0[1-9]|1[012])-(0[1-9]|[12][0-9]|3[01])[Tt]([01][0-9]|2[0-3]):([0-5][0-9]):([0-5][0-9]|60)(\.[0-9]+)?(([Zz])|([\+|\-]([01][0-9]|2[0-3]):[0-5][0-9]))"#;
        static ref CARGO_SEMVER_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_GIT_SEMVER=\d{1}\.\d{1}\.\d{1}"#;
        static ref BUILD_REGEX: Regex = {
            let re_str = vec![*DATE_RE_STR, *TIMESTAMP_RE_STR].join("\n");
            Regex::new(&re_str).unwrap()
        };
    }

    #[cfg(all(feature = "build", not(feature = "git")))]
    lazy_static! {
        static ref BUILD_CARGO_REGEX: Regex = {
            let re_str = vec![*DATE_RE_STR, *TIMESTAMP_RE_STR, *CARGO_SEMVER_RE_STR].join("\n");
            Regex::new(&re_str).unwrap()
        };
    }

    #[cfg(feature = "cargo")]
    lazy_static! {
        static ref CARGO_TT_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_CARGO_TARGET_TRIPLE=[a-zA-Z0-9-_]+"#;
        static ref CARGO_PROF_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_CARGO_PROFILE=[a-zA-Z0-9-_]+"#;
        static ref CARGO_FEA_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_CARGO_FEATURES=[a-zA-Z0-9-_]+,[a-zA-Z0-9-_]+"#;
        static ref CARGO_REGEX: Regex = {
            let re_str = vec![*CARGO_TT_RE_STR, *CARGO_PROF_RE_STR, *CARGO_FEA_RE_STR].join("\n");
            Regex::new(&re_str).unwrap()
        };
    }

    #[cfg(feature = "git")]
    lazy_static! {
        static ref GIT_BRANCH_RE_STR: &'static str = r#"cargo:rustc-env=VERGEN_GIT_BRANCH=.*"#;
        static ref GIT_CD_RE_STR: &'static str = r#"cargo:rustc-env=VERGEN_GIT_COMMIT_DATE=([0-9]+)-(0[1-9]|1[012])-(0[1-9]|[12][0-9]|3[01])[Tt]([01][0-9]|2[0-3]):([0-5][0-9]):([0-5][0-9]|60)(\.[0-9]+)?(([Zz])|([\+|\-]([01][0-9]|2[0-3]):[0-5][0-9]))"#;
        static ref GIT_SEMVER_RE_STR: &'static str = r#"cargo:rustc-env=VERGEN_GIT_SEMVER=(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?"#;
        static ref GIT_SL_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_GIT_SEMVER_LIGHTWEIGHT=.*"#;
        static ref GIT_SHA_RE_STR: &'static str = r#"cargo:rustc-env=VERGEN_GIT_SHA=[0-9a-f]{40}"#;
        static ref GIT_SHA_SHORT_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_GIT_SHA_SHORT=[0-9a-f]{7}"#;
        static ref GIT_RIC_RE_STR: &'static str = r#"cargo:rerun-if-changed=.*\.git/HEAD"#;
        static ref GIT_RIC1_RE_STR: &'static str = r#"cargo:rerun-if-changed=.*"#;
        static ref GIT_REGEX: Regex = {
            let re_str = vec![
                *GIT_BRANCH_RE_STR,
                *GIT_CD_RE_STR,
                *GIT_SEMVER_RE_STR,
                *GIT_SL_RE_STR,
                *GIT_SHA_RE_STR,
                *GIT_SHA_SHORT_RE_STR,
            ]
            .join("\n");
            Regex::new(&re_str).unwrap()
        };
        static ref GIT_RIC_REGEX: Regex = {
            let re_str = vec![*GIT_RIC_RE_STR, *GIT_RIC1_RE_STR].join("\n");
            Regex::new(&re_str).unwrap()
        };
    }

    #[cfg(feature = "rustc")]
    lazy_static! {
        static ref RUSTC_CHANNEL_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_RUSTC_CHANNEL=.*"#;
        static ref RUSTC_CD_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_RUSTC_COMMIT_DATE=\d{4}-\d{2}-\d{2}"#;
        static ref RUSTC_CH_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_RUSTC_COMMIT_HASH=[0-9a-f]{40}"#;
        static ref RUSTC_HT_RE_STR: &'static str = r#"cargo:rustc-env=VERGEN_RUSTC_HOST_TRIPLE=.*"#;
        static ref RUSTC_LLVM_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_RUSTC_LLVM_VERSION=11.0"#;
        static ref RUSTC_SEMVER_RE_STR: &'static str = r#"cargo:rustc-env=VERGEN_RUSTC_SEMVER=(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?"#;
        static ref RUSTC_NIGHTLY_REGEX: Regex = {
            let re_str = vec![
                *RUSTC_CHANNEL_RE_STR,
                *RUSTC_CD_RE_STR,
                *RUSTC_CH_RE_STR,
                *RUSTC_HT_RE_STR,
                *RUSTC_LLVM_RE_STR,
                *RUSTC_SEMVER_RE_STR,
            ]
            .join("\n");
            Regex::new(&re_str).unwrap()
        };
    }

    #[cfg(feature = "rustc")]
    #[rustversion::stable]
    lazy_static! {
        static ref RUSTC_REGEX: Regex = {
            let re_str = vec![
                *RUSTC_CHANNEL_RE_STR,
                *RUSTC_CD_RE_STR,
                *RUSTC_CH_RE_STR,
                *RUSTC_HT_RE_STR,
                *RUSTC_SEMVER_RE_STR,
            ]
            .join("\n");
            Regex::new(&re_str).unwrap()
        };
    }

    #[test]
    #[serial_test::serial]
    #[allow(deprecated)]
    fn gen_works() -> Result<()> {
        setup();
        assert!(gen(ConstantsFlags::all()).is_ok());
        teardown();
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn vergen_works() -> Result<()> {
        setup();
        assert!(vergen(Instructions::default()).is_ok());
        teardown();
        Ok(())
    }

    #[test]
    fn describe_falls_back() -> Result<()> {
        let no_tags_path = PathBuf::from("testdata").join("notagsrepo");
        assert!(
            config_from_flags(ConstantsFlags::all(), Some(no_tags_path), &mut io::sink(),).is_ok()
        );
        Ok(())
    }

    #[test]
    fn describe() -> Result<()> {
        let no_tags_path = PathBuf::from("testdata").join("tagsrepo");
        assert!(
            config_from_flags(ConstantsFlags::all(), Some(no_tags_path), &mut io::sink(),).is_ok()
        );
        Ok(())
    }

    #[test]
    fn detached_head() -> Result<()> {
        let dh_path = PathBuf::from("testdata").join("detachedhead");
        assert!(config_from_flags(ConstantsFlags::all(), Some(dh_path), &mut io::sink(),).is_ok());
        Ok(())
    }

    // TODO: Make this a macro to check all toggles
    #[test]
    fn toggle_works() -> Result<()> {
        let repo_path = PathBuf::from(".");
        let mut flags = ConstantsFlags::all();
        flags.toggle(ConstantsFlags::BUILD_DATE);

        let mut stdout_buf = vec![];
        assert!(config_from_flags(flags, Some(repo_path), &mut stdout_buf).is_ok());
        let stdout = String::from_utf8_lossy(&stdout_buf);
        assert!(!VBD_REGEX.is_match(&stdout));
        Ok(())
    }

    #[cfg(all(
        not(feature = "build"),
        not(feature = "cargo"),
        not(feature = "git"),
        not(feature = "rustc"),
    ))]
    #[test]
    fn no_features_no_output() {
        let repo_path = PathBuf::from(".");
        let mut stdout_buf = vec![];
        assert!(
            config_from_flags(ConstantsFlags::all(), Some(repo_path), &mut stdout_buf,).is_ok()
        );
        assert!(stdout_buf.is_empty());
    }

    #[cfg(all(feature = "build", not(feature = "git")))]
    #[test]
    fn contains_only_build_output() {
        let repo_path = PathBuf::from(".");
        let mut stdout_buf = vec![];
        assert!(
            config_from_flags(ConstantsFlags::all(), Some(repo_path), &mut stdout_buf,).is_ok()
        );
        assert!(BUILD_CARGO_REGEX.is_match(&String::from_utf8_lossy(&stdout_buf)));
    }

    #[cfg(all(feature = "build", feature = "git"))]
    #[test]
    fn contains_build_output() {
        let repo_path = PathBuf::from(".");
        let mut stdout_buf = vec![];
        assert!(
            config_from_flags(ConstantsFlags::all(), Some(repo_path), &mut stdout_buf,).is_ok()
        );
        assert!(BUILD_REGEX.is_match(&String::from_utf8_lossy(&stdout_buf)));
    }

    #[cfg(feature = "cargo")]
    #[test]
    #[serial_test::serial]
    fn contains_cargo_output() {
        setup();
        let repo_path = PathBuf::from(".");
        let mut stdout_buf = vec![];
        assert!(
            config_from_flags(ConstantsFlags::all(), Some(repo_path), &mut stdout_buf,).is_ok()
        );
        assert!(CARGO_REGEX.is_match(&String::from_utf8_lossy(&stdout_buf)));
        teardown();
    }

    #[cfg(feature = "git")]
    #[test]
    fn contains_git_output() {
        let repo_path = PathBuf::from(".");
        let mut stdout_buf = vec![];
        assert!(
            config_from_flags(ConstantsFlags::all(), Some(repo_path), &mut stdout_buf,).is_ok()
        );
        assert!(GIT_REGEX.is_match(&String::from_utf8_lossy(&stdout_buf)));
        assert!(GIT_RIC_REGEX.is_match(&String::from_utf8_lossy(&stdout_buf)));
    }

    #[cfg(feature = "rustc")]
    #[test]
    fn contains_rustc_output() {
        let repo_path = PathBuf::from(".");
        let mut stdout_buf = vec![];
        assert!(
            config_from_flags(ConstantsFlags::all(), Some(repo_path), &mut stdout_buf,).is_ok()
        );
        check_rustc_output(&stdout_buf);
    }

    #[cfg(feature = "rustc")]
    #[rustversion::nightly]
    fn check_rustc_output(stdout: &[u8]) {
        assert!(RUSTC_NIGHTLY_REGEX.is_match(&String::from_utf8_lossy(&stdout)));
    }

    // TODO: Check this on new beta releases, the regex was causing a panic
    // outside of my control
    #[cfg(feature = "rustc")]
    #[rustversion::beta]
    fn check_rustc_output(stdout: &[u8]) {
        assert!(!stdout.is_empty());
    }

    #[cfg(feature = "rustc")]
    #[rustversion::stable]
    fn check_rustc_output(stdout: &[u8]) {
        assert!(RUSTC_REGEX.is_match(&String::from_utf8_lossy(&stdout)));
    }

    #[cfg(all(feature = "build", feature = "git"))]
    #[test]
    fn blah() {
        use super::config_from_instructions;
        use crate::{TimeZone, TimestampKind};

        let repo_path = PathBuf::from(".");
        let mut stdout_buf = vec![];
        let mut config = Instructions::default();
        let _ = config.timezone(TimeZone::Local);
        let _ = config.ts_kind(TimestampKind::TimeOnly);
        assert!(config_from_instructions(config, Some(repo_path), &mut stdout_buf,).is_ok());
    }
}
