pub fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    nightly();
    beta();
    stable();
    msrv();
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
