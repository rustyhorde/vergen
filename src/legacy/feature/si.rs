// Copyright (c) 2016, 2018, 2021 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `vergen` sysinfo feature implementation

use crate::config::{Config, Instructions};
use anyhow::Result;
#[cfg(all(feature = "si", not(target_os = "macos")))]
use {
    crate::error::Error::Pid,
    sysinfo::{get_current_pid, Process, User, UserExt},
};
#[cfg(feature = "si")]
use {
    crate::{config::VergenKey, feature::add_entry},
    getset::{Getters, MutGetters},
    sysinfo::{CpuExt, System, SystemExt},
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
/// * **NOTE** - To keep processing other sections if an Error occurs in this one, set
///     [`Sysinfo::skip_if_error`](Sysinfo::skip_if_error_mut()) to true.
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
#[allow(clippy::struct_excessive_bools)]
#[deprecated(
    since = "8.0.0",
    note = "This has been replaced with [`vergen::VergenBuilder`]"
)]
pub struct Sysinfo {
    /// Enable/Disable the sysinfo output
    enabled: bool,
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
    /// Enable/Disable the `VERGEN_SYSINFO_CPU_NAME` instruction
    cpu_name: bool,
    /// Enable/Disable the `VERGEN_SYSINFO_CPU_BRAND` instruction
    cpu_brand: bool,
    /// Enable/Disable the `VERGEN_SYSINFO_CPU_FREQUENCY` instruction
    cpu_frequency: bool,
    /// Enable/Disable skipping [`Sysinfo`] if an Error occurs.
    /// Use [`option_env!`](std::option_env!) to read the generated environment variables.
    skip_if_error: bool,
}

#[cfg(feature = "si")]
impl Default for Sysinfo {
    fn default() -> Self {
        Self {
            enabled: true,
            name: true,
            os_version: true,
            user: true,
            memory: true,
            cpu_vendor: true,
            cpu_core_count: true,
            cpu_name: true,
            cpu_brand: true,
            cpu_frequency: true,
            skip_if_error: false,
        }
    }
}

#[cfg(feature = "si")]
impl Sysinfo {
    pub(crate) fn has_enabled(self) -> bool {
        self.enabled
            && (self.name
                || self.os_version
                || self.user
                || self.memory
                || self.cpu_vendor
                || self.cpu_core_count
                || self.cpu_name
                || self.cpu_brand
                || self.cpu_frequency)
    }
}

#[cfg(all(feature = "si", not(target_os = "macos")))]
fn setup_system() -> System {
    let mut system = System::new_all();
    system.refresh_all();
    system
}

#[cfg(all(feature = "si", target_os = "macos"))]
fn setup_system() -> System {
    let mut system = System::new();
    system.refresh_memory();
    system.refresh_cpu();
    system
}

