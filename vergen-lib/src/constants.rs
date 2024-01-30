// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Internal Constants

/// The default idempotent output string
pub const VERGEN_IDEMPOTENT_DEFAULT: &str = "VERGEN_IDEMPOTENT_OUTPUT";

#[cfg(any(
    feature = "build",
    feature = "cargo",
    feature = "git",
    feature = "rustc",
    feature = "si"
))]
pub use self::features::*;

/// The names used by [`crate::VergenKey`] for each enabled output
#[cfg(any(
    feature = "build",
    feature = "cargo",
    feature = "git",
    feature = "rustc",
    feature = "si"
))]
pub mod features {
    /// The timestamp of the current build
    #[cfg(feature = "build")]
    pub const BUILD_TIMESTAMP_NAME: &str = "VERGEN_BUILD_TIMESTAMP";
    /// The date of the current build
    #[cfg(feature = "build")]
    pub const BUILD_DATE_NAME: &str = "VERGEN_BUILD_DATE";

    /// The current branch name
    #[cfg(feature = "git")]
    pub const GIT_BRANCH_NAME: &str = "VERGEN_GIT_BRANCH";
    /// The most recent commit author email address
    #[cfg(feature = "git")]
    pub const GIT_COMMIT_AUTHOR_EMAIL: &str = "VERGEN_GIT_COMMIT_AUTHOR_EMAIL";
    /// The most recent commit author name
    #[cfg(feature = "git")]
    pub const GIT_COMMIT_AUTHOR_NAME: &str = "VERGEN_GIT_COMMIT_AUTHOR_NAME";
    /// The commit count
    #[cfg(feature = "git")]
    pub const GIT_COMMIT_COUNT: &str = "VERGEN_GIT_COMMIT_COUNT";
    /// The most recent commit message
    #[cfg(feature = "git")]
    pub const GIT_COMMIT_MESSAGE: &str = "VERGEN_GIT_COMMIT_MESSAGE";
    /// The most recent commit date
    #[cfg(feature = "git")]
    pub const GIT_COMMIT_DATE_NAME: &str = "VERGEN_GIT_COMMIT_DATE";
    /// The most recent commit timestamp
    #[cfg(feature = "git")]
    pub const GIT_COMMIT_TIMESTAMP_NAME: &str = "VERGEN_GIT_COMMIT_TIMESTAMP";
    /// The output of git describe
    #[cfg(feature = "git")]
    pub const GIT_DESCRIBE_NAME: &str = "VERGEN_GIT_DESCRIBE";
    /// The most recent commit SHA
    #[cfg(feature = "git")]
    pub const GIT_SHA_NAME: &str = "VERGEN_GIT_SHA";
    /// The current dirty status
    #[cfg(feature = "git")]
    pub const GIT_DIRTY_NAME: &str = "VERGEN_GIT_DIRTY";

    /// The channel of rustc used for the build (stable, beta, nightly)
    #[cfg(feature = "rustc")]
    pub const RUSTC_CHANNEL_NAME: &str = "VERGEN_RUSTC_CHANNEL";
    /// The host triple of rustc used for the build
    #[cfg(feature = "rustc")]
    pub const RUSTC_HOST_TRIPLE_NAME: &str = "VERGEN_RUSTC_HOST_TRIPLE";
    /// The version of rustc used for the build
    #[cfg(feature = "rustc")]
    pub const RUSTC_SEMVER_NAME: &str = "VERGEN_RUSTC_SEMVER";
    /// The commit hash of rustc used for the build
    #[cfg(feature = "rustc")]
    pub const RUSTC_COMMIT_HASH: &str = "VERGEN_RUSTC_COMMIT_HASH";
    /// The commit date of rustc used for the build
    #[cfg(feature = "rustc")]
    pub const RUSTC_COMMIT_DATE: &str = "VERGEN_RUSTC_COMMIT_DATE";
    /// The LLVM version underlying rustc used for the build (if applicable)
    #[cfg(feature = "rustc")]
    pub const RUSTC_LLVM_VERSION: &str = "VERGEN_RUSTC_LLVM_VERSION";

    /// The value of the `DEBUG` environment variable at build time
    #[cfg(feature = "cargo")]
    pub const CARGO_DEBUG: &str = "VERGEN_CARGO_DEBUG";
    /// The current dependency list (potentiall filtered)
    #[cfg(feature = "cargo")]
    pub const CARGO_DEPENDENCIES: &str = "VERGEN_CARGO_DEPENDENCIES";
    /// The value of the `CARGO_FEATURES` environment variable at build time
    #[cfg(feature = "cargo")]
    pub const CARGO_FEATURES: &str = "VERGEN_CARGO_FEATURES";
    /// The value of the `OPT_LEVEL` environment variable at build time
    #[cfg(feature = "cargo")]
    pub const CARGO_OPT_LEVEL: &str = "VERGEN_CARGO_OPT_LEVEL";
    /// The value of the `TARGET_TRIPLE` environment variable at build time
    #[cfg(feature = "cargo")]
    pub const CARGO_TARGET_TRIPLE: &str = "VERGEN_CARGO_TARGET_TRIPLE";

