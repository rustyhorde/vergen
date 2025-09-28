use anyhow::Result;
use std::env;
#[cfg(feature = "__vergen_test")]
use vergen::EmitBuilder;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-check-cfg=cfg(coverage_nightly)");
    nightly();
    setup_env()
}

fn setup_env() -> Result<()> {
    if env::var("CARGO_FEATURE___VERGEN_TEST").is_ok() {
        emit()?;
    }
    Ok(())
}

#[cfg(not(feature = "__vergen_test"))]
fn emit() -> Result<()> {
    Ok(())
}

#[cfg(feature = "__vergen_test")]
fn emit() -> Result<()> {
    println!("cargo:warning=VERGEN TEST ENABLED!");
    EmitBuilder::builder()
        .all_build()
        .all_cargo()
        .all_git()
        .all_rustc()
        .all_sysinfo()
        .emit()
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
