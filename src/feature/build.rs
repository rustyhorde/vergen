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
        test::get_map_value,
    };
    use lazy_static::lazy_static;
    use regex::Regex;
    use std::collections::BTreeMap;

    lazy_static! {
        static ref DATE_REGEX: Regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
        static ref RFC3339_REGEX: Regex = Regex::new(r"^([0-9]+)-(0[1-9]|1[012])-(0[1-9]|[12][0-9]|3[01])[Tt]([01][0-9]|2[0-3]):([0-5][0-9]):([0-5][0-9]|60)(\.[0-9]+)?(([Zz])|([\+|\-]([01][0-9]|2[0-3]):[0-5][0-9]))$").unwrap();
        static ref SEMVER_REGEX: Regex = Regex::new(r"^(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$").unwrap();
    }

    fn check_build_instructions(cfg_map: &BTreeMap<VergenKey, Option<String>>) {
        assert!(DATE_REGEX.is_match(&get_map_value(VergenKey::BuildDate, cfg_map)));
        assert!(RFC3339_REGEX.is_match(&get_map_value(VergenKey::BuildTimestamp, cfg_map)));
        assert!(SEMVER_REGEX.is_match(&get_map_value(VergenKey::Semver, cfg_map)));
    }

    fn check_build_keys(cfg_map: &BTreeMap<VergenKey, Option<String>>) {
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
        check_build_instructions(config.cfg_map());
    }
}

#[cfg(all(test, not(feature = "build")))]
mod test {
    use super::add_build_config;
    use crate::{
        config::{Config, VergenKey},
        constants::ConstantsFlags,
        error::Result,
    };
    use std::collections::BTreeMap;

    fn check_build_keys(cfg_map: &BTreeMap<VergenKey, Option<String>>) {
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
