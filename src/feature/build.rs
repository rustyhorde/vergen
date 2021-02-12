// Copyright (c) 2016, 2018, 2021 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `vergen` build feature implementation

use crate::{config::Config, constants::ConstantsFlags};
#[cfg(feature = "build")]
use {
    crate::{config::VergenKey, feature::add_entry},
    chrono::Utc,
    std::env,
};

#[cfg(feature = "build")]
pub(crate) fn add_build_config(flags: ConstantsFlags, config: &mut Config) {
    // Setup datetime information
    let now = Utc::now();
    if flags.contains(ConstantsFlags::BUILD_TIMESTAMP) {
        add_entry(
            config.cfg_map_mut(),
            VergenKey::BuildTimestamp,
            Some(now.to_rfc3339()),
        );
    }

    if flags.contains(ConstantsFlags::BUILD_DATE) {
        add_entry(
            config.cfg_map_mut(),
            VergenKey::BuildDate,
            Some(now.format("%Y-%m-%d").to_string()),
        );
    }

    if flags.contains(ConstantsFlags::SEMVER_FROM_CARGO_PKG) {
        add_entry(
            config.cfg_map_mut(),
            VergenKey::Semver,
            env::var("CARGO_PKG_VERSION").ok(),
        );
    }
}

#[cfg(not(feature = "build"))]
pub(crate) fn add_build_config(_flags: ConstantsFlags, _config: &mut Config) {}

#[cfg(all(test, feature = "build"))]
mod test {
    use super::add_build_config;
    use crate::{
        config::{Config, VergenKey},
        constants::ConstantsFlags,
    };
    use std::collections::HashMap;

    fn check_build_keys(cfg_map: &HashMap<VergenKey, Option<String>>) {
        let mut count = 0;
        for (k, v) in cfg_map {
            match *k {
                VergenKey::BuildDate | VergenKey::BuildTimestamp | VergenKey::Semver => {
                    assert!(v.is_some());
                    count += 1;
                }
                _ => {}
            }
        }
        assert_eq!(count, 3);
    }

    #[test]
    fn add_build_config_works() {
        let mut config = Config::default();
        add_build_config(ConstantsFlags::all(), &mut config);
        check_build_keys(config.cfg_map());
    }
}

#[cfg(all(test, not(feature = "build")))]
mod test {
    use super::add_build_config;
    use crate::{config::{Config, VergenKey}, constants::ConstantsFlags, error::Result};
    use std::collections::HashMap;

    fn check_build_keys(cfg_map: &HashMap<VergenKey, Option<String>>) {
        let mut count = 0;
        for (k, v) in cfg_map {
            match *k {
                VergenKey::BuildDate | VergenKey::BuildTimestamp | VergenKey::Semver => {
                    assert!(v.is_none());
                    count += 1;
                }
                _ => {}
            }
        }
        assert_eq!(count, 3);
    }

    #[test]
    fn add_build_config_works() -> Result<()> {
        let mut config = Config::default();
        add_build_config(ConstantsFlags::all(), &mut config);
        check_build_keys(config.cfg_map());
        Ok(())
    }
}
