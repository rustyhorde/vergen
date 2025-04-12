#[cfg(feature = "rustc")]
mod test_rustc {
    use std::sync::LazyLock;

    use anyhow::Result;
    use regex::Regex;
    use vergen::{Emitter, Rustc};

    static RUSTC_CHANNEL_RE_STR: LazyLock<&'static str> =
        LazyLock::new(|| r"cargo:rustc-env=VERGEN_RUSTC_CHANNEL=.*");
    static RUSTC_CD_RE_STR: LazyLock<&'static str> =
        LazyLock::new(|| r"cargo:rustc-env=VERGEN_RUSTC_COMMIT_DATE=\d{4}-\d{2}-\d{2}");
    static RUSTC_CH_RE_STR: LazyLock<&'static str> =
        LazyLock::new(|| r"cargo:rustc-env=VERGEN_RUSTC_COMMIT_HASH=[0-9a-f]{40}");
    static RUSTC_HT_RE_STR: LazyLock<&'static str> =
        LazyLock::new(|| r"cargo:rustc-env=VERGEN_RUSTC_HOST_TRIPLE=.*");
    static RUSTC_LLVM_RE_STR: LazyLock<&'static str> =
        LazyLock::new(|| r"cargo:rustc-env=VERGEN_RUSTC_LLVM_VERSION=\d{2}\.\d{1}");
    static RUSTC_SEMVER_RE_STR: LazyLock<&'static str> = LazyLock::new(
        || r"cargo:rustc-env=VERGEN_RUSTC_SEMVER=(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?",
    );
    static RUSTC_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        let re_str = [
            *RUSTC_CHANNEL_RE_STR,
            *RUSTC_CD_RE_STR,
            *RUSTC_CH_RE_STR,
            *RUSTC_HT_RE_STR,
            *RUSTC_LLVM_RE_STR,
            *RUSTC_SEMVER_RE_STR,
        ]
        .join("\n");
        Regex::new(&re_str).unwrap()
    });

    #[test]
    fn rustc_all_output() -> Result<()> {
        let mut stdout_buf = vec![];
        let rustc = Rustc::all_rustc();
        Emitter::default()
            .add_instructions(&rustc)?
            .emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(RUSTC_REGEX.is_match(&output));
        Ok(())
    }

    #[test]
    fn rustc_all_idempotent_output() -> Result<()> {
        let mut stdout_buf = vec![];
        let rustc = Rustc::all_rustc();
        Emitter::default()
            .idempotent()
            .add_instructions(&rustc)?
            .emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(RUSTC_REGEX.is_match(&output));
        Ok(())
    }
}
