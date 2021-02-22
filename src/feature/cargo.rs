// Copyright (c) 2016, 2018, 2021 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `vergen` cargo feature

use crate::{config::Config, constants::ConstantsFlags};
#[cfg(feature = "cargo")]
use {
    crate::{config::VergenKey, feature::add_entry},
    std::env,
};

#[cfg(feature = "cargo")]
pub(crate) fn add_cargo_config(flags: ConstantsFlags, config: &mut Config) {
    if flags.intersects(
        ConstantsFlags::CARGO_TARGET_TRIPLE
            | ConstantsFlags::CARGO_PROFILE
            | ConstantsFlags::CARGO_FEATURES,
    ) {
        if flags.contains(ConstantsFlags::CARGO_TARGET_TRIPLE) {
            add_entry(
                config.cfg_map_mut(),
                VergenKey::CargoTargetTriple,
                env::var("TARGET").ok(),
            );
        }
        if flags.contains(ConstantsFlags::CARGO_PROFILE) {
            add_entry(
                config.cfg_map_mut(),
                VergenKey::CargoProfile,
                env::var("PROFILE").ok(),
            );
        }
        if flags.contains(ConstantsFlags::CARGO_FEATURES) {
            let features: Vec<String> = env::vars().filter_map(is_cargo_feature).collect();
            let feature_str = features.as_slice().join(",");
            let value = if feature_str.is_empty() {
                Some("default".to_string())
            } else {
                Some(feature_str)
            };
            add_entry(config.cfg_map_mut(), VergenKey::CargoFeatures, value);
        }
    }
}

#[cfg(feature = "cargo")]
fn is_cargo_feature(var: (String, String)) -> Option<String> {
    let (k, v) = var;
    if k.starts_with("CARGO_FEATURE_") {
        Some(v)
    } else {
        None
    }
}

#[cfg(not(feature = "cargo"))]
pub(crate) fn add_cargo_config(_flags: ConstantsFlags, _config: &mut Config) {}

#[cfg(all(test, feature = "cargo"))]
mod test {
    use super::add_cargo_config;
    use crate::{
        config::{Config, VergenKey},
        constants::ConstantsFlags,
        test::get_map_value,
        testutils::{setup, teardown},
    };
    use std::{collections::BTreeMap, env};

    fn check_cargo_instructions(cfg_map: &BTreeMap<VergenKey, Option<String>>) {
        assert_eq!(
            &get_map_value(VergenKey::CargoTargetTriple, cfg_map),
            "x86_64-unknown-linux-gnu"
        );
        assert_eq!(&get_map_value(VergenKey::CargoProfile, cfg_map), "debug");
    }

    fn check_cargo_keys(cfg_map: &BTreeMap<VergenKey, Option<String>>) {
        let mut count = 0;
        for (k, v) in cfg_map {
            match *k {
                VergenKey::CargoTargetTriple
                | VergenKey::CargoProfile
                | VergenKey::CargoFeatures => {
                    assert!(v.is_some());
                    count += 1;
                }
                _ => {}
            }
        }
        assert_eq!(count, 3);
    }

    #[test]
    #[serial_test::serial]
    fn add_cargo_config_works() {
        setup();
        let mut config = Config::default();
        add_cargo_config(ConstantsFlags::all(), &mut config);
        check_cargo_keys(config.cfg_map());
        check_cargo_instructions(config.cfg_map());
        teardown();
    }

    #[test]
    #[serial_test::serial]
    fn default_feature_works() {
        setup();
        env::remove_var("CARGO_FEATURE_GIT");
        env::remove_var("CARGO_FEATURE_BUILD");
        let mut config = Config::default();
        add_cargo_config(ConstantsFlags::all(), &mut config);
        check_cargo_keys(config.cfg_map());
        check_cargo_instructions(config.cfg_map());
        teardown();
    }
}

#[cfg(all(test, not(feature = "cargo")))]
mod test {
    use super::add_cargo_config;
    use crate::{
        config::{Config, VergenKey},
        constants::ConstantsFlags,
    };
    use std::collections::BTreeMap;

    fn check_cargo_keys(cfg_map: &BTreeMap<VergenKey, Option<String>>) {
        let mut count = 0;
        for (k, v) in cfg_map {
            match *k {
                VergenKey::CargoTargetTriple | VergenKey::CargoProfile => {
                    assert!(v.is_none());
                    count += 1;
                }
                _ => {}
            }
        }
        assert_eq!(count, 2);
    }

    #[test]
    fn add_cargo_config_works() {
        let mut config = Config::default();
        add_cargo_config(ConstantsFlags::all(), &mut config);
        check_cargo_keys(config.cfg_map());
    }
}
