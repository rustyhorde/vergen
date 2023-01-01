// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use crate::{
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
    OffsetDateTime,
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
    // git describe --always (optionally --tags, --dirty)
    pub(crate) git_describe: bool,
    git_describe_dirty: bool,
    git_describe_tags: bool,
    // git rev-parse HEAD (optionally with --short)
    pub(crate) git_sha: bool,
    git_sha_short: bool,
    #[cfg(test)]
    git_cmd: Option<&'static str>,
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
/// # use vergen::EmitBuilder;
/// #
/// # fn main() -> Result<()> {
/// EmitBuilder::builder().all_git().emit()?;
/// #   Ok(())
/// # }
/// ```
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
            .git_describe(false, false)
            .git_sha(false)
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
    }

    /// Emit the current git branch
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_GIT_BRANCH=<BRANCH_NAME>
    /// ```
    ///
    /// The following command outputs the current branch
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
    /// The following command outputs the commit author email
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
    /// The following command outputs the commit author name
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
    /// The following command outputs the commit count
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
    /// The following command outputs the commit date
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
    /// The following command outputs the commit message
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
    /// The following command outputs the commit timestamp
    /// ```text
    #[doc = concat!(commit_message!())]
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
    /// The following command outputs describe
    /// ```text
    #[doc = concat!(describe!())]
    /// ```
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

    /// Emit the SHA of the latest commit
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_GIT_SHA=<SHA>
    /// ```
    ///
    /// The following command outputs the SHA
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
            Ok(())
        }
    }

    #[cfg(not(test))]
    pub(crate) fn add_git_map_entries(
        &self,
        idempotent: bool,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
        rerun_if_changed: &mut Vec<String>,
    ) -> Result<()> {
        check_git("git -v").and_then(check_inside_git_worktree)?;
        self.inner_add_git_map_entries(idempotent, map, warnings, rerun_if_changed)
    }

    #[cfg(test)]
    pub(crate) fn add_git_map_entries(
        &self,
        idempotent: bool,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
        rerun_if_changed: &mut Vec<String>,
    ) -> Result<()> {
        let git_cmd = if let Some(cmd) = self.git_config.git_cmd {
            cmd
        } else {
            "git -v"
        };
        check_git(git_cmd).and_then(check_inside_git_worktree)?;
        self.inner_add_git_map_entries(idempotent, map, warnings, rerun_if_changed)
    }

    fn inner_add_git_map_entries(
        &self,
        idempotent: bool,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
        rerun_if_changed: &mut Vec<String>,
    ) -> Result<()> {
        if !idempotent && self.any() {
            add_rerun_if_changed(rerun_if_changed)?;
        }

        if self.git_config.git_branch {
            add_git_cmd_entry(BRANCH_CMD, VergenKey::GitBranch, map)?;
        }

        if self.git_config.git_commit_author_email {
            add_git_cmd_entry(COMMIT_AUTHOR_EMAIL, VergenKey::GitCommitAuthorEmail, map)?;
        }

        if self.git_config.git_commit_author_name {
            add_git_cmd_entry(COMMIT_AUTHOR_NAME, VergenKey::GitCommitAuthorName, map)?;
        }

        if self.git_config.git_commit_count {
            add_git_cmd_entry(COMMIT_COUNT, VergenKey::GitCommitCount, map)?;
        }

        self.add_git_timestamp_entries(COMMIT_TIMESTAMP, idempotent, map, warnings)?;

        if self.git_config.git_commit_message {
            add_git_cmd_entry(COMMIT_MESSAGE, VergenKey::GitCommitMessage, map)?;
        }

        if self.git_config.git_describe {
            let mut describe_cmd = String::from(DESCRIBE);
            if self.git_config.git_describe_dirty {
                describe_cmd.push_str(" --dirty");
            }
            if self.git_config.git_describe_tags {
                describe_cmd.push_str(" --tags");
            }
            add_git_cmd_entry(&describe_cmd, VergenKey::GitDescribe, map)?;
        }

        if self.git_config.git_sha {
            let mut sha_cmd = String::from(SHA);
            if self.git_config.git_sha_short {
                sha_cmd.push_str(" --short");
            }
            sha_cmd.push_str(" HEAD");
            add_git_cmd_entry(&sha_cmd, VergenKey::GitSha, map)?;
        }
        Ok(())
    }

    fn add_git_timestamp_entries(
        &self,
        cmd: &str,
        idempotent: bool,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
    ) -> Result<()> {
        let output = run_cmd(cmd)?;
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout)
                .trim()
                .trim_matches('\'')
                .to_string();

            let (sde, ts) = match env::var("SOURCE_DATE_EPOCH") {
                Ok(v) => (
                    true,
                    OffsetDateTime::from_unix_timestamp(i64::from_str(&v)?)?,
                ),
                Err(std::env::VarError::NotPresent) => {
                    (false, OffsetDateTime::parse(&stdout, &Rfc3339)?)
                }
                Err(e) => return Err(e.into()),
            };

            if idempotent && !sde {
                if self.git_config.git_commit_date {
                    add_default_map_entry(VergenKey::GitCommitDate, map, warnings);
                }

                if self.git_config.git_commit_timestamp {
                    add_default_map_entry(VergenKey::GitCommitTimestamp, map, warnings);
                }
            } else {
                if self.git_config.git_commit_date {
                    let format = format_description::parse("[year]-[month]-[day]")?;
                    add_map_entry(VergenKey::GitCommitDate, ts.format(&format)?, map);
                }

                if self.git_config.git_commit_timestamp {
                    add_map_entry(
                        VergenKey::GitCommitTimestamp,
                        ts.format(&Iso8601::DEFAULT)?,
                        map,
                    );
                }
            }
        } else {
            if self.git_config.git_commit_date {
                add_default_map_entry(VergenKey::GitCommitDate, map, warnings);
            }

            if self.git_config.git_commit_timestamp {
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

fn check_inside_git_worktree(_: ()) -> Result<()> {
    if inside_git_worktree() {
        Ok(())
    } else {
        Err(anyhow!("not within a suitable 'git' worktree!"))
    }
}

fn git_cmd_exists(cmd: &str) -> bool {
    run_cmd(cmd)
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

#[cfg(not(target_env = "msvc"))]
fn run_cmd(command: &str) -> Result<Output> {
    let shell = if let Some(shell_path) = env::var_os("SHELL") {
        shell_path.to_string_lossy().into_owned()
    } else {
        // Fallback to sh if SHELL not defined
        "sh".to_string()
    };
    let mut cmd = Command::new(shell);
    let _ = cmd.arg("-c");
    let _ = cmd.arg(command);
    let _ = cmd.stdout(Stdio::piped());
    let _ = cmd.stderr(Stdio::piped());
    Ok(cmd.output()?)
}

#[cfg(target_env = "msvc")]
fn run_cmd(command: &str) -> Result<Output> {
    let mut cmd = Command::new("cmd");
    let _ = cmd.arg("/c");
    let _ = cmd.arg(command);
    let _ = cmd.stdout(Stdio::piped());
    let _ = cmd.stderr(Stdio::piped());
    Ok(cmd.output()?)
}

fn add_git_cmd_entry(cmd: &str, key: VergenKey, map: &mut RustcEnvMap) -> Result<()> {
    let output = run_cmd(cmd)?;
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout)
            .trim()
            .trim_matches('\'')
            .to_string();
        add_map_entry(key, stdout, map);
    } else {
        return Err(anyhow!("Failed to run '{cmd}'!"));
    }
    Ok(())
}

fn add_rerun_if_changed(rerun_if_changed: &mut Vec<String>) -> Result<()> {
    let git_path = run_cmd("git rev-parse --git-dir")?;
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
        let refp = setup_ref_path()?;
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
fn setup_ref_path() -> Result<Output> {
    run_cmd("git symbolic-ref HEAD")
}

#[cfg(all(test, not(target_os = "windows")))]
fn setup_ref_path() -> Result<Output> {
    run_cmd("pwd")
}

#[cfg(all(test, target_os = "windows"))]
fn setup_ref_path() -> Result<Output> {
    run_cmd("cd")
}

#[cfg(test)]
mod test {
    use super::{add_git_cmd_entry, check_git, check_inside_git_worktree};
    use crate::{emitter::test::count_idempotent, key::VergenKey, EmitBuilder};
    use anyhow::Result;
    use std::{collections::BTreeMap, env};

    #[test]
    #[serial_test::parallel]
    fn bad_command_is_error() -> Result<()> {
        let mut map = BTreeMap::new();
        assert!(
            add_git_cmd_entry("such_a_terrible_cmd", VergenKey::GitCommitMessage, &mut map)
                .is_err()
        );
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn non_working_tree_is_error() -> Result<()> {
        let curr_dir = env::current_dir()?;
        env::set_current_dir("..")?;
        assert!(check_inside_git_worktree(()).is_err());
        env::set_current_dir(curr_dir)?;
        Ok(())
    }

    #[test]
    #[serial_test::parallel]
    fn invalid_git_is_error() -> Result<()> {
        assert!(check_git("such_a_terrible_cmd -v").is_err());
        Ok(())
    }

    #[cfg(not(target_family = "windows"))]
    #[test]
    #[serial_test::serial]
    fn shell_env_works() -> Result<()> {
        let curr_shell = env::var("SHELL");
        env::set_var("SHELL", "bash");
        let mut map = BTreeMap::new();
        assert!(add_git_cmd_entry("git -v", VergenKey::GitCommitMessage, &mut map).is_ok());
        if let Ok(curr_shell) = curr_shell {
            env::set_var("SHELL", curr_shell);
        }
        Ok(())
    }

    #[test]
    #[serial_test::parallel]
    fn git_all_idempotent() -> Result<()> {
        let config = EmitBuilder::builder().idempotent().all_git().test_emit()?;
        assert_eq!(9, config.cargo_rustc_env_map.len());
        assert_eq!(2, count_idempotent(&config.cargo_rustc_env_map));
        assert_eq!(2, config.warnings.len());
        Ok(())
    }

    #[test]
    #[serial_test::parallel]
    fn git_all() -> Result<()> {
        let config = EmitBuilder::builder().all_git().test_emit()?;
        assert_eq!(9, config.cargo_rustc_env_map.len());
        assert_eq!(0, count_idempotent(&config.cargo_rustc_env_map));
        assert_eq!(0, config.warnings.len());
        Ok(())
    }

    #[test]
    #[serial_test::parallel]
    fn git_all_dirty_tags_short() -> Result<()> {
        let config = EmitBuilder::builder()
            .all_git()
            .git_describe(true, true)
            .git_sha(true)
            .test_emit()?;
        assert_eq!(9, config.cargo_rustc_env_map.len());
        assert_eq!(0, count_idempotent(&config.cargo_rustc_env_map));
        assert_eq!(0, config.warnings.len());
        Ok(())
    }

    #[test]
    #[serial_test::parallel]
    fn fails_on_bad_git_command() -> Result<()> {
        let mut config = EmitBuilder::builder();
        let _ = config.fail_on_error();
        let _ = config.all_git();
        config.git_config.git_cmd = Some("this_is_not_a_git_cmd");
        assert!(config.test_emit().is_err());
        Ok(())
    }

    #[test]
    #[serial_test::parallel]
    fn defaults_on_bad_git_command() -> Result<()> {
        let mut config = EmitBuilder::builder();
        let _ = config.all_git();
        config.git_config.git_cmd = Some("this_is_not_a_git_cmd");
        let emitter = config.test_emit()?;
        assert_eq!(9, emitter.cargo_rustc_env_map.len());
        assert_eq!(9, count_idempotent(&emitter.cargo_rustc_env_map));
        assert_eq!(9, emitter.warnings.len());
        Ok(())
    }

    #[test]
    #[serial_test::parallel]
    fn bad_timestamp_defaults() -> Result<()> {
        let mut map = BTreeMap::new();
        let mut warnings = vec![];
        let mut config = EmitBuilder::builder();
        let _ = config.all_git();
        assert!(config
            .add_git_timestamp_entries("this_is_not_a_git_cmd", false, &mut map, &mut warnings)
            .is_ok());
        assert_eq!(2, map.len());
        assert_eq!(2, warnings.len());
        Ok(())
    }
}
