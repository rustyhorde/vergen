pub fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    nightly();
    beta();
    stable();
    msrv();
    lints_fix();
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
