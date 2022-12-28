// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use crate::{
    builder::{Builder, RustcEnvMap},
    key::VergenKey,
};
use anyhow::{Error, Result};
use std::env;

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Config {
    pub(crate) cargo_features: bool,
    pub(crate) cargo_profile: bool,
    pub(crate) cargo_target_triple: bool,
}

impl Config {
    pub(crate) fn add_warnings(
        self,
        skip_if_error: bool,
        e: Error,
        warnings: &mut Vec<String>,
    ) -> Result<()> {
        if skip_if_error {
            if self.cargo_features {
                warnings.push(format!(
                    "Unable to add {} to output",
                    VergenKey::CargoFeatures.name()
                ));
            }
            if self.cargo_profile {
                warnings.push(format!(
                    "Unable to add {} to output",
                    VergenKey::CargoProfile.name()
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

/// The `VERGEN_CARGO_*` configuration features
///
/// | Variable | Sample |
/// | -------  | ------ |
/// | `VERGEN_CARGO_FEATURES` | git,build |
/// | `VERGEN_CARGO_PROFILE` | debug |
/// | `VERGEN_CARGO_TARGET_TRIPLE` | x86_64-unknown-linux-gnu |
///
/// # Example
///
/// ```
/// # use anyhow::Result;
/// # use std::env;
/// # use vergen::Vergen;
/// #
/// # fn main() -> Result<()> {
/// # env::set_var("TARGET", "x86_64-unknown-linux-gnu");
/// # env::set_var("PROFILE", "build,rustc");
/// # env::set_var("CARGO_FEATURE_BUILD", "");
/// Vergen::default().all_cargo().gen()?;
/// # env::remove_var("TARGET");
/// # env::remove_var("PROFILE");
/// # env::remove_var("CARGO_FEATURE_BUILD");
/// #   Ok(())
/// # }
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "cargo")))]
impl Builder {
    /// Enable all of the `VERGEN_CARGO_*` options
    pub fn all_cargo(&mut self) -> &mut Self {
        self.cargo_features().cargo_profile().cargo_target_triple()
    }

    /// Enable the cargo features
    pub fn cargo_features(&mut self) -> &mut Self {
        self.cargo_config.cargo_features = true;
        self
    }

    /// Enable the cargo profile
    pub fn cargo_profile(&mut self) -> &mut Self {
        self.cargo_config.cargo_profile = true;
        self
    }

    /// Enable cargo target triple
    pub fn cargo_target_triple(&mut self) -> &mut Self {
        self.cargo_config.cargo_target_triple = true;
        self
    }

    pub(crate) fn add_cargo_map_entries(&self, map: &mut RustcEnvMap) -> Result<()> {
        if self.cargo_config.cargo_target_triple {
            let _old = map.insert(VergenKey::CargoTargetTriple, env::var("TARGET")?);
        }

        if self.cargo_config.cargo_profile {
            let _old = map.insert(VergenKey::CargoProfile, env::var("PROFILE")?);
        }

        if self.cargo_config.cargo_features {
            let features: Vec<String> = env::vars().filter_map(is_cargo_feature).collect();
            let feature_str = features.as_slice().join(",");
            let _old = map.insert(VergenKey::CargoFeatures, feature_str);
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
        builder::test::count_idempotent,
        utils::testutils::{setup, teardown},
        Vergen,
    };
    use anyhow::Result;

    #[test]
    #[serial_test::serial]
    fn build_all_idempotent() -> Result<()> {
        setup();
        let config = Vergen::default().idempotent().all_cargo().test_gen()?;
        assert_eq!(3, config.cargo_rustc_env_map.len());
        assert_eq!(0, count_idempotent(config.cargo_rustc_env_map));
        assert_eq!(0, config.warnings.len());
        teardown();
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn build_all() -> Result<()> {
        setup();
        let config = Vergen::default().all_cargo().test_gen()?;
        assert_eq!(3, config.cargo_rustc_env_map.len());
        assert_eq!(0, count_idempotent(config.cargo_rustc_env_map));
        assert_eq!(0, config.warnings.len());
        teardown();
        Ok(())
    }
}
