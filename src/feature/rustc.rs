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
    crate::{feature::add_entry, output::VergenKey},
    rustc_version::{version_meta, Channel},
};

#[cfg(feature = "rustc")]
pub(crate) fn add_rustc_config(flags: ConstantsFlags, config: &mut Config) -> Result<()> {
    if flags.intersects(
        ConstantsFlags::RUSTC_CHANNEL
            | ConstantsFlags::RUSTC_HOST_TRIPLE
            | ConstantsFlags::RUSTC_SEMVER,
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
                VergenKey::HostTriple,
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
    use crate::{config::Config, constants::ConstantsFlags, error::Result, output::VergenKey};
    use std::collections::HashMap;

    fn check_rustc_keys(cfg_map: &HashMap<VergenKey, Option<String>>) {
        let mut count = 0;
        for (k, v) in cfg_map {
            match *k {
                VergenKey::HostTriple | VergenKey::RustcChannel | VergenKey::RustcSemver => {
                    assert!(v.is_some());
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

#[cfg(all(test, not(feature = "rustc")))]
mod test {
    use super::add_rustc_config;
    use crate::{config::Config, constants::ConstantsFlags, error::Result, output::VergenKey};
    use std::collections::HashMap;

    fn check_rustc_keys(cfg_map: &HashMap<VergenKey, Option<String>>) {
        let mut count = 0;
        for (k, v) in cfg_map {
            match *k {
                VergenKey::HostTriple | VergenKey::RustcChannel | VergenKey::RustcSemver => {
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
