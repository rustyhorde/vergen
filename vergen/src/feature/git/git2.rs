// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use crate::{
    constants::{
        GIT_BRANCH_NAME, GIT_COMMIT_AUTHOR_EMAIL, GIT_COMMIT_AUTHOR_NAME, GIT_COMMIT_COUNT,
        GIT_COMMIT_DATE_NAME, GIT_COMMIT_MESSAGE, GIT_COMMIT_TIMESTAMP_NAME, GIT_DESCRIBE_NAME,
        GIT_DIRTY_NAME, GIT_SHA_NAME,
    },
    emitter::{EmitBuilder, RustcEnvMap},
    key::VergenKey,
    utils::fns::{add_default_map_entry, add_map_entry},
};
#[cfg(test)]
use anyhow::anyhow;
use anyhow::{Error, Result};
use git2_rs::{
    BranchType, Commit, DescribeFormatOptions, DescribeOptions, Reference, Repository,
    StatusOptions,
};
use std::{
    env,
    path::{Path, PathBuf},
    str::FromStr,
};
use time::{
    format_description::{self, well_known::Iso8601},
    OffsetDateTime, UtcOffset,
};

#[derive(Clone, Copy, Debug, Default)]
#[allow(clippy::struct_excessive_bools)]
pub(crate) struct Config {
    // git rev-parse --abbrev-ref HEAD
    pub(crate) git_branch: bool,
    // git log -1 --pretty=format:'%an'
    pub(crate) git_commit_author_name: bool,
    // git log -1 --pretty=format:'%ae'
    pub(crate) git_commit_author_email: bool,
    // git rev-list --count HEAD
    pub(crate) git_commit_count: bool,
    // git log -1 --format=%s
    pub(crate) git_commit_message: bool,
    // git log -1 --pretty=format:'%cs'
    pub(crate) git_commit_date: bool,
    // git log -1 --pretty=format:'%cI'
    pub(crate) git_commit_timestamp: bool,
    // git describe --always (optionally --tags, --dirty, --match)
    pub(crate) git_describe: bool,
    git_describe_dirty: bool,
    git_describe_tags: bool,
    git_describe_match_pattern: Option<&'static str>,
    // git rev-parse HEAD (optionally with --short)
    pub(crate) git_sha: bool,
    git_sha_short: bool,
    // if output from:
    // git status --porcelain (optionally with "--untracked-files=no")
    pub(crate) git_dirty: bool,
    git_dirty_include_untracked: bool,
    use_local: bool,
    #[cfg(test)]
    fail: bool,
}

/// The `VERGEN_GIT_*` configuration features
///
/// | Variable | Sample |
/// | -------  | ------ |
/// | `VERGEN_GIT_BRANCH` | feature/fun |
/// | `VERGEN_GIT_COMMIT_AUTHOR_EMAIL` | janedoe@email.com |
/// | `VERGEN_GIT_COMMIT_AUTHOR_NAME` | Jane Doe |
/// | `VERGEN_GIT_COMMIT_COUNT` | 330 |
/// | `VERGEN_GIT_COMMIT_DATE` | 2021-02-24 |
/// | `VERGEN_GIT_COMMIT_MESSAGE` | feat: add commit messages |
/// | `VERGEN_GIT_COMMIT_TIMESTAMP` | 2021-02-24T20:55:21+00:00 |
/// | `VERGEN_GIT_DESCRIBE` | 5.0.0-2-gf49246c |
/// | `VERGEN_GIT_SHA` | f49246ce334567bff9f950bfd0f3078184a2738a |
/// | `VERGEN_GIT_DIRTY` | true |
///
/// # Example
///
/// ```
/// # use anyhow::Result;
/// # use vergen::EmitBuilder;
/// #
/// # fn main() -> Result<()> {
/// EmitBuilder::builder().all_git().emit()?;
/// #   Ok(())
/// # }
/// ```
///
/// Override output with your own value
///
/// ```
/// # use anyhow::Result;
/// # use std::env;
/// # use vergen::EmitBuilder;
/// #
/// # fn main() -> Result<()> {
/// env::set_var("VERGEN_GIT_BRANCH", "this is the branch I want output");
/// EmitBuilder::builder().all_git().emit()?;
/// # env::remove_var("VERGEN_GIT_BRANCH");
/// #   Ok(())
/// # }
/// ```
///
#[cfg_attr(docsrs, doc(cfg(feature = "git")))]
impl EmitBuilder {
    /// Emit all of the `VERGEN_GIT_*` instructions
    pub fn all_git(&mut self) -> &mut Self {
        self.git_branch()
            .git_commit_author_email()
            .git_commit_author_name()
            .git_commit_count()
            .git_commit_date()
            .git_commit_message()
            .git_commit_timestamp()
            .git_describe(false, false, None)
            .git_sha(false)
            .git_dirty(false)
    }

