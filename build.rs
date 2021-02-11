use chrono::Utc;

pub fn main() {
    let now = Utc::now();
    println!(
        "cargo:rustc-env=VERGEN_BUILD_TIMESTAMP={}",
        now.to_rfc3339()
    );
}
