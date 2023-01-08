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
use anyhow::{Error, Result};
use std::env;

#[derive(Clone, Copy, Debug, Default)]
#[allow(clippy::struct_excessive_bools)]
pub(crate) struct Config {
    pub(crate) cargo_debug: bool,
    pub(crate) cargo_features: bool,
    pub(crate) cargo_opt_level: bool,
    pub(crate) cargo_target_triple: bool,
}

/// Copnfigure the emission of `VERGEN_CARGO_*` instructions
///
/// **NOTE** - All cargo instructions are considered deterministic.  If you change
/// the version of cargo you are compiling with, these values should change if
/// being used in the generated binary.
///
/// | Variable | Sample |
/// | -------  | ------ |
/// | `VERGEN_CARGO_DEBUG` | true |
/// | `VERGEN_CARGO_FEATURES` | git,build |
/// | `VERGEN_CARGO_OPT_LEVEL` | 1 |
/// | `VERGEN_CARGO_TARGET_TRIPLE` | x86_64-unknown-linux-gnu |
///
/// # Example
/// Emit all of the cargo instructions
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
/// Emit some of the cargo instructions
///
/// ```
/// # use anyhow::Result;
/// # use std::env;
/// # use vergen::EmitBuilder;
/// #
/// # fn main() -> Result<()> {
/// # env::set_var("DEBUG", "true");
/// # env::set_var("OPT_LEVEL", "1");
/// EmitBuilder::builder().cargo_debug().cargo_opt_level().emit()?;
/// # env::remove_var("DEBUG");
/// # env::remove_var("OPT_LEVEL");
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

    /// Emit the `CARGO_FEATURE_*` values set by cargo
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_CARGO_FEATURES=<features>
    /// ```
    ///
    pub fn cargo_features(&mut self) -> &mut Self {
        self.cargo_config.cargo_features = true;
        self
    }

    /// Emit the `OPT_LEVEL` value set by cargo
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

    pub(crate) fn add_cargo_default(
        &self,
        e: Error,
        fail_on_error: bool,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
    ) -> Result<()> {
        if fail_on_error {
            Err(e)
        } else {
            if self.cargo_config.cargo_debug {
                add_default_map_entry(VergenKey::CargoDebug, map, warnings);
            }
            if self.cargo_config.cargo_features {
                add_default_map_entry(VergenKey::CargoFeatures, map, warnings);
            }
            if self.cargo_config.cargo_opt_level {
                add_default_map_entry(VergenKey::CargoOptLevel, map, warnings);
            }
            if self.cargo_config.cargo_target_triple {
                add_default_map_entry(VergenKey::CargoTargetTriple, map, warnings);
            }
            Ok(())
        }
    }

    pub(crate) fn add_cargo_map_entries(&self, map: &mut RustcEnvMap) -> Result<()> {
        if self.cargo_config.cargo_debug {
            add_map_entry(VergenKey::CargoDebug, env::var("DEBUG")?, map);
        }

        if self.cargo_config.cargo_features {
            let features: Vec<String> = env::vars().filter_map(is_cargo_feature).collect();
            let feature_str = features.as_slice().join(",");
            add_map_entry(VergenKey::CargoFeatures, feature_str, map);
        }

        if self.cargo_config.cargo_opt_level {
            add_map_entry(VergenKey::CargoOptLevel, env::var("OPT_LEVEL")?, map);
        }

        if self.cargo_config.cargo_target_triple {
            add_map_entry(VergenKey::CargoTargetTriple, env::var("TARGET")?, map);
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
    use crate::{
        emitter::test::count_idempotent,
        utils::testutils::{setup, teardown},
        EmitBuilder,
    };
    use anyhow::Result;

    #[test]
    #[serial_test::serial]
    fn build_all_idempotent() -> Result<()> {
        setup();
        let config = EmitBuilder::builder()
            .idempotent()
            .all_cargo()
            .test_emit()?;
        assert_eq!(4, config.cargo_rustc_env_map.len());
        assert_eq!(0, count_idempotent(&config.cargo_rustc_env_map));
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
        assert_eq!(0, count_idempotent(&config.cargo_rustc_env_map));
        assert_eq!(0, config.warnings.len());
        teardown();
        Ok(())
    }

    #[test]
    #[serial_test::parallel]
    fn bad_env_fails() {
        assert!(EmitBuilder::builder()
            .fail_on_error()
            .all_cargo()
            .test_emit()
            .is_err());
    }

    #[test]
    #[serial_test::parallel]
    fn bad_env_emits_default() -> Result<()> {
        let emit_res = EmitBuilder::builder().all_cargo().test_emit();
        assert!(emit_res.is_ok());
        let emit = emit_res?;
        assert_eq!(4, emit.cargo_rustc_env_map.len());
        assert_eq!(4, count_idempotent(&emit.cargo_rustc_env_map));
        assert_eq!(4, emit.warnings.len());
        Ok(())
    }
}
