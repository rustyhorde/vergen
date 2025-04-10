// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use anyhow::{anyhow, Error, Result};
use derive_builder::Builder as DeriveBuilder;
use std::{
    env::{self, VarError},
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
use vergen_lib::{
    add_default_map_entry, add_map_entry,
    constants::{
        GIT_BRANCH_NAME, GIT_COMMIT_AUTHOR_EMAIL, GIT_COMMIT_AUTHOR_NAME, GIT_COMMIT_COUNT,
        GIT_COMMIT_DATE_NAME, GIT_COMMIT_MESSAGE, GIT_COMMIT_TIMESTAMP_NAME, GIT_DESCRIBE_NAME,
        GIT_DIRTY_NAME, GIT_SHA_NAME,
    },
    AddEntries, CargoRerunIfChanged, CargoRustcEnvMap, CargoWarning, DefaultConfig, VergenKey,
};

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
/// # use vergen_gitcl::{Emitter, GitclBuilder};
/// #
/// # fn main() -> Result<()> {
/// let gitcl = GitclBuilder::all_git()?;
/// Emitter::default().add_instructions(&gitcl)?.emit()?;
/// #   Ok(())
/// # }
/// ```
///
/// Emit some of the git instructions
///
/// ```
/// # use anyhow::Result;
/// # use vergen_gitcl::{Emitter, GitclBuilder};
/// #
/// # fn main() -> Result<()> {
/// let gitcl = GitclBuilder::default().describe(true, false, None).build()?;
/// Emitter::default().add_instructions(&gitcl)?.emit()?;
/// #   Ok(())
/// # }
/// ```
///
/// Override output with your own value
///
/// ```
/// # use anyhow::Result;
/// # use vergen_gitcl::{Emitter, GitclBuilder};
/// #
/// # fn main() -> Result<()> {
/// temp_env::with_var("VERGEN_GIT_BRANCH", Some("this is the branch I want output"), || {
///     let result = || -> Result<()> {
///         let gitcl = GitclBuilder::all_git()?;
///         Emitter::default().add_instructions(&gitcl)?.emit()?;
///         Ok(())
///     }();
///     assert!(result.is_ok());
/// });
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
/// # use vergen_gitcl::{Emitter, GitclBuilder};
/// #
/// # fn main() -> Result<()> {
/// temp_env::with_var("SOURCE_DATE_EPOCH", Some("1671809360"), || {
///     let result = || -> Result<()> {
///         let gitcl = GitclBuilder::all_git()?;
///         Emitter::default().add_instructions(&gitcl)?.emit()?;
///         Ok(())
///     }();
///     assert!(result.is_ok());
/// });
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
/// # use vergen_gitcl::{Emitter, GitclBuilder};
/// #
/// # fn main() -> Result<()> {
/// let gitcl = GitclBuilder::all_git()?;
/// Emitter::default().idempotent().add_instructions(&gitcl)?.emit()?;
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
#[derive(Clone, Debug, DeriveBuilder, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub struct Gitcl {
    /// An optional path to a repository.
    #[builder(default = "None")]
    repo_path: Option<PathBuf>,
    /// Emit the current git branch
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_GIT_BRANCH=<BRANCH_NAME>
    /// ```
    ///
    #[builder(default = "false")]
    branch: bool,
    /// Emit the author email of the most recent commit
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_GIT_COMMIT_AUTHOR_EMAIL=<AUTHOR_EMAIL>
    /// ```
    ///
    #[builder(default = "false")]
    commit_author_name: bool,
    /// Emit the author name of the most recent commit
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_GIT_COMMIT_AUTHOR_NAME=<AUTHOR_NAME>
    /// ```
    ///
    #[builder(default = "false")]
    commit_author_email: bool,
    /// Emit the total commit count to HEAD
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_GIT_COMMIT_COUNT=<COUNT>
    /// ```
    #[builder(default = "false")]
    commit_count: bool,
    /// Emit the commit message of the latest commit
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_GIT_COMMIT_MESSAGE=<MESSAGE>
    /// ```
    ///
    #[builder(default = "false")]
    commit_message: bool,
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
    #[builder(default = "false")]
    commit_date: bool,
    /// Emit the commit timestamp of the latest commit
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_GIT_COMMIT_TIMESTAMP=<YYYY-MM-DDThh:mm:ssZ>
    /// ```
    ///
    #[builder(default = "false")]
    commit_timestamp: bool,
    /// Emit the describe output
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_GIT_DESCRIBE=<DESCRIBE>
    /// ```
    ///
    /// Optionally, add the `dirty` or `tags` flag to describe.
    /// See [`git describe`](https://git-scm.com/docs/git-describe#_options) for more details
    ///
    #[builder(default = "false", setter(custom))]
    describe: bool,
    /// Instead of using only the annotated tags, use any tag found in refs/tags namespace.
    #[builder(default = "false", private)]
    describe_tags: bool,
    /// If the working tree has local modification "-dirty" is appended to it.
    #[builder(default = "false", private)]
    describe_dirty: bool,
    /// Only consider tags matching the given glob pattern, excluding the "refs/tags/" prefix.
    #[builder(default = "None", private)]
    describe_match_pattern: Option<&'static str>,
    /// Emit the SHA of the latest commit
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_GIT_SHA=<SHA>
    /// ```
    ///
    /// Optionally, add the `short` flag to rev-parse.
    /// See [`git rev-parse`](https://git-scm.com/docs/git-rev-parse#_options_for_output) for more details.
    ///
    #[builder(default = "false", setter(custom))]
    sha: bool,
    /// Shortens the object name to a unique prefix
    #[builder(default = "false", private)]
    sha_short: bool,
    /// Emit the dirty state of the git repository
    /// ```text
    /// cargo:rustc-env=VERGEN_GIT_DIRTY=(true|false)
    /// ```
    ///
    /// Optionally, include untracked files when determining the dirty status of the repository.
    ///
    #[builder(default = "false", setter(custom))]
    dirty: bool,
    /// Should we include/ignore untracked files in deciding whether the repository is dirty.
    #[builder(default = "false", private)]
    dirty_include_untracked: bool,
    /// Enable local offset date/timestamp output
    #[builder(default = "false")]
    use_local: bool,
    /// Specify the git cmd you wish to use, i.e. `/usr/bin/git`
    #[builder(default = "None")]
    git_cmd: Option<&'static str>,
}

