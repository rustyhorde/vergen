#[cfg(feature = "build")]
mod test_build {
    use anyhow::Result;
    use lazy_static::lazy_static;
    use regex::Regex;
    use vergen::EmitBuilder;

    lazy_static! {
        static ref DATE_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_BUILD_DATE=\d{4}-\d{2}-\d{2}"#;
        static ref DATE_IDEM_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_BUILD_DATE=VERGEN_IDEMPOTENT_OUTPUT"#;
        static ref TIME_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_BUILD_TIME=([01][0-9]|2[0-3]):([0-5][0-9]):([0-5][0-9])"#;
        static ref TIME_IDEM_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_BUILD_TIME=VERGEN_IDEMPOTENT_OUTPUT"#;
        static ref TIMESTAMP_RE_STR: &'static str = r#"cargo:rustc-env=VERGEN_BUILD_TIMESTAMP=([0-9]+)-(0[1-9]|1[012])-(0[1-9]|[12][0-9]|3[01])[Tt]([01][0-9]|2[0-3]):([0-5][0-9]):([0-5][0-9]|60)(\.[0-9]+)?(([Zz])|([\+|\-]([01][0-9]|2[0-3]):[0-5][0-9]))"#;
        static ref TIMESTAMP_IDEM_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_BUILD_TIMESTAMP=VERGEN_IDEMPOTENT_OUTPUT"#;
        static ref BUILD_SEMVER_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_BUILD_SEMVER=\d+\.\d+\.\d+"#;
        static ref DATE_WARNING: &'static str =
            r#"cargo:warning=VERGEN_BUILD_DATE set to idempotent default"#;
        static ref TIME_WARNING: &'static str =
            r#"cargo:warning=VERGEN_BUILD_TIME set to idempotent default"#;
        static ref TIMESTAMP_WARNING: &'static str =
            r#"cargo:warning=VERGEN_BUILD_TIMESTAMP set to idempotent default"#;
        static ref BUILD_REGEX_INST: Regex = {
            let re_str = vec![
                *DATE_RE_STR,
                *TIME_RE_STR,
                *TIMESTAMP_RE_STR,
                *BUILD_SEMVER_RE_STR,
            ]
            .join("\n");
            Regex::new(&re_str).unwrap()
        };
        static ref BUILD_IDEM_REGEX_INST: Regex = {
            let re_str = vec![
                *DATE_IDEM_RE_STR,
                *TIME_IDEM_RE_STR,
                *TIMESTAMP_IDEM_RE_STR,
                *BUILD_SEMVER_RE_STR,
                *DATE_WARNING,
                *TIME_WARNING,
                *TIMESTAMP_WARNING,
            ]
            .join("\n");
            Regex::new(&re_str).unwrap()
        };
    }

    #[test]
    fn build_all_output() -> Result<()> {
        let mut stdout_buf = vec![];
        EmitBuilder::builder()
            .all_build()
            .emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(BUILD_REGEX_INST.is_match(&output));
        Ok(())
    }

    #[test]
    fn build_all_idempotent_output() -> Result<()> {
        let mut stdout_buf = vec![];
        EmitBuilder::builder()
            .idempotent()
            .all_build()
            .emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(BUILD_IDEM_REGEX_INST.is_match(&output));
        Ok(())
    }
}
