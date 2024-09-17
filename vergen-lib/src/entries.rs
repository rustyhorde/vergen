use crate::VergenKey;

use anyhow::{Error, Result};
use std::collections::BTreeMap;

/// The map used to emit `cargo:rustc-env=NAME=VALUE` cargo instructions
pub type CargoRustcEnvMap = BTreeMap<VergenKey, String>;
/// The vector of strings used to emit `cargo:rerun-if-changed=VALUE` cargo instructions
pub type CargoRerunIfChanged = Vec<String>;
/// The vector of strings used to emit `cargo:warning=VALUE` cargo instructions
pub type CargoWarning = Vec<String>;

/// The default configuration to use when an issue has occured generating instructions
#[derive(Debug)]
pub struct DefaultConfig {
    /// Should we fail if an error occurs or output idempotent values on error?
    fail_on_error: bool,
    /// The error that caused us to try default instruction output.
    error: Error,
}

impl DefaultConfig {
    /// Create a new [`DefaultConfig`] struct with the given values.
    #[must_use]
    pub fn new(fail_on_error: bool, error: Error) -> Self {
        Self {
            fail_on_error,
            error,
        }
    }
    /// Should we fail if an error occurs or output idempotent values on error?
    #[must_use]
    pub fn fail_on_error(&self) -> &bool {
        &self.fail_on_error
    }
    /// The error that caused us to try default instruction output.
    #[must_use]
    pub fn error(&self) -> &Error {
        &self.error
    }
}

/// This trait should be implemented to allow the `vergen` emitter
/// to properly emit instructions for your feature.
pub trait Add {
    /// Try to add instructions entries to the various given arguments.
    ///
    /// * Write to the `cargo_rustc_env` map to emit 'cargo:rustc-env=NAME=VALUE' instructions.
    /// * Write to the `cargo_rerun_if_changed` vector to emit 'cargo:rerun-if-changed=VALUE' instructions.
    /// * Write to the `cargo_warning` vector to emit 'cargo:warning=VALUE' instructions.
    ///
    /// # Errors
    ///
    /// If an error occurs, the `vergen` emitter will use `add_default_entries` to generate output.
    /// This assumes generating instructions may fail in some manner so a [`anyhow::Result`] is returned.
    ///
    fn add_map_entries(
        &self,
        idempotent: bool,
        cargo_rustc_env: &mut CargoRustcEnvMap,
        cargo_rerun_if_changed: &mut CargoRerunIfChanged,
        cargo_warning: &mut CargoWarning,
    ) -> Result<()>;

    /// Based on the given configuration, emit either default idempotent output or generate a failue.
    ///
    /// * Write to the `cargo_rustc_env` map to emit 'cargo:rustc-env=NAME=VALUE' instructions.
    /// * Write to the `cargo_rerun_if_changed` vector to emit 'cargo:rerun-if-changed=VALUE' instructions.
    /// * Write to the `cargo_warning` vector to emit 'cargo:warning=VALUE' instructions.
    ///
    /// # Errors
    ///
    /// This assumes generating instructions may fail in some manner so a [`anyhow::Result`] is returned.
    ///
    fn add_default_entries(
        &self,
        config: &DefaultConfig,
        cargo_rustc_env_map: &mut CargoRustcEnvMap,
        cargo_rerun_if_changed: &mut CargoRerunIfChanged,
        cargo_warning: &mut CargoWarning,
    ) -> Result<()>;
}

