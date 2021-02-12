// Copyright (c) 2016, 2018, 2021 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `vergen` configuration

use crate::{
    constants::{
        ConstantsFlags, BRANCH_NAME, BUILD_DATE_NAME, BUILD_TIMESTAMP_NAME, COMMIT_DATE_NAME,
        RUSTC_CHANNEL_NAME, RUSTC_HOST_TRIPLE_NAME, RUSTC_SEMVER_NAME, SEMVER_NAME,
        SEMVER_TAGS_NAME, SHA_NAME, SHA_SHORT_NAME, TARGET_TRIPLE_NAME,
    },
    error::Result,
    feature::{add_build_config, add_git_config, add_rustc_config},
};
use enum_iterator::IntoEnumIterator;
use getset::{Getters, MutGetters};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

/// Build information keys.
#[derive(Clone, Copy, Debug, IntoEnumIterator, Hash, Eq, PartialEq)]
pub(crate) enum VergenKey {
    /// The build timestamp. (VERGEN_BUILD_TIMESTAMP)
    BuildTimestamp,
    /// The build date. (VERGEN_BUILD_DATE)
    BuildDate,
    /// The latest commit SHA. (VERGEN_SHA)
    Sha,
    /// The latest commit short SHA. (VERGEN_SHA_SHORT)
    ShortSha,
    /// The commit date. (VERGEN_COMMIT_DATE).
    CommitDate,
    /// The target triple. (VERGEN_TARGET_TRIPLE)
    TargetTriple,
    /// The semver version from the last git tag. (VERGEN_SEMVER)
    Semver,
    /// The semver version from the last git tag, including lightweight.
    /// (VERGEN_SEMVER_LIGHTWEIGHT)
    SemverLightweight,
    /// The version information of the rust compiler. (VERGEN_RUSTC_SEMVER)
    RustcSemver,
    /// The release channel of the rust compiler. (VERGEN_RUSTC_CHANNEL)
    RustcChannel,
    /// The host triple. (VERGEN_HOST_TRIPLE)
    HostTriple,
    /// The current working branch name (VERGEN_BRANCH)
    Branch,
}

impl VergenKey {
    /// Get the name for the given key.
    pub(crate) fn name(self) -> &'static str {
        match self {
            VergenKey::BuildTimestamp => BUILD_TIMESTAMP_NAME,
            VergenKey::BuildDate => BUILD_DATE_NAME,
            VergenKey::Sha => SHA_NAME,
            VergenKey::ShortSha => SHA_SHORT_NAME,
            VergenKey::CommitDate => COMMIT_DATE_NAME,
            VergenKey::TargetTriple => TARGET_TRIPLE_NAME,
            VergenKey::Semver => SEMVER_NAME,
            VergenKey::SemverLightweight => SEMVER_TAGS_NAME,
            VergenKey::RustcSemver => RUSTC_SEMVER_NAME,
            VergenKey::RustcChannel => RUSTC_CHANNEL_NAME,
            VergenKey::HostTriple => RUSTC_HOST_TRIPLE_NAME,
            VergenKey::Branch => BRANCH_NAME,
        }
    }
}

#[derive(Clone, Debug, Getters, MutGetters)]
#[getset(get = "pub(crate)")]
#[getset(get_mut = "pub(crate)")]
pub(crate) struct Config {
    cfg_map: HashMap<VergenKey, Option<String>>,
    head_path: Option<PathBuf>,
    ref_path: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Config {
        Self {
            cfg_map: VergenKey::into_enum_iter().map(|x| (x, None)).collect(),
            head_path: Option::default(),
            ref_path: Option::default(),
        }
    }
}

impl Config {
    pub(crate) fn build<T>(flags: ConstantsFlags, repo_path: Option<T>) -> Result<Config>
    where
        T: AsRef<Path>,
    {
        let mut config = Config::default();

        add_build_config(flags, &mut config);
        add_git_config(flags, repo_path, &mut config)?;
        add_rustc_config(flags, &mut config)?;

        Ok(config)
    }
}

#[cfg(test)]
mod test {
    use super::Config;

    #[test]
    fn default_works() {
        assert!(!Config::default().cfg_map().is_empty());
    }
}
