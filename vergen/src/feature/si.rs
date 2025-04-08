// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use anyhow::{anyhow, Result};
use derive_builder::Builder as DeriveBuilder;
use std::env;
use sysinfo::{get_current_pid, Cpu, Pid, Process, RefreshKind, System, User, Users};
use vergen_lib::{
    add_default_map_entry, add_map_entry,
    constants::{
        SYSINFO_CPU_BRAND, SYSINFO_CPU_CORE_COUNT, SYSINFO_CPU_FREQUENCY, SYSINFO_CPU_NAME,
        SYSINFO_CPU_VENDOR, SYSINFO_MEMORY, SYSINFO_NAME, SYSINFO_OS_VERSION, SYSINFO_USER,
    },
    AddEntries, CargoRerunIfChanged, CargoRustcEnvMap, CargoWarning, DefaultConfig, VergenKey,
};

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
/// # use vergen::Emitter;
/// # use vergen::SysinfoBuilder;
/// #
/// # fn main() -> Result<()> {
/// let si = SysinfoBuilder::all_sysinfo()?;
/// Emitter::default().add_instructions(&si)?.emit()?;
/// #   Ok(())
/// # }
/// ```
///
/// Emit some of the sysinfo instructions
///
/// ```
/// # use anyhow::Result;
/// # use vergen::Emitter;
/// # use vergen::SysinfoBuilder;
/// #
/// # fn main() -> Result<()> {
/// let si = SysinfoBuilder::default().os_version(true).cpu_core_count(true).build()?;
/// Emitter::default()
///     .add_instructions(&si)?
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
/// # use vergen::Emitter;
/// # use vergen::SysinfoBuilder;
/// #
/// # fn main() -> Result<()> {
/// temp_env::with_var("VERGEN_SYSINFO_NAME", Some("this is the name I want output"), || {
///     let result = || -> Result<()> {
///         let si = SysinfoBuilder::all_sysinfo()?;
///         Emitter::default().add_instructions(&si)?.emit()?;
///         Ok(())
///     }();
///     assert!(result.is_ok());
/// });
/// #   Ok(())
/// # }
/// ```
///
/// # Example
/// This feature also recognizes the idempotent flag.
///
/// ```
/// # use anyhow::Result;
/// # use vergen::Emitter;
/// # use vergen::SysinfoBuilder;
/// #
/// # fn main() -> Result<()> {
/// let si = SysinfoBuilder::all_sysinfo()?;
/// Emitter::default().idempotent().add_instructions(&si)?.emit()?;
/// #   Ok(())
/// # }
/// ```
///
/// # Example
/// Use [`SysinfoBuilder::refresh_kind()`] to minimize the amount of data that [`sysinfo`] refreshes.
///
/// ```
/// # use anyhow::Result;
/// # use sysinfo::{CpuRefreshKind, RefreshKind};
/// # use vergen::Emitter;
/// # use vergen::SysinfoBuilder;
/// #
/// # pub fn main() -> Result<()> {
/// let refresh_kind = RefreshKind::nothing();
/// let cpu_refresh_kind = CpuRefreshKind::everything()
///     .without_cpu_usage()
///     .without_frequency();
/// let si = SysinfoBuilder::default()
///     .cpu_brand(true)
///     .refresh_kind(refresh_kind.with_cpu(cpu_refresh_kind))
///     .build()?;
/// let config = Emitter::default()
///     .add_instructions(&si)?
///     .emit()?;
/// #    Ok(())
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
#[derive(Clone, Copy, Debug, DeriveBuilder, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub struct Sysinfo {
    /// Set the [`RefreshKind`](sysinfo::RefreshKind) to use during sysinfo initialization.
    ///
    /// This allows the user to control at a more fine level what `sysinfo`
    /// will refresh on initialization.
    #[builder(default = "None", setter(into))]
    refresh_kind: Option<RefreshKind>,
    /// Enable the sysinfo name
    #[builder(default = "false")]
    name: bool,
    /// Enable the sysinfo OS version
    #[builder(default = "false")]
    os_version: bool,
    /// Enable sysinfo user
    #[builder(default = "false")]
    user: bool,
    /// Enable sysinfo memory
    #[builder(default = "false")]
    memory: bool,
    /// Enable sysinfo cpu vendor
    #[builder(default = "false")]
    cpu_vendor: bool,
    /// Enable sysinfo cpu core count
    #[builder(default = "false")]
    cpu_core_count: bool,
    /// Enable sysinfo cpu name
    #[builder(default = "false")]
    cpu_name: bool,
    /// Enable sysinfo cpu brand
    #[builder(default = "false")]
    cpu_brand: bool,
    /// Enable sysinfo cpu frequency
    #[builder(default = "false")]
    cpu_frequency: bool,
    #[cfg(test)]
    #[builder(setter(skip))]
    fail_pid: bool,
}

