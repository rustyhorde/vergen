// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use crate::{
    builder::{Builder, RustcEnvMap},
    key::VergenKey,
};
use anyhow::{anyhow, Error, Result};
use std::{
    env,
    process::{Command, Output, Stdio},
};

#[derive(Clone, Copy, Debug, Default)]
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
    // git describe --always (optionally --tags, --dirty)
    pub(crate) git_describe: bool,
    git_describe_dirty: bool,
    git_describe_tags: bool,
    // git rev-parse HEAD (optionally with --short)
    pub(crate) git_sha: bool,
    git_sha_short: bool,
}

impl Config {
    #[cfg(test)]
    fn enable_all(&mut self) {
        super::enable_all(self);
    }

    pub(crate) fn add_warnings(
        self,
        skip_if_error: bool,
        e: Error,
        warnings: &mut Vec<String>,
    ) -> Result<()> {
        super::add_warnings(self, skip_if_error, e, warnings)
    }
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
///
/// # Example
///
/// ```
/// # use anyhow::Result;
/// # use vergen::Vergen;
/// #
/// # fn main() -> Result<()> {
/// Vergen::default().all_git().gen()?;
/// #   Ok(())
/// # }
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "git")))]
impl Builder {
    /// Enable all of the `VERGEN_GIT_*` options
    pub fn all_git(&mut self) -> &mut Self {
        self.git_branch()
            .git_commit_author_email()
            .git_commit_author_name()
            .git_commit_count()
            .git_commit_date()
            .git_commit_message()
            .git_commit_timestamp()
            .git_describe(false, false)
            .git_sha(false)
    }

    /// Emit the git branch instruction
    pub fn git_branch(&mut self) -> &mut Self {
        self.git_config.git_branch = true;
        self
    }

    /// Emit the git commit author email instruction
    pub fn git_commit_author_email(&mut self) -> &mut Self {
        self.git_config.git_commit_author_email = true;
        self
    }

    /// Emit the git commit author name instruction
    pub fn git_commit_author_name(&mut self) -> &mut Self {
        self.git_config.git_commit_author_name = true;
        self
    }

    /// Emit the git commit count instruction
    pub fn git_commit_count(&mut self) -> &mut Self {
        self.git_config.git_commit_count = true;
        self
    }

    /// Emit the git commit date instruction
    pub fn git_commit_date(&mut self) -> &mut Self {
        self.git_config.git_commit_date = true;
        self
    }

    /// Emit the git commit message instruction
    pub fn git_commit_message(&mut self) -> &mut Self {
        self.git_config.git_commit_message = true;
        self
    }

    /// Emit the git commit timestamp instruction
    pub fn git_commit_timestamp(&mut self) -> &mut Self {
        self.git_config.git_commit_timestamp = true;
        self
    }

    /// Emit the git describe instruction
    ///
    /// Optionally, add the `dirty` or `tags` flag to describe.
    /// See [`git describe`](https://git-scm.com/docs/git-describe#_options) for more details
    ///
    pub fn git_describe(&mut self, dirty: bool, tags: bool) -> &mut Self {
        self.git_config.git_describe = true;
        self.git_config.git_describe_dirty = dirty;
        self.git_config.git_describe_tags = tags;
        self
    }

    /// Emit the git SHA instruction
    ///
    /// Optionally, add the `short` flag to rev-parse.
    /// See [`git rev-parse`](https://git-scm.com/docs/git-rev-parse#_options_for_output) for more details.
    ///
    pub fn git_sha(&mut self, short: bool) -> &mut Self {
        self.git_config.git_sha = true;
        self.git_config.git_sha_short = short;
        self
    }

    pub(crate) fn add_git_map_entries(&self, map: &mut RustcEnvMap) -> Result<()> {
        check_git()?;
        if self.git_config.git_branch {
            let output = run_cmd("git rev-parse --abbrev-ref --symbolic-full-name HEAD")?;
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let _old = map.insert(VergenKey::GitBranch, stdout);
            } else {
                return Err(anyhow!("'git rev-parse' command failed!"));
            }
        }

