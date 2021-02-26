// Copyright (c) 2016, 2018, 2021 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Flags used to toggle individual `cargo:` instruction generation

use bitflags::bitflags;

bitflags!(
    /// **DEPRECATED** - [`ConstantsFlags`] has been deprecated in favor of [`Config`](crate::config::Instructions).
    ///
    /// Please make the switch to that instead. [`ConstantsFlags`] will be removed in
    /// version 5.
    ///
    /// Flags used to toggle individual `cargo:` instruction generation
    ///
    /// Use these to toggle off instructions you don't wish to generate
    ///
    /// # Example
    /// ```
    /// # use vergen::ConstantsFlags;
    /// #
    /// # fn main() {
    /// let mut actual_flags = ConstantsFlags::all();
    /// actual_flags.toggle(ConstantsFlags::SHA_SHORT);
    /// actual_flags.toggle(ConstantsFlags::BUILD_DATE);
    /// actual_flags.toggle(ConstantsFlags::SEMVER_LIGHTWEIGHT);
    /// actual_flags.toggle(ConstantsFlags::SEMVER_FROM_CARGO_PKG);
    ///
    /// let expected_flags = ConstantsFlags::BUILD_TIMESTAMP |
    ///     ConstantsFlags::SHA |
    ///     ConstantsFlags::COMMIT_DATE |
    ///     ConstantsFlags::RUSTC_HOST_TRIPLE |
    ///     ConstantsFlags::SEMVER |
    ///     ConstantsFlags::RUSTC_SEMVER |
    ///     ConstantsFlags::RUSTC_CHANNEL |
    ///     ConstantsFlags::REBUILD_ON_HEAD_CHANGE |
    ///     ConstantsFlags::BRANCH |
    ///     ConstantsFlags::RUSTC_COMMIT_HASH |
    ///     ConstantsFlags::RUSTC_COMMIT_DATE |
    ///     ConstantsFlags::RUSTC_LLVM_VERSION |
    ///     ConstantsFlags::CARGO_TARGET_TRIPLE |
    ///     ConstantsFlags::CARGO_PROFILE |
    ///     ConstantsFlags::CARGO_FEATURES;
    ///
    /// assert_eq!(actual_flags, expected_flags)
    /// # }
    /// ```
    pub struct ConstantsFlags: u64 {
        /// Output the build timestamp instruction `VERGEN_BUILD_TIMESTAMP`
        ///
        /// `cargo:rustc-env=VERGEN_BUILD_TIMESTAMP=2021-02-12T01:54:15.134750+00:00`
        ///
        /// This is the UTC timestamp when the build was run
        const BUILD_TIMESTAMP        = 0b0000_0000_0000_0001;
        /// Output the build date instruction `VERGEN_BUILD_DATE`
        ///
        /// `cargo:rustc-env=VERGEN_BUILD_DATE=2021-02-12`
        ///
        /// This is the UTC date when the build was run
        const BUILD_DATE             = 0b0000_0000_0000_0010;
        /// Output the SHA instruction `VERGEN_GIT_SHA`
        ///
        /// `cargo:rustc-env=VERGEN_GIT_SHA=95fc0f5d066710f16e0c23ce3239d6e040abca0d`
        ///
        /// This is the most recent commit SHA on the current branch
        const SHA                    = 0b0000_0000_0000_0100;
        /// Output the short SHA instruction `VERGEN_GIT_SHA_SHORT`
        ///
        /// `cargo:rustc-env=VERGEN_GIT_SHA_SHORT=95fc0f5`
        ///
        /// This is the most recent commit SHA on the current branch short version
        const SHA_SHORT              = 0b0000_0000_0000_1000;
        /// Output the short SHA instruction `VERGEN_GIT_COMMIT_DATE`
        ///
        /// `cargo:rustc-env=VERGEN_GIT_COMMIT_DATE=2021-02-11T20:05:53-05:00`
        ///
        /// This is the local timestamp of the most recent commit on the current branch
        const COMMIT_DATE            = 0b0000_0000_0001_0000;
        /// Output the semantic version instruction `VERGEN_GIT_SEMVER`
        ///
        /// `cargo:rustc-env=VERGEN_GIT_SEMVER=v3.2.0-86-g95fc0f5`
        ///
        /// This defaults to the output of [`git2::Repository::describe`].
        /// If the `git` feature is disabled or the describe call fails, generation falls back to the [`CARGO_PKG_VERSION`] environment variable.
        /// Note that the [git describe] method is only useful if you have tags on your repository.
        /// I recommend [`SemVer`] tags, but this will work with any tag format.
        ///
        /// [git describe]: git2::Repository::describe
        /// [`SemVer`]: https://semver.org/
        /// [`CARGO_PKG_VERSION`]: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates
        const SEMVER                 = 0b0000_0000_0100_0000;
        /// Output the semantic version instruction `VERGEN_GIT_SEMVER_LIGHTWEIGHT`
        ///
        /// `cargo:rustc-env=VERGEN_GIT_SEMVER_LIGHTWEIGHT=blah-33-g95fc0f5`
        ///
        /// This follows the same rules as described in [`ConstantsFlags::SEMVER`].
        /// Note that `VERGEN_GIT_SEMVER_LIGHTWEIGHT` will only differ from `VERGEN_GIT_SEMVER` if you use lightweight tags on your repository.
        const SEMVER_LIGHTWEIGHT     = 0b0000_0000_1000_0000;
        /// Output the `cargo:rerun-if-changed` instructions
        ///
        /// `cargo:rerun-if-changed=/Users/yoda/projects/rust-lang/vergen/.git/HEAD`
        ///
        /// `cargo:rerun-if-changed=/Users/yoda/projects/rust-lang/vergen/.git/refs/heads/feature/git2`
        ///
        /// This toggle is useful to force the build script to re-run when you perform different git actions, i.e. change branches.
        const REBUILD_ON_HEAD_CHANGE = 0b0000_0001_0000_0000;
        /// Output the semantic version instruction `VERGEN_GIT_SEMVER`
        ///
        /// `cargo:rustc-env=VERGEN_GIT_SEMVER=0.1.0`
        ///
        /// This flag can be used to force the semver instruction to be generated from the `CARGO_PKG_VERSION` environment variable.
        /// Note that it is mutually exclusive with the `SEMVER` and `SEMVER_LIGHTWEIGHT` flags.
        const SEMVER_FROM_CARGO_PKG  = 0b0000_0010_0000_0000;
        /// Output the rustc compiler version instruction `VERGEN_RUSTC_SEMVER`
        ///
        /// `cargo:rustc-env=VERGEN_RUSTC_SEMVER=1.52.0-nightly`
        ///
        /// This output is generated via the `rustversion` library and refers to the version of rust that was used to create the build.
        const RUSTC_SEMVER           = 0b0000_0100_0000_0000;
        /// Output the rustc compiler channel instruction `VERGEN_RUSTC_CHANNEL`
        ///
        /// `cargo:rustc-env=VERGEN_RUSTC_CHANNEL=nightly`
        ///
        /// This output is generated via the `rustversion` library and refers to the channel (dev, nightly, beta, or stable) of rust that was used to create the build.
        const RUSTC_CHANNEL          = 0b0000_1000_0000_0000;
        /// Output the rustc compiler host triple instruction `VERGEN_RUSTC_HOST_TRIPLE`
        ///
        /// `cargo:rustc-env=VERGEN_RUSTC_HOST_TRIPLE=x86_64-apple-darwin`
        ///
        /// This output is generated via the `rustversion` library and refers to the host triple of rust that was used to create the build.
        const RUSTC_HOST_TRIPLE      = 0b0001_0000_0000_0000;
        /// Output the semantic version instruction `VERGEN_GIT_BRANCH`
        ///
        /// `cargo:rustc-env=VERGEN_GIT_BRANCH=feature/git2`
        ///
        /// This output represents the current branch when the build was performed.
        const BRANCH                 = 0b0010_0000_0000_0000;
        /// Output the rustc compiler commit SHA instruction `VERGEN_RUSTC_COMMIT_HASH`
        ///
        /// `cargo:rustc-env=VERGEN_RUSTC_COMMIT_HASH=07194ffcd25b0871ce560b9f702e52db27ac9f77`
        ///
        /// This output is generated via the `rustversion` library and refers to the commit SHA of rust that was used to create the build.
        const RUSTC_COMMIT_HASH      = 0b0100_0000_0000_0000;
        /// Output the rustc compiler commit date instruction `VERGEN_RUSTC_COMMIT_DATE`
        ///
        /// `cargo:rustc-env=VERGEN_RUSTC_COMMIT_DATE=2021-02-10`
        ///
        /// This output is generated via the `rustversion` library and refers to the commit date of rust that was used to create the build.
        const RUSTC_COMMIT_DATE      = 0b1000_0000_0000_0000;
        /// Output the rustc compiler LLVM instruction `VERGEN_RUSTC_LLVM_VERSION`
        ///
        /// `cargo:rustc-env=VERGEN_RUSTC_LLVM_VERSION=11.0`
        ///
        /// This output is generated via the `rustversion` library and refers to the LLVM version of rust that was used to create the build.
        /// Note that this output is only valid on the `nightly` channel currently.
        const RUSTC_LLVM_VERSION     = 0b0001_0000_0000_0000_0000;
        /// Output the cargo target triple instruction `VERGEN_CARGO_TARGET_TRIPLE`
        ///
        /// `cargo:rustc-env=VERGEN_CARGO_TARGET_TRIPLE=x86_64-unknown-linux-gnu`
        ///
        /// This output is made available by cargo at build time, and may be different than the host triple.
        const CARGO_TARGET_TRIPLE    = 0b0010_0000_0000_0000_0000;
        /// Output the cargo profile instruction `VERGEN_CARGO_PROFILE`
        ///
        /// `cargo:rustc-env=VERGEN_CARGO_PROFILE=debug`
        ///
        /// This output is made available by cargo at build time and represents the current profile cargo is using during the build.
        const CARGO_PROFILE          = 0b0100_0000_0000_0000_0000;
        /// Output the cargo features instruction `VERGEN_CARGO_FEATURES`
        ///
        /// `cargo:rustc-env=VERGEN_CARGO_FEATURES=git,build`
        ///
        /// This output is made available by cargo at build time and represents the current features cargo has enabled during the build.
        const CARGO_FEATURES         = 0b1000_0000_0000_0000_0000;
    }
);

