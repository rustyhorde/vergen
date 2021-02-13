// Copyright (c) 2016, 2018, 2021 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `vergen` git feature implementation

use crate::{config::Config, constants::ConstantsFlags, error::Result};
use std::path::Path;
#[cfg(feature = "git")]
use {
    crate::{config::VergenKey, error::Error, feature::add_entry},
    chrono::{FixedOffset, TimeZone},
    git2::{BranchType, DescribeOptions, Repository},
    std::env,
};

#[cfg(not(feature = "git"))]
pub(crate) fn add_git_config<T>(
    _flags: ConstantsFlags,
    _repo: Option<T>,
    _config: &mut Config,
) -> Result<()>
where
    T: AsRef<Path>,
{
    Ok(())
}

#[cfg(feature = "git")]
pub(crate) fn add_git_config<T>(
    flags: ConstantsFlags,
    repo_path_opt: Option<T>,
    config: &mut Config,
) -> Result<()>
where
    T: AsRef<Path>,
{
    if let Some(repo_path) = repo_path_opt {
        if flags.intersects(
            ConstantsFlags::BRANCH
                | ConstantsFlags::COMMIT_DATE
                | ConstantsFlags::SEMVER
                | ConstantsFlags::SEMVER_LIGHTWEIGHT
                | ConstantsFlags::SHA
                | ConstantsFlags::SHA_SHORT,
        ) {
            let repo = Repository::discover(repo_path)?;
            let ref_head = repo.find_reference("HEAD")?;
            let repo_path = repo.path().to_path_buf();

            if flags.contains(ConstantsFlags::BRANCH) {
                add_branch_name(&repo, config)?;
            }

            if flags.intersects(ConstantsFlags::COMMIT_DATE | ConstantsFlags::SHA) {
                let commit = ref_head.peel_to_commit()?;

                if flags.contains(ConstantsFlags::COMMIT_DATE) {
                    let offset = FixedOffset::east(commit.time().offset_minutes() * 60)
                        .timestamp(commit.time().seconds(), 0);
                    add_entry(
                        config.cfg_map_mut(),
                        VergenKey::CommitDate,
                        Some(offset.to_rfc3339()),
                    );
                }

                if flags.contains(ConstantsFlags::SHA) {
                    add_entry(
                        config.cfg_map_mut(),
                        VergenKey::Sha,
                        Some(commit.id().to_string()),
                    );
                }
            }

            if flags.contains(ConstantsFlags::SEMVER) {
                add_semver(&repo, &DescribeOptions::new(), false, config);
            }

            if flags.contains(ConstantsFlags::SEMVER_LIGHTWEIGHT) {
                let mut opts = DescribeOptions::new();
                let _ = opts.describe_tags();

                add_semver(&repo, &opts, true, config);
            }

            if flags.contains(ConstantsFlags::SHA_SHORT) {
                let obj = repo.revparse_single("HEAD")?;
                add_entry(
                    config.cfg_map_mut(),
                    VergenKey::ShortSha,
                    obj.short_id()?.as_str().map(str::to_string),
                );
            }

            if let Ok(resolved) = ref_head.resolve() {
                if let Some(name) = resolved.name() {
                    *config.ref_path_mut() = Some(repo_path.join(name));
                }
            }
            *config.head_path_mut() = Some(repo_path.join("HEAD"));
        }
    }

    Ok(())
}

#[cfg(feature = "git")]
fn add_branch_name(repo: &Repository, config: &mut Config) -> Result<()> {
    if repo.head_detached()? {
        add_entry(
            config.cfg_map_mut(),
            VergenKey::Branch,
            Some("detached HEAD".to_string()),
        );
    } else {
        let locals = repo.branches(Some(BranchType::Local))?;
        for (local, _bt) in locals.filter_map(std::result::Result::ok) {
            if local.is_head() {
                if let Some(name) = local.name()? {
                    add_entry(
                        config.cfg_map_mut(),
                        VergenKey::Branch,
                        Some(name.to_string()),
                    );
                }
            }
        }
    }
    Ok(())
}

#[cfg(feature = "git")]
fn add_semver(repo: &Repository, opts: &DescribeOptions, lw: bool, config: &mut Config) {
    let key = if lw {
        VergenKey::SemverLightweight
    } else {
        VergenKey::Semver
    };
    let semver: Option<String> = repo
        .describe(opts)
        .map_or_else(
            |_| env::var("CARGO_PKG_VERSION").map_err(Error::from),
            |x| x.format(None).map_err(Error::from),
        )
        .ok();
    add_entry(config.cfg_map_mut(), key, semver);
}

