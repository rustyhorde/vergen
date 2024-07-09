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
use anyhow::{anyhow, Error, Result};
use std::{
    env,
    path::PathBuf,
    process::{Command, Output, Stdio},
    str::FromStr,
};
use time::{
    format_description::{
        self,
        well_known::{Iso8601, Rfc3339},
    },
    OffsetDateTime, UtcOffset,
};

#[derive(Clone, Copy, Debug, Default)]
#[allow(clippy::struct_excessive_bools)]
pub(crate) struct Config {
    // git rev-parse --abbrev-ref --symbolic-full-name HEAD
    pub(crate) git_branch: bool,
    // git log -1 --pretty=format:'%ae'
    pub(crate) git_commit_author_email: bool,
    // git log -1 --pretty=format:'%an'
    pub(crate) git_commit_author_name: bool,
    // git rev-list --count HEAD
    pub(crate) git_commit_count: bool,
    // git log -1 --pretty=format:'%cs'
    pub(crate) git_commit_date: bool,
    // git log -1 --format=%s
    pub(crate) git_commit_message: bool,
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
    // git status --porcelain (optionally with "-u no")
    pub(crate) git_dirty: bool,
    git_dirty_include_untracked: bool,
    git_cmd: Option<&'static str>,
    use_local: bool,
}

// This funkiness allows the command to be output in the docs
macro_rules! branch_cmd {
    () => {
        "git rev-parse --abbrev-ref --symbolic-full-name HEAD"
    };
}
const BRANCH_CMD: &str = branch_cmd!();
macro_rules! author_email {
    () => {
        "git log -1 --pretty=format:'%ae'"
    };
}
const COMMIT_AUTHOR_EMAIL: &str = author_email!();
macro_rules! author_name {
    () => {
        "git log -1 --pretty=format:'%an'"
    };
}
const COMMIT_AUTHOR_NAME: &str = author_name!();
macro_rules! commit_count {
    () => {
        "git rev-list --count HEAD"
    };
}
const COMMIT_COUNT: &str = commit_count!();
macro_rules! commit_date {
    () => {
        "git log -1 --pretty=format:'%cs'"
    };
}
macro_rules! commit_message {
    () => {
        "git log -1 --format=%s"
    };
}
const COMMIT_MESSAGE: &str = commit_message!();
macro_rules! commit_timestamp {
    () => {
        "git log -1 --pretty=format:'%cI'"
    };
}
const COMMIT_TIMESTAMP: &str = commit_timestamp!();
macro_rules! describe {
    () => {
        "git describe --always"
    };
}
const DESCRIBE: &str = describe!();
macro_rules! sha {
    () => {
        "git rev-parse"
    };
}
const SHA: &str = sha!();
macro_rules! dirty {
    () => {
        "git status --porcelain"
    };
}
const DIRTY: &str = dirty!();

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
/// Emit all of the git instructions
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
/// Emit some of the git instructions
///
/// ```
/// # use anyhow::Result;
/// # use vergen::EmitBuilder;
/// #
/// # fn main() -> Result<()> {
/// EmitBuilder::builder().git_describe(true, false, None).emit()?;
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
/// # Example
/// This feature can also be used in conjuction with the [`SOURCE_DATE_EPOCH`](https://reproducible-builds.org/docs/source-date-epoch/)
/// environment variable to generate deterministic timestamps based off the
/// last modification time of the source/package
///
/// ```
/// # use anyhow::Result;
/// # use std::env;
/// # use vergen::EmitBuilder;
/// #
/// # fn main() -> Result<()> {
/// env::set_var("SOURCE_DATE_EPOCH", "1671809360");
#[cfg_attr(
    all(feature = "git", feature = "gitcl"),
    doc = r##"