    fn any(&self) -> bool {
        let cfg = self.git_config;

        cfg.git_branch
            || cfg.git_commit_author_email
            || cfg.git_commit_author_name
            || cfg.git_commit_count
            || cfg.git_commit_date
            || cfg.git_commit_message
            || cfg.git_commit_timestamp
            || cfg.git_describe
            || cfg.git_sha
            || cfg.git_dirty
    }

    /// Emit the current git branch
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_GIT_BRANCH=<BRANCH_NAME>
    /// ```
    ///
    pub fn git_branch(&mut self) -> &mut Self {
        self.git_config.git_branch = true;
        self
    }

    /// Emit the author email of the most recent commit
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_GIT_COMMIT_AUTHOR_EMAIL=<AUTHOR_EMAIL>
    /// ```
    ///
    pub fn git_commit_author_email(&mut self) -> &mut Self {
        self.git_config.git_commit_author_email = true;
        self
    }

    /// Emit the author name of the most recent commit
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_GIT_COMMIT_AUTHOR_NAME=<AUTHOR_NAME>
    /// ```
    ///
    pub fn git_commit_author_name(&mut self) -> &mut Self {
        self.git_config.git_commit_author_name = true;
        self
    }

    /// Emit the total commit count to HEAD
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_GIT_COMMIT_COUNT=<COUNT>
    /// ```
    ///
    pub fn git_commit_count(&mut self) -> &mut Self {
        self.git_config.git_commit_count = true;
        self
    }

    /// Emit the commit date of the latest commit
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_GIT_COMMIT_DATE=<YYYY-MM-DD>
    /// ```
    ///
    pub fn git_commit_date(&mut self) -> &mut Self {
        self.git_config.git_commit_date = true;
        self
    }

    /// Emit the commit message of the latest commit
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_GIT_COMMIT_MESSAGE=<MESSAGE>
    /// ```
    ///
    pub fn git_commit_message(&mut self) -> &mut Self {
        self.git_config.git_commit_message = true;
        self
    }

    /// Emit the commit timestamp of the latest commit
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_GIT_COMMIT_TIMESTAMP=<YYYY-MM-DDThh:mm:ssZ>
    /// ```
    ///
    pub fn git_commit_timestamp(&mut self) -> &mut Self {
        self.git_config.git_commit_timestamp = true;
        self
    }

    /// Emit the describe output
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_GIT_DESCRIBE=<DESCRIBE>
    /// ```
    ///
    /// Optionally, add the `dirty`, `tags`, or `match` flag to describe.
    /// See [`git describe`](https://git-scm.com/docs/git-describe#_options) for more details
    ///
    pub fn git_describe(
        &mut self,
        dirty: bool,
        tags: bool,
        match_pattern: Option<&'static str>,
    ) -> &mut Self {
        self.git_config.git_describe = true;
        self.git_config.git_describe_dirty = dirty;
        self.git_config.git_describe_tags = tags;
        self.git_config.git_describe_match_pattern = match_pattern;
        self
    }

    /// Emit the SHA of the latest commit
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_GIT_SHA=<SHA>
    /// ```
    ///
    /// Optionally, add the `short` flag to rev-parse.
    /// See [`git rev-parse`](https://git-scm.com/docs/git-rev-parse#_options_for_output) for more details.
    ///
    pub fn git_sha(&mut self, short: bool) -> &mut Self {
        self.git_config.git_sha = true;
        self.git_config.git_sha_short = short;
        self
    }