impl GitclBuilder {
    /// Emit all of the `VERGEN_GIT_*` instructions
    ///
    /// # Errors
    /// The underlying build function can error
    ///
    pub fn all_git() -> Result<Gitcl> {
        Self::default()
            .branch(true)
            .commit_author_email(true)
            .commit_author_name(true)
            .commit_count(true)
            .commit_date(true)
            .commit_message(true)
            .commit_timestamp(true)
            .describe(false, false, None)
            .sha(false)
            .dirty(false)
            .build()
            .map_err(Into::into)
    }

    /// Convenience method to setup the [`GitclBuilder`] with all of the `VERGEN_GIT_*` instructions on
    pub fn all(&mut self) -> &mut Self {
        self.branch(true)
            .commit_author_email(true)
            .commit_author_name(true)
            .commit_count(true)
            .commit_date(true)
            .commit_message(true)
            .commit_timestamp(true)
            .describe(false, false, None)
            .sha(false)
            .dirty(false)
    }

    /// Emit the describe output
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_GIT_DESCRIBE=<DESCRIBE>
    /// ```
    ///
    /// Optionally, add the `dirty` or `tags` flag to describe.
    /// See [`git describe`](https://git-scm.com/docs/git-describe#_options) for more details
    ///
    pub fn describe(
        &mut self,
        tags: bool,
        dirty: bool,
        matches: Option<&'static str>,
    ) -> &mut Self {
        self.describe = Some(true);
        let _ = self.describe_tags(tags);
        let _ = self.describe_dirty(dirty);
        let _ = self.describe_match_pattern(matches);
        self
    }

    /// Emit the dirty state of the git repository
    /// ```text
    /// cargo:rustc-env=VERGEN_GIT_DIRTY=(true|false)
    /// ```
    ///
    /// Optionally, include untracked files when determining the dirty status of the repository.
    ///
    pub fn dirty(&mut self, include_untracked: bool) -> &mut Self {
        self.dirty = Some(true);
        let _ = self.dirty_include_untracked(include_untracked);
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
    pub fn sha(&mut self, short: bool) -> &mut Self {
        self.sha = Some(true);
        let _ = self.sha_short(short);
        self
    }
}

impl Gitcl {
    fn any(&self) -> bool {
        self.branch
            || self.commit_author_email
            || self.commit_author_name
            || self.commit_count
            || self.commit_date
            || self.commit_message
            || self.commit_timestamp
            || self.describe
            || self.sha
            || self.dirty
    }

    /// Run at the given path
    pub fn at_path(&mut self, path: PathBuf) -> &mut Self {
        self.repo_path = Some(path);
        self
    }

    // #[cfg(test)]
    // pub(crate) fn fail(&mut self) -> &mut Self {
    //     self.fail = true;
    //     self
    // }

