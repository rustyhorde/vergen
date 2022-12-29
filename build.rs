pub fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    // These are set for some documentation tests
    println!("cargo:rustc-env=VERGEN_BUILD_TIMESTAMP=2022-12-28T21:56:23.764785796Z");
    println!("cargo:rustc-env=VERGEN_GIT_DESCRIBE=7.4.4-16-g2f35555");
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

#[rustversion::before(1.65)]
fn msrv() {}

#[rustversion::since(1.65)]
fn msrv() {
    println!("cargo:rustc-cfg=msrv");
}
