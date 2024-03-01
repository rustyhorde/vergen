// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

/// The [`VergenKey`] enum to use based on the configured features.
#[cfg(any(
    feature = "build",
    feature = "cargo",
    feature = "git",
    feature = "rustc",
    feature = "si"
))]
pub(crate) mod vergen_key {
    #[cfg(feature = "build")]
    use crate::constants::{BUILD_DATE_NAME, BUILD_TIMESTAMP_NAME};
    #[cfg(feature = "cargo")]
    use crate::constants::{
        CARGO_DEBUG, CARGO_DEPENDENCIES, CARGO_FEATURES, CARGO_OPT_LEVEL, CARGO_TARGET_TRIPLE,
    };
    #[cfg(feature = "git")]
    use crate::constants::{
        GIT_BRANCH_NAME, GIT_COMMIT_AUTHOR_EMAIL, GIT_COMMIT_AUTHOR_NAME, GIT_COMMIT_COUNT,
        GIT_COMMIT_DATE_NAME, GIT_COMMIT_MESSAGE, GIT_COMMIT_TIMESTAMP_NAME, GIT_DESCRIBE_NAME,
        GIT_DIRTY_NAME, GIT_SHA_NAME,
    };
    #[cfg(feature = "rustc")]
    use crate::constants::{
        RUSTC_CHANNEL_NAME, RUSTC_COMMIT_DATE, RUSTC_COMMIT_HASH, RUSTC_HOST_TRIPLE_NAME,
        RUSTC_LLVM_VERSION, RUSTC_SEMVER_NAME,
    };
    #[cfg(feature = "si")]
    use crate::constants::{
        SYSINFO_CPU_BRAND, SYSINFO_CPU_CORE_COUNT, SYSINFO_CPU_FREQUENCY, SYSINFO_CPU_NAME,
        SYSINFO_CPU_VENDOR, SYSINFO_MEMORY, SYSINFO_NAME, SYSINFO_OS_VERSION, SYSINFO_USER,
    };

    /// The keys used in the [`crate::CargoRustcEnvMap`]
    #[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
    pub enum VergenKey {
        /// The build date. (`VERGEN_BUILD_DATE`)
        #[cfg(feature = "build")]
        BuildDate,
        /// The build timestamp. (`VERGEN_BUILD_TIMESTAMP`)
        #[cfg(feature = "build")]
        BuildTimestamp,
        /// The cargo debug flag (`VERGEN_CARGO_DEBUG`)
        #[cfg(feature = "cargo")]
        CargoDebug,
        /// The cargo features (`VERGEN_CARGO_FEATURES`)
        #[cfg(feature = "cargo")]
        CargoFeatures,
        /// The cargo opt level (`VERGEN_CARGO_OPT_LEVEL`)
        #[cfg(feature = "cargo")]
        CargoOptLevel,
        /// The cargo target triple (`VERGEN_CARGO_TARGET_TRIPLE`)
        #[cfg(feature = "cargo")]
        CargoTargetTriple,
        /// The cargo dependencies (`VERGEN_CARGO_DEPENDENCIES`)
        #[cfg(feature = "cargo")]
        CargoDependencies,
        /// The current working branch name (`VERGEN_GIT_BRANCH`)
        #[cfg(feature = "git")]
        GitBranch,
        /// The commit author's email. (`VERGEN_GIT_COMMIT_AUTHOR_EMAIL`)
        #[cfg(feature = "git")]
        GitCommitAuthorEmail,
        /// The commit author's name. (`VERGEN_GIT_COMMIT_AUTHOR_NAME`)
        #[cfg(feature = "git")]
        GitCommitAuthorName,
        /// Number of commits in current branch. (`VERGEN_GIT_COMMIT_COUNT`)
        #[cfg(feature = "git")]
        GitCommitCount,
        /// The commit date. (`VERGEN_GIT_COMMIT_DATE`)
        #[cfg(feature = "git")]
        GitCommitDate,
        /// Commit message (`VERGEN_GIT_COMMIT_MESSAGE`)
        #[cfg(feature = "git")]
        GitCommitMessage,
        /// The commit timestamp. (`VERGEN_GIT_COMMIT_TIMESTAMP`)
        #[cfg(feature = "git")]
        GitCommitTimestamp,
        /// The semver version from the last git tag. (`VERGEN_GIT_SEMVER`)
        #[cfg(feature = "git")]
        GitDescribe,
        /// The latest commit SHA. (`VERGEN_GIT_SHA`)
        #[cfg(feature = "git")]
        GitSha,
        /// Whether the repository is dirty. (`VERGEN_GIT_DIRTY`)
        #[cfg(feature = "git")]
        GitDirty,
        /// The release channel of the rust compiler. (`VERGEN_RUSTC_CHANNEL`)
        #[cfg(feature = "rustc")]
        RustcChannel,
        /// The rustc commit date. (`VERGEN_RUSTC_COMMIT_DATE`)
        #[cfg(feature = "rustc")]
        RustcCommitDate,
        /// The rustc commit hash. (`VERGEN_RUSTC_COMMIT_HASH`)
        #[cfg(feature = "rustc")]
        RustcCommitHash,
        /// The host triple. (`VERGEN_HOST_TRIPLE`)
        #[cfg(feature = "rustc")]
        RustcHostTriple,
        /// The rustc LLVM version. (`VERGEN_RUSTC_LLVM_VERSION`)
        #[cfg(feature = "rustc")]
        RustcLlvmVersion,
        /// The version information of the rust compiler. (`VERGEN_RUSTC_SEMVER`)
        #[cfg(feature = "rustc")]
        RustcSemver,
        /// The sysinfo system name (`VERGEN_SYSINFO_NAME`)
        #[cfg(feature = "si")]
        SysinfoName,
        /// The sysinfo os version (`VERGEN_SYSINFO_OS_VERSION`)
        #[cfg(feature = "si")]
        SysinfoOsVersion,
        /// The sysinfo user name (`VERGEN_SYSINFO_USER`)
        #[cfg(feature = "si")]
        SysinfoUser,
        /// The sysinfo total memory (`VERGEN_SYSINFO_TOTAL_MEMORY`)
        #[cfg(feature = "si")]
        SysinfoMemory,
        /// The sysinfo cpu vendor (`VERGEN_SYSINFO_CPU_VENDOR`)
        #[cfg(feature = "si")]
        SysinfoCpuVendor,
        /// The sysinfo cpu core count (`VERGEN_SYSINFO_CPU_CORE_COUNT`)
        #[cfg(feature = "si")]
        SysinfoCpuCoreCount,
        /// The sysinfo cpu core count (`VERGEN_SYSINFO_CPU_NAME`)
        #[cfg(feature = "si")]
        SysinfoCpuName,
        /// The sysinfo cpu core count (`VERGEN_SYSINFO_CPU_BRAND`)
        #[cfg(feature = "si")]
        SysinfoCpuBrand,
        /// The sysinfo cpu core count (`VERGEN_SYSINFO_CPU_FREQUENCY`)
        #[cfg(feature = "si")]
        SysinfoCpuFrequency,
    }

