// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use crate::builder::{Builder, RustcEnvMap};
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
    }

    /// Emit the git branch instruction
    pub fn git_branch(&mut self) -> &mut Self {
        self.git_config.git_branch = true;
        self
    }

    pub(crate) fn add_git_map_entries(&self, _map: &mut RustcEnvMap) -> Result<()> {
        Ok(())
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
        assert_eq!(0, config.cargo_rustc_env_map.len());
        assert_eq!(0, count_idempotent(config.cargo_rustc_env_map));
        assert_eq!(0, config.warnings.len());
        Ok(())
    }

    #[test]
    #[serial_test::parallel]
    fn git_all() -> Result<()> {
        let config = Vergen::default().all_git().test_gen()?;
        assert_eq!(0, config.cargo_rustc_env_map.len());
        assert_eq!(0, count_idempotent(config.cargo_rustc_env_map));
        assert_eq!(0, config.warnings.len());
        Ok(())
    }
}
