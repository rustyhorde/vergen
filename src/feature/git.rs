// Copyright (c) 2016, 2018, 2021 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `vergen` git feature implementation

use crate::{
    config::Config,
    constants::ConstantsFlags,
    error::Result,
    feature::{add_build_config, add_rustc_config},
};
use std::io::{self, Write};
#[cfg(feature = "git")]
use {
    crate::{feature::add_entry, output::VergenKey},
    chrono::{FixedOffset, TimeZone},
    git2::{BranchType, DescribeOptions, Repository},
    std::env,
};

impl Config {
    #[cfg(not(feature = "git"))]
    pub(crate) fn build(flags: ConstantsFlags) -> Result<Config> {
        let mut config = Config::default();

        add_build_config(flags, &mut config);
        add_rustc_config(flags, &mut config)?;

        Ok(config)
    }

    #[cfg(feature = "git")]
    pub(crate) fn build(flags: ConstantsFlags, repo: &Repository) -> Result<Config> {
        let mut config = Config::default();

        add_build_config(flags, &mut config);
        add_git_config(flags, repo, &mut config)?;
        add_rustc_config(flags, &mut config)?;

        Ok(config)
    }
}

#[cfg(feature = "git")]
fn add_git_config(flags: ConstantsFlags, repo: &Repository, config: &mut Config) -> Result<()> {
    if flags.intersects(
        ConstantsFlags::BRANCH
            | ConstantsFlags::COMMIT_DATE
            | ConstantsFlags::SEMVER
            | ConstantsFlags::SEMVER_LIGHTWEIGHT
            | ConstantsFlags::SHA
            | ConstantsFlags::SHA_SHORT,
    ) {
        let ref_head = repo.find_reference("HEAD")?;
        let commit = ref_head.peel_to_commit()?;

        if flags.contains(ConstantsFlags::BRANCH) {
            add_branch_name(&repo, config)?;
        }

        if flags.contains(ConstantsFlags::COMMIT_DATE) {
            let offset = FixedOffset::east(commit.time().offset_minutes() * 60)
                .timestamp(commit.time().seconds(), 0);
            add_entry(
                config.cfg_map_mut(),
                VergenKey::CommitDate,
                Some(offset.to_rfc3339()),
            );
        }

        if flags.contains(ConstantsFlags::SEMVER) {
            add_semver(&repo, &DescribeOptions::new(), false, config);
        }

        if flags.contains(ConstantsFlags::SEMVER_LIGHTWEIGHT) {
            let mut opts = DescribeOptions::new();
            let _ = opts.describe_tags();

            add_semver(&repo, &opts, true, config);
        }

        if flags.contains(ConstantsFlags::SHA) {
            add_entry(
                config.cfg_map_mut(),
                VergenKey::Sha,
                Some(commit.id().to_string()),
            );
        }

        if flags.contains(ConstantsFlags::SHA_SHORT) {
            let obj = repo.revparse_single("HEAD")?;
            add_entry(
                config.cfg_map_mut(),
                VergenKey::ShortSha,
                obj.short_id()?.as_str().map(str::to_string),
            );
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
    match repo.describe(opts) {
        Ok(describe) => {
            add_entry(config.cfg_map_mut(), key, describe.format(None).ok());
        }
        Err(_e) => {
            add_entry(
                config.cfg_map_mut(),
                key,
                env::var("CARGO_PKG_VERSION").ok(),
            );
        }
    }
}

/// Some Docs
///
/// # Errors
///
#[cfg(feature = "git")]
pub fn gen(flags: ConstantsFlags) -> Result<()> {
    gen_cargo_instructions(
        flags,
        &Repository::discover(".")?,
        &mut io::stdout(),
        &mut io::stderr(),
    )
}

/// Some Docs
///
/// # Errors
///
#[cfg(not(feature = "git"))]
pub fn gen(flags: ConstantsFlags) -> Result<()> {
    gen_cargo_instructions(flags, &mut io::stdout(), &mut io::stderr())
}

#[cfg(feature = "git")]
fn gen_cargo_instructions<T, U>(
    flags: ConstantsFlags,
    repo: &Repository,
    _stdout: &mut T,
    _stderr: &mut U,
) -> Result<()>
where
    T: Write,
    U: Write,
{
    let _config = Config::build(flags, repo)?;

    Ok(())
}

#[cfg(not(feature = "git"))]
fn gen_cargo_instructions<T, U>(
    flags: ConstantsFlags,
    _stdout: &mut T,
    _stderr: &mut U,
) -> Result<()>
where
    T: Write,
    U: Write,
{
    let _config = Config::build(flags)?;

    Ok(())
}

#[cfg(all(test, feature = "git"))]
mod test {
    use super::{add_git_config, gen};
    use crate::{config::Config, constants::ConstantsFlags, error::Result, output::VergenKey};
    use git2::Repository;
    use std::collections::HashMap;

    fn check_git_keys(cfg_map: &HashMap<VergenKey, Option<String>>) {
        let mut count = 0;
        for (k, v) in cfg_map {
            match *k {
                VergenKey::Branch
                | VergenKey::CommitDate
                | VergenKey::Semver
                | VergenKey::SemverLightweight
                | VergenKey::Sha
                | VergenKey::ShortSha => {
                    assert!(v.is_some(), format!("value is None for key '{:?}'", *k));
                    count += 1;
                }
                _ => {}
            }
        }
        assert_eq!(count, 6);
    }

    #[test]
    fn add_git_config_works() -> Result<()> {
        let repo = Repository::discover(".")?;
        let mut config = Config::default();
        add_git_config(ConstantsFlags::all(), &repo, &mut config)?;
        check_git_keys(config.cfg_map());
        Ok(())
    }

    #[test]
    fn gen_works() -> Result<()> {
        assert!(gen(ConstantsFlags::all()).is_ok());
        Ok(())
    }
}

#[cfg(all(test, not(feature = "git")))]
mod test {
    use super::gen;
    use crate::{constants::ConstantsFlags, error::Result};

    #[test]
    fn gen_works() -> Result<()> {
        assert!(gen(ConstantsFlags::all()).is_ok());
        Ok(())
    }
}
