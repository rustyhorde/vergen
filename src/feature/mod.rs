// Copyright (c) 2016, 2018, 2021 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `vergen` feature implementations

#[cfg(any(feature = "build", feature = "git", feature = "rustc"))]
use {crate::config::VergenKey, std::collections::BTreeMap};

mod build;
mod git;
mod rustc;

pub(crate) use build::add_build_config;
pub(crate) use git::add_git_config;
pub(crate) use rustc::add_rustc_config;

#[cfg(any(feature = "build", feature = "git", feature = "rustc"))]
pub(crate) fn add_entry(
    map: &mut BTreeMap<VergenKey, Option<String>>,
    key: VergenKey,
    value: Option<String>,
) {
    *map.entry(key).or_insert_with(Option::default) = value;
}

#[cfg(all(test, any(feature = "build", feature = "git", feature = "rustc")))]
mod test {
    use super::add_entry;
    use crate::config::VergenKey;
    use std::collections::BTreeMap;

    #[test]
    fn check_add_entry() {
        let mut hm = BTreeMap::new();
        add_entry(&mut hm, VergenKey::BuildTimestamp, Some("".to_string()));
        assert!(hm.get(&VergenKey::BuildTimestamp).is_some());
    }
}