EmitBuilder::builder().all_git().emit()?;
"##
)]
/// # env::remove_var("SOURCE_DATE_EPOCH");
/// #   Ok(())
/// # }
/// ```
///
/// The above will always generate the following output for the timestamp
/// related instructions
///
/// ```text
/// ...
/// cargo:rustc-env=VERGEN_GIT_COMMIT_DATE=2022-12-23
/// ...
/// cargo:rustc-env=VERGEN_GIT_COMMIT_TIMESTAMP=2022-12-23T15:29:20.000000000Z
/// ...
/// ```
///
/// # Example
/// This feature also recognizes the idempotent flag.
///
/// **NOTE** - `SOURCE_DATE_EPOCH` takes precedence over the idempotent flag. If you
/// use both, the output will be based off `SOURCE_DATE_EPOCH`.  This would still be
/// deterministic.
///
/// # Example
/// ```
/// # use anyhow::Result;
/// # use vergen::EmitBuilder;
/// #
/// # fn main() -> Result<()> {
#[cfg_attr(
    all(feature = "git", feature = "gitcl"),
    doc = r##"
EmitBuilder::builder().idempotent().all_git().emit()?;
"##
)]
/// #   Ok(())
/// # }
/// ```
///
/// The above will always generate the following instructions
///
/// ```text
/// cargo:rustc-env=VERGEN_GIT_BRANCH=VERGEN_IDEMPOTENT_OUTPUT
/// cargo:rustc-env=VERGEN_GIT_COMMIT_AUTHOR_EMAIL=VERGEN_IDEMPOTENT_OUTPUT
/// cargo:rustc-env=VERGEN_GIT_COMMIT_AUTHOR_NAME=VERGEN_IDEMPOTENT_OUTPUT
/// cargo:rustc-env=VERGEN_GIT_COMMIT_COUNT=VERGEN_IDEMPOTENT_OUTPUT
/// cargo:rustc-env=VERGEN_GIT_COMMIT_DATE=VERGEN_IDEMPOTENT_OUTPUT
/// cargo:rustc-env=VERGEN_GIT_COMMIT_MESSAGE=VERGEN_IDEMPOTENT_OUTPUT
/// cargo:rustc-env=VERGEN_GIT_COMMIT_TIMESTAMP=VERGEN_IDEMPOTENT_OUTPUT
/// cargo:rustc-env=VERGEN_GIT_DESCRIBE=VERGEN_IDEMPOTENT_OUTPUT
/// cargo:rustc-env=VERGEN_GIT_SHA=VERGEN_IDEMPOTENT_OUTPUT
/// cargo:warning=VERGEN_GIT_BRANCH set to default
/// cargo:warning=VERGEN_GIT_COMMIT_AUTHOR_EMAIL set to default
/// cargo:warning=VERGEN_GIT_COMMIT_AUTHOR_NAME set to default
/// cargo:warning=VERGEN_GIT_COMMIT_COUNT set to default
/// cargo:warning=VERGEN_GIT_COMMIT_DATE set to default
/// cargo:warning=VERGEN_GIT_COMMIT_MESSAGE set to default
/// cargo:warning=VERGEN_GIT_COMMIT_TIMESTAMP set to default
/// cargo:warning=VERGEN_GIT_DESCRIBE set to default
/// cargo:warning=VERGEN_GIT_SHA set to default
/// cargo:rerun-if-changed=build.rs
/// cargo:rerun-if-env-changed=VERGEN_IDEMPOTENT
/// cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH
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
            .git_cmd(None)
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
    /// The value is determined with the following command
    /// ```text
    #[doc = concat!(branch_cmd!())]
    /// ```
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
    /// The value is determined with the following command
    /// ```text
    #[doc = concat!(author_email!())]
    /// ```
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
    /// The value is determined with the following command
    /// ```text
    #[doc = concat!(author_name!())]
    /// ```
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
    /// The value is determined with the following command
    /// ```text
    #[doc = concat!(commit_count!())]
    /// ```
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
    /// The value is determined with the following command
    /// ```text
    #[doc = concat!(commit_date!())]
    /// ```
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
    /// The value is determined with the following command
    /// ```text
    #[doc = concat!(commit_message!())]
    /// ```
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
    /// The value is determined with the following command
    /// ```text
    #[doc = concat!(commit_timestamp!())]
    /// ```
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
    /// The value is determined with the following command
    /// ```text
    #[doc = concat!(describe!())]
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
    /// The value is determined with the following command
    /// ```text
    #[doc = concat!(sha!(), " HEAD")]
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
    pub fn git_dirty(&mut self, include_untracked: bool) -> &mut Self {
        self.git_config.git_dirty = true;
        self.git_config.git_dirty_include_untracked = include_untracked;
        self
    }

    /// Set the command used to test if git exists on the path.
    /// Defaults to `git --version` if not set explicitly.
    pub fn git_cmd(&mut self, cmd: Option<&'static str>) -> &mut Self {
        self.git_config.git_cmd = cmd;
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
            // Clear any previous data.  We are re-populating
            // map isn't cleared because keys will overwrite.
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

    pub(crate) fn add_git_map_entries(
        &self,
        path: Option<PathBuf>,
        idempotent: bool,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
        rerun_if_changed: &mut Vec<String>,
    ) -> Result<()> {
        if self.any() {
            let git_cmd = self.git_config.git_cmd.unwrap_or("git --version");
            check_git(git_cmd).and_then(|()| check_inside_git_worktree(&path))?;
            self.inner_add_git_map_entries(path, idempotent, map, warnings, rerun_if_changed)?;
        }
        Ok(())
    }

    #[allow(clippy::needless_pass_by_value)]
    fn inner_add_git_map_entries(
        &self,
        path: Option<PathBuf>,
        idempotent: bool,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
        rerun_if_changed: &mut Vec<String>,
    ) -> Result<()> {
        if !idempotent && self.any() {
            add_rerun_if_changed(rerun_if_changed, &path)?;
        }

        if self.git_config.git_branch {
            if let Ok(value) = env::var(GIT_BRANCH_NAME) {
                add_map_entry(VergenKey::GitBranch, value, map);
            } else {
                add_git_cmd_entry(BRANCH_CMD, &path, VergenKey::GitBranch, map)?;
            }
        }

        if self.git_config.git_commit_author_email {
            if let Ok(value) = env::var(GIT_COMMIT_AUTHOR_EMAIL) {
                add_map_entry(VergenKey::GitCommitAuthorEmail, value, map);
            } else {
                add_git_cmd_entry(
                    COMMIT_AUTHOR_EMAIL,
                    &path,
                    VergenKey::GitCommitAuthorEmail,
                    map,
                )?;
            }
        }

        if self.git_config.git_commit_author_name {
            if let Ok(value) = env::var(GIT_COMMIT_AUTHOR_NAME) {
                add_map_entry(VergenKey::GitCommitAuthorName, value, map);
            } else {
                add_git_cmd_entry(
                    COMMIT_AUTHOR_NAME,
                    &path,
                    VergenKey::GitCommitAuthorName,
                    map,
                )?;
            }
        }

        if self.git_config.git_commit_count {
            if let Ok(value) = env::var(GIT_COMMIT_COUNT) {
                add_map_entry(VergenKey::GitCommitCount, value, map);
            } else {
                add_git_cmd_entry(COMMIT_COUNT, &path, VergenKey::GitCommitCount, map)?;
            }
        }

        self.add_git_timestamp_entries(COMMIT_TIMESTAMP, &path, idempotent, map, warnings)?;

        if self.git_config.git_commit_message {
            if let Ok(value) = env::var(GIT_COMMIT_MESSAGE) {
                add_map_entry(VergenKey::GitCommitMessage, value, map);
            } else {
                add_git_cmd_entry(COMMIT_MESSAGE, &path, VergenKey::GitCommitMessage, map)?;
            }
        }

        if self.git_config.git_describe {
            if let Ok(value) = env::var(GIT_DESCRIBE_NAME) {
                add_map_entry(VergenKey::GitDescribe, value, map);
            } else {
                let mut describe_cmd = String::from(DESCRIBE);
                if self.git_config.git_describe_dirty {
                    describe_cmd.push_str(" --dirty");
                }
                if self.git_config.git_describe_tags {
                    describe_cmd.push_str(" --tags");
                }
                if let Some(pattern) = self.git_config.git_describe_match_pattern {
                    describe_cmd.push_str(" --match \"");
                    describe_cmd.push_str(pattern);
                    describe_cmd.push('\"');
                }
                add_git_cmd_entry(&describe_cmd, &path, VergenKey::GitDescribe, map)?;
            }
        }

        if self.git_config.git_sha {
            if let Ok(value) = env::var(GIT_SHA_NAME) {
                add_map_entry(VergenKey::GitSha, value, map);
            } else {
                let mut sha_cmd = String::from(SHA);
                if self.git_config.git_sha_short {
                    sha_cmd.push_str(" --short");
                }
                sha_cmd.push_str(" HEAD");
                add_git_cmd_entry(&sha_cmd, &path, VergenKey::GitSha, map)?;
            }
        }

        if self.git_config.git_dirty {
            if let Ok(value) = env::var(GIT_DIRTY_NAME) {
                add_map_entry(VergenKey::GitDirty, value, map);
            } else {
                let mut dirty_cmd = String::from(DIRTY);
                if !self.git_config.git_dirty_include_untracked {
                    dirty_cmd.push_str(" --untracked-files=no");
                }
                let output = run_cmd(&dirty_cmd, &path)?;
                if output.stdout.is_empty() {
                    add_map_entry(VergenKey::GitDirty, "false", map);
                } else {
                    add_map_entry(VergenKey::GitDirty, "true", map);
                }
            }
        }

        Ok(())
    }

    fn add_git_timestamp_entries(
        &self,
        cmd: &str,
        path: &Option<PathBuf>,
        idempotent: bool,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
    ) -> Result<()> {
        let mut date_override = false;
        if let Ok(value) = env::var(GIT_COMMIT_DATE_NAME) {
            add_map_entry(VergenKey::GitCommitDate, value, map);
            date_override = true;
        }

        let mut timestamp_override = false;
        if let Ok(value) = env::var(GIT_COMMIT_TIMESTAMP_NAME) {
            add_map_entry(VergenKey::GitCommitTimestamp, value, map);
            timestamp_override = true;
        }

        let output = run_cmd(cmd, path)?;
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout)
                .lines()
                .last()
                .ok_or_else(|| anyhow!("invalid 'git log' output"))?
                .trim()
                .trim_matches('\'')
                .to_string();

            let (sde, ts) = match env::var("SOURCE_DATE_EPOCH") {
                Ok(v) => (
                    true,
                    OffsetDateTime::from_unix_timestamp(i64::from_str(&v)?)?,
                ),
                Err(env::VarError::NotPresent) => {
                    let no_offset = OffsetDateTime::parse(&stdout, &Rfc3339)?;
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

            if idempotent && !sde {
                if self.git_config.git_commit_date && !date_override {
                    add_default_map_entry(VergenKey::GitCommitDate, map, warnings);
                }

                if self.git_config.git_commit_timestamp && !timestamp_override {
                    add_default_map_entry(VergenKey::GitCommitTimestamp, map, warnings);
                }
            } else {
                if self.git_config.git_commit_date && !date_override {
                    let format = format_description::parse("[year]-[month]-[day]")?;
                    add_map_entry(VergenKey::GitCommitDate, ts.format(&format)?, map);
                }

                if self.git_config.git_commit_timestamp && !timestamp_override {
                    add_map_entry(
                        VergenKey::GitCommitTimestamp,
                        ts.format(&Iso8601::DEFAULT)?,
                        map,
                    );
                }
            }
        } else {
            if self.git_config.git_commit_date && !date_override {
                add_default_map_entry(VergenKey::GitCommitDate, map, warnings);
            }

            if self.git_config.git_commit_timestamp && !timestamp_override {
                add_default_map_entry(VergenKey::GitCommitTimestamp, map, warnings);
            }
        }

        Ok(())
    }
}

