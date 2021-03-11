// Copyright (c) 2016, 2018, 2021 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `vergen` git feature implementation

use crate::config::{Config, Instructions};
use anyhow::Result;
use std::path::Path;
#[cfg(feature = "git")]
use {
    crate::{
        config::VergenKey,
        error::Error,
        feature::{self, add_entry, TimestampKind},
    },
    chrono::{DateTime, FixedOffset, Local, TimeZone, Utc},
    getset::{Getters, MutGetters},
    git2::{BranchType, DescribeOptions, Repository},
    std::env,
};

/// The semver kind to output
#[cfg(feature = "git")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SemverKind {
    /// Output the `git describe` kind
    Normal,
    /// Output the `git describe` kind including lightweight tags
    Lightweight,
}

/// The SHA kind to output
#[cfg(feature = "git")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShaKind {
    /// Output the normal 40 digit SHA
    Normal,
    /// Output the short 7 digit SHA
    Short,
}

/// Configuration for the `VERGEN_GIT_*` instructions
///
/// # Instructions
/// The following instructions can be generated:
///
/// | Instruction | Default |
/// | ----------- | :-----: |
/// | `cargo:rustc-env=VERGEN_GIT_BRANCH=feature/git2` | * |
/// | `cargo:rustc-env=VERGEN_GIT_COMMIT_DATE=2021-02-12` | |
/// | `cargo:rustc-env=VERGEN_GIT_COMMIT_TIME=01:54:15` | |
/// | `cargo:rustc-env=VERGEN_GIT_COMMIT_TIMESTAMP=2021-02-12T01:54:15.134750+00:00` | * |
/// | `cargo:rustc-env=VERGEN_GIT_SEMVER=v3.2.0-86-g95fc0f5d` | * |
/// | `cargo:rustc-env=VERGEN_GIT_SHA=95fc0f5d066710f16e0c23ce3239d6e040abca0d` | * |
/// | `cargo:rerun-if-changed=/Users/yoda/projects/rust-lang/vergen/.git/HEAD` | * |
/// | `cargo:rerun-if-changed=/Users/yoda/projects/rust-lang/vergen/.git/refs/heads/feature/git2` | * |
///
/// * If the `branch` field is false, the `VERGEN_GIT_BRANCH` instruction will not be generated.
/// * If the `commit_timestamp` field is false, the date/time instructions will not be generated.
/// * If the `rerun_on_head_changed` field is false, the `cargo:rerun-if-changed` instructions will not be generated.
/// * If the `semver` field is false, the `VERGEN_GIT_SEMVER` instruction will not be generated.
/// * If the `sha` field is fale, the `VERGEN_GIT_SHA` instruction will not be generated.
/// * **NOTE** - The SHA defaults to the [`Normal`](ShaKind::Normal) variant, but can be changed via the `sha_kind` field.
/// * **NOTE** - The [SemVer] defaults to the [`Normal`](SemverKind::Normal) variant, but can be changed via the `semver_kind` field.
/// * **NOTE** - The [SemVer] is only useful if you have tags on your repository.  If your repository has no tags, this will default to [`CARGO_PKG_VERSION`].
/// * **NOTE** - The [`Lightweight`](SemverKind::Lightweight) variant will only differ from the [`Normal`](SemverKind::Normal) variant if you use [lightweight] tags in your repository.
/// * **NOTE** - By default, the date/time related instructions will use [`UTC`](crate::TimeZone::Utc).
/// * **NOTE** - The date/time instruction output is determined by the [`kind`](crate::TimestampKind) field and can be any combination of the three.
/// * **NOTE** - If the `rerun_on_head_chaged` instructions are enabled, cargo` will re-run the build script when either `&lt;gitpath&gt;/HEAD` or the file that `&lt;gitpath&gt;/HEAD` points at changes.
///
/// # Example
///
/// ```
/// # use anyhow::Result;
/// use vergen::{vergen, Config};
#[cfg_attr(feature = "git", doc = r##"use vergen::ShaKind;"##)]
///
/// # pub fn main() -> Result<()> {
/// let mut config = Config::default();
#[cfg_attr(
    feature = "git",
    doc = r##"
