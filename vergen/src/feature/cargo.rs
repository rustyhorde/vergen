// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use crate::{
    constants::{
        CARGO_DEBUG, CARGO_DEPENDENCIES, CARGO_FEATURES, CARGO_OPT_LEVEL, CARGO_TARGET_TRIPLE,
    },
    emitter::{EmitBuilder, RustcEnvMap},
    key::VergenKey,
    utils::fns::{add_default_map_entry, add_map_entry},
};
use anyhow::{anyhow, Error, Result};
use cargo_metadata::{DepKindInfo, DependencyKind, MetadataCommand, Package, PackageId};
use regex::Regex;
use std::env;

#[derive(Clone, Copy, Debug, Default)]
#[allow(clippy::struct_excessive_bools, clippy::struct_field_names)]
pub(crate) struct Config {
    pub(crate) cargo_debug: bool,
    pub(crate) cargo_features: bool,
    pub(crate) cargo_opt_level: bool,
    pub(crate) cargo_target_triple: bool,
    pub(crate) cargo_dependencies: bool,
    cargo_name_filter: Option<&'static str>,
    cargo_dep_kind_filter: Option<DependencyKind>,
}

impl Config {
    pub(crate) fn any(self) -> bool {
        self.cargo_debug
            || self.cargo_features
            || self.cargo_opt_level
            || self.cargo_target_triple
            || self.cargo_dependencies
    }
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
///
/// Override output with your own value
///
/// ```
/// # use anyhow::Result;
/// # use std::env;
/// # use vergen::EmitBuilder;
/// #
/// # fn main() -> Result<()> {
/// # env::set_var("DEBUG", "true");
/// # env::set_var("OPT_LEVEL", "1");
/// env::set_var("VERGEN_CARGO_DEBUG", "this is the debug I want output");
/// EmitBuilder::builder().all_cargo().emit()?;
/// # env::remove_var("VERGEN_BUILD_DATE");
/// # env::remove_var("DEBUG");
/// # env::remove_var("OPT_LEVEL");
/// #   Ok(())
/// # }
/// ```
///
#[cfg_attr(docsrs, doc(cfg(feature = "cargo")))]
impl EmitBuilder {
    /// Emit all of the `VERGEN_CARGO_*` instructions
    pub fn all_cargo(&mut self) -> &mut Self {
        self.cargo_debug()
            .cargo_features()
            .cargo_opt_level()
            .cargo_target_triple()
            .cargo_dependencies()
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

    /// Emit the dependencies value derived from `Cargo.toml`
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_CARGO_DEPENDENCIES=<dependencies>
    /// ```
    pub fn cargo_dependencies(&mut self) -> &mut Self {
        self.cargo_config.cargo_dependencies = true;
        self
    }

    /// Add a name [`Regex`](regex::Regex) filter for cargo dependencies
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_CARGO_DEPENDENCIES=<deps_filtered_by_name>
    /// ```
    pub fn cargo_dependencies_name_filter(
        &mut self,
        name_filter: Option<&'static str>,
    ) -> &mut Self {
        self.cargo_config.cargo_name_filter = name_filter;
        self
    }

    /// Add a [`DependencyKind`](cargo_metadata::DependencyKind) filter for cargo dependencies
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_CARGO_DEPENDENCIES=<deps_filtered_by_kind>
    /// ```
    pub fn cargo_dependencies_dep_kind_filter(
        &mut self,
        dep_kind_filter: Option<DependencyKind>,
    ) -> &mut Self {
        self.cargo_config.cargo_dep_kind_filter = dep_kind_filter;
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
            if self.cargo_config.cargo_dependencies {
                add_default_map_entry(VergenKey::CargoDependencies, map, warnings);
            }
            Ok(())
        }
    }

