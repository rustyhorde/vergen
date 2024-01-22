// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use anyhow::{anyhow, Error, Result};
use gix::{discover, head::Kind, Commit, Head, Id, Repository};
use std::{
    env,
    path::{Path, PathBuf},
    str::FromStr,
};
use time::{
    format_description::{self, well_known::Iso8601},
    OffsetDateTime, UtcOffset,
};
use vergen_lib::{
    add_default_map_entry, add_map_entry,
    constants::{
        GIT_BRANCH_NAME, GIT_COMMIT_AUTHOR_EMAIL, GIT_COMMIT_AUTHOR_NAME, GIT_COMMIT_COUNT,
        GIT_COMMIT_DATE_NAME, GIT_COMMIT_MESSAGE, GIT_COMMIT_TIMESTAMP_NAME, GIT_DESCRIBE_NAME,
        GIT_SHA_NAME,
    },
    AddEntries, CargoRerunIfChanged, CargoRustcEnvMap, CargoWarning, DefaultConfig, VergenKey,
};

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
/// # use vergen_gix::{Emitter, GixBuilder};
/// #
/// # fn main() -> Result<()> {
/// let gix = GixBuilder::default().all_git().build();
/// Emitter::default().add_instructions(&gix)?.emit()?;
/// #   Ok(())
/// # }
/// ```
///
/// Override output with your own value
///
/// ```
/// # use anyhow::Result;
/// # use std::env;
/// # use vergen_gix::{Emitter, GixBuilder};
/// #
/// # fn main() -> Result<()> {
/// temp_env::with_var("VERGEN_GIT_BRANCH", Some("this is the branch I want output"), || {
///     let result = || -> Result<()> {
///         let gix = GixBuilder::default().all_git().build();
///         Emitter::default().add_instructions(&gix)?.emit()?;
///         Ok(())
///     }();
/// });
/// #   Ok(())
/// # }
/// ```
///
#[derive(Clone, Copy, Debug, Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct Builder {
    // git rev-parse --abbrev-ref HEAD
    branch: bool,
    // git log -1 --pretty=format:'%an'
    commit_author_name: bool,
    // git log -1 --pretty=format:'%ae'
    commit_author_email: bool,
    // git rev-list --count HEAD
    commit_count: bool,
    // git log -1 --format=%s
    commit_message: bool,
    // git log -1 --pretty=format:'%cs'
    commit_date: bool,
    // git log -1 --pretty=format:'%cI'
    commit_timestamp: bool,
    // git describe --always (optionally --tags, --dirty)
    describe: bool,
    describe_dirty: bool,
    // git rev-parse HEAD (optionally with --short)
    sha: bool,
    sha_short: bool,
    use_local: bool,
}

impl Builder {
    /// Emit all of the `VERGEN_GIT_*` instructions
    pub fn all_git(&mut self) -> &mut Self {
        self.branch()
            .commit_author_email()
            .commit_author_name()
            .commit_count()
            .commit_date()
            .commit_message()
            .commit_timestamp()
            .describe(false)
            .sha(false)
    }

    /// Emit the current git branch
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_GIT_BRANCH=<BRANCH_NAME>
    /// ```
    ///
    pub fn branch(&mut self) -> &mut Self {
        self.branch = true;
        self
    }

    /// Emit the author email of the most recent commit
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_GIT_COMMIT_AUTHOR_EMAIL=<AUTHOR_EMAIL>
    /// ```
    ///
    pub fn commit_author_email(&mut self) -> &mut Self {
        self.commit_author_email = true;
        self
    }

    /// Emit the author name of the most recent commit
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_GIT_COMMIT_AUTHOR_NAME=<AUTHOR_NAME>
    /// ```
    ///
    pub fn commit_author_name(&mut self) -> &mut Self {
        self.commit_author_name = true;
        self
    }

    /// Emit the total commit count to HEAD
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_GIT_COMMIT_COUNT=<COUNT>
    /// ```
    ///
    pub fn commit_count(&mut self) -> &mut Self {
        self.commit_count = true;
        self
    }

    /// Emit the commit date of the latest commit
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_GIT_COMMIT_DATE=<YYYY-MM-DD>
    /// ```
    ///
    pub fn commit_date(&mut self) -> &mut Self {
        self.commit_date = true;
        self
    }

    /// Emit the commit message of the latest commit
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_GIT_COMMIT_MESSAGE=<MESSAGE>
    /// ```
    ///
    pub fn commit_message(&mut self) -> &mut Self {
        self.commit_message = true;
        self
    }

