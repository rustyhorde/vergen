use chrono::Utc;

pub fn main() {
    // These are here so some doc tests work
    let now = Utc::now();
    println!(
        "cargo:rustc-env=VERGEN_BUILD_TIMESTAMP={}",
        now.to_rfc3339()
    );
    println!("cargo:rustc-env=VERGEN_GIT_SEMVER=v3.2.0-86-g95fc0f5");
    nightly_lints();
}

#[rustversion::nightly]
fn nightly_lints() {
    println!("cargo:rustc-cfg=nightly_lints");
}

#[rustversion::not(nightly)]
fn nightly_lints() {}