    pub(crate) fn add_cargo_map_entries(&self, map: &mut RustcEnvMap) -> Result<()> {
        if self.cargo_config.any() {
            if self.cargo_config.cargo_debug {
                if let Ok(value) = env::var(CARGO_DEBUG) {
                    add_map_entry(VergenKey::CargoDebug, value, map);
                } else {
                    add_map_entry(VergenKey::CargoDebug, env::var("DEBUG")?, map);
                }
            }

            if self.cargo_config.cargo_features {
                if let Ok(value) = env::var(CARGO_FEATURES) {
                    add_map_entry(VergenKey::CargoFeatures, value, map);
                } else {
                    let features: Vec<String> = env::vars().filter_map(is_cargo_feature).collect();
                    let feature_str = features.as_slice().join(",");
                    add_map_entry(VergenKey::CargoFeatures, feature_str, map);
                }
            }

            if self.cargo_config.cargo_opt_level {
                if let Ok(value) = env::var(CARGO_OPT_LEVEL) {
                    add_map_entry(VergenKey::CargoOptLevel, value, map);
                } else {
                    add_map_entry(VergenKey::CargoOptLevel, env::var("OPT_LEVEL")?, map);
                }
            }

            if self.cargo_config.cargo_target_triple {
                if let Ok(value) = env::var(CARGO_TARGET_TRIPLE) {
                    add_map_entry(VergenKey::CargoTargetTriple, value, map);
                } else {
                    add_map_entry(VergenKey::CargoTargetTriple, env::var("TARGET")?, map);
                }
            }

            if self.cargo_config.cargo_dependencies {
                if let Ok(value) = env::var(CARGO_DEPENDENCIES) {
                    add_map_entry(VergenKey::CargoDependencies, value, map);
                } else {
                    let value = Self::get_dependencies(
                        self.cargo_config.cargo_name_filter,
                        self.cargo_config.cargo_dep_kind_filter,
                    )?;
                    if !value.is_empty() {
                        add_map_entry(VergenKey::CargoDependencies, value, map);
                    }
                }
            }
        }
        Ok(())
    }

