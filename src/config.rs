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
        ConstantsFlags, BUILD_DATE_NAME, BUILD_TIMESTAMP_NAME, GIT_BRANCH_NAME,
        GIT_COMMIT_DATE_NAME, GIT_SEMVER_NAME, GIT_SEMVER_TAGS_NAME, GIT_SHA_NAME,
        GIT_SHA_SHORT_NAME, RUSTC_CHANNEL_NAME, RUSTC_COMMIT_DATE, RUSTC_COMMIT_HASH,
        RUSTC_HOST_TRIPLE_NAME, RUSTC_LLVM_VERSION, RUSTC_SEMVER_NAME,
    },
    error::Result,
    feature::{add_build_config, add_git_config, add_rustc_config},
};
use enum_iterator::IntoEnumIterator;
use getset::{Getters, MutGetters};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

/// Build information keys.
#[derive(Clone, Copy, Debug, IntoEnumIterator, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum VergenKey {
    /// The build date. (VERGEN_BUILD_DATE)
    BuildDate,
    /// The build timestamp. (VERGEN_BUILD_TIMESTAMP)
    BuildTimestamp,
    /// The current working branch name (VERGEN_BRANCH)
    Branch,
    /// The commit date. (VERGEN_COMMIT_DATE).
    CommitDate,
    /// The semver version from the last git tag. (VERGEN_SEMVER)
    Semver,
    /// The semver version from the last git tag, including lightweight.
    /// (VERGEN_SEMVER_LIGHTWEIGHT)
    SemverLightweight,
    /// The latest commit SHA. (VERGEN_SHA)
    Sha,
    /// The latest commit short SHA. (VERGEN_SHA_SHORT)
    ShortSha,
    /// The release channel of the rust compiler. (VERGEN_RUSTC_CHANNEL)
    RustcChannel,
    /// The rustc commit date. (VERGEN_RUSTC_COMMIT_DATE)
    RustcCommitDate,
    /// The rustc commit hash. (VERGEN_RUSTC_COMMIT_HASH)
    RustcCommitHash,
    /// The host triple. (VERGEN_HOST_TRIPLE)
    RustcHostTriple,
    /// The rustc LLVM version. (VERGEN_RUSTC_LLVM_VERSION)
    RustcLlvmVersion,
    /// The version information of the rust compiler. (VERGEN_RUSTC_SEMVER)
    RustcSemver,
}

impl VergenKey {
    /// Get the name for the given key.
    pub(crate) fn name(self) -> &'static str {
        match self {
            VergenKey::BuildDate => BUILD_DATE_NAME,
            VergenKey::BuildTimestamp => BUILD_TIMESTAMP_NAME,
            VergenKey::Branch => GIT_BRANCH_NAME,
            VergenKey::CommitDate => GIT_COMMIT_DATE_NAME,
            VergenKey::Semver => GIT_SEMVER_NAME,
            VergenKey::SemverLightweight => GIT_SEMVER_TAGS_NAME,
            VergenKey::Sha => GIT_SHA_NAME,
            VergenKey::ShortSha => GIT_SHA_SHORT_NAME,
            VergenKey::RustcChannel => RUSTC_CHANNEL_NAME,
            VergenKey::RustcCommitDate => RUSTC_COMMIT_DATE,
            VergenKey::RustcCommitHash => RUSTC_COMMIT_HASH,
            VergenKey::RustcHostTriple => RUSTC_HOST_TRIPLE_NAME,
            VergenKey::RustcLlvmVersion => RUSTC_LLVM_VERSION,
            VergenKey::RustcSemver => RUSTC_SEMVER_NAME,
        }
    }
}

#[derive(Clone, Debug, Getters, MutGetters)]
#[getset(get = "pub(crate)")]
#[getset(get_mut = "pub(crate)")]
pub(crate) struct Config {
    cfg_map: BTreeMap<VergenKey, Option<String>>,
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
