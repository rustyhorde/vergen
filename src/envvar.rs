// Copyright (c) 2016, 2018 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Build time information.
use chrono::Utc;
use constants::*;
use error::Result;
use std::collections::HashMap;
use std::env;
use std::process::Command;

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
            build_info.insert(
                VergenKey::CommitDate,
                commit_date.trim_matches('\'').to_string(),
            );
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

fn run_command(command: &mut Command) -> String {
    let raw_output = if let Ok(o) = command.output() {
        String::from_utf8_lossy(&o.stdout).into_owned()
    } else {
        "UNKNOWN".to_string()
    };
    raw_output.trim().to_string()
}

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

#[cfg(test)]
mod test {
    use super::{Vergen, VergenKey};
    use constants::ConstantsFlags;
    use regex::Regex;
    use std::collections::HashMap;
    use std::env;

    lazy_static! {
        static ref REGEX_MAP: HashMap<VergenKey, Regex> = {
            let mut regex_map = HashMap::new();
            let timestamp_re =
                Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.(\d+)\+\d{2}:\d{2}$")
                    .expect("Unable to create timestamp regex!");
            let date_re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").expect("Unable to create date regex!");
            let commit_date_re = Regex::new(r"^\d{4}-\d{2}-\d{2}$|^UNKNOWN$")
                .expect("Unable to create commit date regex!");
            let sha_re =
                Regex::new(r"^[a-z0-9]{40}$|^UNKNOWN$").expect("Unable to create SHA regex!");
            let short_sha_re =
                Regex::new(r"^[a-z0-9]{7}|^UNKNOWN$").expect("Unable to create short SHA regex!");
            let target_triple_re =
                Regex::new(r"^[a-z0-9_]+-[a-z0-9_]+-[a-z0-9_]+-[a-z0-9_]+$|^UNKNOWN$")
                    .expect("Unable to create target triple regex!");
            let semver_re = Regex::new(r"^v*\d+\.\d+\.\d+([-a-z.0-9]+)?$")
                .expect("Unable to create semver regex!");

            regex_map.insert(VergenKey::BuildTimestamp, timestamp_re);
            regex_map.insert(VergenKey::BuildDate, date_re.clone());
            regex_map.insert(VergenKey::Sha, sha_re);
            regex_map.insert(VergenKey::ShortSha, short_sha_re);
            regex_map.insert(VergenKey::CommitDate, commit_date_re);
            regex_map.insert(VergenKey::TargetTriple, target_triple_re);
            regex_map.insert(VergenKey::Semver, semver_re.clone());
            regex_map.insert(VergenKey::SemverLightweight, semver_re);

            regex_map
        };
    }

    #[test]
    fn test_build_info_all() {
        let flags = ConstantsFlags::all();
        let vergen = Vergen::new(flags).expect("Unable to create Vergen!");
        let build_info = vergen.build_info();

        assert!(build_info.contains_key(&VergenKey::BuildTimestamp));
        assert!(build_info.contains_key(&VergenKey::BuildDate));
        assert!(build_info.contains_key(&VergenKey::Sha));
        assert!(build_info.contains_key(&VergenKey::ShortSha));
        assert!(build_info.contains_key(&VergenKey::CommitDate));
        assert!(build_info.contains_key(&VergenKey::TargetTriple));
        assert!(build_info.contains_key(&VergenKey::Semver));
        assert!(build_info.contains_key(&VergenKey::SemverLightweight));
    }

    #[test]
    fn test_build_info_some() {
        let mut flags = ConstantsFlags::all();
        flags.toggle(ConstantsFlags::COMMIT_DATE);
        flags.toggle(ConstantsFlags::SEMVER_LIGHTWEIGHT);
        let vergen = Vergen::new(flags).expect("Unable to create Vergen!");
        let build_info = vergen.build_info();

        assert!(build_info.contains_key(&VergenKey::BuildTimestamp));
        assert!(build_info.contains_key(&VergenKey::BuildDate));
        assert!(build_info.contains_key(&VergenKey::Sha));
        assert!(build_info.contains_key(&VergenKey::ShortSha));
        assert!(!build_info.contains_key(&VergenKey::CommitDate));
        assert!(build_info.contains_key(&VergenKey::TargetTriple));
        assert!(build_info.contains_key(&VergenKey::Semver));
        assert!(!build_info.contains_key(&VergenKey::SemverLightweight));
    }

    #[test]
    fn test_build_info_none() {
        let flags = ConstantsFlags::empty();
        let vergen = Vergen::new(flags).expect("Unable to create Vergen!");
        let build_info = vergen.build_info();

        assert!(!build_info.contains_key(&VergenKey::BuildTimestamp));
        assert!(!build_info.contains_key(&VergenKey::BuildDate));
        assert!(!build_info.contains_key(&VergenKey::Sha));
        assert!(!build_info.contains_key(&VergenKey::ShortSha));
        assert!(!build_info.contains_key(&VergenKey::CommitDate));
        assert!(!build_info.contains_key(&VergenKey::TargetTriple));
        assert!(!build_info.contains_key(&VergenKey::Semver));
        assert!(!build_info.contains_key(&VergenKey::SemverLightweight));
    }

    fn assert_on_data(build_info: &HashMap<VergenKey, String>, key: &VergenKey, desc: &str) {
        if let Some(regex) = REGEX_MAP.get(key) {
            if let Some(info) = build_info.get(key) {
                assert!(regex.is_match(info), format!("{} did not match!", info));
            } else {
                assert!(false, format!("The {} wasn't properly set", desc));
            }
        } else {
            assert!(false, format!("No regex defined for {}", key.name()));
        }
    }

    #[test]
    fn test_build_info_data() {
        env::set_var("TARGET", "x86_64-unknown-linux-gnu");
        let flags = ConstantsFlags::all();
        let vergen = Vergen::new(flags).expect("Unable to create Vergen!");
        let build_info = vergen.build_info();

        assert_on_data(&build_info, &VergenKey::BuildTimestamp, "build timestamp");
        assert_on_data(&build_info, &VergenKey::BuildDate, "build date");
        assert_on_data(&build_info, &VergenKey::Sha, "SHA");
        assert_on_data(&build_info, &VergenKey::ShortSha, "short SHA");
        assert_on_data(&build_info, &VergenKey::CommitDate, "commit date");
        assert_on_data(&build_info, &VergenKey::TargetTriple, "target triple");
        assert_on_data(&build_info, &VergenKey::Semver, "semver");
        assert_on_data(
            &build_info,
            &VergenKey::SemverLightweight,
            "lightweight semver",
        );
    }
}
