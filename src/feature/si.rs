// Copyright (c) 2016, 2018, 2021 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `vergen` sysinfo feature implementation

use crate::config::{Config, Instructions};
use anyhow::Result;
#[cfg(feature = "si")]
use {
    crate::{config::VergenKey, error::Error::Pid, feature::add_entry},
    getset::{Getters, MutGetters},
    sysinfo::{get_current_pid, ProcessorExt, System, SystemExt, UserExt},
};

/// Configuration for the `VERGEN_SYSINFO_*` instructions
///
/// # Instructions
/// The following instructions can be generated:
///
/// | Instruction | Default |
/// | ----------- | :-----: |
/// | `cargo:rustc-env=VERGEN_SYSINFO_NAME=Darwin` | * |
/// | `cargo:rustc-env=VERGEN_SYSINFO_OS_VERSION=MacOS 10.15.7 Catalina` | * |
/// | `cargo:rustc-env=VERGEN_SYSINFO_USER=yoda` | * |
/// | `cargo:rustc-env=VERGEN_SYSINFO_TOTAL_MEMORY=16 GB` | * |
/// | `cargo:rustc-env=VERGEN_SYSINFO_CPU_VENDOR=Intel(R) Core(TM) i7-7820HQ CPU @ 2.90GHz` | * |
/// | `cargo:rustc-env=VERGEN_SYSINFO_CPU_CORE_COUNT=4` | * |
///
/// * If the `name` field is false, the `VERGEN_SYSINFO_NAME` instruction will not be generated.
/// * If the `os_version` field is false, the `VERGEN_SYSINFO_OS_VERSION` instruction will not be generated.
/// * If the `user` field is false, the `VERGEN_SYSINFO_USER` instruction will not be generated.
/// * If the `memory` field is false, the `VERGEN_SYSINFO_TOTAL_MEMORY` instruction will not be generated.
/// * If the `cpu_vendor` field is false, the `VERGEN_SYSINFO_CPU_VENDOR` instruction will not be generated.
/// * If the `cpu_core_count` field is false, the `VERGEN_SYSINFO_CPU_CORE_COUNT` instruction will not be generated.
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
    /// Enable/Disable the `VERGEN_SYSINFO_NAME` instruction
    name: bool,
    /// Enable/Disable the `VERGEN_SYSINFO_OS_VERSION` instruction
    os_version: bool,
    /// Enable/Disable the `VERGEN_SYSINFO_USER` instruction
    user: bool,
    /// Enable/Disable the `VERGEN_SYSINFO_TOTAL_MEMORY` instruction
    memory: bool,
    /// Enable/Disable the `VERGEN_SYSINFO_CPU_VENDOR` instruction
    cpu_vendor: bool,
    /// Enable/Disable the `VERGEN_SYSINFO_CPU_CORE_COUNT` instruction
    cpu_core_count: bool,
}

#[cfg(feature = "si")]
impl Default for Sysinfo {
    fn default() -> Self {
        Self {
            name: true,
            os_version: true,
            user: true,
            memory: true,
            cpu_vendor: true,
            cpu_core_count: true,
        }
    }
}

#[cfg(feature = "si")]
impl Sysinfo {
    pub(crate) fn has_enabled(self) -> bool {
        self.name
            || self.os_version
            || self.user
            || self.memory
            || self.cpu_vendor
            || self.cpu_core_count
    }
}

#[cfg(feature = "si")]
pub(crate) fn configure_sysinfo(instructions: Instructions, config: &mut Config) -> Result<()> {
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

        if *sysinfo_config.user() {
            let pid = get_current_pid().map_err(|e| Pid { msg: e })?;
            if let Some(process) = system.get_process(pid) {
                for user in system.get_users() {
                    if *user.get_uid() == process.uid {
                        add_entry(
                            config.cfg_map_mut(),
                            VergenKey::SysinfoUser,
                            Some(user.get_name().to_string()),
                        );
                    }
                }
            }
        }

        if *sysinfo_config.memory() {
            let mut curr_memory = system.get_total_memory();
            let mut count = 0;

            while curr_memory > 1000 {
                curr_memory /= 1000;
                count += 1;
            }

            let suffix = match count {
                0 => "KB",
                1 => "MB",
                2 => "GB",
                _ => "xB",
            };

            add_entry(
                config.cfg_map_mut(),
                VergenKey::SysinfoMemory,
                Some(format!("{} {}", curr_memory, suffix)),
            );
        }

        if *sysinfo_config.cpu_vendor() {
            add_entry(
                config.cfg_map_mut(),
                VergenKey::SysinfoCpuVendor,
                system
                    .get_processors()
                    .get(0)
                    .map(|processor| processor.get_vendor_id().to_string()),
            )
        }

        if *sysinfo_config.cpu_core_count() {
            add_entry(
                config.cfg_map_mut(),
                VergenKey::SysinfoCpuCoreCount,
                system.get_physical_core_count().map(|x| x.to_string()),
            );
        }
    }

    Ok(())
}

#[cfg(not(feature = "si"))]
pub(crate) fn configure_sysinfo(_instructions: Instructions, _config: &mut Config) -> Result<()> {
    Ok(())
}

#[cfg(all(test, feature = "si"))]
mod test {
    use crate::config::Instructions;

    #[test]
    fn rustc_config() {
        let mut config = Instructions::default();
        assert!(config.sysinfo().name);
        assert!(config.sysinfo().os_version);
        assert!(config.sysinfo().user);
        assert!(config.sysinfo().cpu_vendor);
        assert!(config.sysinfo().cpu_core_count);
        config.sysinfo_mut().os_version = false;
        assert!(!config.sysinfo().os_version);
    }
}

#[cfg(all(test, not(feature = "si")))]
mod test {}
