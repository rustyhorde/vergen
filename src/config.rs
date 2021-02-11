// Copyright (c) 2016, 2018, 2021 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `vergen` configuration

use crate::output::VergenKey;
use enum_iterator::IntoEnumIterator;
use getset::{Getters, MutGetters};
use std::collections::HashMap;

#[derive(Clone, Debug, Getters, MutGetters)]
#[getset(get = "pub(crate)")]
#[getset(get_mut = "pub(crate)")]
pub(crate) struct Config {
    cfg_map: HashMap<VergenKey, Option<String>>,
}

impl Default for Config {
    fn default() -> Config {
        Self {
            cfg_map: VergenKey::into_enum_iter().map(|x| (x, None)).collect(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Config;

    #[test]
    fn default_works() {
        assert!(!Config::default().cfg_map().is_empty());
    }
}