fn check_git(cmd: &str) -> Result<()> {
    if git_cmd_exists(cmd) {
        Ok(())
    } else {
        Err(anyhow!("no suitable 'git' command found!"))
    }
}

fn check_inside_git_worktree(path: &Option<PathBuf>) -> Result<()> {
    if inside_git_worktree(path) {
        Ok(())
    } else {
        Err(anyhow!("not within a suitable 'git' worktree!"))
    }
}

fn git_cmd_exists(cmd: &str) -> bool {
    run_cmd(cmd, &None)
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn inside_git_worktree(path: &Option<PathBuf>) -> bool {
    run_cmd("git rev-parse --is-inside-work-tree", path)
        .map(|output| {
            let stdout = String::from_utf8_lossy(&output.stdout);
            output.status.success() && stdout.trim() == "true"
        })
        .unwrap_or(false)
}

#[cfg(not(target_env = "msvc"))]
fn run_cmd(command: &str, path_opt: &Option<PathBuf>) -> Result<Output> {
    let shell = if let Some(shell_path) = env::var_os("SHELL") {
        shell_path.to_string_lossy().into_owned()
    } else {
        // Fallback to sh if SHELL not defined
        "sh".to_string()
    };
    let mut cmd = Command::new(shell);
    if let Some(path) = path_opt {
        _ = cmd.current_dir(path);
    }
    _ = cmd.arg("-c");
    _ = cmd.arg(command);
    _ = cmd.stdout(Stdio::piped());
    _ = cmd.stderr(Stdio::piped());
    Ok(cmd.output()?)
}

#[cfg(target_env = "msvc")]
fn run_cmd(command: &str, path_opt: &Option<PathBuf>) -> Result<Output> {
    let mut cmd = Command::new("cmd");
    if let Some(path) = path_opt {
        _ = cmd.current_dir(path);
    }
    _ = cmd.arg("/c");
    _ = cmd.arg(command);
    _ = cmd.stdout(Stdio::piped());
    _ = cmd.stderr(Stdio::piped());
    Ok(cmd.output()?)
}

fn add_git_cmd_entry(
    cmd: &str,
    path: &Option<PathBuf>,
    key: VergenKey,
    map: &mut RustcEnvMap,
) -> Result<()> {
    let output = run_cmd(cmd, path)?;
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout)
            .trim()
            .trim_matches('\'')
            .to_string();
        add_map_entry(key, stdout, map);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("Failed to run '{cmd}'!  {stderr}"));
    }
    Ok(())
}

