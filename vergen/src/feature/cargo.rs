// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#[cfg(feature = "cargo_metadata")]
use self::cargo_builder::SetDependencies;
use self::cargo_builder::{SetDebug, SetFeatures, SetOptLevel, SetTargetTriple};
#[cfg(feature = "cargo_metadata")]
use anyhow::anyhow;
use anyhow::{Error, Result};
use bon::Builder;
#[cfg(feature = "cargo_metadata")]
use cargo_metadata::{DepKindInfo, DependencyKind, MetadataCommand, Package, PackageId};
#[cfg(feature = "cargo_metadata")]
use regex::Regex;
use std::env;
#[cfg(feature = "cargo_metadata")]
use vergen_lib::constants::CARGO_DEPENDENCIES;
use vergen_lib::{
    AddEntries, CargoRerunIfChanged, CargoRustcEnvMap, CargoWarning, DefaultConfig, VergenKey,
    add_default_map_entry, add_map_entry,
    constants::{CARGO_DEBUG, CARGO_FEATURES, CARGO_OPT_LEVEL, CARGO_TARGET_TRIPLE},
};

/// Configure the emission of `VERGEN_CARGO_*` instructions
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
/// # use vergen::Emitter;
/// # use vergen::Cargo;
/// #
/// fn main() -> Result<()> {
///     temp_env::with_vars([
///         ("CARGO_FEATURE_BUILD", Some("")),
///         ("DEBUG", Some("true")),
///         ("OPT_LEVEL", Some("1")),
///         ("TARGET", Some("x86_64-unknown-linux-gnu"))
///     ], || {
/// #        let result = || -> Result<()> {
///         let cargo = Cargo::all_cargo();
///         Emitter::default().add_instructions(&cargo)?.emit()?;
/// #        Ok(())
/// #        }();
///     });
/// #    Ok(())
/// # }
/// ```
/// Emit some of the cargo instructions
///
/// ```
/// # use anyhow::Result;
/// # use vergen::Emitter;
/// # use vergen::Cargo;
/// #
/// # fn main() -> Result<()> {
///     temp_env::with_vars([
///         ("OPT_LEVEL", Some("1")),
///         ("TARGET", Some("x86_64-unknown-linux-gnu"))
///     ], || {
/// #        let result = || -> Result<()> {
///         let cargo = Cargo::builder().opt_level(true).build();
///         Emitter::default().add_instructions(&cargo)?.emit()?;
/// #        Ok(())
/// #        }();
///     });
/// #    Ok(())
/// # }
/// ```
///
/// Override output with your own value
///
/// ```
/// # use anyhow::Result;
/// # use std::env;
/// # use vergen::Emitter;
/// # use vergen::Cargo;
/// #
/// fn main() -> Result<()> {
///     temp_env::with_vars([
///         ("CARGO_FEATURE_BUILD", Some("")),
///         ("VERGEN_CARGO_DEBUG", Some("my own debug value")),
///         ("OPT_LEVEL", Some("1")),
///         ("TARGET", Some("x86_64-unknown-linux-gnu"))
///      ], || {
/// #        let result = || -> Result<()> {
///          let cargo = Cargo::all_cargo();
///          Emitter::default().add_instructions(&cargo)?.emit()?;
/// #        Ok(())
/// #        }();
///      });
/// #     Ok(())
/// # }
/// ```
///
#[derive(Builder, Clone, Copy, Debug, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub struct Cargo {
    /// Configures the default values.
    /// If set to `true` all defaults are in "enabled" state.
    /// If set to `false` all defaults are in "disabled" state.
    #[builder(field)]
    all: bool,
    /// Emit the DEBUG value set by cargo
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_CARGO_DEBUG=true|false
    /// ```
    ///
    #[builder(default = all)]
    debug: bool,
    /// Emit the `CARGO_FEATURE_*` values set by cargo
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_CARGO_FEATURES=<features>
    /// ```
    ///
    #[builder(default = all)]
    features: bool,
    /// Emit the `OPT_LEVEL` value set by cargo
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_CARGO_OPT_LEVEL=<opt_level>
    /// ```
    ///
    #[builder(default = all)]
    opt_level: bool,
    /// Emit the TARGET value set by cargo
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_CARGO_TARGET_TRIPLE=<target_triple>
    /// ```
    ///
    #[builder(default = all)]
    target_triple: bool,
    /// Emit the dependencies value derived from `Cargo.toml`
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_CARGO_DEPENDENCIES=<dependencies>
    /// ```
    ///
    #[cfg(feature = "cargo_metadata")]
    #[builder(default = all)]
    dependencies: bool,
    /// Add a name [`Regex`](regex::Regex) filter for cargo dependencies
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_CARGO_DEPENDENCIES=<deps_filtered_by_name>
    /// ```
    ///
    #[cfg(feature = "cargo_metadata")]
    #[builder(into)]
    name_filter: Option<&'static str>,
    /// Add a [`DependencyKind`](cargo_metadata::DependencyKind) filter for cargo dependencies
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_CARGO_DEPENDENCIES=<deps_filtered_by_kind>
    /// ```
    ///
    #[cfg(feature = "cargo_metadata")]
    #[builder(into)]
    dep_kind_filter: Option<DependencyKind>,
}

