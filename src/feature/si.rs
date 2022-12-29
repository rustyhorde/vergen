// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use crate::{
    builder::{Builder, RustcEnvMap},
    constants::VERGEN_IDEMPOTENT_DEFAULT,
    key::VergenKey,
};
use anyhow::{Error, Result};
use sysinfo::{CpuExt, System, SystemExt};
#[cfg(not(target_os = "macos"))]
use sysinfo::{Process, User};

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Config {
    pub(crate) si_name: bool,
    pub(crate) si_os_version: bool,
    pub(crate) si_user: bool,
    pub(crate) si_memory: bool,
    pub(crate) si_cpu_vendor: bool,
    pub(crate) si_cpu_core_count: bool,
    pub(crate) si_cpu_name: bool,
    pub(crate) si_cpu_brand: bool,
    pub(crate) si_cpu_frequency: bool,
}

impl Config {
    #[cfg(test)]
    fn enable_all(&mut self) {
        self.si_name = true;
        self.si_os_version = true;
        self.si_user = true;
        self.si_memory = true;
        self.si_cpu_vendor = true;
        self.si_cpu_core_count = true;
        self.si_cpu_name = true;
        self.si_cpu_brand = true;
        self.si_cpu_frequency = true;
    }

    pub(crate) fn add_warnings(
        self,
        skip_if_error: bool,
        e: Error,
        warnings: &mut Vec<String>,
    ) -> Result<()> {
        if skip_if_error {
            if self.si_name {
                warnings.push(format!(
                    "Unable to add {} to output",
                    VergenKey::SysinfoName.name()
                ));
            }
            if self.si_os_version {
                warnings.push(format!(
                    "Unable to add {} to output",
                    VergenKey::SysinfoOsVersion.name()
                ));
            }
            if self.si_user {
                warnings.push(format!(
                    "Unable to add {} to output",
                    VergenKey::SysinfoUser.name()
                ));
            }
            if self.si_memory {
                warnings.push(format!(
                    "Unable to add {} to output",
                    VergenKey::SysinfoMemory.name()
                ));
            }
            if self.si_cpu_vendor {
                warnings.push(format!(
                    "Unable to add {} to output",
                    VergenKey::SysinfoCpuVendor.name()
                ));
            }
            if self.si_cpu_core_count {
                warnings.push(format!(
                    "Unable to add {} to output",
                    VergenKey::SysinfoCpuCoreCount.name()
                ));
            }
            if self.si_cpu_name {
                warnings.push(format!(
                    "Unable to add {} to output",
                    VergenKey::SysinfoCpuName.name()
                ));
            }
            if self.si_cpu_brand {
                warnings.push(format!(
                    "Unable to add {} to output",
                    VergenKey::SysinfoCpuBrand.name()
                ));
            }
            if self.si_cpu_frequency {
                warnings.push(format!(
                    "Unable to add {} to output",
                    VergenKey::SysinfoCpuFrequency.name()
                ));
            }
            Ok(())
        } else {
            Err(e)
        }
    }
}

/// The `VERGEN_SYSINFO_*` configuration features
///
/// | Variable | Sample |
/// | -------  | ------ |
/// | `VERGEN_SYSINFO_NAME` | Manjaro Linux |
/// | `VERGEN_SYSINFO_OS_VERSION` | Linux  Manjaro Linux |
/// | `VERGEN_SYSINFO_USER` | Yoda |
/// | `VERGEN_SYSINFO_TOTAL_MEMORY` | 33 GB |
/// | `VERGEN_SYSINFO_CPU_VENDOR` | Authentic AMD |
/// | `VERGEN_SYSINFO_CPU_CORE_COUNT` | 8 |
/// | `VERGEN_SYSINFO_CPU_NAME` | cpu0,cpu1,cpu2,cpu3,cpu4,cpu5,cpu6,cpu7 |
/// | `VERGEN_SYSINFO_CPU_BRAND` | AMD Ryzen Threadripper 1900X 8-Core Processor |
/// | `VERGEN_SYSINFO_CPU_FREQUENCY` | 3792 |
///
/// # Example
///
/// ```
/// # use anyhow::Result;
/// # use vergen::Vergen;
/// #
/// # fn main() -> Result<()> {
/// Vergen::default().all_sysinfo().gen()?;
/// #   Ok(())
/// # }
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "si")))]
impl Builder {
    /// Enable all of the `VERGEN_SYSINFO_*` options
    pub fn all_sysinfo(&mut self) -> &mut Self {
        self.sysinfo_name()
            .sysinfo_os_version()
            .sysinfo_user()
            .sysinfo_memory()
            .sysinfo_cpu_vendor()
            .sysinfo_cpu_core_count()
            .sysinfo_cpu_name()
            .sysinfo_cpu_brand()
            .sysinfo_cpu_frequency()
    }

    /// Enable the sysinfo name
    pub fn sysinfo_name(&mut self) -> &mut Self {
        self.sysinfo_config.si_name = true;
        self
    }

