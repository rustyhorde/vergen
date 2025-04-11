use anyhow::Result;
use std::env;
#[cfg(all(feature = "__vergen_empty_test", not(feature = "__vergen_test")))]
use vergen_gix::Emitter;
#[cfg(all(feature = "__vergen_test", not(feature = "__vergen_empty_test")))]
use {
    std::collections::BTreeMap,
    vergen_gix::{
        AddCustomEntries, Build, Cargo, CargoRerunIfChanged, CargoWarning, DefaultConfig, Emitter,
        GixBuilder, Rustc, Sysinfo,
    },
};

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    nightly();
    beta();
    stable();
    setup_env()
}

fn setup_env() -> Result<()> {
    if env::var("CARGO_FEATURE___VERGEN_TEST").is_ok() {
        emit()?;
    }
    Ok(())
}

#[cfg(all(feature = "__vergen_test", feature = "__vergen_empty_test"))]
fn emit() -> Result<()> {
    Ok(())
}

#[cfg(not(any(feature = "__vergen_test", feature = "__vergen_empty_test")))]
fn emit() -> Result<()> {
    Ok(())
}

#[cfg(all(feature = "__vergen_test", not(feature = "__vergen_empty_test")))]
#[derive(Default)]
struct Custom {}

#[cfg(all(feature = "__vergen_test", not(feature = "__vergen_empty_test")))]
impl AddCustomEntries<&str, &str> for Custom {
    fn add_calculated_entries(
        &self,
        _idempotent: bool,
        cargo_rustc_env_map: &mut BTreeMap<&str, &str>,
        _cargo_rerun_if_changed: &mut CargoRerunIfChanged,
        cargo_warning: &mut CargoWarning,
    ) -> Result<()> {
        cargo_rustc_env_map.insert("vergen-cl", "custom_instruction");
        cargo_warning.push("custom instruction generated".to_string());
        Ok(())
    }

    fn add_default_entries(
        &self,
        _config: &DefaultConfig,
        _cargo_rustc_env_map: &mut BTreeMap<&str, &str>,
        _cargo_rerun_if_changed: &mut CargoRerunIfChanged,
        _cargo_warning: &mut CargoWarning,
    ) -> Result<()> {
        Ok(())
    }
}

#[cfg(all(feature = "__vergen_test", not(feature = "__vergen_empty_test")))]
fn emit() -> Result<()> {
    println!("cargo:warning=VERGEN TEST ENABLED!");
    let build = Build::all_build();
    let cargo = Cargo::all_cargo();
    let gix = GixBuilder::all_git()?;
    let rustc = Rustc::all_rustc();
    let si = Sysinfo::all_sysinfo();
    Emitter::default()
        .add_instructions(&build)?
        .add_instructions(&cargo)?
        .add_instructions(&gix)?
        .add_instructions(&rustc)?
        .add_instructions(&si)?
        .add_custom_instructions(&Custom::default())?
        .emit()
}

#[cfg(all(not(feature = "__vergen_test"), feature = "__vergen_empty_test"))]
fn emit() -> Result<()> {
    println!("cargo:warning=VERGEN EMPTY TEST ENABLED!");
    Emitter::default().emit()
}

#[rustversion::nightly]
fn nightly() {
    println!("cargo:rustc-check-cfg=cfg(nightly)");
    println!("cargo:rustc-cfg=nightly");
}

#[rustversion::not(nightly)]
fn nightly() {
    println!("cargo:rustc-check-cfg=cfg(nightly)");
}

#[rustversion::beta]
fn beta() {
    println!("cargo:rustc-check-cfg=cfg(beta)");
    println!("cargo:rustc-cfg=beta");
}

#[rustversion::not(beta)]
fn beta() {
    println!("cargo:rustc-check-cfg=cfg(beta)");
}

#[rustversion::stable]
fn stable() {
    println!("cargo:rustc-check-cfg=cfg(stable)");
    println!("cargo:rustc-cfg=stable");
}

#[rustversion::not(stable)]
fn stable() {
    println!("cargo:rustc-check-cfg=cfg(stable)");
}
