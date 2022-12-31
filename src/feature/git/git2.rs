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
use anyhow::{Error, Result};
// remove these when impl complete
use git2_rs as _;
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
    // git rev-parse HEAD (optionally with --short)
    pub(crate) git_sha: bool,
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
    /// Enable all of the `VERGEN_GIT_*` options
    pub fn all_git(&mut self) -> &mut Self {
        self.git_branch()
    }

    /// Emit the git branch instruction
    pub fn git_branch(&mut self) -> &mut Self {
        self.git_config.git_branch = true;
        self
    }

    pub(crate) fn add_git_default(
        &self,
        e: Error,
        fail_on_error: bool,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
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

    pub(crate) fn add_git_map_entries(
        &self,
        _idempotent: bool,
        map: &mut RustcEnvMap,
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
}