    /// Set the command used to test if git exists on the path.
    /// Defaults to `git --version` if not set explicitly.
    pub fn git_cmd(&mut self, cmd: Option<&'static str>) -> &mut Self {
        self.git_cmd = cmd;
        self
    }

    fn check_git(cmd: &str) -> Result<()> {
        if Self::git_cmd_exists(cmd) {
            Ok(())
        } else {
            Err(anyhow!("no suitable 'git' command found!"))
        }
    }

    fn check_inside_git_worktree(path: Option<&PathBuf>) -> Result<()> {
        if Self::inside_git_worktree(path) {
            Ok(())
        } else {
            Err(anyhow!("not within a suitable 'git' worktree!"))
        }
    }

    fn git_cmd_exists(cmd: &str) -> bool {
        Self::run_cmd(cmd, None)
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    fn inside_git_worktree(path: Option<&PathBuf>) -> bool {
        Self::run_cmd("git rev-parse --is-inside-work-tree", path)
            .map(|output| {
                let stdout = String::from_utf8_lossy(&output.stdout);
                output.status.success() && stdout.contains("true")
            })
            .unwrap_or(false)
    }

    #[cfg(not(target_env = "msvc"))]
    fn run_cmd(command: &str, path_opt: Option<&PathBuf>) -> Result<Output> {
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
        // https://git-scm.com/docs/git-status#_background_refresh
        _ = cmd.env("GIT_OPTIONAL_LOCKS", "0");
        _ = cmd.arg("-c");
        _ = cmd.arg(command);
        _ = cmd.stdout(Stdio::piped());
        _ = cmd.stderr(Stdio::piped());

        let output = cmd.output()?;
        if !output.status.success() {
            eprintln!("Command failed: `{command}`");
            eprintln!("--- stdout:\n{}\n", String::from_utf8_lossy(&output.stdout));
            eprintln!("--- stderr:\n{}\n", String::from_utf8_lossy(&output.stderr));
        }

        Ok(output)
    }

    #[cfg(target_env = "msvc")]
    fn run_cmd(command: &str, path_opt: Option<&PathBuf>) -> Result<Output> {
        let mut cmd = Command::new("cmd");
        if let Some(path) = path_opt {
            _ = cmd.current_dir(path);
        }
        // https://git-scm.com/docs/git-status#_background_refresh
        _ = cmd.env("GIT_OPTIONAL_LOCKS", "0");
        _ = cmd.arg("/c");
        _ = cmd.arg(command);
        _ = cmd.stdout(Stdio::piped());
        _ = cmd.stderr(Stdio::piped());

        let output = cmd.output()?;
        if !output.status.success() {
            eprintln!("Command failed: `{command}`");
            eprintln!("--- stdout:\n{}\n", String::from_utf8_lossy(&output.stdout));
            eprintln!("--- stderr:\n{}\n", String::from_utf8_lossy(&output.stderr));
        }

        Ok(output)
    }

    fn run_cmd_checked(command: &str, path_opt: Option<&PathBuf>) -> Result<Vec<u8>> {
        let output = Self::run_cmd(command, path_opt)?;
        if output.status.success() {
            Ok(output.stdout)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(anyhow!("Failed to run '{command}'!  {stderr}"))
        }
    }

    #[allow(clippy::too_many_lines)]
    fn inner_add_git_map_entries(
        &self,
        idempotent: bool,
        cargo_rustc_env: &mut CargoRustcEnvMap,
        cargo_rerun_if_changed: &mut CargoRerunIfChanged,
        cargo_warning: &mut CargoWarning,
    ) -> Result<()> {
        if !idempotent && self.any() {
            Self::add_rerun_if_changed(cargo_rerun_if_changed, self.repo_path.as_ref())?;
        }

        if self.branch {
            if let Ok(_value) = env::var(GIT_BRANCH_NAME) {
                add_default_map_entry(VergenKey::GitBranch, cargo_rustc_env, cargo_warning);
            } else {
                Self::add_git_cmd_entry(
                    BRANCH_CMD,
                    self.repo_path.as_ref(),
                    VergenKey::GitBranch,
                    cargo_rustc_env,
                )?;
            }
        }

        if self.commit_author_email {
            if let Ok(_value) = env::var(GIT_COMMIT_AUTHOR_EMAIL) {
                add_default_map_entry(
                    VergenKey::GitCommitAuthorEmail,
                    cargo_rustc_env,
                    cargo_warning,
                );
            } else {
                Self::add_git_cmd_entry(
                    COMMIT_AUTHOR_EMAIL,
                    self.repo_path.as_ref(),
                    VergenKey::GitCommitAuthorEmail,
                    cargo_rustc_env,
                )?;
            }
        }

        if self.commit_author_name {
            if let Ok(_value) = env::var(GIT_COMMIT_AUTHOR_NAME) {
                add_default_map_entry(
                    VergenKey::GitCommitAuthorName,
                    cargo_rustc_env,
                    cargo_warning,
                );
            } else {
                Self::add_git_cmd_entry(
                    COMMIT_AUTHOR_NAME,
                    self.repo_path.as_ref(),
                    VergenKey::GitCommitAuthorName,
                    cargo_rustc_env,
                )?;
            }
        }

        if self.commit_count {
            if let Ok(_value) = env::var(GIT_COMMIT_COUNT) {
                add_default_map_entry(VergenKey::GitCommitCount, cargo_rustc_env, cargo_warning);
            } else {
                Self::add_git_cmd_entry(
                    COMMIT_COUNT,
                    self.repo_path.as_ref(),
                    VergenKey::GitCommitCount,
                    cargo_rustc_env,
                )?;
            }
        }

        self.add_git_timestamp_entries(
            COMMIT_TIMESTAMP,
            self.repo_path.as_ref(),
            idempotent,
            cargo_rustc_env,
            cargo_warning,
        )?;

        if self.commit_message {
            if let Ok(_value) = env::var(GIT_COMMIT_MESSAGE) {
                add_default_map_entry(VergenKey::GitCommitMessage, cargo_rustc_env, cargo_warning);
            } else {
                Self::add_git_cmd_entry(
                    COMMIT_MESSAGE,
                    self.repo_path.as_ref(),
                    VergenKey::GitCommitMessage,
                    cargo_rustc_env,
                )?;
            }
        }

        let mut dirty_cache = None; // attempt to re-use dirty status later if possible
        if self.dirty {
            if let Ok(_value) = env::var(GIT_DIRTY_NAME) {
                add_default_map_entry(VergenKey::GitDirty, cargo_rustc_env, cargo_warning);
            } else {
                let dirty = self.compute_dirty(self.dirty_include_untracked)?;
                if !self.dirty_include_untracked {
                    dirty_cache = Some(dirty);
                }
                add_map_entry(
                    VergenKey::GitDirty,
                    bool::to_string(&dirty),
                    cargo_rustc_env,
                );
            }
        }

        if self.describe {
            // `git describe --dirty` does not support `GIT_OPTIONAL_LOCKS=0`
            // (see https://github.com/gitgitgadget/git/pull/1872)
            //
            // Instead, always compute the dirty status with `git status`
            if let Ok(_value) = env::var(GIT_DESCRIBE_NAME) {
                add_default_map_entry(VergenKey::GitDescribe, cargo_rustc_env, cargo_warning);
            } else {
                let mut describe_cmd = String::from(DESCRIBE);
                if self.describe_tags {
                    describe_cmd.push_str(" --tags");
                }
                if let Some(pattern) = self.describe_match_pattern {
                    Self::match_pattern_cmd_str(&mut describe_cmd, pattern);
                }
                let stdout = Self::run_cmd_checked(&describe_cmd, self.repo_path.as_ref())?;
                let mut describe_value = String::from_utf8_lossy(&stdout).trim().to_string();
                if self.describe_dirty
                    && (dirty_cache.is_some_and(|dirty| dirty) || self.compute_dirty(false)?)
                {
                    describe_value.push_str("-dirty");
                }
                add_map_entry(VergenKey::GitDescribe, describe_value, cargo_rustc_env);
            }
        }

        if self.sha {
            if let Ok(_value) = env::var(GIT_SHA_NAME) {
                add_default_map_entry(VergenKey::GitSha, cargo_rustc_env, cargo_warning);
            } else {
                let mut sha_cmd = String::from(SHA);
                if self.sha_short {
                    sha_cmd.push_str(" --short");
                }
                sha_cmd.push_str(" HEAD");
                Self::add_git_cmd_entry(
                    &sha_cmd,
                    self.repo_path.as_ref(),
                    VergenKey::GitSha,
                    cargo_rustc_env,
                )?;
            }
        }

        Ok(())
    }

    fn add_rerun_if_changed(
        rerun_if_changed: &mut Vec<String>,
        path: Option<&PathBuf>,
    ) -> Result<()> {
        let git_path = Self::run_cmd("git rev-parse --git-dir", path)?;
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
            let refp = Self::setup_ref_path(path)?;
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

    #[cfg(not(target_os = "windows"))]
    fn match_pattern_cmd_str(describe_cmd: &mut String, pattern: &str) {
        describe_cmd.push_str(" --match \"");
        describe_cmd.push_str(pattern);
        describe_cmd.push('\"');
    }

    #[cfg(target_os = "windows")]
    fn match_pattern_cmd_str(describe_cmd: &mut String, pattern: &str) {
        describe_cmd.push_str(" --match ");
        describe_cmd.push_str(pattern);
    }

    #[cfg(not(test))]
    fn setup_ref_path(path: Option<&PathBuf>) -> Result<Output> {
        Self::run_cmd("git symbolic-ref HEAD", path)
    }

    #[cfg(all(test, not(target_os = "windows")))]
    fn setup_ref_path(path: Option<&PathBuf>) -> Result<Output> {
        Self::run_cmd("pwd", path)
    }

    #[cfg(all(test, target_os = "windows"))]
    fn setup_ref_path(path: Option<&PathBuf>) -> Result<Output> {
        Self::run_cmd("cd", path)
    }

    fn add_git_cmd_entry(
        cmd: &str,
        path: Option<&PathBuf>,
        key: VergenKey,
        cargo_rustc_env: &mut CargoRustcEnvMap,
    ) -> Result<()> {
        let stdout = Self::run_cmd_checked(cmd, path)?;
        let stdout = String::from_utf8_lossy(&stdout)
            .trim()
            .trim_matches('\'')
            .to_string();
        add_map_entry(key, stdout, cargo_rustc_env);
        Ok(())
    }

    fn add_git_timestamp_entries(
        &self,
        cmd: &str,
        path: Option<&PathBuf>,
        idempotent: bool,
        cargo_rustc_env: &mut CargoRustcEnvMap,
        cargo_warning: &mut CargoWarning,
    ) -> Result<()> {
        let mut date_override = false;
        if let Ok(_value) = env::var(GIT_COMMIT_DATE_NAME) {
            add_default_map_entry(VergenKey::GitCommitDate, cargo_rustc_env, cargo_warning);
            date_override = true;
        }

        let mut timestamp_override = false;
        if let Ok(_value) = env::var(GIT_COMMIT_TIMESTAMP_NAME) {
            add_default_map_entry(
                VergenKey::GitCommitTimestamp,
                cargo_rustc_env,
                cargo_warning,
            );
            timestamp_override = true;
        }

        let output = Self::run_cmd(cmd, path)?;
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
                Err(VarError::NotPresent) => self.compute_local_offset(&stdout)?,
                Err(e) => return Err(e.into()),
            };

            if idempotent && !sde {
                if self.commit_date && !date_override {
                    add_default_map_entry(VergenKey::GitCommitDate, cargo_rustc_env, cargo_warning);
                }

                if self.commit_timestamp && !timestamp_override {
                    add_default_map_entry(
                        VergenKey::GitCommitTimestamp,
                        cargo_rustc_env,
                        cargo_warning,
                    );
                }
            } else {
                if self.commit_date && !date_override {
                    let format = format_description::parse("[year]-[month]-[day]")?;
                    add_map_entry(
                        VergenKey::GitCommitDate,
                        ts.format(&format)?,
                        cargo_rustc_env,
                    );
                }

                if self.commit_timestamp && !timestamp_override {
                    add_map_entry(
                        VergenKey::GitCommitTimestamp,
                        ts.format(&Iso8601::DEFAULT)?,
                        cargo_rustc_env,
                    );
                }
            }
        } else {
            if self.commit_date && !date_override {
                add_default_map_entry(VergenKey::GitCommitDate, cargo_rustc_env, cargo_warning);
            }

            if self.commit_timestamp && !timestamp_override {
                add_default_map_entry(
                    VergenKey::GitCommitTimestamp,
                    cargo_rustc_env,
                    cargo_warning,
                );
            }
        }

        Ok(())
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    // this in not included in coverage, because on *nix the local offset is always unsafe
    fn compute_local_offset(&self, stdout: &str) -> Result<(bool, OffsetDateTime)> {
        let no_offset = OffsetDateTime::parse(stdout, &Rfc3339)?;
        if self.use_local {
            let local = UtcOffset::local_offset_at(no_offset)?;
            let local_offset = no_offset.checked_to_offset(local).unwrap_or(no_offset);
            Ok((false, local_offset))
        } else {
            Ok((false, no_offset))
        }
    }

    fn compute_dirty(&self, include_untracked: bool) -> Result<bool> {
        let mut dirty_cmd = String::from(DIRTY);
        if !include_untracked {
            dirty_cmd.push_str(" --untracked-files=no");
        }
        let stdout = Self::run_cmd_checked(&dirty_cmd, self.repo_path.as_ref())?;
        Ok(!stdout.is_empty())
    }
}

impl AddEntries for Gitcl {
    fn add_map_entries(
        &self,
        idempotent: bool,
        cargo_rustc_env: &mut CargoRustcEnvMap,
        cargo_rerun_if_changed: &mut CargoRerunIfChanged,
        cargo_warning: &mut CargoWarning,
    ) -> Result<()> {
        if self.any() {
            let git_cmd = self.git_cmd.unwrap_or("git --version");
            Self::check_git(git_cmd)
                .and_then(|()| Self::check_inside_git_worktree(self.repo_path.as_ref()))?;
            self.inner_add_git_map_entries(
                idempotent,
                cargo_rustc_env,
                cargo_rerun_if_changed,
                cargo_warning,
            )?;
        }
        Ok(())
    }