// Change the SHA output to the short variant
*config.git_mut().sha_kind_mut() = ShaKind::Short;

// Generate the instructions
vergen(config)?;
"##
)]
/// # Ok(())
/// # }
/// ```
///
/// [SemVer]: https://semver.org/
/// [lightweight]: https://git-scm.com/book/en/v2/Git-Basics-Tagging
/// [`CARGO_PKG_VERSION`]: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates
///
#[cfg(feature = "git")]
#[derive(Clone, Copy, Debug, Getters, MutGetters)]
#[getset(get = "pub(crate)", get_mut = "pub")]
pub struct Git {
    /// Enable/Disable the `VERGEN_GIT_BRANCH` instruction
    branch: bool,
    /// Enable/Disable the `VERGEN_GIT_COMMIT_DATE`, `VERGEN_GIT_COMMIT_TIME`, and `VERGEN_GIT_COMMIT_TIMESTAMP` instructions
    commit_timestamp: bool,
    /// The timezone to use for the date/time instructions.
    commit_timestamp_timezone: feature::TimeZone,
    /// The kind of date/time instructions to output.
    commit_timestamp_kind: TimestampKind,
    /// Enable/Disable the `cargo:rerun-if-changed` instructions
    rerun_on_head_change: bool,
    /// Enable/Disable the `VERGEN_GIT_SEMVER` instruction
    semver: bool,
    /// The kind of semver instruction to output.
    semver_kind: SemverKind,
    /// Enable/Disable the `VERGEN_GIT_SHA` instruction
    sha: bool,
    /// The kind of SHA instruction to output.
    sha_kind: ShaKind,
}

#[cfg(feature = "git")]
impl Default for Git {
    fn default() -> Self {
        Self {
            branch: true,
            commit_timestamp: true,
            commit_timestamp_timezone: feature::TimeZone::Utc,
            commit_timestamp_kind: TimestampKind::Timestamp,
            rerun_on_head_change: true,
            semver: true,
            semver_kind: SemverKind::Normal,
            sha: true,
            sha_kind: ShaKind::Normal,
        }
    }
}

#[cfg(feature = "git")]
impl Git {
    pub(crate) fn has_enabled(&self) -> bool {
        self.branch || self.commit_timestamp || self.rerun_on_head_change || self.semver || self.sha
    }
}

#[cfg(not(feature = "git"))]
pub(crate) fn configure_git<T>(
    _instructions: Instructions,
    _repo: Option<T>,
    _config: &mut Config,
) -> Result<()>
where
    T: AsRef<Path>,
{
    Ok(())
}

