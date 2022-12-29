// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#[cfg(feature = "gitcl")]
pub(crate) mod cmd;
#[cfg(feature = "git2")]
pub(crate) mod git2;
#[cfg(feature = "gix")]
pub(crate) mod gix;

#[cfg(all(
    feature = "git",
    any(feature = "git2", feature = "gitcl", feature = "gix")
))]
use {
    crate::key::VergenKey,
    anyhow::{Error, Result},
};

#[cfg(all(feature = "git", feature = "gitcl"))]
pub(crate) use self::cmd::Config;
#[cfg(all(feature = "git", feature = "git2"))]
pub(crate) use self::git2::Config;
#[cfg(all(feature = "git", feature = "gix"))]
pub(crate) use self::gix::Config;

#[cfg(all(
    feature = "git",
    any(feature = "git2", feature = "gitcl", feature = "gix")
))]
fn add_warnings(
    config: Config,
    skip_if_error: bool,
    e: Error,
    warnings: &mut Vec<String>,
) -> Result<()> {
    if skip_if_error {
        if config.git_branch {
            warnings.push(format!(
                "Unable to add {} to output",
                VergenKey::GitBranch.name()
            ));
        }
        if config.git_commit_author_name {
            warnings.push(format!(
                "Unable to add {} to output",
                VergenKey::GitCommitAuthorName.name()
            ));
        }
        if config.git_commit_author_email {
            warnings.push(format!(
                "Unable to add {} to output",
                VergenKey::GitCommitAuthorEmail.name()
            ));
        }
        if config.git_commit_count {
            warnings.push(format!(
                "Unable to add {} to output",
                VergenKey::GitCommitCount.name()
            ));
        }
        if config.git_commit_date {
            warnings.push(format!(
                "Unable to add {} to output",
                VergenKey::GitCommitDate.name()
            ));
        }
        if config.git_commit_message {
            warnings.push(format!(
                "Unable to add {} to output",
                VergenKey::GitCommitMessage.name()
            ));
        }
        if config.git_commit_timestamp {
            warnings.push(format!(
                "Unable to add {} to output",
                VergenKey::GitCommitTimestamp.name()
            ));
        }
        if config.git_describe {
            warnings.push(format!(
                "Unable to add {} to output",
                VergenKey::GitDescribe.name()
            ));
        }
        if config.git_sha {
            warnings.push(format!(
                "Unable to add {} to output",
                VergenKey::GitSha.name()
            ));
        }
        Ok(())
    } else {
        Err(e)
    }
}
