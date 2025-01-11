//! Shared git configuration for different git libraries

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Describe {
    pub tags: bool,
    pub dirty: bool,
    pub match_pattern: Option<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Dirty {
    pub include_untracked: bool,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Sha {
    pub short: bool,
}
