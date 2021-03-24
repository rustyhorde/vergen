// Copyright (c) 2016, 2018, 2021 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `vergen` sysinfo feature implementation

use crate::config::{Config, Instructions};
#[cfg(feature = "si")]
use {
    crate::{config::VergenKey, feature::add_entry},
    getset::{Getters, MutGetters},
    sysinfo::{System, SystemExt},
};

/// Configuration for the `VERGEN_SYSINFO_*` instructions
///
/// # Instructions
/// The following instructions can be generated:
///
/// | Instruction | Default |
/// | ----------- | :-----: |
/// | `cargo:rustc-env=VERGEN_SYSINFO_NAME=nightly` | * |
/// | `cargo:rustc-env=VERGEN_SYSINFO_OS_VERSION=2021-02-10` | * |
///
/// * If the `channel` field is false, the `VERGEN_RUSTC_CHANNEL` instruction will not be generated.
/// * If the `commit_date` field is false, the `VERGEN_RUSTC_COMMIT_DATE` instruction will not be generated.
///
/// # Example
///
/// ```
/// # use anyhow::Result;
/// use vergen::{vergen, Config};
///
/// # pub fn main() -> Result<()> {
/// let mut config = Config::default();
#[cfg_attr(
    feature = "si",
    doc = r##"
// Turn off the name instruction
*config.sysinfo_mut().name_mut() = false;

// Generate the instructions
vergen(config)?;
"##
)]
/// # Ok(())
/// # }
#[cfg(feature = "si")]
#[derive(Clone, Copy, Debug, Getters, MutGetters)]
#[getset(get = "pub(crate)", get_mut = "pub")]
pub struct Sysinfo {
    /// Enable/Disable the `VERGEN_RUSTC_CHANNEL` instruction
    name: bool,
    /// Enable/Disable the `VERGEN_RUSTC_COMMIT_DATE` instruction
    os_version: bool,
}

#[cfg(feature = "si")]
impl Default for Sysinfo {
    fn default() -> Self {
        Self {
            name: true,
            os_version: true,
        }
    }
}

#[cfg(feature = "si")]
impl Sysinfo {
    pub(crate) fn has_enabled(self) -> bool {
        self.name || self.os_version
    }
}

#[cfg(feature = "si")]
pub(crate) fn configure_sysinfo(instructions: Instructions, config: &mut Config) {
    let sysinfo_config = instructions.sysinfo();
    if sysinfo_config.has_enabled() {
        let mut system = System::new_all();
        // First we update all information of our system struct.
        system.refresh_all();

        if *sysinfo_config.name() {
            add_entry(
                config.cfg_map_mut(),
                VergenKey::SysinfoName,
                system.get_name(),
            );
        }

        if *sysinfo_config.os_version() {
            add_entry(
                config.cfg_map_mut(),
                VergenKey::SysinfoOsVersion,
                system.get_long_os_version(),
            );
        }
    }
}

#[cfg(not(feature = "si"))]
pub(crate) fn configure_sysinfo(_instructions: Instructions, _config: &mut Config) {}

#[cfg(all(test, feature = "si"))]
mod test {
    use crate::config::Instructions;

    #[test]
    fn rustc_config() {
        let mut config = Instructions::default();
        assert!(config.sysinfo().name);
        assert!(config.sysinfo().os_version);
        config.sysinfo_mut().os_version = false;
        assert!(!config.sysinfo().os_version);
    }
}

#[cfg(all(test, not(feature = "si")))]
mod test {}
