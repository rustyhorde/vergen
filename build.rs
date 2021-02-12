use chrono::Utc;

pub fn main() {
    let now = Utc::now();
    println!(
        "cargo:rustc-env=VERGEN_BUILD_TIMESTAMP={}",
        now.to_rfc3339()
    );
    println!("cargo:rustc-env=VERGEN_GIT_SEMVER=v3.2.0-86-g95fc0f5");
}
