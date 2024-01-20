use crate::VergenKey;

use anyhow::{Error, Result};
use getset::Getters;
use std::collections::BTreeMap;

///
pub type CargoRustcEnvMap = BTreeMap<VergenKey, String>;
///
pub type CargoRerunIfChanged = Vec<String>;
///
pub type CargoWarning = Vec<String>;

///
#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct DefaultConfig {
    ///
    fail_on_error: bool,
    ///
    error: Error,
}

impl DefaultConfig {
    ///
    pub fn new(fail_on_error: bool, error: Error) -> Self {
        Self {
            fail_on_error,
            error,
        }
    }
}

///
pub trait AddEntries {
    ///
    fn add_map_entries(
        &self,
        idempotent: bool,
        cargo_rustc_env: &mut CargoRustcEnvMap,
        cargo_rerun_if_changed: &mut CargoRerunIfChanged,
        cargo_warning: &mut CargoWarning,
    ) -> Result<()>;
    ///
    fn add_default_entries(
        &self,
        config: &DefaultConfig,
        cargo_rustc_env_map: &mut CargoRustcEnvMap,
        cargo_rerun_if_changed: &mut CargoRerunIfChanged,
        cargo_warning: &mut CargoWarning,
    ) -> Result<()>;
}