    /// Emit the commit timestamp of the latest commit
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_GIT_COMMIT_TIMESTAMP=<YYYY-MM-DDThh:mm:ssZ>
    /// ```
    ///
    pub fn commit_timestamp(&mut self) -> &mut Self {
        self.commit_timestamp = true;
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
    pub fn describe(&mut self, dirty: bool) -> &mut Self {
        self.describe = true;
        self.describe_dirty = dirty;
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
        self.sha = true;
        self.sha_short = short;
        self
    }

    /// Enable local offset date/timestamp output
    pub fn use_local(&mut self) -> &mut Self {
        self.use_local = true;
        self
    }

    ///
    #[must_use]
    pub fn build(self) -> Gix {
        Gix {
            repo_path: None,
            branch: self.branch,
            commit_author_name: self.commit_author_name,
            commit_author_email: self.commit_author_email,
            commit_count: self.commit_count,
            commit_message: self.commit_message,
            commit_date: self.commit_date,
            commit_timestamp: self.commit_timestamp,
            describe: self.describe,
            describe_dirty: self.describe_dirty,
            sha: self.sha,
            sha_short: self.sha_short,
            use_local: self.use_local,
        }
    }
}

///
#[derive(Clone, Debug, Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct Gix {
    repo_path: Option<PathBuf>,
    // git rev-parse --abbrev-ref HEAD
    branch: bool,
    // git log -1 --pretty=format:'%an'
    commit_author_name: bool,
    // git log -1 --pretty=format:'%ae'
    commit_author_email: bool,
    // git rev-list --count HEAD
    commit_count: bool,
    // git log -1 --format=%s
    commit_message: bool,
    // git log -1 --pretty=format:'%cs'
    commit_date: bool,
    // git log -1 --pretty=format:'%cI'
    commit_timestamp: bool,
    // git describe --always (optionally --tags, --dirty)
    describe: bool,
    describe_dirty: bool,
    // git rev-parse HEAD (optionally with --short)
    sha: bool,
    sha_short: bool,
    use_local: bool,
}

