// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use bon::Builder;
use getset::{CopyGetters, Getters};

// Common configuration structs

/// git configuration for the `describe` output
#[derive(Builder, Clone, Copy, CopyGetters, Debug, Eq, Getters, PartialEq)]
pub struct Describe {
    /// Instead of using only the annotated tags, use any tag found in refs/tags namespace.
    #[builder(default = false)]
    #[getset(get_copy = "pub")]
    tags: bool,
    /// If the working tree has local modification "-dirty" is appended to it.
    #[builder(default = false)]
    #[getset(get_copy = "pub")]
    dirty: bool,
    /// Only consider tags matching the given glob pattern, excluding the "refs/tags/" prefix.
    #[getset(get = "pub")]
    match_pattern: Option<&'static str>,
}

/// git configuration for the `sha` output
#[derive(Builder, Clone, Copy, CopyGetters, Debug, Eq, PartialEq)]
pub struct Sha {
    /// Shortens the object name to a unique prefix
    #[builder(default = false)]
    #[getset(get_copy = "pub")]
    short: bool,
}

/// git configuration for the `dirty` output
#[derive(Builder, Clone, Copy, CopyGetters, Debug, Eq, PartialEq)]
pub struct Dirty {
    /// Should we include/ignore untracked files in deciding whether the repository is dirty.
    #[builder(default = false)]
    #[getset(get_copy = "pub")]
    include_untracked: bool,
}