impl<S: cargo_builder::State> CargoBuilder<S> {
    /// Convenience method that switches the defaults of [`CargoBuilder`]
    /// to enable all of the `VERGEN_CARGO_*` instructions. It can only be
    /// called at the start of the building process, i.e. when no config
    /// has been set yet to avoid overwrites.
    fn all(mut self) -> Self {
        self.all = true;
        self
    }
}

impl Cargo {
    /// Emit all of the `VERGEN_CARGO_*` instructions
    #[must_use]
    pub fn all_cargo() -> Self {
        Self::builder().all().build()
    }

    /// Emit all of the `VERGEN_CARGO_*` instructions and return the builder
    #[cfg(not(feature = "cargo_metadata"))]
    pub fn all_cargo_builder() -> CargoBuilder<SetTargetTriple<SetOptLevel<SetFeatures<SetDebug>>>>
    {
        Self::builder()
            .debug(true)
            .features(true)
            .opt_level(true)
            .target_triple(true)
    }

    /// Emit all of the `VERGEN_CARGO_*` instructions and return the builder
    #[cfg(feature = "cargo_metadata")]
    pub fn all_cargo_builder()
    -> CargoBuilder<SetDependencies<SetTargetTriple<SetOptLevel<SetFeatures<SetDebug>>>>> {
        Self::builder()
            .debug(true)
            .features(true)
            .opt_level(true)
            .target_triple(true)
            .dependencies(true)
    }

    #[cfg(not(feature = "cargo_metadata"))]
    fn any(self) -> bool {
        self.debug || self.features || self.opt_level || self.target_triple
    }

    #[cfg(feature = "cargo_metadata")]
    fn any(self) -> bool {
        self.debug || self.features || self.opt_level || self.target_triple || self.dependencies
    }

    fn is_cargo_feature(var: (String, String)) -> Option<String> {
        let (k, _) = var;
        if k.starts_with("CARGO_FEATURE_") {
            Some(k.replace("CARGO_FEATURE_", "").to_lowercase())
        } else {
            None
        }
    }

    #[cfg(feature = "cargo_metadata")]
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