    /// The system name
    #[cfg(feature = "si")]
    pub const SYSINFO_NAME: &str = "VERGEN_SYSINFO_NAME";
    /// The OS version
    #[cfg(feature = "si")]
    pub const SYSINFO_OS_VERSION: &str = "VERGEN_SYSINFO_OS_VERSION";
    /// The user that ran the build
    #[cfg(feature = "si")]
    pub const SYSINFO_USER: &str = "VERGEN_SYSINFO_USER";
    /// The total memory on the system used to run the build
    #[cfg(feature = "si")]
    pub const SYSINFO_MEMORY: &str = "VERGEN_SYSINFO_TOTAL_MEMORY";
    /// The CPU vender on the system used to run the build
    #[cfg(feature = "si")]
    pub const SYSINFO_CPU_VENDOR: &str = "VERGEN_SYSINFO_CPU_VENDOR";
    /// The CPU core count on the system use to run the build
    #[cfg(feature = "si")]
    pub const SYSINFO_CPU_CORE_COUNT: &str = "VERGEN_SYSINFO_CPU_CORE_COUNT";
    /// The CPU name on the system use to run the build
    #[cfg(feature = "si")]
    pub const SYSINFO_CPU_NAME: &str = "VERGEN_SYSINFO_CPU_NAME";
    /// The CPU brand on the system use to run the build
    #[cfg(feature = "si")]
    pub const SYSINFO_CPU_BRAND: &str = "VERGEN_SYSINFO_CPU_BRAND";
    /// The CPU frequency on the system use to run the build
    #[cfg(feature = "si")]
    pub const SYSINFO_CPU_FREQUENCY: &str = "VERGEN_SYSINFO_CPU_FREQUENCY";
}

/// An empty list of names to use with [`crate::VergenKey`] when
/// no features are enabled.
#[cfg(not(any(
    feature = "build",
    feature = "cargo",
    feature = "git",
    feature = "rustc",
    feature = "si"
)))]
pub mod features {}

#[cfg(all(
    test,
    any(
        feature = "build",
        feature = "cargo",
        feature = "git",
        feature = "rustc",
        feature = "si"
    )
))]
mod test {
    use super::*;

    #[cfg(feature = "build")]
    #[test]
    fn build_constants_dont_change() {
        // Build Constants
        assert_eq!(BUILD_TIMESTAMP_NAME, "VERGEN_BUILD_TIMESTAMP");
        assert_eq!(BUILD_DATE_NAME, "VERGEN_BUILD_DATE");
    }

    #[cfg(feature = "cargo")]
    #[test]
    fn cargo_constants_dont_change() {
        // cargo Constants
        assert_eq!(CARGO_TARGET_TRIPLE, "VERGEN_CARGO_TARGET_TRIPLE");
        assert_eq!(CARGO_FEATURES, "VERGEN_CARGO_FEATURES");
    }

    #[cfg(feature = "git")]
    #[test]
    fn git_constants_dont_change() {
        // git Constants
        assert_eq!(GIT_BRANCH_NAME, "VERGEN_GIT_BRANCH");
        assert_eq!(GIT_COMMIT_AUTHOR_EMAIL, "VERGEN_GIT_COMMIT_AUTHOR_EMAIL");
        assert_eq!(GIT_COMMIT_AUTHOR_NAME, "VERGEN_GIT_COMMIT_AUTHOR_NAME");
        assert_eq!(GIT_COMMIT_COUNT, "VERGEN_GIT_COMMIT_COUNT");
        assert_eq!(GIT_COMMIT_MESSAGE, "VERGEN_GIT_COMMIT_MESSAGE");
        assert_eq!(GIT_COMMIT_DATE_NAME, "VERGEN_GIT_COMMIT_DATE");
        assert_eq!(GIT_COMMIT_TIMESTAMP_NAME, "VERGEN_GIT_COMMIT_TIMESTAMP");
        assert_eq!(GIT_DESCRIBE_NAME, "VERGEN_GIT_DESCRIBE");
        assert_eq!(GIT_SHA_NAME, "VERGEN_GIT_SHA");
        assert_eq!(GIT_DIRTY_NAME, "VERGEN_GIT_DIRTY");
    }

    #[cfg(feature = "rustc")]
    #[test]
    fn rustc_constants_dont_change() {
        // rustc Constants
        assert_eq!(RUSTC_SEMVER_NAME, "VERGEN_RUSTC_SEMVER");
        assert_eq!(RUSTC_CHANNEL_NAME, "VERGEN_RUSTC_CHANNEL");
        assert_eq!(RUSTC_HOST_TRIPLE_NAME, "VERGEN_RUSTC_HOST_TRIPLE");
        assert_eq!(RUSTC_COMMIT_HASH, "VERGEN_RUSTC_COMMIT_HASH");
        assert_eq!(RUSTC_COMMIT_DATE, "VERGEN_RUSTC_COMMIT_DATE");
        assert_eq!(RUSTC_LLVM_VERSION, "VERGEN_RUSTC_LLVM_VERSION");
    }

    #[cfg(feature = "si")]
    #[test]
    fn sysinfo_constants_dont_change() {
        // sysinfo Constants
        assert_eq!(SYSINFO_NAME, "VERGEN_SYSINFO_NAME");
        assert_eq!(SYSINFO_OS_VERSION, "VERGEN_SYSINFO_OS_VERSION");
        assert_eq!(SYSINFO_USER, "VERGEN_SYSINFO_USER");
        assert_eq!(SYSINFO_MEMORY, "VERGEN_SYSINFO_TOTAL_MEMORY");
        assert_eq!(SYSINFO_CPU_VENDOR, "VERGEN_SYSINFO_CPU_VENDOR");
        assert_eq!(SYSINFO_CPU_CORE_COUNT, "VERGEN_SYSINFO_CPU_CORE_COUNT");
        assert_eq!(SYSINFO_CPU_NAME, "VERGEN_SYSINFO_CPU_NAME");
        assert_eq!(SYSINFO_CPU_BRAND, "VERGEN_SYSINFO_CPU_BRAND");
        assert_eq!(SYSINFO_CPU_FREQUENCY, "VERGEN_SYSINFO_CPU_FREQUENCY");
    }
}
