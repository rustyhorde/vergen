// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use crate::{
    constants::{
        SYSINFO_CPU_BRAND, SYSINFO_CPU_CORE_COUNT, SYSINFO_CPU_FREQUENCY, SYSINFO_CPU_NAME,
        SYSINFO_CPU_VENDOR, SYSINFO_MEMORY, SYSINFO_NAME, SYSINFO_OS_VERSION, SYSINFO_USER,
    },
    emitter::{EmitBuilder, RustcEnvMap},
    key::VergenKey,
    utils::fns::{add_default_map_entry, add_map_entry},
};
use anyhow::{anyhow, Result};
use std::env;
use sysinfo::{get_current_pid, Cpu, Pid, Process, RefreshKind, System, User, Users};

#[derive(Clone, Copy, Debug, Default)]
#[allow(clippy::struct_excessive_bools, clippy::struct_field_names)]
pub(crate) struct Config {
    pub(crate) si_refresh_kind: Option<RefreshKind>,
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

impl Config {
    pub(crate) fn any(&self) -> bool {
        self.si_name
            || self.si_os_version
            || self.si_user
            || self.si_memory
            || self.si_cpu_vendor
            || self.si_cpu_core_count
            || self.si_cpu_name
            || self.si_cpu_brand
            || self.si_cpu_frequency
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
/// Emit all sysinfo instructions
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
///
/// Emit some of the sysinfo instructions
///
/// ```
/// # use anyhow::Result;
/// # use vergen::EmitBuilder;
/// #
/// # fn main() -> Result<()> {
/// EmitBuilder::builder()
///     .sysinfo_os_version()
///     .sysinfo_cpu_core_count()
///     .emit()?;
/// #   Ok(())
/// # }
/// ```
///
/// Override output with your own value
///
/// ```
/// # use anyhow::Result;
/// # use std::env;
/// # use vergen::EmitBuilder;
/// #
/// # fn main() -> Result<()> {
/// env::set_var("VERGEN_SYSINFO_NAME", "this is the name I want output");
/// EmitBuilder::builder().all_sysinfo().emit()?;
/// # env::remove_var("VERGEN_SYSINFO_NAME");
/// #   Ok(())
/// # }
/// ```
///
/// # Example
/// This feature also recognizes the idempotent flag.
///
/// ```
/// # use anyhow::Result;
/// # use vergen::EmitBuilder;
/// #
/// # fn main() -> Result<()> {
#[cfg_attr(
    feature = "sysinfo",
    doc = r##"
EmitBuilder::builder().idempotent().all_sysinfo().emit()?;
"##
)]
/// #   Ok(())
/// # }
/// ```
///
/// The above will always generate the following output
///
/// ```text
/// cargo:rustc-env=VERGEN_SYSINFO_NAME=VERGEN_IDEMPOTENT_OUTPUT
/// cargo:rustc-env=VERGEN_SYSINFO_OS_VERSION=VERGEN_IDEMPOTENT_OUTPUT
/// cargo:rustc-env=VERGEN_SYSINFO_USER=VERGEN_IDEMPOTENT_OUTPUT
/// cargo:rustc-env=VERGEN_SYSINFO_TOTAL_MEMORY=VERGEN_IDEMPOTENT_OUTPUT
/// cargo:rustc-env=VERGEN_SYSINFO_CPU_VENDOR=VERGEN_IDEMPOTENT_OUTPUT
/// cargo:rustc-env=VERGEN_SYSINFO_CPU_CORE_COUNT=VERGEN_IDEMPOTENT_OUTPUT
/// cargo:rustc-env=VERGEN_SYSINFO_CPU_NAME=VERGEN_IDEMPOTENT_OUTPUT
/// cargo:rustc-env=VERGEN_SYSINFO_CPU_BRAND=VERGEN_IDEMPOTENT_OUTPUT
/// cargo:rustc-env=VERGEN_SYSINFO_CPU_FREQUENCY=VERGEN_IDEMPOTENT_OUTPUT
/// cargo:warning=VERGEN_SYSINFO_NAME set to default
/// cargo:warning=VERGEN_SYSINFO_OS_VERSION set to default
/// cargo:warning=VERGEN_SYSINFO_USER set to default
/// cargo:warning=VERGEN_SYSINFO_TOTAL_MEMORY set to default
/// cargo:warning=VERGEN_SYSINFO_CPU_VENDOR set to default
/// cargo:warning=VERGEN_SYSINFO_CPU_CORE_COUNT set to default
/// cargo:warning=VERGEN_SYSINFO_CPU_NAME set to default
/// cargo:warning=VERGEN_SYSINFO_CPU_BRAND set to default
/// cargo:warning=VERGEN_SYSINFO_CPU_FREQUENCY set to default
/// cargo:rerun-if-changed=build.rs
/// cargo:rerun-if-env-changed=VERGEN_IDEMPOTENT
/// cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH
/// ```
///
#[cfg_attr(docsrs, doc(cfg(feature = "si")))]
impl EmitBuilder {
    /// Enable all of the `VERGEN_SYSINFO_*` options
    pub fn all_sysinfo(&mut self) -> &mut Self {
        self.sysinfo_refresh_kind(None)
            .sysinfo_name()
            .sysinfo_os_version()
            .sysinfo_user()
            .sysinfo_memory()
            .sysinfo_cpu_vendor()
            .sysinfo_cpu_core_count()
            .sysinfo_cpu_name()
            .sysinfo_cpu_brand()
            .sysinfo_cpu_frequency()
    }