    fn get_dependencies(
        name_filter: Option<&'static str>,
        dep_kind_filter: Option<DependencyKind>,
    ) -> Result<String> {
        let metadata = MetadataCommand::new().exec()?;
        let resolved_crates = metadata.resolve.ok_or_else(|| anyhow!("No resolve"))?;
        let root_id = resolved_crates.root.ok_or_else(|| anyhow!("No root id"))?;
        let root = resolved_crates
            .nodes
            .into_iter()
            .find(|node| node.id == root_id)
            .ok_or_else(|| anyhow!("No root node"))?;
        let package_ids: Vec<(PackageId, Vec<DepKindInfo>)> = root
            .deps
            .into_iter()
            .map(|node_dep| (node_dep.pkg, node_dep.dep_kinds))
            .collect();

        let packages: Vec<(&Package, &Vec<DepKindInfo>)> = package_ids
            .iter()
            .filter_map(|(package_id, dep_kinds)| {
                metadata
                    .packages
                    .iter()
                    .find(|&package| package.id == *package_id)
                    .map(|package| (package, dep_kinds))
            })
            .collect();

        let results: Vec<String> = packages
            .iter()
            .filter_map(|(package, dep_kind_info)| {
                if let Some(name_regex) = name_filter {
                    if let Ok(regex) = Regex::new(name_regex) {
                        if regex.is_match(&package.name) {
                            Some((package, dep_kind_info))
                        } else {
                            None
                        }
                    } else {
                        Some((package, dep_kind_info))
                    }
                } else {
                    Some((package, dep_kind_info))
                }
            })
            .filter_map(|(package, dep_kind_info)| {
                if let Some(dep_kind_filter) = dep_kind_filter {
                    let kinds: Vec<DependencyKind> = dep_kind_info
                        .iter()
                        .map(|dep_kind_info| dep_kind_info.kind)
                        .collect();
                    if kinds.contains(&dep_kind_filter) {
                        Some(package)
                    } else {
                        None
                    }
                } else {
                    Some(package)
                }
            })
            .map(|package| format!("{} {}", package.name, package.version))
            .collect();
        Ok(results.join(","))
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
    use std::env;

    #[test]
    #[serial_test::serial]
    fn build_all_idempotent() -> Result<()> {
        setup();
        let config = EmitBuilder::builder()
            .idempotent()
            .all_cargo()
            .test_emit()?;
        assert_eq!(5, config.cargo_rustc_env_map.len());
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
        assert_eq!(5, config.cargo_rustc_env_map.len());
        assert_eq!(0, count_idempotent(&config.cargo_rustc_env_map));
        assert_eq!(0, config.warnings.len());
        teardown();
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn bad_env_fails() {
        assert!(EmitBuilder::builder()
            .fail_on_error()
            .all_cargo()
            .test_emit()
            .is_err());
    }

    #[test]
    #[serial_test::serial]
    fn bad_env_emits_default() -> Result<()> {
        let emit_res = EmitBuilder::builder().all_cargo().test_emit();
        assert!(emit_res.is_ok());
        let emit = emit_res?;
        assert_eq!(5, emit.cargo_rustc_env_map.len());
        assert_eq!(5, count_idempotent(&emit.cargo_rustc_env_map));
        assert_eq!(5, emit.warnings.len());
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn cargo_debug_override_works() -> Result<()> {
        setup();
        env::set_var("VERGEN_CARGO_DEBUG", "this is a bad date");
        let mut stdout_buf = vec![];
        assert!(EmitBuilder::builder()
            .all_cargo()
            .emit_to(&mut stdout_buf)
            .is_ok());
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(output.contains("cargo:rustc-env=VERGEN_CARGO_DEBUG=this is a bad date"));
        env::remove_var("VERGEN_CARGO_DEBUG");
        teardown();
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn cargo_features_override_works() -> Result<()> {
        setup();
        env::set_var("VERGEN_CARGO_FEATURES", "this is a bad date");
        let mut stdout_buf = vec![];
        assert!(EmitBuilder::builder()
            .all_cargo()
            .emit_to(&mut stdout_buf)
            .is_ok());
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(output.contains("cargo:rustc-env=VERGEN_CARGO_FEATURES=this is a bad date"));
        env::remove_var("VERGEN_CARGO_FEATURES");
        teardown();
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn cargo_opt_level_override_works() -> Result<()> {
        setup();
        env::set_var("VERGEN_CARGO_OPT_LEVEL", "this is a bad date");
        let mut stdout_buf = vec![];
        assert!(EmitBuilder::builder()
            .all_cargo()
            .emit_to(&mut stdout_buf)
            .is_ok());
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(output.contains("cargo:rustc-env=VERGEN_CARGO_OPT_LEVEL=this is a bad date"));
        env::remove_var("VERGEN_CARGO_OPT_LEVEL");
        teardown();
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn cargo_target_triple_override_works() -> Result<()> {
        setup();
        env::set_var("VERGEN_CARGO_TARGET_TRIPLE", "this is a bad date");
        let mut stdout_buf = vec![];
        assert!(EmitBuilder::builder()
            .all_cargo()
            .emit_to(&mut stdout_buf)
            .is_ok());
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(output.contains("cargo:rustc-env=VERGEN_CARGO_TARGET_TRIPLE=this is a bad date"));
        env::remove_var("VERGEN_CARGO_TARGET_TRIPLE");
        teardown();
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn cargo_dependencies_override_works() -> Result<()> {
        setup();
        env::set_var("VERGEN_CARGO_DEPENDENCIES", "this is a bad some dep");
        let mut stdout_buf = vec![];
        assert!(EmitBuilder::builder()
            .all_cargo()
            .emit_to(&mut stdout_buf)
            .is_ok());
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(output.contains("cargo:rustc-env=VERGEN_CARGO_DEPENDENCIES=this is a bad some dep"));
        env::remove_var("VERGEN_CARGO_DEPENDENCIES");
        teardown();
        Ok(())
    }
}