    /// Emit the dirty state of the git repository
    /// ```text
    /// cargo:rustc-env=VERGEN_GIT_DIRTY=(true|false)
    /// ```
    ///
    /// Optionally, include/ignore untracked files in deciding whether the repository
    /// is dirty.
    pub fn git_dirty(&mut self, include_untracked_files: bool) -> &mut Self {
        self.git_config.git_dirty = true;
        self.git_config.git_dirty_include_untracked = include_untracked_files;
        self
    }

    /// Enable local offset date/timestamp output
    pub fn use_local_git(&mut self) -> &mut Self {
        self.git_config.use_local = true;
        self
    }

    pub(crate) fn add_git_default(
        &self,
        e: Error,
        fail_on_error: bool,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
        rerun_if_changed: &mut Vec<String>,
    ) -> Result<()> {
        if fail_on_error {
            Err(e)
        } else {
            // Clear any previous warnings.  This should be it.
            warnings.clear();
            rerun_if_changed.clear();

            warnings.push(format!("{e}"));

            if self.git_config.git_branch {
                add_default_map_entry(VergenKey::GitBranch, map, warnings);
            }
            if self.git_config.git_commit_author_email {
                add_default_map_entry(VergenKey::GitCommitAuthorEmail, map, warnings);
            }
            if self.git_config.git_commit_author_name {
                add_default_map_entry(VergenKey::GitCommitAuthorName, map, warnings);
            }
            if self.git_config.git_commit_count {
                add_default_map_entry(VergenKey::GitCommitCount, map, warnings);
            }
            if self.git_config.git_commit_date {
                add_default_map_entry(VergenKey::GitCommitDate, map, warnings);
            }
            if self.git_config.git_commit_message {
                add_default_map_entry(VergenKey::GitCommitMessage, map, warnings);
            }
            if self.git_config.git_commit_timestamp {
                add_default_map_entry(VergenKey::GitCommitTimestamp, map, warnings);
            }
            if self.git_config.git_describe {
                add_default_map_entry(VergenKey::GitDescribe, map, warnings);
            }
            if self.git_config.git_sha {
                add_default_map_entry(VergenKey::GitSha, map, warnings);
            }
            if self.git_config.git_dirty {
                add_default_map_entry(VergenKey::GitDirty, map, warnings);
            }
            Ok(())
        }
    }