#[cfg(feature = "git")]
pub(crate) fn configure_git<T>(
    instructions: Instructions,
    repo_path_opt: Option<T>,
    config: &mut Config,
) -> Result<()>
where
    T: AsRef<Path>,
{
    if let Some(repo_path) = repo_path_opt {
        let git_config = instructions.git();
        if git_config.has_enabled() {
            let repo = Repository::discover(repo_path)?;
            let ref_head = repo.find_reference("HEAD")?;
            let repo_path = repo.path().to_path_buf();

            if *git_config.branch() {
                add_branch_name(&repo, config)?;
            }

            if *git_config.commit_timestamp() || *git_config.sha() {
                let commit = ref_head.peel_to_commit()?;

                if *git_config.commit_timestamp() {
                    let offset = if commit.time().sign() == '-' {
                        FixedOffset::west(commit.time().offset_minutes() * 60)
                            .timestamp(commit.time().seconds(), 0)
                    } else {
                        FixedOffset::east(commit.time().offset_minutes() * 60)
                            .timestamp(commit.time().seconds(), 0)
                    };

                    match git_config.commit_timestamp_timezone() {
                        crate::TimeZone::Utc => {
                            add_config_entries(config, *git_config, &offset.with_timezone(&Utc))
                        }
                        crate::TimeZone::Local => {
                            add_config_entries(config, *git_config, &offset.with_timezone(&Local))
                        }
                    }
                }

                if *git_config.sha() {
                    match git_config.sha_kind() {
                        crate::ShaKind::Normal => {
                            add_entry(
                                config.cfg_map_mut(),
                                VergenKey::Sha,
                                Some(commit.id().to_string()),
                            );
                        }
                        crate::ShaKind::Short => {
                            let obj = repo.revparse_single("HEAD")?;
                            add_entry(
                                config.cfg_map_mut(),
                                VergenKey::ShortSha,
                                obj.short_id()?.as_str().map(str::to_string),
                            );
                        }
                    }
                }
            }

            if *git_config.semver() {
                match *git_config.semver_kind() {
                    crate::SemverKind::Normal => {
                        add_semver(&repo, &DescribeOptions::new(), false, config)
                    }
                    crate::SemverKind::Lightweight => {
                        let mut opts = DescribeOptions::new();
                        let _ = opts.describe_tags();

                        add_semver(&repo, &opts, true, config);
                    }
                }
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
fn add_config_entries<T>(config: &mut Config, git_config: Git, now: &DateTime<T>)
where
    T: TimeZone,
    T::Offset: std::fmt::Display,
{
    match git_config.commit_timestamp_kind() {
        TimestampKind::DateOnly => add_date_entry(config, now),
        TimestampKind::TimeOnly => add_time_entry(config, now),
        TimestampKind::DateAndTime => {
            add_date_entry(config, now);
            add_time_entry(config, now);
        }
        TimestampKind::Timestamp => add_timestamp_entry(config, now),
        TimestampKind::All => {
            add_date_entry(config, now);
            add_time_entry(config, now);
            add_timestamp_entry(config, now);
        }
    }
}

#[cfg(feature = "git")]
fn add_date_entry<T>(config: &mut Config, now: &DateTime<T>)
where
    T: TimeZone,
    T::Offset: std::fmt::Display,
{
    add_entry(
        config.cfg_map_mut(),
        VergenKey::CommitDate,
        Some(now.format("%Y-%m-%d").to_string()),
    );
}

#[cfg(feature = "git")]
fn add_time_entry<T>(config: &mut Config, now: &DateTime<T>)
where
    T: TimeZone,
    T::Offset: std::fmt::Display,
{
    add_entry(
        config.cfg_map_mut(),
        VergenKey::CommitTime,
        Some(now.format("%H:%M:%S").to_string()),
    );
}

#[cfg(feature = "git")]
fn add_timestamp_entry<T>(config: &mut Config, now: &DateTime<T>)
where
    T: TimeZone,
    T::Offset: std::fmt::Display,
{
    add_entry(
        config.cfg_map_mut(),
        VergenKey::CommitTimestamp,
        Some(now.to_rfc3339()),
    );
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
    use super::{SemverKind, ShaKind};
    use crate::{
        config::Instructions,
        feature::{TimeZone, TimestampKind},
    };

    #[test]
    fn git_config() {
        let mut config = Instructions::default();
        assert!(config.git().branch);
        assert!(config.git().commit_timestamp);
        assert_eq!(config.git().commit_timestamp_timezone, TimeZone::Utc);
        assert_eq!(config.git().commit_timestamp_kind, TimestampKind::Timestamp);
        assert!(config.git().rerun_on_head_change);
        assert!(config.git().semver);
        assert_eq!(config.git().semver_kind, SemverKind::Normal);
        assert!(config.git().sha);
        assert_eq!(config.git().sha_kind, ShaKind::Normal);
        config.git_mut().commit_timestamp_kind = TimestampKind::All;
        assert_eq!(config.git().commit_timestamp_kind, TimestampKind::All);
    }
}

#[cfg(all(test, not(feature = "git")))]
mod test {}
