// Copyright (c) 2016, 2018, 2021 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `vergen` rustc feature implementation

use crate::{config::Config, constants::ConstantsFlags, error::Result};
#[cfg(feature = "rustc")]
use {
    crate::{config::VergenKey, feature::add_entry},
    rustc_version::{version_meta, Channel},
};

#[cfg(feature = "rustc")]
pub(crate) fn add_rustc_config(flags: ConstantsFlags, config: &mut Config) -> Result<()> {
    if flags.intersects(
        ConstantsFlags::RUSTC_CHANNEL
            | ConstantsFlags::RUSTC_HOST_TRIPLE
            | ConstantsFlags::RUSTC_SEMVER
            | ConstantsFlags::RUSTC_COMMIT_HASH
            | ConstantsFlags::RUSTC_COMMIT_DATE
            | ConstantsFlags::RUSTC_LLVM_VERSION,
    ) {
        let rustc = version_meta()?;

        if flags.contains(ConstantsFlags::RUSTC_CHANNEL) {
            add_entry(
                config.cfg_map_mut(),
                VergenKey::RustcChannel,
                Some(
                    match rustc.channel {
                        Channel::Dev => "dev",
                        Channel::Nightly => "nightly",
                        Channel::Beta => "beta",
                        Channel::Stable => "stable",
                    }
                    .to_string(),
                ),
            );
        }

        if flags.contains(ConstantsFlags::RUSTC_HOST_TRIPLE) {
            add_entry(
                config.cfg_map_mut(),
                VergenKey::RustcHostTriple,
                Some(rustc.host),
            );
        }

        if flags.contains(ConstantsFlags::RUSTC_SEMVER) {
            add_entry(
                config.cfg_map_mut(),
                VergenKey::RustcSemver,
                Some(format!("{}", rustc.semver)),
            );
        }

        if flags.contains(ConstantsFlags::RUSTC_COMMIT_HASH) {
            add_entry(
                config.cfg_map_mut(),
                VergenKey::RustcCommitHash,
                rustc.commit_hash,
            );
        }

        if flags.contains(ConstantsFlags::RUSTC_COMMIT_DATE) {
            add_entry(
                config.cfg_map_mut(),
                VergenKey::RustcCommitDate,
                rustc.commit_date,
            );
        }

        if flags.contains(ConstantsFlags::RUSTC_LLVM_VERSION) {
            if let Some(llvmver) = rustc.llvm_version {
                add_entry(
                    config.cfg_map_mut(),
                    VergenKey::RustcLlvmVersion,
                    Some(format!("{}", llvmver)),
                );
            }
        }
    }
    Ok(())
}

#[cfg(not(feature = "rustc"))]
pub(crate) fn add_rustc_config(_flags: ConstantsFlags, _config: &mut Config) -> Result<()> {
    Ok(())
}

#[cfg(all(test, feature = "rustc"))]
mod test {
    use super::add_rustc_config;
    use crate::{
        config::{Config, VergenKey},
        constants::ConstantsFlags,
        error::Result,
        test::get_map_value,
    };
    use lazy_static::lazy_static;
    use regex::Regex;
    use std::collections::BTreeMap;

    lazy_static! {
        static ref DATE_REGEX: Regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
        static ref SHA_REGEX: Regex = Regex::new(r"^[0-9a-f]{40}$").unwrap();
        static ref SEMVER_REGEX: Regex = Regex::new(r"^(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$").unwrap();
    }

    fn check_rustc_instructions(cfg_map: &BTreeMap<VergenKey, Option<String>>) {
        assert!(SHA_REGEX.is_match(&get_map_value(VergenKey::RustcCommitHash, cfg_map)));
        assert!(DATE_REGEX.is_match(&get_map_value(VergenKey::RustcCommitDate, cfg_map)));
        assert!(SEMVER_REGEX.is_match(&get_map_value(VergenKey::RustcSemver, cfg_map)));
    }

    #[rustversion::nightly]
    fn check_rustc_keys(cfg_map: &BTreeMap<VergenKey, Option<String>>) {
        let mut count = 0;
        for (k, v) in cfg_map {
            match *k {
                VergenKey::RustcHostTriple
                | VergenKey::RustcChannel
                | VergenKey::RustcSemver
                | VergenKey::RustcCommitDate
                | VergenKey::RustcCommitHash
                | VergenKey::RustcLlvmVersion => {
                    assert!(v.is_some());
                    count += 1;
                }
                _ => {}
            }
        }
        assert_eq!(count, 6);
    }

    #[rustversion::any(beta, stable)]
    fn check_rustc_keys(cfg_map: &BTreeMap<VergenKey, Option<String>>) {
        let mut count = 0;
        for (k, v) in cfg_map {
            match *k {
                VergenKey::RustcHostTriple
                | VergenKey::RustcChannel
                | VergenKey::RustcSemver
                | VergenKey::RustcCommitDate
                | VergenKey::RustcCommitHash => {
                    assert!(v.is_some());
                    count += 1;
                }
                _ => {}
            }
        }
        assert_eq!(count, 5);
    }

    #[test]
    fn add_rustc_config_works() -> Result<()> {
        let mut config = Config::default();
        add_rustc_config(ConstantsFlags::all(), &mut config)?;
        check_rustc_keys(config.cfg_map());
        check_rustc_instructions(config.cfg_map());
        Ok(())
    }
}

#[cfg(all(test, not(feature = "rustc")))]
mod test {
    use super::add_rustc_config;
    use crate::{
        config::{Config, VergenKey},
        constants::ConstantsFlags,
        error::Result,
    };
    use std::collections::BTreeMap;

    fn check_rustc_keys(cfg_map: &BTreeMap<VergenKey, Option<String>>) {
        let mut count = 0;
        for (k, v) in cfg_map {
            match *k {
                VergenKey::RustcHostTriple | VergenKey::RustcChannel | VergenKey::RustcSemver => {
                    assert!(v.is_none());
                    count += 1;
                }
                _ => {}
            }
        }
        assert_eq!(count, 3);
    }

    #[test]
    fn add_rustc_config_works() -> Result<()> {
        let mut config = Config::default();
        add_rustc_config(ConstantsFlags::all(), &mut config)?;
        check_rustc_keys(config.cfg_map());
        Ok(())
    }
}