        let regex_opt = if let Some(name_regex) = name_filter {
            Regex::new(name_regex).ok()
        } else {
            None
        };
        let results: Vec<String> = packages
            .iter()
            .filter_map(|(package, dep_kind_info)| {
                if let Some(regex) = &regex_opt {
                    if regex.is_match(&package.name) {
                        Some((package, dep_kind_info))
                    } else {
                        None
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

    /// Add a name [`Regex`](regex::Regex) filter for cargo dependencies
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_CARGO_DEPENDENCIES=<deps_filtered_by_name>
    /// ```
    ///
    #[cfg(feature = "cargo_metadata")]
    pub fn set_name_filter(&mut self, val: Option<&'static str>) -> &mut Self {
        self.name_filter = val;
        self
    }
    /// Add a [`DependencyKind`](cargo_metadata::DependencyKind) filter for cargo dependencies
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_CARGO_DEPENDENCIES=<deps_filtered_by_kind>
    /// ```
    ///
    #[cfg(feature = "cargo_metadata")]
    pub fn set_dep_kind_filter(&mut self, val: Option<DependencyKind>) -> &mut Self {
        self.dep_kind_filter = val;
        self
    }

    #[cfg(not(feature = "cargo_metadata"))]
    #[allow(
        clippy::unnecessary_wraps,
        clippy::unused_self,
        clippy::trivially_copy_pass_by_ref
    )]
    fn add_dependencies(&self, _cargo_rustc_env: &mut CargoRustcEnvMap) -> Result<()> {
        Ok(())
    }

    #[cfg(feature = "cargo_metadata")]
    fn add_dependencies(&self, cargo_rustc_env: &mut CargoRustcEnvMap) -> Result<()> {
        if self.dependencies {
            if let Ok(value) = env::var(CARGO_DEPENDENCIES) {
                add_map_entry(VergenKey::CargoDependencies, value, cargo_rustc_env);
            } else {
                let value = Self::get_dependencies(self.name_filter, self.dep_kind_filter)?;
                if !value.is_empty() {
                    add_map_entry(VergenKey::CargoDependencies, value, cargo_rustc_env);
                }
            }
        }
        Ok(())
    }

    #[cfg(not(feature = "cargo_metadata"))]
    #[allow(clippy::unused_self, clippy::trivially_copy_pass_by_ref)]
    fn add_default_dependencies(
        &self,
        _config: &DefaultConfig,
        _cargo_rustc_env_map: &mut CargoRustcEnvMap,
        _cargo_warning: &mut CargoWarning,
    ) {
    }

    #[cfg(feature = "cargo_metadata")]
    fn add_default_dependencies(
        &self,
        config: &DefaultConfig,
        cargo_rustc_env_map: &mut CargoRustcEnvMap,
        cargo_warning: &mut CargoWarning,
    ) {
        if self.dependencies {
            add_default_map_entry(
                *config.idempotent(),
                VergenKey::CargoDependencies,
                cargo_rustc_env_map,
                cargo_warning,
            );
        }
    }
}

impl AddEntries for Cargo {
    fn add_map_entries(
        &self,
        _idempotent: bool,
        cargo_rustc_env: &mut CargoRustcEnvMap,
        _cargo_rerun_if_changed: &mut CargoRerunIfChanged,
        _cargo_warning: &mut CargoWarning,
    ) -> Result<()> {
        if self.any() {
            if self.debug {
                if let Ok(value) = env::var(CARGO_DEBUG) {
                    add_map_entry(VergenKey::CargoDebug, value, cargo_rustc_env);
                } else {
                    add_map_entry(VergenKey::CargoDebug, env::var("DEBUG")?, cargo_rustc_env);
                }
            }

            if self.features {
                if let Ok(value) = env::var(CARGO_FEATURES) {
                    add_map_entry(VergenKey::CargoFeatures, value, cargo_rustc_env);
                } else {
                    let features: Vec<String> =
                        env::vars().filter_map(Self::is_cargo_feature).collect();
                    let feature_str = features.as_slice().join(",");
                    add_map_entry(VergenKey::CargoFeatures, feature_str, cargo_rustc_env);
                }
            }

            if self.opt_level {
                if let Ok(value) = env::var(CARGO_OPT_LEVEL) {
                    add_map_entry(VergenKey::CargoOptLevel, value, cargo_rustc_env);
                } else {
                    add_map_entry(
                        VergenKey::CargoOptLevel,
                        env::var("OPT_LEVEL")?,
                        cargo_rustc_env,
                    );
                }
            }

            if self.target_triple {
                if let Ok(value) = env::var(CARGO_TARGET_TRIPLE) {
                    add_map_entry(VergenKey::CargoTargetTriple, value, cargo_rustc_env);
                } else {
                    add_map_entry(
                        VergenKey::CargoTargetTriple,
                        env::var("TARGET")?,
                        cargo_rustc_env,
                    );
                }
            }

            self.add_dependencies(cargo_rustc_env)?;
        }
        Ok(())
    }

    fn add_default_entries(
        &self,
        config: &DefaultConfig,
        cargo_rustc_env_map: &mut CargoRustcEnvMap,
        _cargo_rerun_if_changed: &mut CargoRerunIfChanged,
        cargo_warning: &mut CargoWarning,
    ) -> Result<()> {
        if *config.fail_on_error() {
            let error = Error::msg(format!("{:?}", config.error()));
            Err(error)
        } else {
            if self.debug {
                add_default_map_entry(
                    *config.idempotent(),
                    VergenKey::CargoDebug,
                    cargo_rustc_env_map,
                    cargo_warning,
                );
            }
            if self.features {
                add_default_map_entry(
                    *config.idempotent(),
                    VergenKey::CargoFeatures,
                    cargo_rustc_env_map,
                    cargo_warning,
                );
            }
            if self.opt_level {
                add_default_map_entry(
                    *config.idempotent(),
                    VergenKey::CargoOptLevel,
                    cargo_rustc_env_map,
                    cargo_warning,
                );
            }
            if self.target_triple {
                add_default_map_entry(
                    *config.idempotent(),
                    VergenKey::CargoTargetTriple,
                    cargo_rustc_env_map,
                    cargo_warning,
                );
            }
            self.add_default_dependencies(config, cargo_rustc_env_map, cargo_warning);
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::Cargo;
    use crate::Emitter;
    use anyhow::Result;
    use serial_test::serial;
    use std::io::Write;
    use test_util::{with_cargo_vars, with_cargo_vars_ext};
    use vergen_lib::count_idempotent;

    #[test]
    #[serial]
    #[allow(clippy::clone_on_copy, clippy::redundant_clone)]
    fn cargo_clone_works() -> Result<()> {
        let cargo = Cargo::all_cargo();
        let another = cargo.clone();
        assert_eq!(another, cargo);
        Ok(())
    }

    #[test]
    #[serial]
    fn cargo_debug_works() -> Result<()> {
        let cargo = Cargo::all_cargo();
        let mut buf = vec![];
        write!(buf, "{cargo:?}")?;
        assert!(!buf.is_empty());
        Ok(())
    }

    #[test]
    #[serial]
    fn cargo_default() -> Result<()> {
        let cargo = Cargo::builder().build();
        let emitter = Emitter::default().add_instructions(&cargo)?.test_emit();
        assert_eq!(0, emitter.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(0, emitter.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn all_idempotent() {
        let result = with_cargo_vars(|| {
            let cargo = Cargo::all_cargo();
            let config = Emitter::default()
                .idempotent()
                .add_instructions(&cargo)?
                .test_emit();
            #[cfg(feature = "cargo_metadata")]
            assert_eq!(5, config.cargo_rustc_env_map().len());
            #[cfg(not(feature = "cargo_metadata"))]
            assert_eq!(4, config.cargo_rustc_env_map().len());
            assert_eq!(0, count_idempotent(config.cargo_rustc_env_map()));
            assert_eq!(0, config.cargo_warning().len());
            Ok(())
        });
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn all() {
        let result = with_cargo_vars(|| {
            let cargo = Cargo::all_cargo();
            let config = Emitter::default().add_instructions(&cargo)?.test_emit();
            #[cfg(feature = "cargo_metadata")]
            assert_eq!(5, config.cargo_rustc_env_map().len());
            #[cfg(not(feature = "cargo_metadata"))]
            assert_eq!(4, config.cargo_rustc_env_map().len());
            assert_eq!(0, count_idempotent(config.cargo_rustc_env_map()));
            assert_eq!(0, config.cargo_warning().len());
            Ok(())
        });
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn debug() {
        let result = with_cargo_vars(|| {
            let cargo = Cargo::builder().debug(true).build();
            let config = Emitter::default().add_instructions(&cargo)?.test_emit();
            assert_eq!(1, config.cargo_rustc_env_map().len());
            assert_eq!(0, count_idempotent(config.cargo_rustc_env_map()));
            assert_eq!(0, config.cargo_warning().len());
            Ok(())
        });
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn features() {
        let result = with_cargo_vars(|| {
            let cargo = Cargo::builder().features(true).build();
            let config = Emitter::default().add_instructions(&cargo)?.test_emit();
            assert_eq!(1, config.cargo_rustc_env_map().len());
            assert_eq!(0, count_idempotent(config.cargo_rustc_env_map()));
            assert_eq!(0, config.cargo_warning().len());
            Ok(())
        });
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn opt_level() {
        let result = with_cargo_vars(|| {
            let cargo = Cargo::builder().opt_level(true).build();
            let config = Emitter::default().add_instructions(&cargo)?.test_emit();
            assert_eq!(1, config.cargo_rustc_env_map().len());
            assert_eq!(0, count_idempotent(config.cargo_rustc_env_map()));
            assert_eq!(0, config.cargo_warning().len());
            Ok(())
        });
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn target_triple() {
        let result = with_cargo_vars(|| {
            let cargo = Cargo::builder().target_triple(true).build();
            let config = Emitter::default().add_instructions(&cargo)?.test_emit();
            assert_eq!(1, config.cargo_rustc_env_map().len());
            assert_eq!(0, count_idempotent(config.cargo_rustc_env_map()));
            assert_eq!(0, config.cargo_warning().len());
            Ok(())
        });
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    #[cfg(feature = "cargo_metadata")]
    fn dependencies() {
        let result = with_cargo_vars(|| {
            let cargo = Cargo::builder()
                .dependencies(true)
                .name_filter("anyhow")
                .build();
            let config = Emitter::default().add_instructions(&cargo)?.test_emit();
            assert_eq!(1, config.cargo_rustc_env_map().len());
            assert_eq!(0, count_idempotent(config.cargo_rustc_env_map()));
            assert_eq!(0, config.cargo_warning().len());
            Ok(())
        });
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    #[cfg(feature = "cargo_metadata")]
    fn dependencies_bad_name_filter() {
        let result = with_cargo_vars(|| {
            let cargo = Cargo::builder().dependencies(true).name_filter("(").build();
            let config = Emitter::default().add_instructions(&cargo)?.test_emit();
            assert_eq!(1, config.cargo_rustc_env_map().len());
            assert_eq!(0, count_idempotent(config.cargo_rustc_env_map()));
            assert_eq!(0, config.cargo_warning().len());
            Ok(())
        });
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn bad_env_fails() -> Result<()> {
        let cargo = Cargo::all_cargo();
        assert!(
            Emitter::default()
                .fail_on_error()
                .add_instructions(&cargo)
                .is_err()
        );
        Ok(())
    }

    #[test]
    #[serial]
    fn bad_env_emits_warnings() -> Result<()> {
        let cargo = Cargo::all_cargo();
        let config = Emitter::default().add_instructions(&cargo)?.test_emit();
        assert_eq!(0, config.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(config.cargo_rustc_env_map()));
        #[cfg(feature = "cargo_metadata")]
        assert_eq!(5, config.cargo_warning().len());
        #[cfg(not(feature = "cargo_metadata"))]
        assert_eq!(4, config.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn bad_env_emits_idempotent() -> Result<()> {
        let cargo = Cargo::all_cargo();
        let config = Emitter::default()
            .idempotent()
            .add_instructions(&cargo)?
            .test_emit();
        #[cfg(feature = "cargo_metadata")]
        assert_eq!(5, config.cargo_rustc_env_map().len());
        #[cfg(not(feature = "cargo_metadata"))]
        assert_eq!(4, config.cargo_rustc_env_map().len());
        #[cfg(feature = "cargo_metadata")]
        assert_eq!(5, count_idempotent(config.cargo_rustc_env_map()));
        #[cfg(not(feature = "cargo_metadata"))]
        assert_eq!(4, count_idempotent(config.cargo_rustc_env_map()));
        #[cfg(feature = "cargo_metadata")]
        assert_eq!(5, config.cargo_warning().len());
        #[cfg(not(feature = "cargo_metadata"))]
        assert_eq!(4, config.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn cargo_debug_override_works() {
        let result = with_cargo_vars_ext(
            &[("VERGEN_CARGO_DEBUG", Some("this is a bad date"))],
            || {
                let mut stdout_buf = vec![];
                let cargo = Cargo::all_cargo();
                assert!(
                    Emitter::default()
                        .add_instructions(&cargo)?
                        .emit_to(&mut stdout_buf)
                        .is_ok()
                );
                let output = String::from_utf8_lossy(&stdout_buf);
                assert!(output.contains("cargo:rustc-env=VERGEN_CARGO_DEBUG=this is a bad date"));
                Ok(())
            },
        );
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn cargo_features_override_works() {
        let result = with_cargo_vars_ext(
            &[("VERGEN_CARGO_FEATURES", Some("this is a bad date"))],
            || {
                let mut stdout_buf = vec![];
                let cargo = Cargo::all_cargo();
                assert!(
                    Emitter::default()
                        .add_instructions(&cargo)?
                        .emit_to(&mut stdout_buf)
                        .is_ok()
                );
                let output = String::from_utf8_lossy(&stdout_buf);
                assert!(
                    output.contains("cargo:rustc-env=VERGEN_CARGO_FEATURES=this is a bad date")
                );
                Ok(())
            },
        );
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn cargo_opt_level_override_works() {
        let result = with_cargo_vars_ext(
            &[("VERGEN_CARGO_OPT_LEVEL", Some("this is a bad date"))],
            || {
                let mut stdout_buf = vec![];
                let cargo = Cargo::all_cargo();
                assert!(
                    Emitter::default()
                        .add_instructions(&cargo)?
                        .emit_to(&mut stdout_buf)
                        .is_ok()
                );
                let output = String::from_utf8_lossy(&stdout_buf);
                assert!(
                    output.contains("cargo:rustc-env=VERGEN_CARGO_OPT_LEVEL=this is a bad date")
                );
                Ok(())
            },
        );
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn cargo_target_triple_override_works() {
        let result = with_cargo_vars_ext(
            &[("VERGEN_CARGO_TARGET_TRIPLE", Some("this is a bad date"))],
            || {
                let mut stdout_buf = vec![];
                let cargo = Cargo::all_cargo();
                assert!(
                    Emitter::default()
                        .add_instructions(&cargo)?
                        .emit_to(&mut stdout_buf)
                        .is_ok()
                );
                let output = String::from_utf8_lossy(&stdout_buf);
                assert!(
                    output
                        .contains("cargo:rustc-env=VERGEN_CARGO_TARGET_TRIPLE=this is a bad date")
                );
                Ok(())
            },
        );
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    #[cfg(feature = "cargo_metadata")]
    fn cargo_dependencies_override_works() {
        let result = with_cargo_vars_ext(
            &[("VERGEN_CARGO_DEPENDENCIES", Some("this is a bad date"))],
            || {
                let mut stdout_buf = vec![];
                let cargo = Cargo::all_cargo();
                assert!(
                    Emitter::default()
                        .add_instructions(&cargo)?
                        .emit_to(&mut stdout_buf)
                        .is_ok()
                );
                let output = String::from_utf8_lossy(&stdout_buf);
                assert!(
                    output.contains("cargo:rustc-env=VERGEN_CARGO_DEPENDENCIES=this is a bad date")
                );
                Ok(())
            },
        );
        assert!(result.is_ok());
    }
}
