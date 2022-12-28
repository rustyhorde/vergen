// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use crate::constants::{
    BUILD_DATE_NAME, BUILD_SEMVER_NAME, BUILD_TIMESTAMP_NAME, BUILD_TIME_NAME, CARGO_FEATURES,
    CARGO_PROFILE, CARGO_TARGET_TRIPLE, GIT_BRANCH_NAME, GIT_COMMIT_AUTHOR_EMAIL,
    GIT_COMMIT_AUTHOR_NAME, GIT_COMMIT_COUNT, GIT_COMMIT_DATE_NAME, GIT_COMMIT_MESSAGE,
    GIT_COMMIT_TIMESTAMP_NAME, GIT_COMMIT_TIME_NAME, GIT_SEMVER_NAME, GIT_SEMVER_TAGS_NAME,
    GIT_SHA_NAME, GIT_SHA_SHORT_NAME, RUSTC_CHANNEL_NAME, RUSTC_COMMIT_DATE, RUSTC_COMMIT_HASH,
    RUSTC_HOST_TRIPLE_NAME, RUSTC_LLVM_VERSION, RUSTC_SEMVER_NAME, SYSINFO_CPU_BRAND,
    SYSINFO_CPU_CORE_COUNT, SYSINFO_CPU_FREQUENCY, SYSINFO_CPU_NAME, SYSINFO_CPU_VENDOR,
    SYSINFO_MEMORY, SYSINFO_NAME, SYSINFO_OS_VERSION, SYSINFO_USER,
};

/// Build information keys.
#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[allow(dead_code)]
pub(crate) enum VergenKey {
    /// The build date. (VERGEN_BUILD_DATE)
    BuildDate,
    /// The build time. (VERGEN_BUILD_TIME)
    BuildTime,
    /// The build timestamp. (VERGEN_BUILD_TIMESTAMP)
    BuildTimestamp,
    /// The build semver. (VERGEN_BUILD_SEMVER)
    BuildSemver,
    /// The cargo target triple (VERGEN_CARGO_TARGET_TRIPLE)
    CargoTargetTriple,
    /// The cargo profile (VERGEN_CARGO_PROFILE)
    CargoProfile,
    /// The cargo features (VERGEN_CARGO_FEATURES)
    CargoFeatures,
    /// The current working branch name (VERGEN_GIT_BRANCH)
    Branch,
    /// The commit author's email. (VERGEN_GIT_COMMIT_AUTHOR_EMAIL)
    CommitAuthorEmail,
    /// The commit author's name. (VERGEN_GIT_COMMIT_AUTHOR_NAME)
    CommitAuthorName,
    /// Number of commits in current branch. (VERGEN_GIT_COMMIT_COUNT)
    CommitCount,
    /// The commit date. (VERGEN_GIT_COMMIT_DATE)
    CommitDate,
    /// Commit message (VERGEN_GIT_COMMIT_MESSAGE)
    CommitMessage,
    /// The commit time. (VERGEN_GIT_COMMIT_TIME)
    CommitTime,
    /// The commit timestamp. (VERGEN_GIT_COMMIT_TIMESTAMP)
    CommitTimestamp,
    /// The semver version from the last git tag. (VERGEN_GIT_SEMVER)
    Semver,
    /// The semver version from the last git tag, including lightweight.
    /// (VERGEN_GIT_SEMVER_LIGHTWEIGHT)
    SemverLightweight,
    /// The latest commit SHA. (VERGEN_GIT_SHA)
    Sha,
    /// The latest commit short SHA. (VERGEN_GIT_SHA_SHORT)
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
    /// The sysinfo system name (VERGEN_SYSINFO_NAME)
    SysinfoName,
    /// The sysinfo os version (VERGEN_SYSINFO_OS_VERSION)
    SysinfoOsVersion,
    /// The sysinfo user name (VERGEN_SYSINFO_USER)
    SysinfoUser,
    /// The sysinfo total memory (VERGEN_SYSINFO_TOTAL_MEMORY)
    SysinfoMemory,
    /// The sysinfo cpu vendor (VERGEN_SYSINFO_CPU_VENDOR)
    SysinfoCpuVendor,
    /// The sysinfo cpu core count (VERGEN_SYSINFO_CPU_CORE_COUNT)
    SysinfoCpuCoreCount,
    /// The sysinfo cpu core count (VERGEN_SYSINFO_CPU_NAME)
    SysinfoCpuName,
    /// The sysinfo cpu core count (VERGEN_SYSINFO_CPU_BRAND)
    SysinfoCpuBrand,
    /// The sysinfo cpu core count (VERGEN_SYSINFO_CPU_FREQUENCY)
    SysinfoCpuFrequency,
}

impl VergenKey {
    /// Get the name for the given key.
    pub(crate) fn name(self) -> &'static str {
        match self {
            VergenKey::BuildDate => BUILD_DATE_NAME,
            VergenKey::BuildTime => BUILD_TIME_NAME,
            VergenKey::BuildTimestamp => BUILD_TIMESTAMP_NAME,
            VergenKey::BuildSemver => BUILD_SEMVER_NAME,
            VergenKey::CargoTargetTriple => CARGO_TARGET_TRIPLE,
            VergenKey::CargoProfile => CARGO_PROFILE,
            VergenKey::CargoFeatures => CARGO_FEATURES,
            VergenKey::Branch => GIT_BRANCH_NAME,
            VergenKey::CommitAuthorEmail => GIT_COMMIT_AUTHOR_EMAIL,
            VergenKey::CommitAuthorName => GIT_COMMIT_AUTHOR_NAME,
            VergenKey::CommitCount => GIT_COMMIT_COUNT,
            VergenKey::CommitDate => GIT_COMMIT_DATE_NAME,
            VergenKey::CommitMessage => GIT_COMMIT_MESSAGE,
            VergenKey::CommitTime => GIT_COMMIT_TIME_NAME,
            VergenKey::CommitTimestamp => GIT_COMMIT_TIMESTAMP_NAME,
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
            VergenKey::SysinfoName => SYSINFO_NAME,
            VergenKey::SysinfoOsVersion => SYSINFO_OS_VERSION,
            VergenKey::SysinfoUser => SYSINFO_USER,
            VergenKey::SysinfoMemory => SYSINFO_MEMORY,
            VergenKey::SysinfoCpuVendor => SYSINFO_CPU_VENDOR,
            VergenKey::SysinfoCpuCoreCount => SYSINFO_CPU_CORE_COUNT,
            VergenKey::SysinfoCpuName => SYSINFO_CPU_NAME,
            VergenKey::SysinfoCpuBrand => SYSINFO_CPU_BRAND,
            VergenKey::SysinfoCpuFrequency => SYSINFO_CPU_FREQUENCY,
        }
    }
}
