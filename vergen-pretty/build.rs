use anyhow::Result;
use std::env;
#[cfg(feature = "__vergen_empty_test")]
use vergen_gix::Emitter;
#[cfg(feature = "__vergen_test")]
use vergen_gix::{BuildBuilder, CargoBuilder, Emitter, GixBuilder, RustcBuilder, SysinfoBuilder};

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
fn emit() -> Result<()> {
    println!("cargo:warning=VERGEN TEST ENABLED!");
    let build = BuildBuilder::default().all_build().build();
    let cargo = CargoBuilder::default().all_cargo().build();
    let rustc = RustcBuilder::default().all_rustc().build();
    let si = SysinfoBuilder::default().all_sysinfo().build();
    let gix = GixBuilder::default().all_git().build();
    Emitter::default()
        .add_instructions(&build)?
        .add_instructions(&cargo)?
        .add_instructions(&rustc)?
        .add_instructions(&si)?
        .add_instructions(&gix)?
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