#[cfg(feature = "si")]
#[allow(clippy::unnecessary_wraps, clippy::too_many_lines, deprecated)]
pub(crate) fn configure_sysinfo(instructions: &Instructions, config: &mut Config) -> Result<()> {
    let sysinfo_config = instructions.sysinfo();

    let mut add_entries = || {
        let system = setup_system();

        if *sysinfo_config.name() {
            add_entry(config.cfg_map_mut(), VergenKey::SysinfoName, system.name());
        }

        if *sysinfo_config.os_version() {
            add_entry(
                config.cfg_map_mut(),
                VergenKey::SysinfoOsVersion,
                system.long_os_version(),
            );
        }

        if *sysinfo_config.user() {
            cfg_if::cfg_if! {
                if #[cfg(target_os = "macos")] {
                } else {
                    let pid = get_current_pid().map_err(|e| Pid { msg: e })?;
                    if let Some(process) = system.process(pid) {
                        for user in system.users() {
                            if check_user(process, user) {
                                add_entry(
                                    config.cfg_map_mut(),
                                    VergenKey::SysinfoUser,
                                    Some(user.name().to_string()),
                                );
                            }
                        }
                    }
                }
            }
        }

        if *sysinfo_config.memory() {
            let mut curr_memory = system.total_memory();
            let mut count = 0;

            while curr_memory > 1000 {
                curr_memory /= 1000;
                count += 1;
            }

            add_entry(
                config.cfg_map_mut(),
                VergenKey::SysinfoMemory,
                Some(format!("{curr_memory} {}", suffix(count))),
            );
        }

        if *sysinfo_config.cpu_vendor() {
            add_entry(
                config.cfg_map_mut(),
                VergenKey::SysinfoCpuVendor,
                system
                    .cpus()
                    .get(0)
                    .map(|processor| processor.vendor_id().to_string()),
            );
        }

        if *sysinfo_config.cpu_core_count() {
            add_entry(
                config.cfg_map_mut(),
                VergenKey::SysinfoCpuCoreCount,
                system.physical_core_count().map(|x| x.to_string()),
            );
        }

        if *sysinfo_config.cpu_name() {
            add_entry(
                config.cfg_map_mut(),
                VergenKey::SysinfoCpuName,
                Some(
                    system
                        .cpus()
                        .iter()
                        .map(CpuExt::name)
                        .collect::<Vec<&str>>()
                        .join(","),
                ),
            );
        }

        if *sysinfo_config.cpu_brand() {
            add_entry(
                config.cfg_map_mut(),
                VergenKey::SysinfoCpuBrand,
                system
                    .cpus()
                    .get(0)
                    .map(|processor| processor.brand().to_string()),
            );
        }

        if *sysinfo_config.cpu_frequency() {
            add_entry(
                config.cfg_map_mut(),
                VergenKey::SysinfoCpuFrequency,
                system
                    .cpus()
                    .get(0)
                    .map(|processor| processor.frequency().to_string()),
            );
        }
        Ok(())
    };

    if sysinfo_config.has_enabled() {
        if sysinfo_config.skip_if_error {
            // hide errors, but emit a warning
            let result = add_entries();
            if result.is_err() {
                let warning = format!(
                    "An Error occurred during processing of {}. \
                    VERGEN_{}_* may be incomplete.",
                    "Sysinfo", "SYSINFO"
                );
                config.warnings_mut().push(warning);
            }
            Ok(())
        } else {
            add_entries()
        }
    } else {
        Ok(())
    }
}

#[cfg(not(feature = "si"))]
#[allow(
    clippy::unnecessary_wraps,
    clippy::trivially_copy_pass_by_ref,
    deprecated
)]
pub(crate) fn configure_sysinfo(_instructions: &Instructions, _config: &mut Config) -> Result<()> {
    Ok(())
}

#[cfg(all(feature = "si", not(target_os = "windows"), not(target_os = "macos")))]
fn check_user(process: &Process, user: &User) -> bool {
    use sysinfo::ProcessExt;

    Some(user.id()) == process.user_id()
}

#[cfg(all(feature = "si", target_os = "windows"))]
fn check_user(_process: &Process, _user: &User) -> bool {
    false
}

#[cfg(feature = "si")]
fn suffix(val: usize) -> &'static str {
    match val {
        0 => "B",
        1 => "KB",
        2 => "MB",
        3 => "GB",
        4 => "TB",
        _ => "xB",
    }
}

#[cfg(all(test, feature = "si"))]
mod test {
    use super::{suffix, Sysinfo};
    use crate::config::Instructions;

    #[test]
    fn rustc_config() {
        let mut config = Instructions::default();
        assert!(config.sysinfo().name);
        assert!(config.sysinfo().os_version);
        assert!(config.sysinfo().user);
        assert!(config.sysinfo().cpu_vendor);
        assert!(config.sysinfo().cpu_core_count);
        assert!(config.sysinfo().cpu_name);
        assert!(config.sysinfo().cpu_brand);
        assert!(config.sysinfo().cpu_frequency);
        config.sysinfo_mut().os_version = false;
        assert!(!config.sysinfo().os_version);
    }

    #[test]
    fn has_enabled_works() {
        let mut sysinfo = Sysinfo::default();
        assert!(sysinfo.has_enabled());
        *sysinfo.name_mut() = false;
        assert!(sysinfo.has_enabled());
        *sysinfo.os_version_mut() = false;
        assert!(sysinfo.has_enabled());
        *sysinfo.user_mut() = false;
        assert!(sysinfo.has_enabled());
        *sysinfo.memory_mut() = false;
        assert!(sysinfo.has_enabled());
        *sysinfo.cpu_vendor_mut() = false;
        assert!(sysinfo.has_enabled());
        *sysinfo.cpu_core_count_mut() = false;
        assert!(sysinfo.has_enabled());
        *sysinfo.cpu_name_mut() = false;
        assert!(sysinfo.has_enabled());
        *sysinfo.cpu_brand_mut() = false;
        assert!(sysinfo.has_enabled());
        *sysinfo.cpu_frequency_mut() = false;
        assert!(!sysinfo.has_enabled());
    }

