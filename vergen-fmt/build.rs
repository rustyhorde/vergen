use anyhow::Result;
use std::env;
#[cfg(feature = "__vergen_test")]
use vergen::EmitBuilder;

fn main() -> Result<()> {
    nightly();
    beta();
    stable();
    msrv();
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

#[rustversion::before(1.66)]
fn msrv() {}

#[rustversion::since(1.66)]
fn msrv() {
    println!("cargo:rustc-cfg=msrv");
}