    /// Enable the sysinfo OS version
    pub fn sysinfo_os_version(&mut self) -> &mut Self {
        self.sysinfo_config.si_os_version = true;
        self
    }

    /// Enable sysinfo user
    pub fn sysinfo_user(&mut self) -> &mut Self {
        self.sysinfo_config.si_user = true;
        self
    }

    /// Enable sysinfo memory
    pub fn sysinfo_memory(&mut self) -> &mut Self {
        self.sysinfo_config.si_memory = true;
        self
    }

    /// Enable sysinfo cpu vendor
    pub fn sysinfo_cpu_vendor(&mut self) -> &mut Self {
        self.sysinfo_config.si_cpu_vendor = true;
        self
    }

    /// Enable sysinfo cpu core count
    pub fn sysinfo_cpu_core_count(&mut self) -> &mut Self {
        self.sysinfo_config.si_cpu_core_count = true;
        self
    }

    /// Enable sysinfo cpu name
    pub fn sysinfo_cpu_name(&mut self) -> &mut Self {
        self.sysinfo_config.si_cpu_name = true;
        self
    }

    /// Enable sysinfo cpu brand
    pub fn sysinfo_cpu_brand(&mut self) -> &mut Self {
        self.sysinfo_config.si_cpu_brand = true;
        self
    }

    /// Enable sysinfo cpu frequency
    pub fn sysinfo_cpu_frequency(&mut self) -> &mut Self {
        self.sysinfo_config.si_cpu_frequency = true;
        self
    }

    pub(crate) fn add_sysinfo_map_entries(
        &self,
        idempotent: bool,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
    ) -> Result<()> {
        let system = setup_system();

        if self.sysinfo_config.si_name {
            if idempotent {
                add_idempotent_entry(VergenKey::SysinfoName, map, warnings);
            } else {
                let _old = map.insert(
                    VergenKey::SysinfoName,
                    system
                        .name()
                        .unwrap_or(VERGEN_IDEMPOTENT_DEFAULT.to_string()),
                );
            }
        }

        if self.sysinfo_config.si_os_version {
            if idempotent {
                add_idempotent_entry(VergenKey::SysinfoOsVersion, map, warnings);
            } else {
                let _old = map.insert(
                    VergenKey::SysinfoOsVersion,
                    system
                        .long_os_version()
                        .unwrap_or(VERGEN_IDEMPOTENT_DEFAULT.to_string()),
                );
            }
        }

        if self.sysinfo_config.si_user {
            if idempotent {
                add_idempotent_entry(VergenKey::SysinfoUser, map, warnings);
            } else {
                add_user_entry(&system, map)?;
            }
        }

        if self.sysinfo_config.si_memory {
            if idempotent {
                add_idempotent_entry(VergenKey::SysinfoMemory, map, warnings);
            } else {
                let _old = map.insert(
                    VergenKey::SysinfoMemory,
                    format!("{}", suffix(system.total_memory())),
                );
            }
        }

        if self.sysinfo_config.si_cpu_vendor {
            if idempotent {
                add_idempotent_entry(VergenKey::SysinfoCpuVendor, map, warnings);
            } else {
                let _old = map.insert(
                    VergenKey::SysinfoCpuVendor,
                    system
                        .cpus()
                        .get(0)
                        .map(|proc| proc.vendor_id().to_string())
                        .unwrap_or(VERGEN_IDEMPOTENT_DEFAULT.to_string()),
                );
            }
        }

        if self.sysinfo_config.si_cpu_core_count {
            if idempotent {
                add_idempotent_entry(VergenKey::SysinfoCpuCoreCount, map, warnings);
            } else {
                let _old = map.insert(
                    VergenKey::SysinfoCpuCoreCount,
                    system
                        .physical_core_count()
                        .as_ref()
                        .map(usize::to_string)
                        .unwrap_or(VERGEN_IDEMPOTENT_DEFAULT.to_string()),
                );
            }
        }

        if self.sysinfo_config.si_cpu_name {
            if idempotent {
                add_idempotent_entry(VergenKey::SysinfoCpuName, map, warnings);
            } else {
                let _old = map.insert(
                    VergenKey::SysinfoCpuName,
                    system
                        .cpus()
                        .iter()
                        .map(CpuExt::name)
                        .collect::<Vec<&str>>()
                        .join(","),
                );
            }
        }

        if self.sysinfo_config.si_cpu_brand {
            if idempotent {
                add_idempotent_entry(VergenKey::SysinfoCpuBrand, map, warnings);
            } else {
                let _old = map.insert(
                    VergenKey::SysinfoCpuBrand,
                    system
                        .cpus()
                        .get(0)
                        .map(|processor| processor.brand().to_string())
                        .unwrap_or(VERGEN_IDEMPOTENT_DEFAULT.to_string()),
                );
            }
        }

        if self.sysinfo_config.si_cpu_frequency {
            if idempotent {
                add_idempotent_entry(VergenKey::SysinfoCpuFrequency, map, warnings);
            } else {
                let _old = map.insert(
                    VergenKey::SysinfoCpuFrequency,
                    system
                        .cpus()
                        .get(0)
                        .map(|proc| proc.frequency().to_string())
                        .unwrap_or(VERGEN_IDEMPOTENT_DEFAULT.to_string()),
                );
            }
        }
        Ok(())
    }
}

