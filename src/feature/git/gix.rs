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
#[cfg(test)]
use anyhow::anyhow;
use anyhow::{Error, Result};
// remove these when impl complete
use git_repository as _;
use time as _;

#[derive(Clone, Copy, Debug, Default)]
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
    // git describe --always (optionally --tags, --dirty)
    pub(crate) git_describe: bool,
    git_describe_dirty: bool,
    git_describe_tags: bool,
    // git rev-parse HEAD (optionally with --short)
    pub(crate) git_sha: bool,
    git_sha_short: bool,
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
        _rerun_if_changed: &mut Vec<String>,
    ) -> Result<()> {
        if fail_on_error {
            Err(e)
        } else {
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
        if self.git_config.fail {
            Err(anyhow!("failed to create entries"))
        } else {
            self.inner_add_git_map_entries(idempotent, map, warnings, rerun_if_changed)
        }
    }

    fn inner_add_git_map_entries(
        &self,
        _idempotent: bool,
        map: &mut RustcEnvMap,
        _warnings: &mut Vec<String>,
        _rerun_if_changed: &mut Vec<String>,
    ) -> Result<()> {
        if self.git_config.git_branch {
            add_map_entry(VergenKey::GitBranch, "stuff", map);
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{emitter::test::count_idempotent, EmitBuilder};
    use anyhow::Result;

    #[test]
    #[serial_test::parallel]
    fn git_all_idempotent() -> Result<()> {
        let config = EmitBuilder::builder().idempotent().all_git().test_emit()?;
        assert_eq!(1, config.cargo_rustc_env_map.len());
        assert_eq!(0, count_idempotent(config.cargo_rustc_env_map));
        assert_eq!(0, config.warnings.len());
        Ok(())
    }

    #[test]
    #[serial_test::parallel]
    fn git_all() -> Result<()> {
        let config = EmitBuilder::builder().all_git().test_emit()?;
        assert_eq!(1, config.cargo_rustc_env_map.len());
        assert_eq!(0, count_idempotent(config.cargo_rustc_env_map));
        assert_eq!(0, config.warnings.len());
        Ok(())
    }

    #[test]
    #[serial_test::parallel]
    fn git_error_fails() -> Result<()> {
        let mut config = EmitBuilder::builder();
        let _ = config.fail_on_error();
        let _ = config.all_git();
        config.git_config.fail = true;
        assert!(config.test_emit().is_err());
        Ok(())
    }

    #[test]
    #[serial_test::parallel]
    fn git_error_defaults() -> Result<()> {
        let mut config = EmitBuilder::builder();
        let _ = config.all_git();
        config.git_config.fail = true;
        let emitter = config.test_emit()?;
        assert_eq!(9, emitter.cargo_rustc_env_map.len());
        assert_eq!(9, count_idempotent(emitter.cargo_rustc_env_map));
        assert_eq!(9, emitter.warnings.len());
        Ok(())
    }
}