impl Gix {
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
    }

    ///
    pub fn at_path(&mut self, path: PathBuf) -> &mut Self {
        self.repo_path = Some(path);
        self
    }

    fn add_entries(
        &self,
        idempotent: bool,
        cargo_rustc_env: &mut CargoRustcEnvMap,
        cargo_rerun_if_changed: &mut CargoRerunIfChanged,
        cargo_warning: &mut CargoWarning,
    ) -> Result<()> {
        if self.any() {
            self.inner_add_git_map_entries(
                idempotent,
                cargo_rustc_env,
                cargo_rerun_if_changed,
                cargo_warning,
            )?;
        }
        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    fn inner_add_git_map_entries(
        &self,
        idempotent: bool,
        cargo_rustc_env: &mut CargoRustcEnvMap,
        cargo_rerun_if_changed: &mut CargoRerunIfChanged,
        cargo_warning: &mut CargoWarning,
    ) -> Result<()> {
        let repo_dir = if let Some(path) = &self.repo_path {
            path.clone()
        } else {
            env::current_dir()?
        };
        let repo = discover(repo_dir)?;
        let mut head = repo.head()?;
        let git_path = repo.git_dir().to_path_buf();
        let commit = Self::get_commit(&repo, &mut head)?;

        if !idempotent && self.any() {
            Self::add_rerun_if_changed(&head, &git_path, cargo_rerun_if_changed);
        }

        if self.branch {
            if let Ok(_value) = env::var(GIT_BRANCH_NAME) {
                add_default_map_entry(VergenKey::GitBranch, cargo_rustc_env, cargo_warning);
            } else {
                let branch_name = head
                    .referent_name()
                    .map_or_else(|| "HEAD".to_string(), |name| format!("{}", name.shorten()));
                add_map_entry(VergenKey::GitBranch, branch_name, cargo_rustc_env);
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
                let email = String::from_utf8_lossy(commit.author()?.email);
                add_map_entry(
                    VergenKey::GitCommitAuthorEmail,
                    email.into_owned(),
                    cargo_rustc_env,
                );
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
                let name = String::from_utf8_lossy(commit.author()?.name);
                add_map_entry(
                    VergenKey::GitCommitAuthorName,
                    name.into_owned(),
                    cargo_rustc_env,
                );
            }
        }

        if self.commit_count {
            if let Ok(_value) = env::var(GIT_COMMIT_COUNT) {
                add_default_map_entry(VergenKey::GitCommitCount, cargo_rustc_env, cargo_warning);
            } else {
                add_map_entry(
                    VergenKey::GitCommitCount,
                    commit.ancestors().all()?.count().to_string(),
                    cargo_rustc_env,
                );
            }
        }

        self.add_git_timestamp_entries(idempotent, &commit, cargo_rustc_env, cargo_warning)?;

        if self.commit_message {
            if let Ok(_value) = env::var(GIT_COMMIT_MESSAGE) {
                add_default_map_entry(VergenKey::GitCommitMessage, cargo_rustc_env, cargo_warning);
            } else {
                let message = String::from_utf8_lossy(commit.message_raw()?);
                add_map_entry(
                    VergenKey::GitCommitMessage,
                    message.into_owned().trim(),
                    cargo_rustc_env,
                );
            }
        }

        if self.describe {
            if let Ok(_value) = env::var(GIT_DESCRIBE_NAME) {
                add_default_map_entry(VergenKey::GitDescribe, cargo_rustc_env, cargo_warning);
            } else {
                let describe = if let Some(mut fmt) = commit.describe().try_format()? {
                    if fmt.depth > 0 && self.describe_dirty {
                        fmt.dirty_suffix = Some("dirty".to_string());
                    }
                    fmt.to_string()
                } else {
                    String::new()
                };
                add_map_entry(VergenKey::GitDescribe, describe, cargo_rustc_env);
            }
        }

        if self.sha {
            if let Ok(_value) = env::var(GIT_SHA_NAME) {
                add_default_map_entry(VergenKey::GitSha, cargo_rustc_env, cargo_warning);
            } else {
                let id = if self.sha_short {
                    commit.short_id()?.to_string()
                } else {
                    commit.id().to_string()
                };
                add_map_entry(VergenKey::GitSha, id, cargo_rustc_env);
            }
        }

        Ok(())
    }

    fn get_commit<'a>(repo: &Repository, head: &mut Head<'a>) -> Result<Commit<'a>> {
        Ok(if repo.is_shallow() {
            let id = Self::get_id(head)?.ok_or_else(|| anyhow!("Not an Id"))?;
            let object = id.try_object()?.ok_or_else(|| anyhow!("Not an Object"))?;
            object.try_into_commit()?
        } else {
            head.peel_to_commit_in_place()?
        })
    }

    fn get_id<'a>(head: &mut Head<'a>) -> Result<Option<Id<'a>>> {
        head.try_peel_to_id_in_place().map_err(Into::into)
    }

    fn add_rerun_if_changed(
        head: &Head<'_>,
        git_path: &Path,
        cargo_rerun_if_changed: &mut Vec<String>,
    ) {
        // Setup the head path
        let mut head_path = git_path.to_path_buf();
        head_path.push("HEAD");

        // Check whether the path exists in the filesystem before emitting it
        if head_path.exists() {
            cargo_rerun_if_changed.push(format!("{}", head_path.display()));
        }

        if let Kind::Symbolic(reference) = &head.kind {
            let mut ref_path = git_path.to_path_buf();
            ref_path.push(reference.name.to_path());
            // Check whether the path exists in the filesystem before emitting it
            if ref_path.exists() {
                cargo_rerun_if_changed.push(format!("{}", ref_path.display()));
            }
        }
    }

    fn add_git_timestamp_entries(
        &self,
        idempotent: bool,
        commit: &Commit<'_>,
        cargo_rustc_env: &mut CargoRustcEnvMap,
        cargo_warning: &mut CargoWarning,
    ) -> Result<()> {
        let (sde, ts) = match env::var("SOURCE_DATE_EPOCH") {
            Ok(v) => (
                true,
                OffsetDateTime::from_unix_timestamp(i64::from_str(&v)?)?,
            ),
            Err(std::env::VarError::NotPresent) => self.compute_local_offset(commit)?,
            Err(e) => return Err(e.into()),
        };

        if let Ok(_value) = env::var(GIT_COMMIT_DATE_NAME) {
            add_default_map_entry(VergenKey::GitCommitDate, cargo_rustc_env, cargo_warning);
        } else {
            self.add_git_date_entry(idempotent, sde, &ts, cargo_rustc_env, cargo_warning)?;
        }
        if let Ok(_value) = env::var(GIT_COMMIT_TIMESTAMP_NAME) {
            add_default_map_entry(
                VergenKey::GitCommitTimestamp,
                cargo_rustc_env,
                cargo_warning,
            );
        } else {
            self.add_git_timestamp_entry(idempotent, sde, &ts, cargo_rustc_env, cargo_warning)?;
        }
        Ok(())
    }

    #[cfg(not(tarpaulin_include))]
    // this in not included in coverage, because on *nix the local offset is always unsafe
    fn compute_local_offset(&self, commit: &Commit<'_>) -> Result<(bool, OffsetDateTime)> {
        let no_offset = OffsetDateTime::from_unix_timestamp(commit.time()?.seconds)?;
        if self.use_local {
            let local = UtcOffset::local_offset_at(no_offset)?;
            let local_offset = no_offset.checked_to_offset(local).unwrap_or(no_offset);
            Ok((false, local_offset))
        } else {
            Ok((false, no_offset))
        }
    }

    fn add_git_date_entry(
        &self,
        idempotent: bool,
        source_date_epoch: bool,
        ts: &OffsetDateTime,
        cargo_rustc_env: &mut CargoRustcEnvMap,
        cargo_warning: &mut CargoWarning,
    ) -> Result<()> {
        if self.commit_date {
            if idempotent && !source_date_epoch {
                add_default_map_entry(VergenKey::GitCommitDate, cargo_rustc_env, cargo_warning);
            } else {
                let format = format_description::parse("[year]-[month]-[day]")?;
                add_map_entry(
                    VergenKey::GitCommitDate,
                    ts.format(&format)?,
                    cargo_rustc_env,
                );
            }
        }
        Ok(())
    }

    fn add_git_timestamp_entry(
        &self,
        idempotent: bool,
        source_date_epoch: bool,
        ts: &OffsetDateTime,
        cargo_rustc_env: &mut CargoRustcEnvMap,
        cargo_warning: &mut CargoWarning,
    ) -> Result<()> {
        if self.commit_timestamp {
            if idempotent && !source_date_epoch {
                add_default_map_entry(
                    VergenKey::GitCommitTimestamp,
                    cargo_rustc_env,
                    cargo_warning,
                );
            } else {
                add_map_entry(
                    VergenKey::GitCommitTimestamp,
                    ts.format(&Iso8601::DEFAULT)?,
                    cargo_rustc_env,
                );
            }
        }
        Ok(())
    }
}

