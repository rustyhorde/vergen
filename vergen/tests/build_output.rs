#[cfg(feature = "build")]
mod test_build {
    use anyhow::Result;
    use lazy_static::lazy_static;
    use regex::Regex;
    use std::env;
    use vergen::EmitBuilder;

    lazy_static! {
        static ref DATE_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_BUILD_DATE=\d{4}-\d{2}-\d{2}"#;
        static ref DATE_IDEM_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_BUILD_DATE=VERGEN_IDEMPOTENT_OUTPUT"#;
        static ref TIMESTAMP_RE_STR: &'static str = r#"cargo:rustc-env=VERGEN_BUILD_TIMESTAMP=([0-9]+)-(0[1-9]|1[012])-(0[1-9]|[12][0-9]|3[01])[Tt]([01][0-9]|2[0-3]):([0-5][0-9]):([0-5][0-9]|60)(\.[0-9]+)?(([Zz])|([\+|\-]([01][0-9]|2[0-3]):[0-5][0-9]))"#;
        static ref TIMESTAMP_IDEM_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_BUILD_TIMESTAMP=VERGEN_IDEMPOTENT_OUTPUT"#;
        static ref DATE_WARNING: &'static str = r#"cargo:warning=VERGEN_BUILD_DATE set to default"#;
        static ref TIMESTAMP_WARNING: &'static str =
            r#"cargo:warning=VERGEN_BUILD_TIMESTAMP set to default"#;
        static ref BUILD_REGEX_INST: Regex = {
            let re_str = vec![*DATE_RE_STR, *TIMESTAMP_RE_STR].join("\n");
            Regex::new(&re_str).unwrap()
        };
    }

    const IDEM_OUTPUT: &str = r#"cargo:rustc-env=VERGEN_BUILD_DATE=VERGEN_IDEMPOTENT_OUTPUT
cargo:rustc-env=VERGEN_BUILD_TIMESTAMP=VERGEN_IDEMPOTENT_OUTPUT
cargo:warning=VERGEN_BUILD_DATE set to default
cargo:warning=VERGEN_BUILD_TIMESTAMP set to default
cargo:rerun-if-changed=build.rs
cargo:rerun-if-env-changed=VERGEN_IDEMPOTENT
cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH
"#;

    const SOURCE_DATE_EPOCH_IDEM_OUTPUT: &str = r#"cargo:rustc-env=VERGEN_BUILD_DATE=2022-12-23
cargo:rustc-env=VERGEN_BUILD_TIMESTAMP=2022-12-23T15:29:20.000000000Z
cargo:rerun-if-changed=build.rs
cargo:rerun-if-env-changed=VERGEN_IDEMPOTENT
cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH
"#;

    const QUIET_IDEM_OUTPUT: &str = r#"cargo:rustc-env=VERGEN_BUILD_DATE=VERGEN_IDEMPOTENT_OUTPUT
cargo:rustc-env=VERGEN_BUILD_TIMESTAMP=VERGEN_IDEMPOTENT_OUTPUT
cargo:rerun-if-changed=build.rs
cargo:rerun-if-env-changed=VERGEN_IDEMPOTENT
cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH
"#;

    #[test]
    #[serial_test::serial]
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
    #[serial_test::serial]
    fn build_all_idempotent_output() -> Result<()> {
        let mut stdout_buf = vec![];
        EmitBuilder::builder()
            .idempotent()
            .all_build()
            .emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert_eq!(IDEM_OUTPUT, output);
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn build_all_idempotent_output_quiet() -> Result<()> {
        let mut stdout_buf = vec![];
        EmitBuilder::builder()
            .idempotent()
            .quiet()
            .all_build()
            .emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert_eq!(QUIET_IDEM_OUTPUT, output);
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn build_all_sde_output() -> Result<()> {
        env::set_var("SOURCE_DATE_EPOCH", "1671809360");
        let mut stdout_buf = vec![];
        EmitBuilder::builder()
            .all_build()
            .emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert_eq!(SOURCE_DATE_EPOCH_IDEM_OUTPUT, output);
        env::remove_var("SOURCE_DATE_EPOCH");
        Ok(())
    }
}