fn add_rerun_if_changed(rerun_if_changed: &mut Vec<String>, path: &Option<PathBuf>) -> Result<()> {
    let git_path = run_cmd("git rev-parse --git-dir", path)?;
    if git_path.status.success() {
        let git_path_str = String::from_utf8_lossy(&git_path.stdout).trim().to_string();
        let git_path = PathBuf::from(&git_path_str);

        // Setup the head path
        let mut head_path = git_path.clone();
        head_path.push("HEAD");

        if head_path.exists() {
            rerun_if_changed.push(format!("{}", head_path.display()));
        }

        // Setup the ref path
        let refp = setup_ref_path(path)?;
        if refp.status.success() {
            let ref_path_str = String::from_utf8_lossy(&refp.stdout).trim().to_string();
            let mut ref_path = git_path;
            ref_path.push(ref_path_str);
            if ref_path.exists() {
                rerun_if_changed.push(format!("{}", ref_path.display()));
            }
        }
    }
    Ok(())
}

#[cfg(not(test))]
fn setup_ref_path(path: &Option<PathBuf>) -> Result<Output> {
    run_cmd("git symbolic-ref HEAD", path)
}

#[cfg(all(test, not(target_os = "windows")))]
fn setup_ref_path(path: &Option<PathBuf>) -> Result<Output> {
    run_cmd("pwd", path)
}

