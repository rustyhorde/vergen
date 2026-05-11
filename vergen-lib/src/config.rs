// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use bon::Builder;

// Common configuration structs

/// git configuration for the `describe` output
#[derive(Builder, Clone, Copy, Debug, Eq, PartialEq)]
pub struct Describe {
    /// Instead of using only the annotated tags, use any tag found in refs/tags namespace.
    #[builder(default = false)]
    tags: bool,
    /// If the working tree has local modification "-dirty" is appended to it.
    #[builder(default = false)]
    dirty: bool,
    /// Only consider tags matching the given glob pattern, excluding the "refs/tags/" prefix.
    match_pattern: Option<&'static str>,
}

impl Describe {
    /// Instead of using only the annotated tags, use any tag found in refs/tags namespace.
    #[must_use]
    pub fn tags(&self) -> bool {
        self.tags
    }

    /// If the working tree has local modification "-dirty" is appended to it.
    #[must_use]
    pub fn dirty(&self) -> bool {
        self.dirty
    }

    /// Only consider tags matching the given glob pattern, excluding the "refs/tags/" prefix.
    #[must_use]
    pub fn match_pattern(&self) -> &Option<&'static str> {
        &self.match_pattern
    }
}

/// git configuration for the `sha` output
#[derive(Builder, Clone, Copy, Debug, Eq, PartialEq)]
pub struct Sha {
    /// Shortens the object name to a unique prefix
    #[builder(default = false)]
    short: bool,
}

impl Sha {
    /// Shortens the object name to a unique prefix
    #[must_use]
    pub fn short(&self) -> bool {
        self.short
    }
}

/// git configuration for the `dirty` output
#[derive(Builder, Clone, Copy, Debug, Eq, PartialEq)]
pub struct Dirty {
    /// Should we include/ignore untracked files in deciding whether the repository is dirty.
    #[builder(default = false)]
    include_untracked: bool,
}

impl Dirty {
    /// Should we include/ignore untracked files in deciding whether the repository is dirty.
    #[must_use]
    pub fn include_untracked(&self) -> bool {
        self.include_untracked
    }
}