// Build Constants
pub(crate) const BUILD_TIMESTAMP_NAME: &str = "VERGEN_BUILD_TIMESTAMP";
pub(crate) const BUILD_DATE_NAME: &str = "VERGEN_BUILD_DATE";
pub(crate) const BUILD_TIME_NAME: &str = "VERGEN_BUILD_TIME";
pub(crate) const BUILD_SEMVER_NAME: &str = "VERGEN_BUILD_SEMVER";

// git Constants
pub(crate) const GIT_BRANCH_NAME: &str = "VERGEN_GIT_BRANCH";
pub(crate) const GIT_COMMIT_DATE_NAME: &str = "VERGEN_GIT_COMMIT_DATE";
pub(crate) const GIT_COMMIT_TIME_NAME: &str = "VERGEN_GIT_COMMIT_TIME";
pub(crate) const GIT_COMMIT_TIMESTAMP_NAME: &str = "VERGEN_GIT_COMMIT_TIMESTAMP";
pub(crate) const GIT_SEMVER_NAME: &str = "VERGEN_GIT_SEMVER";
pub(crate) const GIT_SEMVER_TAGS_NAME: &str = "VERGEN_GIT_SEMVER_LIGHTWEIGHT";
pub(crate) const GIT_SHA_NAME: &str = "VERGEN_GIT_SHA";
pub(crate) const GIT_SHA_SHORT_NAME: &str = "VERGEN_GIT_SHA_SHORT";

