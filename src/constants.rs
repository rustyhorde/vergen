// Copyright (c) 2016, 2018 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Flags used to control the build script output.

use bitflags::bitflags;

bitflags!(
    /// Constants Flags
    ///
    /// Use these to toggle off the generation of constants you won't use.
    ///
    /// ```
    /// # extern crate vergen;
    /// #
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
    ///     ConstantsFlags::TARGET_TRIPLE |
    ///     ConstantsFlags::HOST_TRIPLE |
    ///     ConstantsFlags::SEMVER |
    ///     ConstantsFlags::RUSTC_SEMVER |
    ///     ConstantsFlags::RUSTC_CHANNEL |
    ///     ConstantsFlags::REBUILD_ON_HEAD_CHANGE |
    ///     ConstantsFlags::BRANCH |
    ///     ConstantsFlags::TAG_DIRTY;
    ///
    /// assert_eq!(actual_flags, expected_flags)
    /// # }
    /// ```
    pub struct ConstantsFlags: u64 {
        /// Generate the build timestamp constant.
        ///
        /// `2018-08-09T15:15:57.282334589+00:00`
        const BUILD_TIMESTAMP        = 0b0000_0000_0000_0001;
        /// Generate the build date constant.
        ///
        /// `2018-08-09`
        const BUILD_DATE             = 0b0000_0000_0000_0010;
        /// Generate the SHA constant.
        ///
        /// `75b390dc6c05a6a4aa2791cc7b3934591803bc22`
        const SHA                    = 0b0000_0000_0000_0100;
        /// Generate the short SHA constant.
        ///
        /// `75b390d`
        const SHA_SHORT              = 0b0000_0000_0000_1000;
        /// Generate the commit date constant.
        ///
        /// `2018-08-08`
        const COMMIT_DATE            = 0b0000_0000_0001_0000;
        /// Generate the target triple constant.
        ///
        /// `x86_64-unknown-linux-gnu`
        const TARGET_TRIPLE          = 0b0000_0000_0010_0000;
        /// Generate the semver constant.
        ///
        /// This defaults to the output of `git describe`.  If that output is
        /// empty, the the `CARGO_PKG_VERSION` environment variable is used.
        ///
        /// `v0.1.0`
        const SEMVER                 = 0b0000_0000_0100_0000;
        /// Generate the semver constant, including lightweight tags.
        ///
        /// This defaults to the output of `git describe --tags`.  If that output
        /// is empty, the the `CARGO_PKG_VERSION` environment variable is used.
        ///
        /// `v0.1.0`
        const SEMVER_LIGHTWEIGHT     = 0b0000_0000_1000_0000;
        /// Generate the `cargo:rebuild-if-changed=.git/HEAD` and the
        /// `cargo:rebuild-if-changed=.git/<ref>` cargo build output.
        const REBUILD_ON_HEAD_CHANGE = 0b0000_0001_0000_0000;
        /// Generate the semver constant from `CARGO_PKG_VERSION`.  This is
        /// mutually exclusive with the `SEMVER` flag.
        ///
        /// `0.1.0`
        const SEMVER_FROM_CARGO_PKG  = 0b0000_0010_0000_0000;
        /// Generates the rustc compiler version.
        ///
        /// `1.43.1`
        const RUSTC_SEMVER           = 0b0000_0100_0000_0000;
        /// Generates the channel the rust compiler is installed from.
        ///
        /// `nightly`
        const RUSTC_CHANNEL          = 0b0000_1000_0000_0000;
        /// Generate the host triple constant.
        ///
        /// `x86_64-unknown-linux-gnu`
        const HOST_TRIPLE            = 0b0001_0000_0000_0000;
        /// Generate the branch name constant.
        ///
        /// `master`
        const BRANCH                 = 0b0010_0000_0000_0000;
        /// Include 'dirty' indicator on the SHAs when built on directory with changes
        ///
        /// `75b390d-dirty`
        const TAG_DIRTY              = 0b0100_0000_0000_0000;
    }
);

/// const prefix for codegen
pub(crate) const CONST_PREFIX: &str = "pub const ";
/// const type for codegen
pub(crate) const CONST_TYPE: &str = ": &str = ";

