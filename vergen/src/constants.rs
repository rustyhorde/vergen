// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Internal Constants

// Idempotent Constant
#[cfg(any(
    feature = "build",
    feature = "cargo",
    all(
        feature = "git",
        any(feature = "git2", feature = "gitcl", feature = "gix")
    ),
    feature = "rustc",
    feature = "si",
))]
pub(crate) const VERGEN_IDEMPOTENT_DEFAULT: &str = "VERGEN_IDEMPOTENT_OUTPUT";

#[cfg(any(
    feature = "build",
    feature = "cargo",
    all(
        feature = "git",
        any(feature = "gitcl", feature = "git2", feature = "gix")
    ),
    feature = "rustc",
    feature = "si"
))]
pub(crate) use self::features::*;

#[cfg(any(
    feature = "build",
    feature = "cargo",
    all(
        feature = "git",
        any(feature = "gitcl", feature = "git2", feature = "gix")
    ),
    feature = "rustc",
    feature = "si"
))]
mod features {
    // Build Constants
    #[cfg(feature = "build")]
    pub(crate) const BUILD_TIMESTAMP_NAME: &str = "VERGEN_BUILD_TIMESTAMP";
    #[cfg(feature = "build")]
    pub(crate) const BUILD_DATE_NAME: &str = "VERGEN_BUILD_DATE";

    // git Constants
    #[cfg(all(
        feature = "git",
        any(feature = "gitcl", feature = "git2", feature = "gix")
    ))]
    pub(crate) const GIT_BRANCH_NAME: &str = "VERGEN_GIT_BRANCH";
    #[cfg(all(
        feature = "git",
        any(feature = "gitcl", feature = "git2", feature = "gix")
    ))]
    pub(crate) const GIT_COMMIT_AUTHOR_EMAIL: &str = "VERGEN_GIT_COMMIT_AUTHOR_EMAIL";
    #[cfg(all(
        feature = "git",
        any(feature = "gitcl", feature = "git2", feature = "gix")
    ))]
    pub(crate) const GIT_COMMIT_AUTHOR_NAME: &str = "VERGEN_GIT_COMMIT_AUTHOR_NAME";
    #[cfg(all(
        feature = "git",
        any(feature = "gitcl", feature = "git2", feature = "gix")
    ))]
    pub(crate) const GIT_COMMIT_COUNT: &str = "VERGEN_GIT_COMMIT_COUNT";
    #[cfg(all(
        feature = "git",
        any(feature = "gitcl", feature = "git2", feature = "gix")
    ))]
    pub(crate) const GIT_COMMIT_MESSAGE: &str = "VERGEN_GIT_COMMIT_MESSAGE";
    #[cfg(all(
        feature = "git",
        any(feature = "gitcl", feature = "git2", feature = "gix")
    ))]
    pub(crate) const GIT_COMMIT_DATE_NAME: &str = "VERGEN_GIT_COMMIT_DATE";
    #[cfg(all(
        feature = "git",
        any(feature = "gitcl", feature = "git2", feature = "gix")
    ))]
    pub(crate) const GIT_COMMIT_TIMESTAMP_NAME: &str = "VERGEN_GIT_COMMIT_TIMESTAMP";
    #[cfg(all(
        feature = "git",
        any(feature = "gitcl", feature = "git2", feature = "gix")
    ))]
    pub(crate) const GIT_DESCRIBE_NAME: &str = "VERGEN_GIT_DESCRIBE";
    #[cfg(all(
        feature = "git",
        any(feature = "gitcl", feature = "git2", feature = "gix")
    ))]
    pub(crate) const GIT_SHA_NAME: &str = "VERGEN_GIT_SHA";
    #[cfg(all(
        feature = "git",
        any(feature = "gitcl", feature = "git2", feature = "gix")
    ))]
    pub(crate) const GIT_DIRTY_NAME: &str = "VERGEN_GIT_DIRTY";

    // rustc Constants
    #[cfg(feature = "rustc")]
    pub(crate) const RUSTC_CHANNEL_NAME: &str = "VERGEN_RUSTC_CHANNEL";
    #[cfg(feature = "rustc")]
    pub(crate) const RUSTC_HOST_TRIPLE_NAME: &str = "VERGEN_RUSTC_HOST_TRIPLE";
    #[cfg(feature = "rustc")]
    pub(crate) const RUSTC_SEMVER_NAME: &str = "VERGEN_RUSTC_SEMVER";
    #[cfg(feature = "rustc")]
    pub(crate) const RUSTC_COMMIT_HASH: &str = "VERGEN_RUSTC_COMMIT_HASH";
    #[cfg(feature = "rustc")]
    pub(crate) const RUSTC_COMMIT_DATE: &str = "VERGEN_RUSTC_COMMIT_DATE";
    #[cfg(feature = "rustc")]
    pub(crate) const RUSTC_LLVM_VERSION: &str = "VERGEN_RUSTC_LLVM_VERSION";

    // cargo Constants
    #[cfg(feature = "cargo")]
    pub(crate) const CARGO_DEBUG: &str = "VERGEN_CARGO_DEBUG";
    #[cfg(feature = "cargo")]
    pub(crate) const CARGO_DEPENDENCIES: &str = "VERGEN_CARGO_DEPENDENCIES";
    #[cfg(feature = "cargo")]
    pub(crate) const CARGO_FEATURES: &str = "VERGEN_CARGO_FEATURES";
    #[cfg(feature = "cargo")]
    pub(crate) const CARGO_OPT_LEVEL: &str = "VERGEN_CARGO_OPT_LEVEL";
    #[cfg(feature = "cargo")]
    pub(crate) const CARGO_TARGET_TRIPLE: &str = "VERGEN_CARGO_TARGET_TRIPLE";

    // sysinfo Constants
    #[cfg(feature = "si")]
    pub(crate) const SYSINFO_NAME: &str = "VERGEN_SYSINFO_NAME";
    #[cfg(feature = "si")]
    pub(crate) const SYSINFO_OS_VERSION: &str = "VERGEN_SYSINFO_OS_VERSION";
    #[cfg(feature = "si")]
    pub(crate) const SYSINFO_USER: &str = "VERGEN_SYSINFO_USER";
    #[cfg(feature = "si")]
    pub(crate) const SYSINFO_MEMORY: &str = "VERGEN_SYSINFO_TOTAL_MEMORY";
    #[cfg(feature = "si")]
    pub(crate) const SYSINFO_CPU_VENDOR: &str = "VERGEN_SYSINFO_CPU_VENDOR";
    #[cfg(feature = "si")]
    pub(crate) const SYSINFO_CPU_CORE_COUNT: &str = "VERGEN_SYSINFO_CPU_CORE_COUNT";
    #[cfg(feature = "si")]
    pub(crate) const SYSINFO_CPU_NAME: &str = "VERGEN_SYSINFO_CPU_NAME";
    #[cfg(feature = "si")]
    pub(crate) const SYSINFO_CPU_BRAND: &str = "VERGEN_SYSINFO_CPU_BRAND";
    #[cfg(feature = "si")]
    pub(crate) const SYSINFO_CPU_FREQUENCY: &str = "VERGEN_SYSINFO_CPU_FREQUENCY";
}

#[cfg(all(
    test,
    any(
        feature = "build",
        feature = "cargo",
        all(
            feature = "git",
            any(feature = "gitcl", feature = "git2", feature = "gix")
        )
    ),
    feature = "rustc",
    feature = "si"
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

    #[cfg(all(
        feature = "git",
        any(feature = "gitcl", feature = "git2", feature = "gix")
    ))]
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