#[cfg(all(test, feature = "git"))]
mod test {
    use super::add_git_config;
    use crate::{
        config::{Config, VergenKey},
        constants::ConstantsFlags,
        error::Result,
        test::get_map_value,
    };
    use lazy_static::lazy_static;
    use regex::Regex;
    use std::{collections::BTreeMap, env, path::PathBuf};

    lazy_static! {
        static ref SHORT_SHA_REGEX: Regex = Regex::new(r"^[0-9a-f]{7}$").unwrap();
        static ref SHA_REGEX: Regex = Regex::new(r"^[0-9a-f]{40}$").unwrap();
        static ref SEMVER_REGEX: Regex = Regex::new(r"^(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$").unwrap();
        static ref RFC3339_REGEX: Regex = Regex::new(r"^([0-9]+)-(0[1-9]|1[012])-(0[1-9]|[12][0-9]|3[01])[Tt]([01][0-9]|2[0-3]):([0-5][0-9]):([0-5][0-9]|60)(\.[0-9]+)?(([Zz])|([\+|\-]([01][0-9]|2[0-3]):[0-5][0-9]))$").unwrap();
    }

    fn check_git_keys(cfg_map: &BTreeMap<VergenKey, Option<String>>) {
        let mut count = 0;
        for (k, v) in cfg_map {
            match *k {
                VergenKey::Branch
                | VergenKey::CommitDate
                | VergenKey::Semver
                | VergenKey::SemverLightweight
                | VergenKey::Sha
                | VergenKey::ShortSha => {
                    assert!(v.is_some());
                    count += 1;
                }
                _ => {}
            }
        }
        assert_eq!(count, 6);
    }

    fn check_git_instructions(cfg_map: &BTreeMap<VergenKey, Option<String>>) {
        assert!(SHORT_SHA_REGEX.is_match(&get_map_value(VergenKey::ShortSha, cfg_map)));
        assert!(SHA_REGEX.is_match(&get_map_value(VergenKey::Sha, cfg_map)));
        assert!(SEMVER_REGEX
            .is_match(&get_map_value(VergenKey::Semver, cfg_map).trim_start_matches("v")));
        assert!(SEMVER_REGEX.is_match(
            &get_map_value(VergenKey::SemverLightweight, cfg_map).trim_start_matches("v")
        ));
        assert!(RFC3339_REGEX.is_match(&get_map_value(VergenKey::CommitDate, cfg_map)));
    }

    #[test]
    fn semver_fallback_works() -> Result<()> {
        let mut config = Config::default();
        let no_tags_path = PathBuf::from("testdata").join("notagsrepo");
        add_git_config(ConstantsFlags::all(), Some(no_tags_path), &mut config)?;
        check_git_keys(config.cfg_map());
        assert_eq!(
            config
                .cfg_map()
                .get(&VergenKey::Semver)
                .map(|x| x.as_ref().unwrap().to_string()),
            env::var("CARGO_PKG_VERSION").ok()
        );
        check_git_instructions(config.cfg_map());
        assert!(config.ref_path().is_some());
        assert!(config.head_path().is_some());
        // Remove a key an check
        let _ = config.cfg_map_mut().remove(&VergenKey::BuildDate);
        assert!(get_map_value(VergenKey::BuildDate, config.cfg_map()).is_empty());
        Ok(())
    }

    #[test]
    fn git_describe_works() -> Result<()> {
        let mut config = Config::default();
        let tags_path = PathBuf::from("testdata").join("tagsrepo");
        add_git_config(ConstantsFlags::all(), Some(tags_path), &mut config)?;
        check_git_keys(config.cfg_map());
        assert!(
            config
                .cfg_map()
                .get(&VergenKey::Semver)
                .map(|x| x.as_ref().unwrap().to_string())
                != env::var("CARGO_PKG_VERSION").ok()
        );
        check_git_instructions(config.cfg_map());
        assert!(config.ref_path().is_some());
        assert!(config.head_path().is_some());
        // Remove a key an check
        let _ = config.cfg_map_mut().remove(&VergenKey::BuildDate);
        assert!(get_map_value(VergenKey::BuildDate, config.cfg_map()).is_empty());
        Ok(())
    }
}

#[cfg(all(test, not(feature = "git")))]
mod test {}