    impl VergenKey {
        /// Get the name for the given key.
        #[must_use]
        pub fn name(self) -> &'static str {
            match self {
                #[cfg(feature = "build")]
                VergenKey::BuildDate => BUILD_DATE_NAME,
                #[cfg(feature = "build")]
                VergenKey::BuildTimestamp => BUILD_TIMESTAMP_NAME,
                #[cfg(feature = "cargo")]
                VergenKey::CargoDebug => CARGO_DEBUG,
                #[cfg(feature = "cargo")]
                VergenKey::CargoFeatures => CARGO_FEATURES,
                #[cfg(feature = "cargo")]
                VergenKey::CargoOptLevel => CARGO_OPT_LEVEL,
                #[cfg(feature = "cargo")]
                VergenKey::CargoTargetTriple => CARGO_TARGET_TRIPLE,
                #[cfg(feature = "cargo")]
                VergenKey::CargoDependencies => CARGO_DEPENDENCIES,
                #[cfg(feature = "git")]
                VergenKey::GitBranch => GIT_BRANCH_NAME,
                #[cfg(feature = "git")]
                VergenKey::GitCommitAuthorEmail => GIT_COMMIT_AUTHOR_EMAIL,
                #[cfg(feature = "git")]
                VergenKey::GitCommitAuthorName => GIT_COMMIT_AUTHOR_NAME,
                #[cfg(feature = "git")]
                VergenKey::GitCommitCount => GIT_COMMIT_COUNT,
                #[cfg(feature = "git")]
                VergenKey::GitCommitDate => GIT_COMMIT_DATE_NAME,
                #[cfg(feature = "git")]
                VergenKey::GitCommitMessage => GIT_COMMIT_MESSAGE,
                #[cfg(feature = "git")]
                VergenKey::GitCommitTimestamp => GIT_COMMIT_TIMESTAMP_NAME,
                #[cfg(feature = "git")]
                VergenKey::GitDescribe => GIT_DESCRIBE_NAME,
                #[cfg(feature = "git")]
                VergenKey::GitSha => GIT_SHA_NAME,
                #[cfg(feature = "git")]
                VergenKey::GitDirty => GIT_DIRTY_NAME,
                #[cfg(feature = "rustc")]
                VergenKey::RustcChannel => RUSTC_CHANNEL_NAME,
                #[cfg(feature = "rustc")]
                VergenKey::RustcCommitDate => RUSTC_COMMIT_DATE,
                #[cfg(feature = "rustc")]
                VergenKey::RustcCommitHash => RUSTC_COMMIT_HASH,
                #[cfg(feature = "rustc")]
                VergenKey::RustcHostTriple => RUSTC_HOST_TRIPLE_NAME,
                #[cfg(feature = "rustc")]
                VergenKey::RustcLlvmVersion => RUSTC_LLVM_VERSION,
                #[cfg(feature = "rustc")]
                VergenKey::RustcSemver => RUSTC_SEMVER_NAME,
                #[cfg(feature = "si")]
                VergenKey::SysinfoName => SYSINFO_NAME,
                #[cfg(feature = "si")]
                VergenKey::SysinfoOsVersion => SYSINFO_OS_VERSION,
                #[cfg(feature = "si")]
                VergenKey::SysinfoUser => SYSINFO_USER,
                #[cfg(feature = "si")]
                VergenKey::SysinfoMemory => SYSINFO_MEMORY,
                #[cfg(feature = "si")]
                VergenKey::SysinfoCpuVendor => SYSINFO_CPU_VENDOR,
                #[cfg(feature = "si")]
                VergenKey::SysinfoCpuCoreCount => SYSINFO_CPU_CORE_COUNT,
                #[cfg(feature = "si")]
                VergenKey::SysinfoCpuName => SYSINFO_CPU_NAME,
                #[cfg(feature = "si")]
                VergenKey::SysinfoCpuBrand => SYSINFO_CPU_BRAND,
                #[cfg(feature = "si")]
                VergenKey::SysinfoCpuFrequency => SYSINFO_CPU_FREQUENCY,
            }
        }
    }
}