    fn add_default_entries(
        &self,
        config: &DefaultConfig,
        cargo_rustc_env_map: &mut CargoRustcEnvMap,
        cargo_rerun_if_changed: &mut CargoRerunIfChanged,
        cargo_warning: &mut CargoWarning,
    ) -> Result<()> {
        if *config.fail_on_error() {
            let error = Error::msg(format!("{}", config.error()));
            Err(error)
        } else {
            // Clear any previous data.  We are re-populating
            // map isn't cleared because keys will overwrite.
            cargo_warning.clear();
            cargo_rerun_if_changed.clear();

            cargo_warning.push(format!("{}", config.error()));

            if self.branch {
                add_default_map_entry(VergenKey::GitBranch, cargo_rustc_env_map, cargo_warning);
            }
            if self.commit_author_email {
                add_default_map_entry(
                    VergenKey::GitCommitAuthorEmail,
                    cargo_rustc_env_map,
                    cargo_warning,
                );
            }
            if self.commit_author_name {
                add_default_map_entry(
                    VergenKey::GitCommitAuthorName,
                    cargo_rustc_env_map,
                    cargo_warning,
                );
            }
            if self.commit_count {
                add_default_map_entry(
                    VergenKey::GitCommitCount,
                    cargo_rustc_env_map,
                    cargo_warning,
                );
            }
            if self.commit_date {
                add_default_map_entry(VergenKey::GitCommitDate, cargo_rustc_env_map, cargo_warning);
            }
            if self.commit_message {
                add_default_map_entry(
                    VergenKey::GitCommitMessage,
                    cargo_rustc_env_map,
                    cargo_warning,
                );
            }
            if self.commit_timestamp {
                add_default_map_entry(
                    VergenKey::GitCommitTimestamp,
                    cargo_rustc_env_map,
                    cargo_warning,
                );
            }
            if self.describe {
                add_default_map_entry(VergenKey::GitDescribe, cargo_rustc_env_map, cargo_warning);
            }
            if self.sha {
                add_default_map_entry(VergenKey::GitSha, cargo_rustc_env_map, cargo_warning);
            }
            if self.dirty {
                add_default_map_entry(VergenKey::GitDirty, cargo_rustc_env_map, cargo_warning);
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Gitcl, GitclBuilder};
    use crate::Emitter;
    use anyhow::Result;
    use serial_test::serial;
    #[cfg(unix)]
    use std::io::stdout;
    use std::{collections::BTreeMap, env::temp_dir, io::Write};
    use test_util::TestRepos;
    #[cfg(unix)]
    use test_util::TEST_MTIME;
    use vergen_lib::{count_idempotent, VergenKey};

    #[test]
    #[serial]
    #[allow(clippy::clone_on_copy, clippy::redundant_clone)]
    fn gitcl_clone_works() -> Result<()> {
        let gitcl = GitclBuilder::all_git()?;
        let another = gitcl.clone();
        assert_eq!(another, gitcl);
        Ok(())
    }

    #[test]
    #[serial]
    fn gitcl_debug_works() -> Result<()> {
        let gitcl = GitclBuilder::all_git()?;
        let mut buf = vec![];
        write!(buf, "{gitcl:?}")?;
        assert!(!buf.is_empty());
        Ok(())
    }

    #[test]
    #[serial]
    fn gix_default() -> Result<()> {
        let gitcl = GitclBuilder::default().build()?;
        let emitter = Emitter::default().add_instructions(&gitcl)?.test_emit();
        assert_eq!(0, emitter.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(0, emitter.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn bad_command_is_error() -> Result<()> {
        let mut map = BTreeMap::new();
        assert!(Gitcl::add_git_cmd_entry(
            "such_a_terrible_cmd",
            None,
            VergenKey::GitCommitMessage,
            &mut map
        )
        .is_err());
        Ok(())
    }

    #[test]
    #[serial]
    fn non_working_tree_is_error() -> Result<()> {
        assert!(Gitcl::check_inside_git_worktree(Some(&temp_dir())).is_err());
        Ok(())
    }

    #[test]
    #[serial]
    fn invalid_git_is_error() -> Result<()> {
        assert!(Gitcl::check_git("such_a_terrible_cmd -v").is_err());
        Ok(())
    }

    #[cfg(not(target_family = "windows"))]
    #[test]
    #[serial]
    fn shell_env_works() -> Result<()> {
        temp_env::with_var("SHELL", Some("bash"), || {
            let mut map = BTreeMap::new();
            assert!(Gitcl::add_git_cmd_entry(
                "git -v",
                None,
                VergenKey::GitCommitMessage,
                &mut map
            )
            .is_ok());
        });
        Ok(())
    }

    #[test]
    #[serial]
    fn git_all_idempotent() -> Result<()> {
        let gitcl = GitclBuilder::all_git()?;
        let emitter = Emitter::default()
            .idempotent()
            .add_instructions(&gitcl)?
            .test_emit();
        assert_eq!(10, emitter.cargo_rustc_env_map().len());
        assert_eq!(2, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(2, emitter.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn git_all_idempotent_no_warn() -> Result<()> {
        let gitcl = GitclBuilder::all_git()?;
        let emitter = Emitter::default()
            .idempotent()
            .quiet()
            .add_instructions(&gitcl)?
            .test_emit();
        assert_eq!(10, emitter.cargo_rustc_env_map().len());
        assert_eq!(2, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(2, emitter.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn git_all_at_path() -> Result<()> {
        let repo = TestRepos::new(false, false, false)?;
        let mut gitcl = GitclBuilder::all_git()?;
        let _ = gitcl.at_path(repo.path());
        let emitter = Emitter::default().add_instructions(&gitcl)?.test_emit();
        assert_eq!(10, emitter.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(0, emitter.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn git_all() -> Result<()> {
        let gitcl = GitclBuilder::all_git()?;
        let emitter = Emitter::default().add_instructions(&gitcl)?.test_emit();
        assert_eq!(10, emitter.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(0, emitter.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn git_all_shallow_clone() -> Result<()> {
        let repo = TestRepos::new(false, false, true)?;
        let mut gitcl = GitclBuilder::all_git()?;
        let _ = gitcl.at_path(repo.path());
        let emitter = Emitter::default().add_instructions(&gitcl)?.test_emit();
        assert_eq!(10, emitter.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(0, emitter.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn git_all_dirty_tags_short() -> Result<()> {
        let gitcl = GitclBuilder::default()
            .all()
            .describe(true, true, None)
            .sha(true)
            .build()?;
        let emitter = Emitter::default().add_instructions(&gitcl)?.test_emit();
        assert_eq!(10, emitter.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(0, emitter.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn fails_on_bad_git_command() -> Result<()> {
        let mut gitcl = GitclBuilder::all_git()?;
        let _ = gitcl.git_cmd(Some("this_is_not_a_git_cmd"));
        assert!(Emitter::default()
            .fail_on_error()
            .add_instructions(&gitcl)
            .is_err());
        Ok(())
    }

    #[test]
    #[serial]
    fn defaults_on_bad_git_command() -> Result<()> {
        let mut gitcl = GitclBuilder::all_git()?;
        let _ = gitcl.git_cmd(Some("this_is_not_a_git_cmd"));
        let emitter = Emitter::default().add_instructions(&gitcl)?.test_emit();
        assert_eq!(10, emitter.cargo_rustc_env_map().len());
        assert_eq!(10, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(11, emitter.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn bad_timestamp_defaults() -> Result<()> {
        let mut map = BTreeMap::new();
        let mut warnings = vec![];
        let gitcl = GitclBuilder::all_git()?;
        assert!(gitcl
            .add_git_timestamp_entries(
                "this_is_not_a_git_cmd",
                None,
                false,
                &mut map,
                &mut warnings
            )
            .is_ok());
        assert_eq!(2, map.len());
        assert_eq!(2, warnings.len());
        Ok(())
    }

    #[test]
    #[serial]
    fn source_date_epoch_works() {
        temp_env::with_var("SOURCE_DATE_EPOCH", Some("1671809360"), || {
            let result = || -> Result<()> {
                let mut stdout_buf = vec![];
                let gitcl = GitclBuilder::default()
                    .commit_date(true)
                    .commit_timestamp(true)
                    .build()?;
                _ = Emitter::new()
                    .idempotent()
                    .add_instructions(&gitcl)?
                    .emit_to(&mut stdout_buf)?;
                let output = String::from_utf8_lossy(&stdout_buf);
                for (idx, line) in output.lines().enumerate() {
                    if idx == 0 {
                        assert_eq!("cargo:rustc-env=VERGEN_GIT_COMMIT_DATE=2022-12-23", line);
                    } else if idx == 1 {
                        assert_eq!(
                            "cargo:rustc-env=VERGEN_GIT_COMMIT_TIMESTAMP=2022-12-23T15:29:20.000000000Z",
                            line
                        );
                    }
                }
                Ok(())
            }();
            assert!(result.is_ok());
        });
    }

    #[test]
    #[serial]
    #[cfg(unix)]
    fn bad_source_date_epoch_fails() {
        use std::ffi::OsStr;
        use std::os::unix::prelude::OsStrExt;

        let source = [0x66, 0x6f, 0x80, 0x6f];
        let os_str = OsStr::from_bytes(&source[..]);
        temp_env::with_var("SOURCE_DATE_EPOCH", Some(os_str), || {
            let result = || -> Result<bool> {
                let mut stdout_buf = vec![];
                let gitcl = GitclBuilder::default().commit_date(true).build()?;
                Emitter::new()
                    .idempotent()
                    .fail_on_error()
                    .add_instructions(&gitcl)?
                    .emit_to(&mut stdout_buf)
            }();
            assert!(result.is_err());
        });
    }

    #[test]
    #[serial]
    #[cfg(unix)]
    fn bad_source_date_epoch_defaults() {
        use std::ffi::OsStr;
        use std::os::unix::prelude::OsStrExt;

        let source = [0x66, 0x6f, 0x80, 0x6f];
        let os_str = OsStr::from_bytes(&source[..]);
        temp_env::with_var("SOURCE_DATE_EPOCH", Some(os_str), || {
            let result = || -> Result<bool> {
                let mut stdout_buf = vec![];
                let gitcl = GitclBuilder::default().commit_date(true).build()?;
                Emitter::new()
                    .idempotent()
                    .add_instructions(&gitcl)?
                    .emit_to(&mut stdout_buf)
            }();
            assert!(result.is_ok());
        });
    }

    #[test]
    #[serial]
    #[cfg(windows)]
    fn bad_source_date_epoch_fails() {
        use std::ffi::OsString;
        use std::os::windows::prelude::OsStringExt;

        let source = [0x0066, 0x006f, 0xD800, 0x006f];
        let os_string = OsString::from_wide(&source[..]);
        let os_str = os_string.as_os_str();
        temp_env::with_var("SOURCE_DATE_EPOCH", Some(os_str), || {
            let result = || -> Result<bool> {
                let mut stdout_buf = vec![];
                let gitcl = GitclBuilder::default().commit_date(true).build()?;
                Emitter::new()
                    .fail_on_error()
                    .idempotent()
                    .add_instructions(&gitcl)?
                    .emit_to(&mut stdout_buf)
            }();
            assert!(result.is_err());
        });
    }

    #[test]
    #[serial]
    #[cfg(windows)]
    fn bad_source_date_epoch_defaults() {
        use std::ffi::OsString;
        use std::os::windows::prelude::OsStringExt;

        let source = [0x0066, 0x006f, 0xD800, 0x006f];
        let os_string = OsString::from_wide(&source[..]);
        let os_str = os_string.as_os_str();
        temp_env::with_var("SOURCE_DATE_EPOCH", Some(os_str), || {
            let result = || -> Result<bool> {
                let mut stdout_buf = vec![];
                let gitcl = GitclBuilder::default().commit_date(true).build()?;
                Emitter::new()
                    .idempotent()
                    .add_instructions(&gitcl)?
                    .emit_to(&mut stdout_buf)
            }();
            assert!(result.is_ok());
        });
    }

    #[test]
    #[serial]
    #[cfg(unix)]
    fn git_no_index_update() -> Result<()> {
        let repo = TestRepos::new(true, true, false)?;
        repo.set_index_magic_mtime()?;

        // The GIT_OPTIONAL_LOCKS=0 environment variable should prevent modifications to the index
        let mut gitcl = GitclBuilder::default()
            .all()
            .describe(true, true, None)
            .build()?;
        let _ = gitcl.at_path(repo.path());
        let failed = Emitter::default()
            .add_instructions(&gitcl)?
            .emit_to(&mut stdout())?;
        assert!(!failed);

        assert_eq!(*TEST_MTIME, repo.get_index_magic_mtime()?);
        Ok(())
    }
}
