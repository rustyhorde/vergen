use anyhow::Result;
use std::env;
#[cfg(feature = "__vergen_empty_test")]
use vergen_gix::Emitter;
#[cfg(feature = "__vergen_test")]
use {
    std::collections::BTreeMap,
    vergen_gix::{
        AddCustomEntries, BuildBuilder, CargoBuilder, CargoRerunIfChanged, CargoWarning,
        DefaultConfig, Emitter, GixBuilder, RustcBuilder, SysinfoBuilder,
    },
};

fn main() -> Result<()> {
    nightly();
    beta();
    stable();
    msrv();
    lints_fix();
    setup_env()
}

fn setup_env() -> Result<()> {
    if env::var("CARGO_FEATURE___VERGEN_TEST").is_ok() {
        emit()?;
    }
    Ok(())
}

#[cfg(all(not(feature = "__vergen_test"), not(feature = "__vergen_empty_test")))]
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
    let build = BuildBuilder::all_build()?;
    let cargo = CargoBuilder::all_cargo()?;
    let gix = GixBuilder::all_git()?;
    let rustc = RustcBuilder::all_rustc()?;
    let si = SysinfoBuilder::all_sysinfo()?;
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
    println!("cargo:rustc-cfg=nightly");
}

#[rustversion::not(nightly)]
fn nightly() {}

#[rustversion::beta]
fn beta() {
    println!("cargo:rustc-cfg=beta");
}

#[rustversion::not(beta)]
fn beta() {}

#[rustversion::stable]
fn stable() {
    println!("cargo:rustc-cfg=stable");
}

#[rustversion::not(stable)]
fn stable() {}

#[rustversion::before(1.70)]
fn msrv() {}

#[rustversion::since(1.70)]
fn msrv() {
    println!("cargo:rustc-cfg=msrv");
}

#[rustversion::before(1.75)]
fn lints_fix() {}

#[rustversion::since(1.75)]
fn lints_fix() {
    println!("cargo:rustc-cfg=lints_fix")
}