#[cfg(all(test, target_os = "windows"))]
fn setup_ref_path(path: &Option<PathBuf>) -> Result<Output> {
    run_cmd("cd", path)
}

#[cfg(test)]
mod test {
    use super::{add_git_cmd_entry, check_git, check_inside_git_worktree};
    use crate::{emitter::test::count_idempotent, key::VergenKey, EmitBuilder};
    use anyhow::Result;
    use repo_util::TestRepos;
    use std::{collections::BTreeMap, env};

    #[test]
    #[serial_test::serial]
    fn bad_command_is_error() -> Result<()> {
        let mut map = BTreeMap::new();
        assert!(add_git_cmd_entry(
            "such_a_terrible_cmd",
            &None,
            VergenKey::GitCommitMessage,
            &mut map
        )
        .is_err());
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn non_working_tree_is_error() -> Result<()> {
        assert!(check_inside_git_worktree(&Some(env::temp_dir())).is_err());
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn invalid_git_is_error() -> Result<()> {
        assert!(check_git("such_a_terrible_cmd -v").is_err());
        Ok(())
    }

    #[cfg(not(target_family = "windows"))]
    #[test]
    #[serial_test::serial]
    fn shell_env_works() -> Result<()> {
        temp_env::with_var("SHELL", Some("bash"), || {
            let mut map = BTreeMap::new();
            assert!(
                add_git_cmd_entry("git -v", &None, VergenKey::GitCommitMessage, &mut map).is_ok()
            );
        });
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
        assert_eq!(2, count_idempotent(&config.cargo_rustc_env_map));
        assert_eq!(2, config.warnings.len());
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
        assert_eq!(2, count_idempotent(&config.cargo_rustc_env_map));
        assert_eq!(2, config.warnings.len());
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn git_all_at_path() -> Result<()> {
        let repo = TestRepos::new(false, false, false)?;
        let config = EmitBuilder::builder()
            .all_git()
            .test_emit_at(Some(repo.path()))?;
        assert_eq!(10, config.cargo_rustc_env_map.len());
        assert_eq!(0, count_idempotent(&config.cargo_rustc_env_map));
        assert_eq!(0, config.warnings.len());
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn git_all() -> Result<()> {
        let config = EmitBuilder::builder().all_git().test_emit_at(None)?;
        assert_eq!(10, config.cargo_rustc_env_map.len());
        assert_eq!(0, count_idempotent(&config.cargo_rustc_env_map));
        assert_eq!(0, config.warnings.len());
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
    fn git_all_dirty_tags_short() -> Result<()> {
        let config = EmitBuilder::builder()
            .all_git()
            .git_describe(true, true, None)
            .git_sha(true)
            .test_emit()?;
        assert_eq!(10, config.cargo_rustc_env_map.len());
        assert_eq!(0, count_idempotent(&config.cargo_rustc_env_map));
        assert_eq!(0, config.warnings.len());
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn fails_on_bad_git_command() -> Result<()> {
        let mut config = EmitBuilder::builder();
        _ = config.fail_on_error();
        _ = config.all_git();
        config.git_config.git_cmd = Some("this_is_not_a_git_cmd");
        assert!(config.test_emit().is_err());
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn defaults_on_bad_git_command() -> Result<()> {
        let mut config = EmitBuilder::builder();
        _ = config.all_git();
        config.git_config.git_cmd = Some("this_is_not_a_git_cmd");
        let emitter = config.test_emit()?;
        assert_eq!(10, emitter.cargo_rustc_env_map.len());
        assert_eq!(10, count_idempotent(&emitter.cargo_rustc_env_map));
        assert_eq!(11, emitter.warnings.len());
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn bad_timestamp_defaults() -> Result<()> {
        let mut map = BTreeMap::new();
        let mut warnings = vec![];
        let mut config = EmitBuilder::builder();
        _ = config.all_git();
        assert!(config
            .add_git_timestamp_entries(
                "this_is_not_a_git_cmd",
                &None,
                false,
                &mut map,
                &mut warnings
            )
            .is_ok());
        assert_eq!(2, map.len());
        assert_eq!(2, warnings.len());
        Ok(())
    }
}