// rustc Constants
pub(crate) const RUSTC_CHANNEL_NAME: &str = "VERGEN_RUSTC_CHANNEL";
pub(crate) const RUSTC_HOST_TRIPLE_NAME: &str = "VERGEN_RUSTC_HOST_TRIPLE";
pub(crate) const RUSTC_SEMVER_NAME: &str = "VERGEN_RUSTC_SEMVER";
pub(crate) const RUSTC_COMMIT_HASH: &str = "VERGEN_RUSTC_COMMIT_HASH";
pub(crate) const RUSTC_COMMIT_DATE: &str = "VERGEN_RUSTC_COMMIT_DATE";
pub(crate) const RUSTC_LLVM_VERSION: &str = "VERGEN_RUSTC_LLVM_VERSION";

// cargo Constants
pub(crate) const CARGO_TARGET_TRIPLE: &str = "VERGEN_CARGO_TARGET_TRIPLE";
pub(crate) const CARGO_PROFILE: &str = "VERGEN_CARGO_PROFILE";
pub(crate) const CARGO_FEATURES: &str = "VERGEN_CARGO_FEATURES";

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bitflags_dont_change() {
        assert_eq!(ConstantsFlags::BUILD_TIMESTAMP.bits(), 0b0000_0001);
        assert_eq!(ConstantsFlags::BUILD_DATE.bits(), 0b0000_0010);
        assert_eq!(ConstantsFlags::SHA.bits(), 0b0000_0100);
        assert_eq!(ConstantsFlags::SHA_SHORT.bits(), 0b0000_1000);
        assert_eq!(ConstantsFlags::COMMIT_DATE.bits(), 0b0001_0000);
        assert_eq!(ConstantsFlags::SEMVER.bits(), 0b0100_0000);
        assert_eq!(ConstantsFlags::SEMVER_LIGHTWEIGHT.bits(), 0b1000_0000);
        assert_eq!(
            ConstantsFlags::REBUILD_ON_HEAD_CHANGE.bits(),
            0b0001_0000_0000
        );
        assert_eq!(
            ConstantsFlags::SEMVER_FROM_CARGO_PKG.bits(),
            0b0010_0000_0000
        );
        assert_eq!(ConstantsFlags::RUSTC_SEMVER.bits(), 0b0100_0000_0000);
        assert_eq!(ConstantsFlags::RUSTC_CHANNEL.bits(), 0b1000_0000_0000);
        assert_eq!(
            ConstantsFlags::RUSTC_HOST_TRIPLE.bits(),
            0b0001_0000_0000_0000
        );
        assert_eq!(ConstantsFlags::BRANCH.bits(), 0b0010_0000_0000_0000);
        assert_eq!(
            ConstantsFlags::RUSTC_COMMIT_HASH.bits(),
            0b0100_0000_0000_0000
        );
        assert_eq!(
            ConstantsFlags::RUSTC_COMMIT_DATE.bits(),
            0b1000_0000_0000_0000
        );
        assert_eq!(
            ConstantsFlags::RUSTC_LLVM_VERSION.bits(),
            0b0001_0000_0000_0000_0000
        );
        assert_eq!(
            ConstantsFlags::CARGO_TARGET_TRIPLE.bits(),
            0b0010_0000_0000_0000_0000
        );
        assert_eq!(
            ConstantsFlags::CARGO_PROFILE.bits(),
            0b0100_0000_0000_0000_0000
        );
        assert_eq!(
            ConstantsFlags::CARGO_FEATURES.bits(),
            0b1000_0000_0000_0000_0000
        );
    }

    #[test]
    fn constants_dont_change() {
        // Build Constants
        assert_eq!(BUILD_TIMESTAMP_NAME, "VERGEN_BUILD_TIMESTAMP");
        assert_eq!(BUILD_DATE_NAME, "VERGEN_BUILD_DATE");

        // git Constants
        assert_eq!(GIT_BRANCH_NAME, "VERGEN_GIT_BRANCH");
        assert_eq!(GIT_SHA_NAME, "VERGEN_GIT_SHA");
        assert_eq!(GIT_SHA_SHORT_NAME, "VERGEN_GIT_SHA_SHORT");
        assert_eq!(GIT_COMMIT_DATE_NAME, "VERGEN_GIT_COMMIT_DATE");
        assert_eq!(GIT_SEMVER_NAME, "VERGEN_GIT_SEMVER");
        assert_eq!(GIT_SEMVER_TAGS_NAME, "VERGEN_GIT_SEMVER_LIGHTWEIGHT");

        // rustc Constants
        assert_eq!(RUSTC_SEMVER_NAME, "VERGEN_RUSTC_SEMVER");
        assert_eq!(RUSTC_CHANNEL_NAME, "VERGEN_RUSTC_CHANNEL");
        assert_eq!(RUSTC_HOST_TRIPLE_NAME, "VERGEN_RUSTC_HOST_TRIPLE");
        assert_eq!(RUSTC_COMMIT_HASH, "VERGEN_RUSTC_COMMIT_HASH");
        assert_eq!(RUSTC_COMMIT_DATE, "VERGEN_RUSTC_COMMIT_DATE");
        assert_eq!(RUSTC_LLVM_VERSION, "VERGEN_RUSTC_LLVM_VERSION");

        // cargo Constants
        assert_eq!(CARGO_TARGET_TRIPLE, "VERGEN_CARGO_TARGET_TRIPLE");
        assert_eq!(CARGO_PROFILE, "VERGEN_CARGO_PROFILE");
        assert_eq!(CARGO_FEATURES, "VERGEN_CARGO_FEATURES");
    }
}
