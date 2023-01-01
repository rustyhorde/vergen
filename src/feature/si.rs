// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use crate::{
    emitter::{EmitBuilder, RustcEnvMap},
    key::VergenKey,
    utils::fns::{add_default_map_entry, add_map_entry},
};
#[cfg(not(target_os = "macos"))]
use anyhow::{anyhow, Result};
use sysinfo::{CpuExt, System, SystemExt};
#[cfg(not(target_os = "macos"))]
use sysinfo::{Pid, Process, User};

#[derive(Clone, Copy, Debug, Default)]
#[allow(clippy::struct_excessive_bools)]
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
    #[cfg(test)]
    fail_pid: bool,
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
/// # use vergen::EmitBuilder;
/// #
/// # fn main() -> Result<()> {
/// EmitBuilder::builder().all_sysinfo().emit()?;
/// #   Ok(())
/// # }
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "si")))]
impl EmitBuilder {
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
    ) {
        let system = setup_system();

        if self.sysinfo_config.si_name {
            add_sysinfo_map_entry(
                VergenKey::SysinfoName,
                idempotent,
                system.name(),
                map,
                warnings,
            );
        }

        if self.sysinfo_config.si_os_version {
            add_sysinfo_map_entry(
                VergenKey::SysinfoOsVersion,
                idempotent,
                system.long_os_version(),
                map,
                warnings,
            );
        }

        if self.sysinfo_config.si_user {
            add_sysinfo_map_entry(
                VergenKey::SysinfoUser,
                idempotent,
                self.get_user(&system),
                map,
                warnings,
            );
        }

        if self.sysinfo_config.si_memory {
            add_sysinfo_map_entry(
                VergenKey::SysinfoMemory,
                idempotent,
                Some(suffix(system.total_memory())),
                map,
                warnings,
            );
        }

        if self.sysinfo_config.si_cpu_vendor {
            add_sysinfo_map_entry(
                VergenKey::SysinfoCpuVendor,
                idempotent,
                system
                    .cpus()
                    .get(0)
                    .map(|proc| proc.vendor_id().to_string()),
                map,
                warnings,
            );
        }

        if self.sysinfo_config.si_cpu_core_count {
            add_sysinfo_map_entry(
                VergenKey::SysinfoCpuCoreCount,
                idempotent,
                system.physical_core_count().as_ref().map(usize::to_string),
                map,
                warnings,
            );
        }

        if self.sysinfo_config.si_cpu_name {
            add_sysinfo_map_entry(
                VergenKey::SysinfoCpuName,
                idempotent,
                Some(
                    system
                        .cpus()
                        .iter()
                        .map(CpuExt::name)
                        .collect::<Vec<&str>>()
                        .join(","),
                ),
                map,
                warnings,
            );
        }

        if self.sysinfo_config.si_cpu_brand {
            add_sysinfo_map_entry(
                VergenKey::SysinfoCpuBrand,
                idempotent,
                system
                    .cpus()
                    .get(0)
                    .map(|processor| processor.brand().to_string()),
                map,
                warnings,
            );
        }

        if self.sysinfo_config.si_cpu_frequency {
            add_sysinfo_map_entry(
                VergenKey::SysinfoCpuFrequency,
                idempotent,
                system
                    .cpus()
                    .get(0)
                    .map(|proc| proc.frequency().to_string()),
                map,
                warnings,
            );
        }
    }

    #[cfg(target_os = "macos")]
    #[allow(clippy::unused_self)]
    fn get_user(&self, _system: &System) -> Option<String> {
        None
    }

    #[cfg(not(target_os = "macos"))]
    fn get_user(&self, system: &System) -> Option<String> {
        use sysinfo::UserExt;

        if let Ok(pid) = self.get_pid() {
            if let Some(process) = system.process(pid) {
                for user in system.users() {
                    if check_user(process, user) {
                        return Some(user.name().to_string());
                    }
                }
            }
        }
        None
    }
    #[cfg(all(not(test), not(target_os = "macos")))]
    #[allow(clippy::unused_self)]
    fn get_pid(&self) -> Result<Pid> {
        use sysinfo::get_current_pid;

        get_current_pid().map_err(|e| anyhow!(format!("{e}")))
    }

    #[cfg(all(test, not(target_os = "macos")))]
    fn get_pid(&self) -> Result<Pid> {
        use sysinfo::get_current_pid;

        if self.sysinfo_config.fail_pid {
            Err(anyhow!("unable to determine pid"))
        } else {
            get_current_pid().map_err(|e| anyhow!(format!("{e}")))
        }
    }
}

fn add_sysinfo_map_entry(
    key: VergenKey,
    idempotent: bool,
    value: Option<String>,
    map: &mut RustcEnvMap,
    warnings: &mut Vec<String>,
) {
    if idempotent {
        add_default_map_entry(key, map, warnings);
    } else if let Some(val) = value {
        add_map_entry(key, val, map);
    } else {
        add_default_map_entry(key, map, warnings);
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

#[cfg(test)]
mod test {
    use super::{add_sysinfo_map_entry, suffix};
    use crate::{emitter::test::count_idempotent, key::VergenKey, EmitBuilder};
    use anyhow::Result;
    use std::collections::BTreeMap;

    #[cfg(any(target_os = "macos", target_os = "windows"))]
    const IDEM_COUNT: usize = 1;
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    const IDEM_COUNT: usize = 0;
    const SYSINFO_COUNT: usize = 9;

    #[test]
    #[serial_test::parallel]
    fn sysinfo_all_idempotent() -> Result<()> {
        let config = EmitBuilder::builder()
            .idempotent()
            .all_sysinfo()
            .test_emit()?;
        assert_eq!(SYSINFO_COUNT, config.cargo_rustc_env_map.len());
        assert_eq!(SYSINFO_COUNT, count_idempotent(&config.cargo_rustc_env_map));
        assert_eq!(SYSINFO_COUNT, config.warnings.len());
        Ok(())
    }

    #[test]
    #[serial_test::parallel]
    fn sysinfo_all() -> Result<()> {
        let config = EmitBuilder::builder().all_sysinfo().test_emit()?;
        assert_eq!(SYSINFO_COUNT, config.cargo_rustc_env_map.len());
        assert_eq!(IDEM_COUNT, count_idempotent(&config.cargo_rustc_env_map));
        assert_eq!(IDEM_COUNT, config.warnings.len());
        Ok(())
    }

    #[test]
    #[serial_test::parallel]
    fn adding_none_defaults() -> Result<()> {
        let mut map = BTreeMap::new();
        let mut warnings = vec![];
        add_sysinfo_map_entry(
            VergenKey::SysinfoCpuBrand,
            false,
            None,
            &mut map,
            &mut warnings,
        );
        Ok(())
    }

    #[test]
    #[serial_test::parallel]
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

    #[test]
    #[serial_test::parallel]
    fn pid_lookup_fails() -> Result<()> {
        let mut config = EmitBuilder::builder();
        let _ = config.all_sysinfo();
        config.sysinfo_config.fail_pid = true;
        let emitter = config.test_emit()?;
        assert_eq!(SYSINFO_COUNT, emitter.cargo_rustc_env_map.len());
        assert_eq!(1, count_idempotent(&emitter.cargo_rustc_env_map));
        assert_eq!(1, emitter.warnings.len());
        Ok(())
    }
}