#[cfg(not(target_os = "macos"))]
fn setup_system() -> System {
    let mut system = System::new_all();
    system.refresh_all();
    system
}

#[cfg(target_os = "macos")]
fn setup_system() -> System {
    let mut system = System::new();
    system.refresh_memory();
    system.refresh_cpu();
    system
}

#[cfg(not(target_os = "macos"))]
fn add_user_entry(system: &System, map: &mut RustcEnvMap) -> Result<()> {
    use anyhow::anyhow;
    use sysinfo::{get_current_pid, UserExt};

    let pid = get_current_pid().map_err(|e| anyhow!("{e}"))?;
    if let Some(process) = system.process(pid) {
        for user in system.users() {
            if check_user(process, user) {
                let _old = map.insert(VergenKey::SysinfoUser, user.name().to_string());
            }
        }
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn add_user_entry(_system: &System, _map: &mut RustcEnvMap) -> Result<()> {
    Ok(())
}

#[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
fn check_user(process: &Process, user: &User) -> bool {
    use sysinfo::{ProcessExt, UserExt};

    Some(user.id()) == process.user_id()
}

#[cfg(target_os = "windows")]
fn check_user(_process: &Process, _user: &User) -> bool {
    false
}

fn suffix(mut curr_memory: u64) -> String {
    let mut count = 0;

    while curr_memory >= 1024 {
        curr_memory /= 1024;
        count += 1;
    }
    format!(
        "{curr_memory} {}",
        match count {
            0 => "B",
            1 => "KiB",
            2 => "MiB",
            3 => "GiB",
            4 => "TiB",
            5 => "PiB",
            // This is the highest we can reach
            // at u64::MAX
            _ => "EiB",
        }
    )
}

fn add_idempotent_entry(key: VergenKey, map: &mut RustcEnvMap, warnings: &mut Vec<String>) {
    let _old = map.insert(key, VERGEN_IDEMPOTENT_DEFAULT.to_string());
    warnings.push(format!("{} set to idempotent default", key.name()));
}

#[cfg(test)]
mod test {
    use super::{suffix, Config};
    use crate::{builder::test::count_idempotent, Vergen};
    use anyhow::{anyhow, Result};

    #[cfg(any(target_os = "macos", target_os = "windows"))]
    const SYSINFO_COUNT: usize = 8;
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    const SYSINFO_COUNT: usize = 9;

    #[test]
    fn add_warnings_is_err() -> Result<()> {
        let config = Config::default();
        let mut warnings = vec![];
        assert!(config
            .add_warnings(false, anyhow!("test"), &mut warnings)
            .is_err());
        Ok(())
    }

    #[test]
    fn add_warnings_adds_warnings() -> Result<()> {
        let mut config = Config::default();
        config.enable_all();

        let mut warnings = vec![];
        assert!(config
            .add_warnings(true, anyhow!("test"), &mut warnings)
            .is_ok());
        assert_eq!(9, warnings.len());
        Ok(())
    }

    #[test]
    #[serial_test::parallel]
    fn sysinfo_all_idempotent() -> Result<()> {
        let config = Vergen::default().idempotent().all_sysinfo().test_gen()?;
        assert_eq!(9, config.cargo_rustc_env_map.len());
        assert_eq!(9, count_idempotent(config.cargo_rustc_env_map));
        assert_eq!(9, config.warnings.len());
        Ok(())
    }

    #[test]
    #[serial_test::parallel]
    fn sysinfo_all() -> Result<()> {
        let config = Vergen::default().all_sysinfo().test_gen()?;
        assert_eq!(SYSINFO_COUNT, config.cargo_rustc_env_map.len());
        assert_eq!(0, count_idempotent(config.cargo_rustc_env_map));
        assert_eq!(0, config.warnings.len());
        Ok(())
    }

    #[test]
    fn suffix_works() {
        assert_eq!(suffix(1023), "1023 B");
        assert_eq!(suffix(1024), "1 KiB");
        assert_eq!(suffix(1_048_575), "1023 KiB");
        assert_eq!(suffix(1_048_576), "1 MiB");
        assert_eq!(suffix(1_073_741_823), "1023 MiB");
        assert_eq!(suffix(1_073_741_824), "1 GiB");
        assert_eq!(suffix(1_099_511_627_775), "1023 GiB");
        assert_eq!(suffix(1_099_511_627_776), "1 TiB");
        assert_eq!(suffix(1_125_899_906_842_623), "1023 TiB");
        assert_eq!(suffix(1_125_899_906_842_624), "1 PiB");
        assert_eq!(suffix((1_125_899_906_842_624 * 1024) - 1), "1023 PiB");
        assert_eq!(suffix(1_125_899_906_842_624 * 1024), "1 EiB");
        assert_eq!(suffix(u64::MAX), "15 EiB");
    }
}