    #[cfg(not(test))]
    pub(crate) fn add_git_map_entries(
        &self,
        path: Option<PathBuf>,
        idempotent: bool,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
        rerun_if_changed: &mut Vec<String>,
    ) -> Result<()> {
        if self.any() {
            self.inner_add_git_map_entries(path, idempotent, map, warnings, rerun_if_changed)?;
        }
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn add_git_map_entries(
        &self,
        path: Option<PathBuf>,
        idempotent: bool,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
        rerun_if_changed: &mut Vec<String>,
    ) -> Result<()> {
        if self.any() {
            if self.git_config.fail {
                return Err(anyhow!("failed to create entries"));
            }
            self.inner_add_git_map_entries(path, idempotent, map, warnings, rerun_if_changed)?;
        }
        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    fn inner_add_git_map_entries(
        &self,
        path: Option<PathBuf>,
        idempotent: bool,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
        rerun_if_changed: &mut Vec<String>,
    ) -> Result<()> {
        let curr_dir = if let Some(path) = path {
            path
        } else {
            env::current_dir()?
        };
        let repo = Repository::discover(curr_dir)?;
        let ref_head = repo.find_reference("HEAD")?;
        let git_path = repo.path().to_path_buf();
        let commit = ref_head.peel_to_commit()?;

        if !idempotent && self.any() {
            self.add_rerun_if_changed(&ref_head, &git_path, rerun_if_changed);
        }

        if self.git_config.git_branch {
            if let Ok(value) = env::var(GIT_BRANCH_NAME) {
                add_map_entry(VergenKey::GitBranch, value, map);
            } else {
                add_branch_name(false, &repo, map, warnings)?;
            }
        }

        if self.git_config.git_commit_author_email {
            if let Ok(value) = env::var(GIT_COMMIT_AUTHOR_EMAIL) {
                add_map_entry(VergenKey::GitCommitAuthorEmail, value, map);
            } else {
                add_opt_value(
                    commit.author().email(),
                    VergenKey::GitCommitAuthorEmail,
                    map,
                    warnings,
                );
            }
        }

        if self.git_config.git_commit_author_name {
            if let Ok(value) = env::var(GIT_COMMIT_AUTHOR_NAME) {
                add_map_entry(VergenKey::GitCommitAuthorName, value, map);
            } else {
                add_opt_value(
                    commit.author().name(),
                    VergenKey::GitCommitAuthorName,
                    map,
                    warnings,
                );
            }
        }

        if self.git_config.git_commit_count {
            if let Ok(value) = env::var(GIT_COMMIT_COUNT) {
                add_map_entry(VergenKey::GitCommitCount, value, map);
            } else {
                add_commit_count(false, &repo, map, warnings);
            }
        }

        self.add_git_timestamp_entries(&commit, idempotent, map, warnings)?;

        if self.git_config.git_commit_message {
            if let Ok(value) = env::var(GIT_COMMIT_MESSAGE) {
                add_map_entry(VergenKey::GitCommitMessage, value, map);
            } else {
                add_opt_value(commit.message(), VergenKey::GitCommitMessage, map, warnings);
            }
        }

        if self.git_config.git_sha {
            if let Ok(value) = env::var(GIT_SHA_NAME) {
                add_map_entry(VergenKey::GitSha, value, map);
            } else if self.git_config.git_sha_short {
                let obj = repo.revparse_single("HEAD")?;
                add_opt_value(obj.short_id()?.as_str(), VergenKey::GitSha, map, warnings);
            } else {
                add_map_entry(VergenKey::GitSha, commit.id().to_string(), map);
            }
        }

        if self.git_config.git_dirty {
            if let Ok(value) = env::var(GIT_DIRTY_NAME) {
                add_map_entry(VergenKey::GitDirty, value, map);
            } else {
                let mut status_options = StatusOptions::new();

                _ = status_options.include_untracked(self.git_config.git_dirty_include_untracked);
                let statuses = repo.statuses(Some(&mut status_options))?;

                let n_dirty = statuses
                    .iter()
                    .filter(|each_status| !each_status.status().is_ignored())
                    .count();

                add_map_entry(VergenKey::GitDirty, format!("{}", n_dirty > 0), map);
            }
        }

        if self.git_config.git_describe {
            if let Ok(value) = env::var(GIT_DESCRIBE_NAME) {
                add_map_entry(VergenKey::GitDescribe, value, map);
            } else {
                let mut describe_opts = DescribeOptions::new();
                let mut format_opts = DescribeFormatOptions::new();

                _ = describe_opts.show_commit_oid_as_fallback(true);

                if self.git_config.git_describe_dirty {
                    _ = format_opts.dirty_suffix("-dirty");
                }

                if self.git_config.git_describe_tags {
                    _ = describe_opts.describe_tags();
                }

                if let Some(pattern) = self.git_config.git_describe_match_pattern {
                    _ = describe_opts.pattern(pattern);
                }

                let describe = repo
                    .describe(&describe_opts)
                    .map(|x| x.format(Some(&format_opts)).map_err(Error::from))??;
                add_map_entry(VergenKey::GitDescribe, describe, map);
            }
        }

        Ok(())
    }

    fn add_git_timestamp_entries(
        &self,
        commit: &Commit<'_>,
        idempotent: bool,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
    ) -> Result<()> {
        let (sde, ts) = match env::var("SOURCE_DATE_EPOCH") {
            Ok(v) => (
                true,
                OffsetDateTime::from_unix_timestamp(i64::from_str(&v)?)?,
            ),
            Err(env::VarError::NotPresent) => {
                let no_offset = OffsetDateTime::from_unix_timestamp(commit.time().seconds())?;
                if self.git_config.use_local {
                    let local = UtcOffset::local_offset_at(no_offset)?;
                    let local_offset = no_offset.checked_to_offset(local).unwrap_or(no_offset);
                    (false, local_offset)
                } else {
                    (false, no_offset)
                }
            }
            Err(e) => return Err(e.into()),
        };

        if let Ok(value) = env::var(GIT_COMMIT_DATE_NAME) {
            add_map_entry(VergenKey::GitCommitDate, value, map);
        } else {
            self.add_git_date_entry(idempotent, sde, &ts, map, warnings)?;
        }
        if let Ok(value) = env::var(GIT_COMMIT_TIMESTAMP_NAME) {
            add_map_entry(VergenKey::GitCommitTimestamp, value, map);
        } else {
            self.add_git_timestamp_entry(idempotent, sde, &ts, map, warnings)?;
        }
        Ok(())
    }

    fn add_git_date_entry(
        &self,
        idempotent: bool,
        source_date_epoch: bool,
        ts: &OffsetDateTime,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
    ) -> Result<()> {
        if self.git_config.git_commit_date {
            if idempotent && !source_date_epoch {
                add_default_map_entry(VergenKey::GitCommitDate, map, warnings);
            } else {
                let format = format_description::parse("[year]-[month]-[day]")?;
                add_map_entry(VergenKey::GitCommitDate, ts.format(&format)?, map);
            }
        }
        Ok(())
    }

    fn add_git_timestamp_entry(
        &self,
        idempotent: bool,
        source_date_epoch: bool,
        ts: &OffsetDateTime,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
    ) -> Result<()> {
        if self.git_config.git_commit_timestamp {
            if idempotent && !source_date_epoch {
                add_default_map_entry(VergenKey::GitCommitTimestamp, map, warnings);
            } else {
                add_map_entry(
                    VergenKey::GitCommitTimestamp,
                    ts.format(&Iso8601::DEFAULT)?,
                    map,
                );
            }
        }
        Ok(())
    }

    #[allow(clippy::unused_self)]
    fn add_rerun_if_changed(
        &self,
        ref_head: &Reference<'_>,
        git_path: &Path,
        rerun_if_changed: &mut Vec<String>,
    ) {
        // Setup the head path
        let mut head_path = git_path.to_path_buf();
        head_path.push("HEAD");

        // Check whether the path exists in the filesystem before emitting it
        if head_path.exists() {
            rerun_if_changed.push(format!("{}", head_path.display()));
        }

        if let Ok(resolved) = ref_head.resolve() {
            if let Some(name) = resolved.name() {
                let ref_path = git_path.to_path_buf();
                let path = ref_path.join(name);
                // Check whether the path exists in the filesystem before emitting it
                if path.exists() {
                    rerun_if_changed.push(format!("{}", ref_path.display()));
                }
            }
        }
    }
}

#[allow(clippy::map_unwrap_or)]
fn add_opt_value(
    value: Option<&str>,
    key: VergenKey,
    map: &mut RustcEnvMap,
    warnings: &mut Vec<String>,
) {
    value
        .map(|val| add_map_entry(key, val, map))
        .unwrap_or_else(|| add_default_map_entry(key, map, warnings));
}

fn add_commit_count(
    add_default: bool,
    repo: &Repository,
    map: &mut RustcEnvMap,
    warnings: &mut Vec<String>,
) {
    let key = VergenKey::GitCommitCount;
    if !add_default {
        if let Ok(mut revwalk) = repo.revwalk() {
            if revwalk.push_head().is_ok() {
                add_map_entry(key, revwalk.count().to_string(), map);
                return;
            }
        }
    }
    add_default_map_entry(key, map, warnings);
}

fn add_branch_name(
    add_default: bool,
    repo: &Repository,
    map: &mut RustcEnvMap,
    warnings: &mut Vec<String>,
) -> Result<()> {
    if repo.head_detached()? {
        if add_default {
            add_default_map_entry(VergenKey::GitBranch, map, warnings);
        } else {
            add_map_entry(VergenKey::GitBranch, "HEAD", map);
        }
    } else {
        let locals = repo.branches(Some(BranchType::Local))?;
        let mut found_head = false;
        for (local, _bt) in locals.filter_map(std::result::Result::ok) {
            if local.is_head() {
                if let Some(name) = local.name()? {
                    add_map_entry(VergenKey::GitBranch, name, map);
                    found_head = !add_default;
                    break;
                }
            }
        }
        if !found_head {
            add_default_map_entry(VergenKey::GitBranch, map, warnings);
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::{add_branch_name, add_commit_count, add_opt_value};
    use crate::{emitter::test::count_idempotent, key::VergenKey, EmitBuilder};
    use anyhow::Result;
    use git2_rs::Repository;
    use repo_util::TestRepos;
    use std::{collections::BTreeMap, env, vec};

    fn repo_exists() -> Result<bool> {
        let curr_dir = env::current_dir()?;
        let _repo = Repository::discover(curr_dir)?;
        Ok(true)
    }

    #[test]
    #[serial_test::serial]
    fn empty_email_is_default() -> Result<()> {
        let mut map = BTreeMap::new();
        let mut warnings = vec![];
        add_opt_value(
            None,
            VergenKey::GitCommitAuthorEmail,
            &mut map,
            &mut warnings,
        );
        assert_eq!(1, map.len());
        assert_eq!(1, warnings.len());
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn bad_revwalk_is_default() -> Result<()> {
        let mut map = BTreeMap::new();
        let mut warnings = vec![];
        if let Ok(repo) = Repository::discover(env::current_dir()?) {
            add_commit_count(true, &repo, &mut map, &mut warnings);
            assert_eq!(1, map.len());
            assert_eq!(1, warnings.len());
        }
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn head_not_found_is_default() -> Result<()> {
        let repo = TestRepos::new(false, false, false)?;
        let mut map = BTreeMap::new();
        let mut warnings = vec![];
        if let Ok(repo) = Repository::discover(env::current_dir()?) {
            add_branch_name(true, &repo, &mut map, &mut warnings)?;
            assert_eq!(1, map.len());
            assert_eq!(1, warnings.len());
        }
        let mut map = BTreeMap::new();
        let mut warnings = vec![];
        if let Ok(repo) = Repository::discover(repo.path()) {
            add_branch_name(true, &repo, &mut map, &mut warnings)?;
            assert_eq!(1, map.len());
            assert_eq!(1, warnings.len());
        }
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn git_all_idempotent() -> Result<()> {
        let config = EmitBuilder::builder()
            .idempotent()
            .all_git()
            .test_emit_at(None)?;
        assert_eq!(10, config.cargo_rustc_env_map.len());

        if repo_exists().is_ok() && !config.failed {
            assert_eq!(2, count_idempotent(&config.cargo_rustc_env_map));
            assert_eq!(2, config.warnings.len());
        } else {
            assert_eq!(10, count_idempotent(&config.cargo_rustc_env_map));
            assert_eq!(11, config.warnings.len());
        }
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn git_all_shallow_clone() -> Result<()> {
        let repo = TestRepos::new(false, false, true)?;
        let emitter = EmitBuilder::builder()
            .all_git()
            .test_emit_at(Some(repo.path()))?;
        assert_eq!(10, emitter.cargo_rustc_env_map.len());
        assert_eq!(0, count_idempotent(&emitter.cargo_rustc_env_map));
        assert_eq!(0, emitter.warnings.len());

        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn git_all_idempotent_no_warn() -> Result<()> {
        let config = EmitBuilder::builder()
            .idempotent()
            .quiet()
            .all_git()
            .test_emit_at(None)?;

        assert_eq!(10, config.cargo_rustc_env_map.len());

        if repo_exists().is_ok() && !config.failed {
            assert_eq!(2, count_idempotent(&config.cargo_rustc_env_map));
            assert_eq!(2, config.warnings.len());
        } else {
            assert_eq!(9, count_idempotent(&config.cargo_rustc_env_map));
            assert_eq!(10, config.warnings.len());
        }
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn git_all() -> Result<()> {
        let config = EmitBuilder::builder().all_git().test_emit_at(None)?;
        assert_eq!(10, config.cargo_rustc_env_map.len());

        if repo_exists().is_ok() && !config.failed {
            assert_eq!(0, count_idempotent(&config.cargo_rustc_env_map));
            assert_eq!(0, config.warnings.len());
        } else {
            assert_eq!(9, count_idempotent(&config.cargo_rustc_env_map));
            assert_eq!(10, config.warnings.len());
        }
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn git_error_fails() -> Result<()> {
        let mut config = EmitBuilder::builder();
        _ = config.fail_on_error();
        _ = config.all_git();
        config.git_config.fail = true;
        assert!(config.test_emit().is_err());
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn git_error_defaults() -> Result<()> {
        let mut config = EmitBuilder::builder();
        _ = config.all_git();
        config.git_config.fail = true;
        let emitter = config.test_emit()?;
        assert_eq!(10, emitter.cargo_rustc_env_map.len());
        assert_eq!(10, count_idempotent(&emitter.cargo_rustc_env_map));
        assert_eq!(11, emitter.warnings.len());
        Ok(())
    }
}