/// This trait should be implemented to allow the `vergen` emitter to properly emit your custom instructions.
///
/// # Example
/// ```
/// # use anyhow::Result;
/// # use std::collections::BTreeMap;
/// # use vergen_lib::{AddCustomEntries, CargoRerunIfChanged, CargoWarning, DefaultConfig};
/// #[derive(Default)]
/// struct Custom {}
///
/// impl AddCustomEntries<&str, &str> for Custom {
///     fn add_calculated_entries(
///         &self,
///         _idempotent: bool,
///         cargo_rustc_env_map: &mut BTreeMap<&str, &str>,
///         _cargo_rerun_if_changed: &mut CargoRerunIfChanged,
///         cargo_warning: &mut CargoWarning,
///     ) -> Result<()> {
///         cargo_rustc_env_map.insert("vergen-cl", "custom_instruction");
///         cargo_warning.push("custom instruction generated".to_string());
///         Ok(())
///     }
///
///     fn add_default_entries(
///         &self,
///         _config: &DefaultConfig,
///         _cargo_rustc_env_map: &mut BTreeMap<&str, &str>,
///         _cargo_rerun_if_changed: &mut CargoRerunIfChanged,
///         _cargo_warning: &mut CargoWarning,
///     ) -> Result<()> {
///         Ok(())
///     }
/// }
/// ```
/// ## Then in [`build.rs`]
///
/// ```will_not_compile
/// let build = BuildBuilder::all_build()?;
/// let cargo = CargoBuilder::all_cargo()?;
/// let gix = GixBuilder::all_git()?;
/// let rustc = RustcBuilder::all_rustc()?;
/// let si = SysinfoBuilder::all_sysinfo()?;
/// Emitter::default()
///     .add_instructions(&build)?
///     .add_instructions(&cargo)?
///     .add_instructions(&gix)?
///     .add_instructions(&rustc)?
///     .add_instructions(&si)?
///     .add_custom_instructions(&Custom::default())?
///     .emit()
/// ```
pub trait AddCustom<K: Into<String> + Ord, V: Into<String>> {
    /// Try to add instructions entries to the various given arguments.
    ///
    /// * Write to the `cargo_rustc_env` map to emit 'cargo:rustc-env=NAME=VALUE' instructions.
    /// * Write to the `cargo_rerun_if_changed` vector to emit 'cargo:rerun-if-changed=VALUE' instructions.
    /// * Write to the `cargo_warning` vector to emit 'cargo:warning=VALUE' instructions.
    ///
    /// # Errors
    ///
    /// If an error occurs, the `vergen` emitter will use `add_default_entries` to generate output.
    /// This assumes generating instructions may fail in some manner so a [`anyhow::Result`] is returned.
    ///
    fn add_calculated_entries(
        &self,
        idempotent: bool,
        cargo_rustc_env_map: &mut BTreeMap<K, V>,
        cargo_rerun_if_changed: &mut CargoRerunIfChanged,
        cargo_warning: &mut CargoWarning,
    ) -> Result<()>;

    /// Based on the given configuration, emit either default idempotent output or generate a failue.
    ///
    /// * Write to the `cargo_rustc_env` map to emit 'cargo:rustc-env=NAME=VALUE' instructions.
    /// * Write to the `cargo_rerun_if_changed` vector to emit 'cargo:rerun-if-changed=VALUE' instructions.
    /// * Write to the `cargo_warning` vector to emit 'cargo:warning=VALUE' instructions.
    ///
    /// # Errors
    ///
    /// This assumes generating instructions may fail in some manner so a [`anyhow::Result`] is returned.
    ///
    fn add_default_entries(
        &self,
        config: &DefaultConfig,
        cargo_rustc_env_map: &mut BTreeMap<K, V>,
        cargo_rerun_if_changed: &mut CargoRerunIfChanged,
        cargo_warning: &mut CargoWarning,
    ) -> Result<()>;
}

#[doc(hidden)]
pub(crate) mod test_gen {
    use crate::{AddCustomEntries, CargoRerunIfChanged, CargoWarning};
    use anyhow::{anyhow, Result};
    use derive_builder::Builder;
    use std::collections::BTreeMap;

    #[doc(hidden)]
    #[derive(Builder, Clone, Copy, Debug, Default)]
    pub struct CustomInsGen {
        fail: bool,
    }

    impl AddCustomEntries<&str, &str> for CustomInsGen {
        fn add_calculated_entries(
            &self,
            idempotent: bool,
            cargo_rustc_env_map: &mut BTreeMap<&str, &str>,
            _cargo_rerun_if_changed: &mut CargoRerunIfChanged,
            _cargo_warning: &mut CargoWarning,
        ) -> Result<()> {
            if self.fail {
                Err(anyhow!("We have failed"))
            } else {
                if idempotent {
                    let _ = cargo_rustc_env_map.insert("test", "VERGEN_IDEMPOTENT_OUTPUT");
                } else {
                    let _ = cargo_rustc_env_map.insert("test", "value");
                }
                Ok(())
            }
        }

        fn add_default_entries(
            &self,
            config: &crate::DefaultConfig,
            cargo_rustc_env_map: &mut BTreeMap<&str, &str>,
            _cargo_rerun_if_changed: &mut CargoRerunIfChanged,
            _cargo_warning: &mut CargoWarning,
        ) -> Result<()> {
            if *config.fail_on_error() {
                let error = anyhow!(format!("{}", config.error()));
                Err(error)
            } else {
                let _ = cargo_rustc_env_map.insert("test", "VERGEN_IDEMPOTENT_OUTPUT");
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::DefaultConfig;
    use anyhow::{anyhow, Result};
    use std::io::Write;

    #[test]
    fn default_config_debug() -> Result<()> {
        let config = DefaultConfig::new(true, anyhow!("blah"));
        let mut buf = vec![];
        write!(buf, "{config:?}")?;
        assert!(!buf.is_empty());
        Ok(())
    }
}
