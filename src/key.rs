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
    GIT_COMMIT_TIMESTAMP_NAME, GIT_DESCRIBE_NAME, GIT_SHA_NAME, RUSTC_CHANNEL_NAME,
    RUSTC_COMMIT_DATE, RUSTC_COMMIT_HASH, RUSTC_HOST_TRIPLE_NAME, RUSTC_LLVM_VERSION,
    RUSTC_SEMVER_NAME, SYSINFO_CPU_BRAND, SYSINFO_CPU_CORE_COUNT, SYSINFO_CPU_FREQUENCY,
    SYSINFO_CPU_NAME, SYSINFO_CPU_VENDOR, SYSINFO_MEMORY, SYSINFO_NAME, SYSINFO_OS_VERSION,
    SYSINFO_USER,
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
    GitBranch,
    /// The commit author's email. (VERGEN_GIT_COMMIT_AUTHOR_EMAIL)
    GitCommitAuthorEmail,
    /// The commit author's name. (VERGEN_GIT_COMMIT_AUTHOR_NAME)
    GitCommitAuthorName,
    /// Number of commits in current branch. (VERGEN_GIT_COMMIT_COUNT)
    GitCommitCount,
    /// The commit date. (VERGEN_GIT_COMMIT_DATE)
    GitCommitDate,
    /// Commit message (VERGEN_GIT_COMMIT_MESSAGE)
    GitCommitMessage,
    /// The commit timestamp. (VERGEN_GIT_COMMIT_TIMESTAMP)
    GitCommitTimestamp,
    /// The semver version from the last git tag. (VERGEN_GIT_SEMVER)
    GitDescribe,
    /// The latest commit SHA. (VERGEN_GIT_SHA)
    GitSha,
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
            VergenKey::GitBranch => GIT_BRANCH_NAME,
            VergenKey::GitCommitAuthorEmail => GIT_COMMIT_AUTHOR_EMAIL,
            VergenKey::GitCommitAuthorName => GIT_COMMIT_AUTHOR_NAME,
            VergenKey::GitCommitCount => GIT_COMMIT_COUNT,
            VergenKey::GitCommitDate => GIT_COMMIT_DATE_NAME,
            VergenKey::GitCommitMessage => GIT_COMMIT_MESSAGE,
            VergenKey::GitCommitTimestamp => GIT_COMMIT_TIMESTAMP_NAME,
            VergenKey::GitDescribe => GIT_DESCRIBE_NAME,
            VergenKey::GitSha => GIT_SHA_NAME,
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