    /// Set the [`RefreshKind`](sysinfo::RefreshKind) to use during sysinfo initialization.
    ///
    /// This allows the user to control at a more fine level what `sysinfo`
    /// will refresh on initialization.
    ///
    /// # Example
    /// ```
    /// # use anyhow::Result;
    /// # #[cfg(feature = "si")]
    /// # use sysinfo::{CpuRefreshKind, RefreshKind};
    /// # use vergen::EmitBuilder;
    /// #
    /// # pub fn main() -> Result<()> {
    #[cfg_attr(
        feature = "si",
        doc = r##"
let refresh_kind = RefreshKind::new();
let cpu_refresh_kind = CpuRefreshKind::everything()
    .without_cpu_usage()
    .without_frequency();
let config = EmitBuilder::builder()
    .sysinfo_refresh_kind(Some(refresh_kind.with_cpu(cpu_refresh_kind)))
    .sysinfo_cpu_brand()
    .emit()?;
"##
    )]
    /// #    Ok(())
    /// # }
    /// ```
    pub fn sysinfo_refresh_kind(&mut self, refresh_kind: Option<RefreshKind>) -> &mut Self {
        self.sysinfo_config.si_refresh_kind = refresh_kind;
        self
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
        if self.sysinfo_config.any() {
            let system = setup_system(self.sysinfo_config.si_refresh_kind);

            self.add_sysinfo_name(&system, idempotent, map, warnings);
            self.add_sysinfo_os_verison(&system, idempotent, map, warnings);
            self.add_sysinfo_user(&system, idempotent, map, warnings);
            self.add_sysinfo_total_memory(&system, idempotent, map, warnings);
            self.add_sysinfo_cpu_vendor(&system, idempotent, map, warnings);
            self.add_sysinfo_cpu_core_count(&system, idempotent, map, warnings);
            self.add_sysinfo_cpu_name(&system, idempotent, map, warnings);
            self.add_sysinfo_cpu_brand(&system, idempotent, map, warnings);
            self.add_sysinfo_cpu_frequency(&system, idempotent, map, warnings);
        }
    }

    fn add_sysinfo_name(
        &self,
        _system: &System,
        idempotent: bool,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
    ) {
        if self.sysinfo_config.si_name {
            if let Ok(value) = env::var(SYSINFO_NAME) {
                add_map_entry(VergenKey::SysinfoName, value, map);
            } else {
                add_sysinfo_map_entry(
                    VergenKey::SysinfoName,
                    idempotent,
                    System::name(),
                    map,
                    warnings,
                );
            }
        }
    }

    fn add_sysinfo_os_verison(
        &self,
        _system: &System,
        idempotent: bool,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
    ) {
        if self.sysinfo_config.si_os_version {
            if let Ok(value) = env::var(SYSINFO_OS_VERSION) {
                add_map_entry(VergenKey::SysinfoOsVersion, value, map);
            } else {
                add_sysinfo_map_entry(
                    VergenKey::SysinfoOsVersion,
                    idempotent,
                    System::long_os_version(),
                    map,
                    warnings,
                );
            }
        }
    }

    fn add_sysinfo_user(
        &self,
        system: &System,
        idempotent: bool,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
    ) {
        if self.sysinfo_config.si_user {
            if let Ok(value) = env::var(SYSINFO_USER) {
                add_map_entry(VergenKey::SysinfoUser, value, map);
            } else {
                add_sysinfo_map_entry(
                    VergenKey::SysinfoUser,
                    idempotent,
                    self.get_user(system),
                    map,
                    warnings,
                );
            }
        }
    }

    fn add_sysinfo_total_memory(
        &self,
        system: &System,
        idempotent: bool,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
    ) {
        if self.sysinfo_config.si_memory {
            if let Ok(value) = env::var(SYSINFO_MEMORY) {
                add_map_entry(VergenKey::SysinfoMemory, value, map);
            } else {
                add_sysinfo_map_entry(
                    VergenKey::SysinfoMemory,
                    idempotent,
                    Some(suffix(system.total_memory())),
                    map,
                    warnings,
                );
            }
        }
    }

    fn add_sysinfo_cpu_vendor(
        &self,
        system: &System,
        idempotent: bool,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
    ) {
        if self.sysinfo_config.si_cpu_vendor {
            if let Ok(value) = env::var(SYSINFO_CPU_VENDOR) {
                add_map_entry(VergenKey::SysinfoCpuVendor, value, map);
            } else {
                add_sysinfo_map_entry(
                    VergenKey::SysinfoCpuVendor,
                    idempotent,
                    system
                        .cpus()
                        .first()
                        .map(|proc| proc.vendor_id().to_string()),
                    map,
                    warnings,
                );
            }
        }
    }

    fn add_sysinfo_cpu_core_count(
        &self,
        system: &System,
        idempotent: bool,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
    ) {
        if self.sysinfo_config.si_cpu_core_count {
            if let Ok(value) = env::var(SYSINFO_CPU_CORE_COUNT) {
                add_map_entry(VergenKey::SysinfoCpuCoreCount, value, map);
            } else {
                add_sysinfo_map_entry(
                    VergenKey::SysinfoCpuCoreCount,
                    idempotent,
                    system.physical_core_count().as_ref().map(usize::to_string),
                    map,
                    warnings,
                );
            }
        }
    }

    fn add_sysinfo_cpu_name(
        &self,
        system: &System,
        idempotent: bool,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
    ) {
        if self.sysinfo_config.si_cpu_name {
            if let Ok(value) = env::var(SYSINFO_CPU_NAME) {
                add_map_entry(VergenKey::SysinfoCpuName, value, map);
            } else {
                add_sysinfo_map_entry(
                    VergenKey::SysinfoCpuName,
                    idempotent,
                    Some(
                        system
                            .cpus()
                            .iter()
                            .map(Cpu::name)
                            .collect::<Vec<&str>>()
                            .join(","),
                    ),
                    map,
                    warnings,
                );
            }
        }
    }

    fn add_sysinfo_cpu_brand(
        &self,
        system: &System,
        idempotent: bool,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
    ) {
        if self.sysinfo_config.si_cpu_brand {
            if let Ok(value) = env::var(SYSINFO_CPU_BRAND) {
                add_map_entry(VergenKey::SysinfoCpuBrand, value, map);
            } else {
                add_sysinfo_map_entry(
                    VergenKey::SysinfoCpuBrand,
                    idempotent,
                    system
                        .cpus()
                        .first()
                        .map(|processor| processor.brand().to_string()),
                    map,
                    warnings,
                );
            }
        }
    }

    fn add_sysinfo_cpu_frequency(
        &self,
        system: &System,
        idempotent: bool,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
    ) {
        if self.sysinfo_config.si_cpu_frequency {
            if let Ok(value) = env::var(SYSINFO_CPU_FREQUENCY) {
                add_map_entry(VergenKey::SysinfoCpuFrequency, value, map);
            } else {
                add_sysinfo_map_entry(
                    VergenKey::SysinfoCpuFrequency,
                    idempotent,
                    system
                        .cpus()
                        .first()
                        .map(|proc| proc.frequency().to_string()),
                    map,
                    warnings,
                );
            }
        }
    }

    fn get_user(&self, system: &System) -> Option<String> {
        if let Ok(pid) = self.get_pid() {
            if let Some(process) = system.process(pid) {
                let users = Users::new_with_refreshed_list();
                for user in &users {
                    if check_user(process, user) {
                        return Some(user.name().to_string());
                    }
                }
            }
        }
        None
    }

    #[cfg(not(test))]
    #[allow(clippy::unused_self)]
    fn get_pid(&self) -> Result<Pid> {
        get_current_pid().map_err(|e| anyhow!(format!("{e}")))
    }

    #[cfg(test)]
    fn get_pid(&self) -> Result<Pid> {
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

fn setup_system(refresh_kind: Option<RefreshKind>) -> System {
    if let Some(refresh_kind) = refresh_kind {
        let mut system = System::new();
        system.refresh_specifics(refresh_kind);
        system
    } else {
        System::new_all()
    }
}

fn check_user(process: &Process, user: &User) -> bool {
    Some(user.id()) == process.user_id()
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
    use std::{collections::BTreeMap, env};
    use sysinfo::{CpuRefreshKind, RefreshKind};

    const IDEM_COUNT: usize = 0;
    const SYSINFO_COUNT: usize = 9;

    #[test]
    #[serial_test::serial]
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
    #[serial_test::serial]
    fn sysinfo_all() -> Result<()> {
        let config = EmitBuilder::builder().all_sysinfo().test_emit()?;
        assert_eq!(SYSINFO_COUNT, config.cargo_rustc_env_map.len());
        assert_eq!(IDEM_COUNT, count_idempotent(&config.cargo_rustc_env_map));
        assert_eq!(IDEM_COUNT, config.warnings.len());
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn sysinfo_refresh_kind() -> Result<()> {
        let refresh_kind = RefreshKind::new();
        let cpu_refresh_kind = CpuRefreshKind::everything()
            .without_cpu_usage()
            .without_frequency();
        let config = EmitBuilder::builder()
            .sysinfo_refresh_kind(Some(refresh_kind.with_cpu(cpu_refresh_kind)))
            .sysinfo_cpu_brand()
            .test_emit()?;
        assert_eq!(1, config.cargo_rustc_env_map.len());
        assert_eq!(IDEM_COUNT, count_idempotent(&config.cargo_rustc_env_map));
        assert_eq!(IDEM_COUNT, config.warnings.len());
        Ok(())
    }

    #[test]
    #[serial_test::serial]
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
    #[serial_test::serial]
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
    #[serial_test::serial]
    fn pid_lookup_fails() -> Result<()> {
        let mut config = EmitBuilder::builder();
        _ = config.all_sysinfo();
        config.sysinfo_config.fail_pid = true;
        let emitter = config.test_emit()?;
        assert_eq!(SYSINFO_COUNT, emitter.cargo_rustc_env_map.len());
        assert_eq!(1, count_idempotent(&emitter.cargo_rustc_env_map));
        assert_eq!(1, emitter.warnings.len());
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn sysinfo_name_override_works() -> Result<()> {
        env::set_var("VERGEN_SYSINFO_NAME", "this is a bad date");
        let mut stdout_buf = vec![];
        assert!(EmitBuilder::builder()
            .all_sysinfo()
            .emit_to(&mut stdout_buf)
            .is_ok());
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(output.contains("cargo:rustc-env=VERGEN_SYSINFO_NAME=this is a bad date"));
        env::remove_var("VERGEN_SYSINFO_NAME");
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn sysinfo_os_version_override_works() -> Result<()> {
        env::set_var("VERGEN_SYSINFO_OS_VERSION", "this is a bad date");
        let mut stdout_buf = vec![];
        assert!(EmitBuilder::builder()
            .all_sysinfo()
            .emit_to(&mut stdout_buf)
            .is_ok());
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(output.contains("cargo:rustc-env=VERGEN_SYSINFO_OS_VERSION=this is a bad date"));
        env::remove_var("VERGEN_SYSINFO_OS_VERSION");
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn sysinfo_user_override_works() -> Result<()> {
        env::set_var("VERGEN_SYSINFO_USER", "this is a bad date");
        let mut stdout_buf = vec![];
        assert!(EmitBuilder::builder()
            .all_sysinfo()
            .emit_to(&mut stdout_buf)
            .is_ok());
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(output.contains("cargo:rustc-env=VERGEN_SYSINFO_USER=this is a bad date"));
        env::remove_var("VERGEN_SYSINFO_USER");
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn sysinfo_total_memory_override_works() -> Result<()> {
        env::set_var("VERGEN_SYSINFO_TOTAL_MEMORY", "this is a bad date");
        let mut stdout_buf = vec![];
        assert!(EmitBuilder::builder()
            .all_sysinfo()
            .emit_to(&mut stdout_buf)
            .is_ok());
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(output.contains("cargo:rustc-env=VERGEN_SYSINFO_TOTAL_MEMORY=this is a bad date"));
        env::remove_var("VERGEN_SYSINFO_TOTAL_MEMORY");
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn sysinfo_cpu_vendor_override_works() -> Result<()> {
        env::set_var("VERGEN_SYSINFO_CPU_VENDOR", "this is a bad date");
        let mut stdout_buf = vec![];
        assert!(EmitBuilder::builder()
            .all_sysinfo()
            .emit_to(&mut stdout_buf)
            .is_ok());
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(output.contains("cargo:rustc-env=VERGEN_SYSINFO_CPU_VENDOR=this is a bad date"));
        env::remove_var("VERGEN_SYSINFO_CPU_VENDOR");
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn sysinfo_cpu_core_count_override_works() -> Result<()> {
        env::set_var("VERGEN_SYSINFO_CPU_CORE_COUNT", "this is a bad date");
        let mut stdout_buf = vec![];
        assert!(EmitBuilder::builder()
            .all_sysinfo()
            .emit_to(&mut stdout_buf)
            .is_ok());
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(output.contains("cargo:rustc-env=VERGEN_SYSINFO_CPU_CORE_COUNT=this is a bad date"));
        env::remove_var("VERGEN_SYSINFO_CPU_CORE_COUNT");
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn sysinfo_cpu_name_override_works() -> Result<()> {
        env::set_var("VERGEN_SYSINFO_CPU_NAME", "this is a bad date");
        let mut stdout_buf = vec![];
        assert!(EmitBuilder::builder()
            .all_sysinfo()
            .emit_to(&mut stdout_buf)
            .is_ok());
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(output.contains("cargo:rustc-env=VERGEN_SYSINFO_CPU_NAME=this is a bad date"));
        env::remove_var("VERGEN_SYSINFO_CPU_NAME");
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn sysinfo_cpu_brand_override_works() -> Result<()> {
        env::set_var("VERGEN_SYSINFO_CPU_BRAND", "this is a bad date");
        let mut stdout_buf = vec![];
        assert!(EmitBuilder::builder()
            .all_sysinfo()
            .emit_to(&mut stdout_buf)
            .is_ok());
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(output.contains("cargo:rustc-env=VERGEN_SYSINFO_CPU_BRAND=this is a bad date"));
        env::remove_var("VERGEN_SYSINFO_CPU_BRAND");
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn sysinfo_cpu_frequency_override_works() -> Result<()> {
        env::set_var("VERGEN_SYSINFO_CPU_FREQUENCY", "this is a bad date");
        let mut stdout_buf = vec![];
        assert!(EmitBuilder::builder()
            .all_sysinfo()
            .emit_to(&mut stdout_buf)
            .is_ok());
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(output.contains("cargo:rustc-env=VERGEN_SYSINFO_CPU_FREQUENCY=this is a bad date"));
        env::remove_var("VERGEN_SYSINFO_CPU_FREQUENCY");
        Ok(())
    }
}