        if self.git_config.git_commit_author_email {
            let output = run_cmd("git log -1 --pretty=format:'%ae'")?;
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let _old = map.insert(VergenKey::GitCommitAuthorEmail, stdout);
            } else {
                return Err(anyhow!("'git log' command failed!"));
            }
        }

        if self.git_config.git_commit_author_name {
            let output = run_cmd("git log -1 --pretty=format:'%an'")?;
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let _old = map.insert(VergenKey::GitCommitAuthorName, stdout);
            } else {
                return Err(anyhow!("'git log' command failed!"));
            }
        }

        if self.git_config.git_commit_count {
            let output = run_cmd("git rev-list --count HEAD")?;
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let _old = map.insert(VergenKey::GitCommitCount, stdout);
            } else {
                return Err(anyhow!("'git rev-list' command failed!"));
            }
        }

        if self.git_config.git_commit_date {
            let output = run_cmd("git log -1 --pretty=format:'%cs'")?;
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let _old = map.insert(VergenKey::GitCommitDate, stdout);
            } else {
                return Err(anyhow!("'git log' command failed!"));
            }
        }

        if self.git_config.git_commit_message {
            let output = run_cmd("git log -1 --format=%s")?;
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let _old = map.insert(VergenKey::GitCommitMessage, stdout);
            } else {
                return Err(anyhow!("'git log' command failed!"));
            }
        }

        if self.git_config.git_commit_timestamp {
            let output = run_cmd("git log -1 --pretty=format:'%cI'")?;
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let _old = map.insert(VergenKey::GitCommitTimestamp, stdout);
            } else {
                return Err(anyhow!("'git log' command failed!"));
            }
        }

        if self.git_config.git_describe {
            let mut describe_cmd = String::from("git describe --always");
            if self.git_config.git_describe_dirty {
                describe_cmd.push_str(" --dirty");
            }
            if self.git_config.git_describe_tags {
                describe_cmd.push_str(" --tags");
            }
            let output = run_cmd(&describe_cmd)?;
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let _old = map.insert(VergenKey::GitDescribe, stdout);
            } else {
                return Err(anyhow!("'git describe' command failed!"));
            }
        }

        if self.git_config.git_sha {
            let mut sha_cmd = String::from("git rev-parse");
            if self.git_config.git_sha_short {
                sha_cmd.push_str(" --short");
            }
            sha_cmd.push_str(" HEAD");
            let output = run_cmd(&sha_cmd)?;
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let _old = map.insert(VergenKey::GitSha, stdout);
            } else {
                return Err(anyhow!("'git rev-parse' command failed!"));
            }
        }
        Ok(())
    }
}

fn check_git() -> Result<()> {
    if git_cmd_exists() {
        if inside_git_worktree() {
            Ok(())
        } else {
            Err(anyhow!("not within a suitable 'git' worktree!"))
        }
    } else {
        Err(anyhow!("no suitable 'git' command found!"))
    }
}

fn git_cmd_exists() -> bool {
    run_cmd("git -v")
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn inside_git_worktree() -> bool {
    run_cmd("git rev-parse --is-inside-work-tree")
        .map(|output| {
            let stdout = String::from_utf8_lossy(&output.stdout);
            output.status.success() && stdout.trim() == "true"
        })
        .unwrap_or(false)
}

fn run_cmd(command: &str) -> Result<Output> {
    if let Some(shell_path) = env::var_os("SHELL") {
        let shell = shell_path.to_string_lossy().to_string();
        let mut cmd = Command::new(shell);
        let _ = cmd.arg("-c");
        let _ = cmd.arg(command);
        let _ = cmd.stdout(Stdio::piped());
        let _ = cmd.stderr(Stdio::piped());
        Ok(cmd.output()?)
    } else {
        Err(anyhow!("cannot find suitable shell"))
    }
}

#[cfg(test)]
mod test {
    use super::Config;
    use crate::{builder::test::count_idempotent, Vergen};
    use anyhow::{anyhow, Result};

    #[test]
    #[serial_test::parallel]
    fn add_warnings_is_err() -> Result<()> {
        let config = Config::default();
        let mut warnings = vec![];
        assert!(config
            .add_warnings(false, anyhow!("test"), &mut warnings)
            .is_err());
        Ok(())
    }

    #[test]
    #[serial_test::parallel]
    fn add_warnings_adds_warnings() -> Result<()> {
        let mut config = Config::default();
        config.enable_all();

        let mut warnings = vec![];
        assert!(config
            .add_warnings(true, anyhow!("test"), &mut warnings)
            .is_ok());
        assert_eq!(9, warnings.len());
        Ok(())
    }

    #[test]
    #[serial_test::parallel]
    fn git_all_idempotent() -> Result<()> {
        let config = Vergen::default().idempotent().all_git().test_gen()?;
        assert_eq!(9, config.cargo_rustc_env_map.len());
        assert_eq!(0, count_idempotent(config.cargo_rustc_env_map));
        assert_eq!(0, config.warnings.len());
        Ok(())
    }

    #[test]
    #[serial_test::parallel]
    fn git_all() -> Result<()> {
        let config = Vergen::default().all_git().test_gen()?;
        assert_eq!(9, config.cargo_rustc_env_map.len());
        for (k, v) in &config.cargo_rustc_env_map {
            println!("cargo:rustc-env={}={v}", k.name());
        }
        assert_eq!(0, count_idempotent(config.cargo_rustc_env_map));
        assert_eq!(0, config.warnings.len());
        Ok(())
    }
}