impl SysinfoBuilder {
    /// Enable all of the `VERGEN_SYSINFO_*` options
    ///
    /// # Errors
    /// The underlying build function can error
    ///
    pub fn all_sysinfo() -> Result<Sysinfo> {
        Self::default()
            .name(true)
            .os_version(true)
            .user(true)
            .memory(true)
            .cpu_vendor(true)
            .cpu_core_count(true)
            .cpu_name(true)
            .cpu_brand(true)
            .cpu_frequency(true)
            .build()
            .map_err(Into::into)
    }
}

impl Sysinfo {
    fn any(&self) -> bool {
        self.name
            || self.os_version
            || self.user
            || self.memory
            || self.cpu_vendor
            || self.cpu_core_count
            || self.cpu_name
            || self.cpu_brand
            || self.cpu_frequency
    }

    #[cfg(test)]
    pub(crate) fn fail_pid(&mut self) -> &mut Self {
        self.fail_pid = true;
        self
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

    fn add_sysinfo_map_entry(
        key: VergenKey,
        idempotent: bool,
        value: Option<String>,
        cargo_rustc_env: &mut CargoRustcEnvMap,
        cargo_warning: &mut CargoWarning,
    ) {
        if idempotent {
            add_default_map_entry(key, cargo_rustc_env, cargo_warning);
        } else if let Some(val) = value {
            add_map_entry(key, val, cargo_rustc_env);
        } else {
            add_default_map_entry(key, cargo_rustc_env, cargo_warning);
        }
    }

    fn add_sysinfo_name(
        &self,
        _system: &System,
        idempotent: bool,
        cargo_rustc_env: &mut CargoRustcEnvMap,
        cargo_warning: &mut CargoWarning,
    ) {
        if self.name {
            if let Ok(_value) = env::var(SYSINFO_NAME) {
                add_default_map_entry(VergenKey::SysinfoName, cargo_rustc_env, cargo_warning);
            } else {
                Self::add_sysinfo_map_entry(
                    VergenKey::SysinfoName,
                    idempotent,
                    System::name(),
                    cargo_rustc_env,
                    cargo_warning,
                );
            }
        }
    }

    fn add_sysinfo_os_verison(
        &self,
        _system: &System,
        idempotent: bool,
        cargo_rustc_env: &mut CargoRustcEnvMap,
        cargo_warning: &mut CargoWarning,
    ) {
        if self.os_version {
            if let Ok(value) = env::var(SYSINFO_OS_VERSION) {
                add_map_entry(VergenKey::SysinfoOsVersion, value, cargo_rustc_env);
            } else {
                Self::add_sysinfo_map_entry(
                    VergenKey::SysinfoOsVersion,
                    idempotent,
                    System::long_os_version(),
                    cargo_rustc_env,
                    cargo_warning,
                );
            }
        }
    }

    fn add_sysinfo_user(
        &self,
        system: &System,
        idempotent: bool,
        cargo_rustc_env: &mut CargoRustcEnvMap,
        cargo_warning: &mut CargoWarning,
    ) {
        if self.user {
            if let Ok(value) = env::var(SYSINFO_USER) {
                add_map_entry(VergenKey::SysinfoUser, value, cargo_rustc_env);
            } else {
                Self::add_sysinfo_map_entry(
                    VergenKey::SysinfoUser,
                    idempotent,
                    self.get_user(system),
                    cargo_rustc_env,
                    cargo_warning,
                );
            }
        }
    }

    fn get_user(&self, system: &System) -> Option<String> {
        if let Ok(pid) = self.get_pid() {
            if let Some(process) = system.process(pid) {
                let users = Users::new_with_refreshed_list();
                for user in &users {
                    if Self::check_user(process, user) {
                        return Some(user.name().to_string());
                    }
                }
            }
        }
        None
    }

    fn check_user(process: &Process, user: &User) -> bool {
        Some(user.id()) == process.user_id()
    }

    #[cfg(not(test))]
    #[allow(clippy::unused_self)]
    fn get_pid(&self) -> Result<Pid> {
        get_current_pid().map_err(|e| anyhow!(format!("{e}")))
    }

    #[cfg(test)]
    fn get_pid(&self) -> Result<Pid> {
        if self.fail_pid {
            Err(anyhow!("unable to determine pid"))
        } else {
            get_current_pid().map_err(|e| anyhow!(format!("{e}")))
        }
    }

    fn add_sysinfo_total_memory(
        &self,
        system: &System,
        idempotent: bool,
        cargo_rustc_env: &mut CargoRustcEnvMap,
        cargo_warning: &mut CargoWarning,
    ) {
        if self.memory {
            if let Ok(value) = env::var(SYSINFO_MEMORY) {
                add_map_entry(VergenKey::SysinfoMemory, value, cargo_rustc_env);
            } else {
                Self::add_sysinfo_map_entry(
                    VergenKey::SysinfoMemory,
                    idempotent,
                    Some(Self::suffix(system.total_memory())),
                    cargo_rustc_env,
                    cargo_warning,
                );
            }
        }
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

    fn add_sysinfo_cpu_vendor(
        &self,
        system: &System,
        idempotent: bool,
        cargo_rustc_env: &mut CargoRustcEnvMap,
        cargo_warning: &mut CargoWarning,
    ) {
        if self.cpu_vendor {
            if let Ok(value) = env::var(SYSINFO_CPU_VENDOR) {
                add_map_entry(VergenKey::SysinfoCpuVendor, value, cargo_rustc_env);
            } else {
                Self::add_sysinfo_map_entry(
                    VergenKey::SysinfoCpuVendor,
                    idempotent,
                    system
                        .cpus()
                        .first()
                        .map(|proc| proc.vendor_id().to_string()),
                    cargo_rustc_env,
                    cargo_warning,
                );
            }
        }
    }

    fn add_sysinfo_cpu_core_count(
        &self,
        idempotent: bool,
        cargo_rustc_env: &mut CargoRustcEnvMap,
        cargo_warning: &mut CargoWarning,
    ) {
        if self.cpu_core_count {
            if let Ok(value) = env::var(SYSINFO_CPU_CORE_COUNT) {
                add_map_entry(VergenKey::SysinfoCpuCoreCount, value, cargo_rustc_env);
            } else {
                Self::add_sysinfo_map_entry(
                    VergenKey::SysinfoCpuCoreCount,
                    idempotent,
                    System::physical_core_count().as_ref().map(usize::to_string),
                    cargo_rustc_env,
                    cargo_warning,
                );
            }
        }
    }

    fn add_sysinfo_cpu_name(
        &self,
        system: &System,
        idempotent: bool,
        cargo_rustc_env: &mut CargoRustcEnvMap,
        cargo_warning: &mut CargoWarning,
    ) {
        if self.cpu_name {
            if let Ok(value) = env::var(SYSINFO_CPU_NAME) {
                add_map_entry(VergenKey::SysinfoCpuName, value, cargo_rustc_env);
            } else {
                Self::add_sysinfo_map_entry(
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
                    cargo_rustc_env,
                    cargo_warning,
                );
            }
        }
    }

    fn add_sysinfo_cpu_brand(
        &self,
        system: &System,
        idempotent: bool,
        cargo_rustc_env: &mut CargoRustcEnvMap,
        cargo_warning: &mut CargoWarning,
    ) {
        if self.cpu_brand {
            if let Ok(value) = env::var(SYSINFO_CPU_BRAND) {
                add_map_entry(VergenKey::SysinfoCpuBrand, value, cargo_rustc_env);
            } else {
                Self::add_sysinfo_map_entry(
                    VergenKey::SysinfoCpuBrand,
                    idempotent,
                    system
                        .cpus()
                        .first()
                        .map(|processor| processor.brand().to_string()),
                    cargo_rustc_env,
                    cargo_warning,
                );
            }
        }
    }

    fn add_sysinfo_cpu_frequency(
        &self,
        system: &System,
        idempotent: bool,
        cargo_rustc_env: &mut CargoRustcEnvMap,
        cargo_warning: &mut CargoWarning,
    ) {
        if self.cpu_frequency {
            if let Ok(value) = env::var(SYSINFO_CPU_FREQUENCY) {
                add_map_entry(VergenKey::SysinfoCpuFrequency, value, cargo_rustc_env);
            } else {
                Self::add_sysinfo_map_entry(
                    VergenKey::SysinfoCpuFrequency,
                    idempotent,
                    system
                        .cpus()
                        .first()
                        .map(|proc| proc.frequency().to_string()),
                    cargo_rustc_env,
                    cargo_warning,
                );
            }
        }
    }
}

impl AddEntries for Sysinfo {
    fn add_map_entries(
        &self,
        idempotent: bool,
        cargo_rustc_env: &mut CargoRustcEnvMap,
        _cargo_rerun_if_changed: &mut CargoRerunIfChanged,
        cargo_warning: &mut CargoWarning,
    ) -> Result<()> {
        if self.any() {
            let system = Self::setup_system(self.refresh_kind);

            self.add_sysinfo_name(&system, idempotent, cargo_rustc_env, cargo_warning);
            self.add_sysinfo_os_verison(&system, idempotent, cargo_rustc_env, cargo_warning);
            self.add_sysinfo_user(&system, idempotent, cargo_rustc_env, cargo_warning);
            self.add_sysinfo_total_memory(&system, idempotent, cargo_rustc_env, cargo_warning);
            self.add_sysinfo_cpu_vendor(&system, idempotent, cargo_rustc_env, cargo_warning);
            self.add_sysinfo_cpu_core_count(idempotent, cargo_rustc_env, cargo_warning);
            self.add_sysinfo_cpu_name(&system, idempotent, cargo_rustc_env, cargo_warning);
            self.add_sysinfo_cpu_brand(&system, idempotent, cargo_rustc_env, cargo_warning);
            self.add_sysinfo_cpu_frequency(&system, idempotent, cargo_rustc_env, cargo_warning);
        }
        Ok(())
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    fn add_default_entries(
        &self,
        _config: &DefaultConfig,
        _cargo_rustc_env_map: &mut CargoRustcEnvMap,
        _cargo_rerun_if_changed: &mut CargoRerunIfChanged,
        _cargo_warning: &mut CargoWarning,
    ) -> Result<()> {
        // currently add_map_entries can't error for sysinfo
        // so this will never be used.
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::{Sysinfo, SysinfoBuilder};
    use crate::Emitter;
    use anyhow::Result;
    use serial_test::serial;
    use std::{collections::BTreeMap, io::Write};
    use sysinfo::{CpuRefreshKind, RefreshKind};
    use temp_env::with_var;
    use vergen_lib::{count_idempotent, VergenKey};

    const IDEM_COUNT: usize = 0;
    const SYSINFO_COUNT: usize = 9;

    #[test]
    #[serial]
    #[allow(clippy::clone_on_copy, clippy::redundant_clone)]
    fn si_clone_works() -> Result<()> {
        let si = SysinfoBuilder::all_sysinfo()?;
        let another = si.clone();
        assert_eq!(another, si);
        Ok(())
    }

    #[test]
    #[serial]
    fn si_debug_works() -> Result<()> {
        let si = SysinfoBuilder::all_sysinfo()?;
        let mut buf = vec![];
        write!(buf, "{si:?}")?;
        assert!(!buf.is_empty());
        Ok(())
    }

    #[test]
    #[serial]
    fn si_default() -> Result<()> {
        let si = SysinfoBuilder::default().build()?;
        let emitter = Emitter::default().add_instructions(&si)?.test_emit();
        assert_eq!(0, emitter.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(0, emitter.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn sysinfo_all_idempotent() -> Result<()> {
        let si = SysinfoBuilder::all_sysinfo()?;
        let config = Emitter::default()
            .idempotent()
            .add_instructions(&si)?
            .test_emit();
        assert_eq!(SYSINFO_COUNT, config.cargo_rustc_env_map().len());
        assert_eq!(
            SYSINFO_COUNT,
            count_idempotent(config.cargo_rustc_env_map())
        );
        assert_eq!(SYSINFO_COUNT, config.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn sysinfo_all() -> Result<()> {
        let si = SysinfoBuilder::all_sysinfo()?;
        let config = Emitter::default().add_instructions(&si)?.test_emit();
        assert_eq!(SYSINFO_COUNT, config.cargo_rustc_env_map().len());
        assert_eq!(IDEM_COUNT, count_idempotent(config.cargo_rustc_env_map()));
        assert_eq!(IDEM_COUNT, config.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn sysinfo_name() -> Result<()> {
        let si = SysinfoBuilder::default().name(true).build()?;
        let config = Emitter::default().add_instructions(&si)?.test_emit();
        assert_eq!(1, config.cargo_rustc_env_map().len());
        assert_eq!(IDEM_COUNT, count_idempotent(config.cargo_rustc_env_map()));
        assert_eq!(IDEM_COUNT, config.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn sysinfo_os_version() -> Result<()> {
        let si = SysinfoBuilder::default().os_version(true).build()?;
        let config = Emitter::default().add_instructions(&si)?.test_emit();
        assert_eq!(1, config.cargo_rustc_env_map().len());
        assert_eq!(IDEM_COUNT, count_idempotent(config.cargo_rustc_env_map()));
        assert_eq!(IDEM_COUNT, config.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn sysinfo_user() -> Result<()> {
        let si = SysinfoBuilder::default().user(true).build()?;
        let config = Emitter::default().add_instructions(&si)?.test_emit();
        assert_eq!(1, config.cargo_rustc_env_map().len());
        assert_eq!(IDEM_COUNT, count_idempotent(config.cargo_rustc_env_map()));
        assert_eq!(IDEM_COUNT, config.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn sysinfo_memory() -> Result<()> {
        let si = SysinfoBuilder::default().memory(true).build()?;
        let config = Emitter::default().add_instructions(&si)?.test_emit();
        assert_eq!(1, config.cargo_rustc_env_map().len());
        assert_eq!(IDEM_COUNT, count_idempotent(config.cargo_rustc_env_map()));
        assert_eq!(IDEM_COUNT, config.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn sysinfo_cpu_vendor() -> Result<()> {
        let si = SysinfoBuilder::default().cpu_vendor(true).build()?;
        let config = Emitter::default().add_instructions(&si)?.test_emit();
        assert_eq!(1, config.cargo_rustc_env_map().len());
        assert_eq!(IDEM_COUNT, count_idempotent(config.cargo_rustc_env_map()));
        assert_eq!(IDEM_COUNT, config.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn sysinfo_cpu_core_count() -> Result<()> {
        let si = SysinfoBuilder::default().cpu_core_count(true).build()?;
        let config = Emitter::default().add_instructions(&si)?.test_emit();
        assert_eq!(1, config.cargo_rustc_env_map().len());
        assert_eq!(IDEM_COUNT, count_idempotent(config.cargo_rustc_env_map()));
        assert_eq!(IDEM_COUNT, config.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn sysinfo_cpu_name() -> Result<()> {
        let si = SysinfoBuilder::default().cpu_name(true).build()?;
        let config = Emitter::default().add_instructions(&si)?.test_emit();
        assert_eq!(1, config.cargo_rustc_env_map().len());
        assert_eq!(IDEM_COUNT, count_idempotent(config.cargo_rustc_env_map()));
        assert_eq!(IDEM_COUNT, config.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn sysinfo_cpu_brand() -> Result<()> {
        let si = SysinfoBuilder::default().cpu_brand(true).build()?;
        let config = Emitter::default().add_instructions(&si)?.test_emit();
        assert_eq!(1, config.cargo_rustc_env_map().len());
        assert_eq!(IDEM_COUNT, count_idempotent(config.cargo_rustc_env_map()));
        assert_eq!(IDEM_COUNT, config.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn sysinfo_cpu_frequency() -> Result<()> {
        let si = SysinfoBuilder::default().cpu_frequency(true).build()?;
        let config = Emitter::default().add_instructions(&si)?.test_emit();
        assert_eq!(1, config.cargo_rustc_env_map().len());
        assert_eq!(IDEM_COUNT, count_idempotent(config.cargo_rustc_env_map()));
        assert_eq!(IDEM_COUNT, config.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn sysinfo_refresh_kind() -> Result<()> {
        let refresh_kind = RefreshKind::nothing();
        let cpu_refresh_kind = CpuRefreshKind::everything()
            .without_cpu_usage()
            .without_frequency();
        let si = SysinfoBuilder::default()
            .refresh_kind(Some(refresh_kind.with_cpu(cpu_refresh_kind)))
            .cpu_brand(true)
            .build()?;
        let config = Emitter::default().add_instructions(&si)?.test_emit();
        assert_eq!(1, config.cargo_rustc_env_map().len());
        assert_eq!(IDEM_COUNT, count_idempotent(config.cargo_rustc_env_map()));
        assert_eq!(IDEM_COUNT, config.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn adding_none_defaults() -> Result<()> {
        let mut map = BTreeMap::new();
        let mut cargo_warning = vec![];
        Sysinfo::add_sysinfo_map_entry(
            VergenKey::SysinfoCpuBrand,
            false,
            None,
            &mut map,
            &mut cargo_warning,
        );
        Ok(())
    }

    #[test]
    #[serial]
    fn suffix_works() {
        assert_eq!(Sysinfo::suffix(1023), "1023 B");
        assert_eq!(Sysinfo::suffix(1024), "1 KiB");
        assert_eq!(Sysinfo::suffix(1_048_575), "1023 KiB");
        assert_eq!(Sysinfo::suffix(1_048_576), "1 MiB");
        assert_eq!(Sysinfo::suffix(1_073_741_823), "1023 MiB");
        assert_eq!(Sysinfo::suffix(1_073_741_824), "1 GiB");
        assert_eq!(Sysinfo::suffix(1_099_511_627_775), "1023 GiB");
        assert_eq!(Sysinfo::suffix(1_099_511_627_776), "1 TiB");
        assert_eq!(Sysinfo::suffix(1_125_899_906_842_623), "1023 TiB");
        assert_eq!(Sysinfo::suffix(1_125_899_906_842_624), "1 PiB");
        assert_eq!(
            Sysinfo::suffix((1_125_899_906_842_624 * 1024) - 1),
            "1023 PiB"
        );
        assert_eq!(Sysinfo::suffix(1_125_899_906_842_624 * 1024), "1 EiB");
        assert_eq!(Sysinfo::suffix(u64::MAX), "15 EiB");
    }

    #[test]
    #[serial_test::serial]
    fn pid_lookup_fails() -> Result<()> {
        let mut si = SysinfoBuilder::all_sysinfo()?;
        let _ = si.fail_pid();
        let emitter = Emitter::default().add_instructions(&si)?.test_emit();
        assert_eq!(SYSINFO_COUNT, emitter.cargo_rustc_env_map().len());
        assert_eq!(1, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(1, emitter.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn sysinfo_name_override_works() {
        with_var("VERGEN_SYSINFO_NAME", Some("this is a bad date"), || {
            let result = || -> Result<()> {
                let mut stdout_buf = vec![];
                let si = SysinfoBuilder::all_sysinfo()?;
                let _failed = Emitter::default()
                    .add_instructions(&si)?
                    .emit_to(&mut stdout_buf)?;
                let output = String::from_utf8_lossy(&stdout_buf);
                assert!(output.contains("cargo:rustc-env=VERGEN_SYSINFO_NAME=this is a bad date"));
                Ok(())
            }();
            assert!(result.is_ok());
        });
    }

    #[test]
    #[serial]
    fn sysinfo_os_version_override_works() {
        with_var(
            "VERGEN_SYSINFO_OS_VERSION",
            Some("this is a bad date"),
            || {
                let result = || -> Result<()> {
                    let mut stdout_buf = vec![];
                    let si = SysinfoBuilder::all_sysinfo()?;
                    let _failed = Emitter::default()
                        .add_instructions(&si)?
                        .emit_to(&mut stdout_buf)?;
                    let output = String::from_utf8_lossy(&stdout_buf);
                    assert!(output
                        .contains("cargo:rustc-env=VERGEN_SYSINFO_OS_VERSION=this is a bad date"));
                    Ok(())
                }();
                assert!(result.is_ok());
            },
        );
    }

    #[test]
    #[serial]
    fn sysinfo_user_override_works() {
        with_var("VERGEN_SYSINFO_USER", Some("this is a bad date"), || {
            let result = || -> Result<()> {
                let mut stdout_buf = vec![];
                let si = SysinfoBuilder::all_sysinfo()?;
                let _failed = Emitter::default()
                    .add_instructions(&si)?
                    .emit_to(&mut stdout_buf)?;
                let output = String::from_utf8_lossy(&stdout_buf);
                assert!(output.contains("cargo:rustc-env=VERGEN_SYSINFO_USER=this is a bad date"));
                Ok(())
            }();
            assert!(result.is_ok());
        });
    }

    #[test]
    #[serial]
    fn sysinfo_total_memory_override_works() {
        with_var(
            "VERGEN_SYSINFO_TOTAL_MEMORY",
            Some("this is a bad date"),
            || {
                let result = || -> Result<()> {
                    let mut stdout_buf = vec![];
                    let si = SysinfoBuilder::all_sysinfo()?;
                    let _failed = Emitter::default()
                        .add_instructions(&si)?
                        .emit_to(&mut stdout_buf)?;
                    let output = String::from_utf8_lossy(&stdout_buf);
                    assert!(output.contains(
                        "cargo:rustc-env=VERGEN_SYSINFO_TOTAL_MEMORY=this is a bad date"
                    ));
                    Ok(())
                }();
                assert!(result.is_ok());
            },
        );
    }

    #[test]
    #[serial]
    fn sysinfo_cpu_vendor_override_works() {
        with_var(
            "VERGEN_SYSINFO_CPU_VENDOR",
            Some("this is a bad date"),
            || {
                let result = || -> Result<()> {
                    let mut stdout_buf = vec![];
                    let si = SysinfoBuilder::all_sysinfo()?;
                    let _failed = Emitter::default()
                        .add_instructions(&si)?
                        .emit_to(&mut stdout_buf)?;
                    let output = String::from_utf8_lossy(&stdout_buf);
                    assert!(output
                        .contains("cargo:rustc-env=VERGEN_SYSINFO_CPU_VENDOR=this is a bad date"));
                    Ok(())
                }();
                assert!(result.is_ok());
            },
        );
    }

    #[test]
    #[serial]
    fn sysinfo_cpu_core_count_override_works() {
        with_var(
            "VERGEN_SYSINFO_CPU_CORE_COUNT",
            Some("this is a bad date"),
            || {
                let result = || -> Result<()> {
                    let mut stdout_buf = vec![];
                    let si = SysinfoBuilder::all_sysinfo()?;
                    let _failed = Emitter::default()
                        .add_instructions(&si)?
                        .emit_to(&mut stdout_buf)?;
                    let output = String::from_utf8_lossy(&stdout_buf);
                    assert!(output.contains(
                        "cargo:rustc-env=VERGEN_SYSINFO_CPU_CORE_COUNT=this is a bad date"
                    ));
                    Ok(())
                }();
                assert!(result.is_ok());
            },
        );
    }

    #[test]
    #[serial]
    fn sysinfo_cpu_name_override_works() {
        with_var(
            "VERGEN_SYSINFO_CPU_NAME",
            Some("this is a bad date"),
            || {
                let result = || -> Result<()> {
                    let mut stdout_buf = vec![];
                    let si = SysinfoBuilder::all_sysinfo()?;
                    let _failed = Emitter::default()
                        .add_instructions(&si)?
                        .emit_to(&mut stdout_buf)?;
                    let output = String::from_utf8_lossy(&stdout_buf);
                    assert!(output
                        .contains("cargo:rustc-env=VERGEN_SYSINFO_CPU_NAME=this is a bad date"));
                    Ok(())
                }();
                assert!(result.is_ok());
            },
        );
    }

    #[test]
    #[serial]
    fn sysinfo_cpu_brand_override_works() {
        with_var(
            "VERGEN_SYSINFO_CPU_BRAND",
            Some("this is a bad date"),
            || {
                let result = || -> Result<()> {
                    let mut stdout_buf = vec![];
                    let si = SysinfoBuilder::all_sysinfo()?;
                    let _failed = Emitter::default()
                        .add_instructions(&si)?
                        .emit_to(&mut stdout_buf)?;
                    let output = String::from_utf8_lossy(&stdout_buf);
                    assert!(output
                        .contains("cargo:rustc-env=VERGEN_SYSINFO_CPU_BRAND=this is a bad date"));
                    Ok(())
                }();
                assert!(result.is_ok());
            },
        );
    }

    #[test]
    #[serial]
    fn sysinfo_cpu_frequency_override_works() {
        with_var(
            "VERGEN_SYSINFO_CPU_FREQUENCY",
            Some("this is a bad date"),
            || {
                let result = || -> Result<()> {
                    let mut stdout_buf = vec![];
                    let si = SysinfoBuilder::all_sysinfo()?;
                    let _failed = Emitter::default()
                        .add_instructions(&si)?
                        .emit_to(&mut stdout_buf)?;
                    let output = String::from_utf8_lossy(&stdout_buf);
                    assert!(output.contains(
                        "cargo:rustc-env=VERGEN_SYSINFO_CPU_FREQUENCY=this is a bad date"
                    ));
                    Ok(())
                }();
                assert!(result.is_ok());
            },
        );
    }
}