impl AddEntries for Gix {
    fn add_map_entries(
        &self,
        idempotent: bool,
        cargo_rustc_env: &mut CargoRustcEnvMap,
        cargo_rerun_if_changed: &mut CargoRerunIfChanged,
        cargo_warning: &mut CargoWarning,
    ) -> Result<()> {
        self.add_entries(
            idempotent,
            cargo_rustc_env,
            cargo_rerun_if_changed,
            cargo_warning,
        )
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
            // Clear any previous cargo_warning.  This should be it.
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
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use std::env::temp_dir;

    use super::Builder;
    use anyhow::Result;
    use serial_test::serial;
    use test_util::TestRepos;
    use vergen::Emitter;
    use vergen_lib::count_idempotent;

    #[test]
    #[serial]
    fn git_all_idempotent() -> Result<()> {
        let gix = Builder::default().all_git().build();
        let emitter = Emitter::default()
            .idempotent()
            .add_instructions(&gix)?
            .test_emit();
        assert_eq!(9, emitter.cargo_rustc_env_map().len());
        assert_eq!(2, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(2, emitter.warnings().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn git_all_idempotent_no_warn() -> Result<()> {
        let gix = Builder::default().all_git().build();
        let emitter = Emitter::default()
            .idempotent()
            .quiet()
            .add_instructions(&gix)?
            .test_emit();
        assert_eq!(9, emitter.cargo_rustc_env_map().len());
        assert_eq!(2, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(2, emitter.warnings().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn git_all() -> Result<()> {
        let gix = Builder::default().all_git().build();
        let emitter = Emitter::default().add_instructions(&gix)?.test_emit();
        assert_eq!(9, emitter.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(0, emitter.warnings().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn git_branch() -> Result<()> {
        let gix = Builder::default().branch().build();
        let emitter = Emitter::default().add_instructions(&gix)?.test_emit();
        assert_eq!(1, emitter.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(0, emitter.warnings().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn git_commit_author_name() -> Result<()> {
        let gix = Builder::default().commit_author_name().build();
        let emitter = Emitter::default().add_instructions(&gix)?.test_emit();
        assert_eq!(1, emitter.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(0, emitter.warnings().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn git_commit_author_email() -> Result<()> {
        let gix = Builder::default().commit_author_email().build();
        let emitter = Emitter::default().add_instructions(&gix)?.test_emit();
        assert_eq!(1, emitter.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(0, emitter.warnings().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn git_commit_count() -> Result<()> {
        let gix = Builder::default().commit_count().build();
        let emitter = Emitter::default().add_instructions(&gix)?.test_emit();
        assert_eq!(1, emitter.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(0, emitter.warnings().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn git_commit_message() -> Result<()> {
        let gix = Builder::default().commit_message().build();
        let emitter = Emitter::default().add_instructions(&gix)?.test_emit();
        assert_eq!(1, emitter.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(0, emitter.warnings().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn git_commit_date() -> Result<()> {
        let gix = Builder::default().commit_date().build();
        let emitter = Emitter::default().add_instructions(&gix)?.test_emit();
        assert_eq!(1, emitter.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(0, emitter.warnings().len());
        Ok(())
    }

    #[cfg(any(unix, target_os = "macos"))]
    #[test]
    #[serial]
    fn git_commit_date_local() {
        let result = || -> Result<()> {
            let gix = Builder::default().commit_date().use_local().build();
            let _emitter = Emitter::default()
                .fail_on_error()
                .add_instructions(&gix)?
                .test_emit();
            Ok(())
        }();
        assert!(result.is_err());
    }

    #[test]
    #[serial]
    fn git_commit_timestamp() -> Result<()> {
        let gix = Builder::default().commit_timestamp().build();
        let emitter = Emitter::default().add_instructions(&gix)?.test_emit();
        assert_eq!(1, emitter.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(0, emitter.warnings().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn git_describe() -> Result<()> {
        let gix = Builder::default().describe(true).build();
        let emitter = Emitter::default().add_instructions(&gix)?.test_emit();
        assert_eq!(1, emitter.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(0, emitter.warnings().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn git_sha() -> Result<()> {
        let gix = Builder::default().sha(false).build();
        let emitter = Emitter::default().add_instructions(&gix)?.test_emit();
        assert_eq!(1, emitter.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(0, emitter.warnings().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn git_all_at_path() -> Result<()> {
        let repo = TestRepos::new(false, false, false)?;
        let mut gix = Builder::default().all_git().build();
        let _ = gix.at_path(repo.path());
        let emitter = Emitter::default().add_instructions(&gix)?.test_emit();
        assert_eq!(9, emitter.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(0, emitter.warnings().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn git_all_shallow_clone() -> Result<()> {
        let repo = TestRepos::new(false, false, true)?;
        let mut gix = Builder::default().all_git().build();
        let _ = gix.at_path(repo.path());
        let emitter = Emitter::default().add_instructions(&gix)?.test_emit();
        assert_eq!(9, emitter.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(0, emitter.warnings().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn git_error_fails() -> Result<()> {
        let mut gix = Builder::default().all_git().build();
        let _ = gix.at_path(temp_dir());
        let result = || -> Result<()> {
            let _emitter = Emitter::default()
                .fail_on_error()
                .add_instructions(&gix)?
                .test_emit();
            Ok(())
        }();
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    #[serial]
    fn git_error_defaults() -> Result<()> {
        let mut gix = Builder::default().all_git().build();
        let _ = gix.at_path(temp_dir());
        let emitter = Emitter::default().add_instructions(&gix)?.test_emit();
        assert_eq!(9, emitter.cargo_rustc_env_map().len());
        assert_eq!(9, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(10, emitter.warnings().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn source_date_epoch_works() {
        temp_env::with_var("SOURCE_DATE_EPOCH", Some("1671809360"), || {
            let result = || -> Result<()> {
                let mut stdout_buf = vec![];
                let gix = Builder::default().commit_date().commit_timestamp().build();
                _ = Emitter::new()
                    .idempotent()
                    .add_instructions(&gix)?
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
                let gix = Builder::default().commit_date().build();
                Emitter::new()
                    .idempotent()
                    .fail_on_error()
                    .add_instructions(&gix)?
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
                let gix = Builder::default().commit_date().build();
                Emitter::new()
                    .idempotent()
                    .add_instructions(&gix)?
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
                let gix = Builder::default().commit_date().build();
                Emitter::new()
                    .idempotent()
                    .add_instructions(&gix)?
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
                let gix = Builder::default().commit_date().build();
                Emitter::new()
                    .idempotent()
                    .add_instructions(&gix)?
                    .emit_to(&mut stdout_buf)
            }();
            assert!(result.is_ok());
        });
    }
}