pub(crate) const BUILD_TIMESTAMP_NAME: &str = "VERGEN_BUILD_TIMESTAMP";
pub(crate) const BUILD_TIMESTAMP_COMMENT: &str = "/// Build Timestamp (UTC)";
pub(crate) const BUILD_DATE_NAME: &str = "VERGEN_BUILD_DATE";
pub(crate) const BUILD_DATE_COMMENT: &str = "/// Compile Time - Short (UTC)";
pub(crate) const SHA_NAME: &str = "VERGEN_SHA";
pub(crate) const SHA_COMMENT: &str = "/// Commit SHA";
pub(crate) const SHA_SHORT_NAME: &str = "VERGEN_SHA_SHORT";
pub(crate) const SHA_SHORT_COMMENT: &str = "/// Commit SHA - Short";
pub(crate) const COMMIT_DATE_NAME: &str = "VERGEN_COMMIT_DATE";
pub(crate) const COMMIT_DATE_COMMENT: &str = "/// Commit Date";
pub(crate) const TARGET_TRIPLE_NAME: &str = "VERGEN_TARGET_TRIPLE";
pub(crate) const TARGET_TRIPLE_COMMENT: &str = "/// Target Triple";
pub(crate) const SEMVER_NAME: &str = "VERGEN_SEMVER";
pub(crate) const SEMVER_COMMENT: &str = "/// Semver";
pub(crate) const SEMVER_TAGS_NAME: &str = "VERGEN_SEMVER_LIGHTWEIGHT";
pub(crate) const SEMVER_TAGS_COMMENT: &str = "/// Semver (Lightweight)";
pub(crate) const RUSTC_SEMVER_NAME: &str = "VERGEN_RUSTC_SEMVER";
pub(crate) const RUSTC_SEMVER_COMMENT: &str = "/// Rustc Version";
pub(crate) const RUSTC_CHANNEL_NAME: &str = "VERGEN_RUSTC_CHANNEL";
pub(crate) const RUSTC_CHANNEL_COMMENT: &str = "/// Rustc Release Channel";
pub(crate) const HOST_TRIPLE_NAME: &str = "VERGEN_HOST_TRIPLE";
pub(crate) const HOST_TRIPLE_COMMENT: &str = "/// Host Triple";
pub(crate) const BRANCH_NAME: &str = "VERGEN_BRANCH";
pub(crate) const BRANCH_COMMENT: &str = "/// Branch name";

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
        assert_eq!(ConstantsFlags::TARGET_TRIPLE.bits(), 0b0010_0000);
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
        assert_eq!(ConstantsFlags::HOST_TRIPLE.bits(), 0b0001_0000_0000_0000);
        assert_eq!(ConstantsFlags::BRANCH.bits(), 0b0010_0000_0000_0000);
        assert_eq!(ConstantsFlags::TAG_DIRTY.bits(), 0b0100_0000_0000_0000);
    }

    #[test]
    fn constants_dont_change() {
        assert_eq!(CONST_PREFIX, "pub const ");
        assert_eq!(CONST_TYPE, ": &str = ");
        assert_eq!(BUILD_TIMESTAMP_NAME, "VERGEN_BUILD_TIMESTAMP");
        assert_eq!(BUILD_TIMESTAMP_COMMENT, "/// Build Timestamp (UTC)");
        assert_eq!(BUILD_DATE_NAME, "VERGEN_BUILD_DATE");
        assert_eq!(BUILD_DATE_COMMENT, "/// Compile Time - Short (UTC)");
        assert_eq!(SHA_NAME, "VERGEN_SHA");
        assert_eq!(SHA_COMMENT, "/// Commit SHA");
        assert_eq!(SHA_SHORT_NAME, "VERGEN_SHA_SHORT");
        assert_eq!(SHA_SHORT_COMMENT, "/// Commit SHA - Short");
        assert_eq!(COMMIT_DATE_NAME, "VERGEN_COMMIT_DATE");
        assert_eq!(COMMIT_DATE_COMMENT, "/// Commit Date");
        assert_eq!(TARGET_TRIPLE_NAME, "VERGEN_TARGET_TRIPLE");
        assert_eq!(TARGET_TRIPLE_COMMENT, "/// Target Triple");
        assert_eq!(SEMVER_NAME, "VERGEN_SEMVER");
        assert_eq!(SEMVER_COMMENT, "/// Semver");
        assert_eq!(SEMVER_TAGS_NAME, "VERGEN_SEMVER_LIGHTWEIGHT");
        assert_eq!(SEMVER_TAGS_COMMENT, "/// Semver (Lightweight)");
        assert_eq!(RUSTC_SEMVER_NAME, "VERGEN_RUSTC_SEMVER");
        assert_eq!(RUSTC_SEMVER_COMMENT, "/// Rustc Version");
        assert_eq!(RUSTC_CHANNEL_NAME, "VERGEN_RUSTC_CHANNEL");
        assert_eq!(RUSTC_CHANNEL_COMMENT, "/// Rustc Release Channel");
        assert_eq!(HOST_TRIPLE_NAME, "VERGEN_HOST_TRIPLE");
        assert_eq!(HOST_TRIPLE_COMMENT, "/// Host Triple");
        assert_eq!(BRANCH_NAME, "VERGEN_BRANCH");
        assert_eq!(BRANCH_COMMENT, "/// Branch name");
    }
}