    #[test]
    fn suffix_works() {
        assert_eq!("B", suffix(0));
        assert_eq!("KB", suffix(1));
        assert_eq!("MB", suffix(2));
        assert_eq!("GB", suffix(3));
        assert_eq!("TB", suffix(4));
        assert_eq!("xB", suffix(5));
    }

    #[test]
    fn not_enabled() {
        let mut config = Instructions::default();
        *config.sysinfo_mut().enabled_mut() = false;
        assert!(!config.sysinfo().has_enabled());
    }

    #[test]
    fn no_name() {
        let mut config = Instructions::default();
        *config.sysinfo_mut().name_mut() = false;
        assert!(config.sysinfo().has_enabled());
    }

    #[test]
    fn no_os_version() {
        let mut config = Instructions::default();
        *config.sysinfo_mut().name_mut() = false;
        *config.sysinfo_mut().os_version_mut() = false;
        assert!(config.sysinfo().has_enabled());
    }

    #[test]
    fn no_user() {
        let mut config = Instructions::default();
        *config.sysinfo_mut().name_mut() = false;
        *config.sysinfo_mut().os_version_mut() = false;
        *config.sysinfo_mut().user_mut() = false;
        assert!(config.sysinfo().has_enabled());
    }

    #[test]
    fn no_memory() {
        let mut config = Instructions::default();
        *config.sysinfo_mut().name_mut() = false;
        *config.sysinfo_mut().os_version_mut() = false;
        *config.sysinfo_mut().user_mut() = false;
        *config.sysinfo_mut().memory_mut() = false;
        assert!(config.sysinfo().has_enabled());
    }
    #[test]
    fn no_cpu_vendor() {
        let mut config = Instructions::default();
        *config.sysinfo_mut().name_mut() = false;
        *config.sysinfo_mut().os_version_mut() = false;
        *config.sysinfo_mut().user_mut() = false;
        *config.sysinfo_mut().memory_mut() = false;
        *config.sysinfo_mut().cpu_vendor_mut() = false;
        assert!(config.sysinfo().has_enabled());
    }

    #[test]
    fn no_cpu_core_count() {
        let mut config = Instructions::default();
        *config.sysinfo_mut().name_mut() = false;
        *config.sysinfo_mut().os_version_mut() = false;
        *config.sysinfo_mut().user_mut() = false;
        *config.sysinfo_mut().memory_mut() = false;
        *config.sysinfo_mut().cpu_vendor_mut() = false;
        *config.sysinfo_mut().cpu_core_count_mut() = false;
        assert!(config.sysinfo().has_enabled());
    }

    #[test]
    fn no_cpu_name() {
        let mut config = Instructions::default();
        *config.sysinfo_mut().name_mut() = false;
        *config.sysinfo_mut().os_version_mut() = false;
        *config.sysinfo_mut().user_mut() = false;
        *config.sysinfo_mut().memory_mut() = false;
        *config.sysinfo_mut().cpu_vendor_mut() = false;
        *config.sysinfo_mut().cpu_core_count_mut() = false;
        *config.sysinfo_mut().cpu_name_mut() = false;
        assert!(config.sysinfo().has_enabled());
    }

    #[test]
    fn no_cpu_brand() {
        let mut config = Instructions::default();
        *config.sysinfo_mut().name_mut() = false;
        *config.sysinfo_mut().os_version_mut() = false;
        *config.sysinfo_mut().user_mut() = false;
        *config.sysinfo_mut().memory_mut() = false;
        *config.sysinfo_mut().cpu_vendor_mut() = false;
        *config.sysinfo_mut().cpu_core_count_mut() = false;
        *config.sysinfo_mut().cpu_name_mut() = false;
        *config.sysinfo_mut().cpu_brand_mut() = false;
        assert!(config.sysinfo().has_enabled());
    }

    #[test]
    fn nothing() {
        let mut config = Instructions::default();
        *config.sysinfo_mut().name_mut() = false;
        *config.sysinfo_mut().os_version_mut() = false;
        *config.sysinfo_mut().user_mut() = false;
        *config.sysinfo_mut().memory_mut() = false;
        *config.sysinfo_mut().cpu_vendor_mut() = false;
        *config.sysinfo_mut().cpu_core_count_mut() = false;
        *config.sysinfo_mut().cpu_name_mut() = false;
        *config.sysinfo_mut().cpu_brand_mut() = false;
        *config.sysinfo_mut().cpu_frequency_mut() = false;
        assert!(!config.sysinfo().has_enabled());
    }
}

#[cfg(all(test, not(feature = "si")))]
mod test {}
