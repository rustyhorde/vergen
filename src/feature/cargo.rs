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
};
use anyhow::{Error, Result};
use std::env;

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Config {
    pub(crate) cargo_debug: bool,
    pub(crate) cargo_features: bool,
    pub(crate) cargo_opt_level: bool,
    pub(crate) cargo_target_triple: bool,
}

impl Config {
    #[cfg(test)]
    fn enable_all(&mut self) {
        self.cargo_debug = true;
        self.cargo_features = true;
        self.cargo_opt_level = true;
        self.cargo_target_triple = true;
    }

    pub(crate) fn add_warnings(
        self,
        skip_if_error: bool,
        e: Error,
        warnings: &mut Vec<String>,
    ) -> Result<()> {
        if skip_if_error {
            if self.cargo_debug {
                warnings.push(format!(
                    "Unable to add {} to output",
                    VergenKey::CargoDebug.name()
                ));
            }
            if self.cargo_features {
                warnings.push(format!(
                    "Unable to add {} to output",
                    VergenKey::CargoFeatures.name()
                ));
            }
            if self.cargo_opt_level {
                warnings.push(format!(
                    "Unable to add {} to output",
                    VergenKey::CargoOptLevel.name()
                ));
            }
            if self.cargo_target_triple {
                warnings.push(format!(
                    "Unable to add {} to output",
                    VergenKey::CargoTargetTriple.name()
                ));
            }
            Ok(())
        } else {
            Err(e)
        }
    }
}

/// Copnfigure the emission of `VERGEN_CARGO_*` instructions
///
/// | Variable | Sample |
/// | -------  | ------ |
/// | `VERGEN_CARGO_DEBUG` | true |
/// | `VERGEN_CARGO_FEATURES` | git,build |
/// | `VERGEN_CARGO_OPT_LEVEL` | 1 |
/// | `VERGEN_CARGO_TARGET_TRIPLE` | x86_64-unknown-linux-gnu |
///
/// # Example
///
/// ```
/// # use anyhow::Result;
/// # use std::env;
/// # use vergen::EmitBuilder;
/// #
/// # fn main() -> Result<()> {
/// # env::set_var("CARGO_FEATURE_BUILD", "");
/// # env::set_var("DEBUG", "true");
/// # env::set_var("OPT_LEVEL", "1");
/// # env::set_var("TARGET", "x86_64-unknown-linux-gnu");
/// EmitBuilder::builder().all_cargo().emit()?;
/// # env::remove_var("CARGO_FEATURE_BUILD");
/// # env::remove_var("DEBUG");
/// # env::remove_var("OPT_LEVEL");
/// # env::remove_var("TARGET");
/// #   Ok(())
/// # }
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "cargo")))]
impl EmitBuilder {
    /// Emit all of the `VERGEN_CARGO_*` instructions
    pub fn all_cargo(&mut self) -> &mut Self {
        self.cargo_debug()
            .cargo_features()
            .cargo_opt_level()
            .cargo_target_triple()
    }

    /// Emit the DEBUG value set by cargo
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_CARGO_DEBUG=true|false
    /// ```
    ///
    pub fn cargo_debug(&mut self) -> &mut Self {
        self.cargo_config.cargo_debug = true;
        self
    }

    /// Emit the CARGO_FEATURE_* values set by cargo
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_CARGO_FEATURES=<features>
    /// ```
    ///
    pub fn cargo_features(&mut self) -> &mut Self {
        self.cargo_config.cargo_features = true;
        self
    }

    /// Emit the OPT_LEVEL value set by cargo
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_CARGO_OPT_LEVEL=<opt_level>
    /// ```
    ///
    pub fn cargo_opt_level(&mut self) -> &mut Self {
        self.cargo_config.cargo_opt_level = true;
        self
    }

    /// Emit the TARGET value set by cargo
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_CARGO_TARGET_TRIPLE=<target_triple>
    /// ```
    ///
    pub fn cargo_target_triple(&mut self) -> &mut Self {
        self.cargo_config.cargo_target_triple = true;
        self
    }

    pub(crate) fn add_cargo_map_entries(&self, map: &mut RustcEnvMap) -> Result<()> {
        if self.cargo_config.cargo_debug {
            let _old = map.insert(VergenKey::CargoDebug, env::var("DEBUG")?);
        }

        if self.cargo_config.cargo_features {
            let features: Vec<String> = env::vars().filter_map(is_cargo_feature).collect();
            let feature_str = features.as_slice().join(",");
            let _old = map.insert(VergenKey::CargoFeatures, feature_str);
        }

        if self.cargo_config.cargo_opt_level {
            let _old = map.insert(VergenKey::CargoOptLevel, env::var("OPT_LEVEL")?);
        }

        if self.cargo_config.cargo_target_triple {
            let _old = map.insert(VergenKey::CargoTargetTriple, env::var("TARGET")?);
        }

        Ok(())
    }
}

fn is_cargo_feature(var: (String, String)) -> Option<String> {
    let (k, _) = var;
    if k.starts_with("CARGO_FEATURE_") {
        Some(k.replace("CARGO_FEATURE_", "").to_lowercase())
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::Config;
    use crate::{
        emitter::test::count_idempotent,
        utils::testutils::{setup, teardown},
        EmitBuilder,
    };
    use anyhow::{anyhow, Result};

    #[test]
    #[serial_test::parallel]
    fn add_warnings_is_err() -> Result<()> {
        let config = Config::default();
        let mut warnings = vec![];
        assert!(config
            .add_warnings(false, anyhow!("test"), &mut warnings)
            .is_err());
        Ok(())
    }

    #[test]
    #[serial_test::parallel]
    fn add_warnings_adds_warnings() -> Result<()> {
        let mut config = Config::default();
        config.enable_all();

        let mut warnings = vec![];
        assert!(config
            .add_warnings(true, anyhow!("test"), &mut warnings)
            .is_ok());
        assert_eq!(4, warnings.len());
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn build_all_idempotent() -> Result<()> {
        setup();
        let config = EmitBuilder::builder()
            .idempotent()
            .all_cargo()
            .test_emit()?;
        assert_eq!(4, config.cargo_rustc_env_map.len());
        assert_eq!(0, count_idempotent(config.cargo_rustc_env_map));
        assert_eq!(0, config.warnings.len());
        teardown();
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn build_all() -> Result<()> {
        setup();
        let config = EmitBuilder::builder().all_cargo().test_emit()?;
        assert_eq!(4, config.cargo_rustc_env_map.len());
        assert_eq!(0, count_idempotent(config.cargo_rustc_env_map));
        assert_eq!(0, config.warnings.len());
        teardown();
        Ok(())
    }
}