/// The [`VergenKey`] enum to use when no features are configured.
#[cfg(not(any(
    feature = "build",
    feature = "cargo",
    feature = "git",
    feature = "rustc",
    feature = "si"
)))]
pub(crate) mod vergen_key {
    /// The [`VergenKey`] enum to use when no features are configured.
    #[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
    pub enum VergenKey {
        /// An empty vergen key
        Empty,
    }

    impl VergenKey {
        /// Get the name for the given key.
        #[must_use]
        pub fn name(self) -> &'static str {
            match self {
                VergenKey::Empty => "",
            }
        }
    }
}

#[cfg(all(
    test,
    not(any(
        feature = "build",
        feature = "cargo",
        feature = "git",
        feature = "rustc",
        feature = "si"
    ))
))]
mod test {
    use super::vergen_key::VergenKey;
    use anyhow::Result;
    use std::{
        cmp::Ordering,
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
        io::Write,
    };

    #[test]
    fn empty_name() {
        assert!(VergenKey::Empty.name().is_empty());
    }

    #[test]
    #[allow(clippy::clone_on_copy, clippy::redundant_clone)]
    fn vergen_key_clone_works() {
        let key = VergenKey::Empty;
        let another = key.clone();
        assert_eq!(another, key);
    }

    #[test]
    fn vergen_key_debug_works() -> Result<()> {
        let key = VergenKey::Empty;
        let mut buf = vec![];
        write!(buf, "{key:?}")?;
        assert!(!buf.is_empty());
        Ok(())
    }

    #[test]
    fn vergen_key_ord_works() {
        assert_eq!(VergenKey::Empty.cmp(&VergenKey::Empty), Ordering::Equal);
    }

    #[test]
    fn vergen_key_partial_ord_works() {
        assert_eq!(
            VergenKey::Empty.partial_cmp(&VergenKey::Empty),
            Some(Ordering::Equal)
        );
    }

    #[test]
    fn vergen_key_hash_works() {
        let mut hasher = DefaultHasher::new();
        VergenKey::Empty.hash(&mut hasher);
        assert_eq!(15_130_871_412_783_076_140, hasher.finish());
    }
}

#[cfg(all(test, all(feature = "build", feature = "cargo")))]
mod test {
    use super::vergen_key::VergenKey;
    use anyhow::Result;
    use std::{
        cmp::Ordering,
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
        io::Write,
    };

    #[test]
    #[allow(clippy::clone_on_copy, clippy::redundant_clone)]
    fn vergen_key_clone_works() {
        let key = VergenKey::BuildDate;
        let another = key.clone();
        assert_eq!(another, key);
    }

    #[test]
    fn vergen_key_debug_works() -> Result<()> {
        let key = VergenKey::BuildDate;
        let mut buf = vec![];
        write!(buf, "{key:?}")?;
        assert!(!buf.is_empty());
        Ok(())
    }

    #[test]
    fn vergen_key_ord_works() {
        assert_eq!(
            VergenKey::CargoDebug.cmp(&VergenKey::BuildDate),
            Ordering::Greater
        );
    }

    #[test]
    fn vergen_key_partial_ord_works() {
        assert_eq!(
            VergenKey::CargoDebug.partial_cmp(&VergenKey::BuildDate),
            Some(Ordering::Greater)
        );
    }

    #[test]
    fn vergen_key_hash_works() {
        let mut hasher = DefaultHasher::new();
        VergenKey::BuildDate.hash(&mut hasher);
        assert_eq!(13_646_096_770_106_105_413, hasher.finish());
    }
}
