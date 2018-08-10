// Copyright (c) 2016, 2018 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `vergen` constants.

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
    /// # fn foo() {
    /// let mut flags = ConstantsFlags::all();
    /// flags.toggle(ConstantsFlags::SHA_SHORT);
    /// flags.toggle(ConstantsFlags::COMMIT_DATE);
    ///
    /// assert_eq!(
    ///   flags,
    ///   ConstantsFlags::BUILD_TIMESTAMP &
    ///   ConstantsFlags::BUILD_DATE &
    ///   ConstantsFlags::SHA &
    ///   ConstantsFlags::TARGET_TRIPLE &
    ///   ConstantsFlags::SEMVER &
    ///   ConstantsFlags::SEMVER_LIGHTWEIGHT
    /// )
    /// # }
    /// ```
    pub struct ConstantsFlags: u32 {
        /// Generate the build timestamp constant.
        ///
        /// "2018-08-09T15:15:57.282334589+00:00"
        const BUILD_TIMESTAMP    = 0x0000_0001;
        /// Generate the build date constant.
        ///
        /// "2018-08-09"
        const BUILD_DATE         = 0x0000_0010;
        /// Generate the SHA constant.
        ///
        /// "75b390dc6c05a6a4aa2791cc7b3934591803bc22"
        const SHA                = 0x0000_0100;
        /// Generate the short SHA constant.
        ///
        /// "75b390d"
        const SHA_SHORT          = 0x0000_1000;
        /// Generate the commit date constant.
        ///
        /// "2018-08-08"
        const COMMIT_DATE        = 0x0001_0000;
        /// Generate the target triple constant.
        ///
        /// "x86_64-unknown-linux-gnu"
        const TARGET_TRIPLE      = 0x0010_0000;
        /// Generate the semver constant.
        ///
        /// This defaults to the output of `git describe`.  If that output is
        /// empty, the the `CARGO_PKG_VERSION` environment variable is used.
        ///
        /// "v0.1.0-pre.0"
        const SEMVER             = 0x0100_0000;
        /// Generate the semver constant, including lightweight tags.
        ///
        /// This defaults to the output of `git describe`.  If that output is
        /// empty, the the `CARGO_PKG_VERSION` environment variable is used.
        ///
        /// "v0.1.0-pre.0"
        const SEMVER_LIGHTWEIGHT = 0x0200_0000;
    }
);

/// const prefix for codegen
pub const CONST_PREFIX: &str = "pub const ";
/// const type for codegen
pub const CONST_TYPE: &str = ": &str = ";

pub const BUILD_TIMESTAMP_NAME: &str = "VERGEN_BUILD_TIMESTAMP";
pub const BUILD_TIMESTAMP_COMMENT: &str = "/// Build Timestamp (UTC)";
pub const BUILD_DATE_NAME: &str = "VERGEN_BUILD_DATE";
pub const BUILD_DATE_COMMENT: &str = "/// Compile Time - Short (UTC)";
pub const SHA_NAME: &str = "VERGEN_SHA";
pub const SHA_COMMENT: &str = "/// Commit SHA";
pub const SHA_SHORT_NAME: &str = "VERGEN_SHA_SHORT";
pub const SHA_SHORT_COMMENT: &str = "/// Commit SHA - Short";
pub const COMMIT_DATE_NAME: &str = "VERGEN_COMMIT_DATE";
pub const COMMIT_DATE_COMMENT: &str = "/// Commit Date";
pub const TARGET_TRIPLE_NAME: &str = "VERGEN_TARGET_TRIPLE";
pub const TARGET_TRIPLE_COMMENT: &str = "/// Target Triple";
pub const SEMVER_NAME: &str = "VERGEN_SEMVER";
pub const SEMVER_COMMENT: &str = "/// Semver";
pub const SEMVER_TAGS_NAME: &str = "VERGEN_SEMVER_LIGHTWEIGHT";
pub const SEMVER_TAGS_COMMENT: &str = "/// Semver (Lightweight)";
