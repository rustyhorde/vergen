#[cfg(feature = "build")]
mod test_build {
    use anyhow::Result;
    use lazy_static::lazy_static;
    use regex::Regex;
    use serial_test::serial;
    use vergen::BuildBuilder;
    use vergen::Emitter;
    use vergen_lib::{CustomInsGen, CustomInsGenBuilder};

    lazy_static! {
        static ref DATE_RE_STR: &'static str =
            r"cargo:rustc-env=VERGEN_BUILD_DATE=\d{4}-\d{2}-\d{2}";
        static ref DATE_IDEM_RE_STR: &'static str =
            r"cargo:rustc-env=VERGEN_BUILD_DATE=VERGEN_IDEMPOTENT_OUTPUT";
        static ref TIMESTAMP_RE_STR: &'static str = r"cargo:rustc-env=VERGEN_BUILD_TIMESTAMP=([0-9]+)-(0[1-9]|1[012])-(0[1-9]|[12][0-9]|3[01])[Tt]([01][0-9]|2[0-3]):([0-5][0-9]):([0-5][0-9]|60)(\.[0-9]+)?(([Zz])|([\+|\-]([01][0-9]|2[0-3]):[0-5][0-9]))";
        static ref TIMESTAMP_IDEM_RE_STR: &'static str =
            r"cargo:rustc-env=VERGEN_BUILD_TIMESTAMP=VERGEN_IDEMPOTENT_OUTPUT";
        static ref CUSTOM_RE_STR: &'static str = r"cargo:rustc-env=test=value";
        static ref CUSTOM_IDEM_RE_STR: &'static str =
            r"cargo:rustc-env=test=VERGEN_IDEMPOTENT_OUTPUT";
        static ref DATE_WARNING: &'static str = r"cargo:warning=VERGEN_BUILD_DATE set to default";
        static ref TIMESTAMP_WARNING: &'static str =
            r"cargo:warning=VERGEN_BUILD_TIMESTAMP set to default";
        static ref BUILD_REGEX_INST: Regex = {
            let re_str = [*DATE_RE_STR, *TIMESTAMP_RE_STR].join("\n");
            Regex::new(&re_str).unwrap()
        };
        static ref BUILD_CUSTOM_REGEX_INST: Regex = {
            let re_str = [*DATE_RE_STR, *TIMESTAMP_RE_STR, *CUSTOM_RE_STR].join("\n");
            Regex::new(&re_str).unwrap()
        };
        static ref BUILD_CUSTOM_IDEM_INST: Regex = {
            let re_str = [
                *DATE_IDEM_RE_STR,
                *TIMESTAMP_IDEM_RE_STR,
                *CUSTOM_IDEM_RE_STR,
            ]
            .join("\n");
            Regex::new(&re_str).unwrap()
        };
        static ref BUILD_CUSTOM_FAIL_IDEM_INST: Regex = {
            let re_str = [*DATE_RE_STR, *TIMESTAMP_RE_STR, *CUSTOM_IDEM_RE_STR].join("\n");
            Regex::new(&re_str).unwrap()
        };
    }

    const IDEM_OUTPUT: &str = r"cargo:rustc-env=VERGEN_BUILD_DATE=VERGEN_IDEMPOTENT_OUTPUT
cargo:rustc-env=VERGEN_BUILD_TIMESTAMP=VERGEN_IDEMPOTENT_OUTPUT
cargo:warning=VERGEN_BUILD_DATE set to default
cargo:warning=VERGEN_BUILD_TIMESTAMP set to default
cargo:rerun-if-changed=build.rs
cargo:rerun-if-env-changed=VERGEN_IDEMPOTENT
cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH
";

    const IDEM_OUTPUT_CUSTOM_BUILDRS: &str = r"cargo:rustc-env=VERGEN_BUILD_DATE=VERGEN_IDEMPOTENT_OUTPUT
cargo:rustc-env=VERGEN_BUILD_TIMESTAMP=VERGEN_IDEMPOTENT_OUTPUT
cargo:warning=VERGEN_BUILD_DATE set to default
cargo:warning=VERGEN_BUILD_TIMESTAMP set to default
cargo:rerun-if-changed=a/custom_build.rs
cargo:rerun-if-env-changed=VERGEN_IDEMPOTENT
cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH
";

    const SOURCE_DATE_EPOCH_IDEM_OUTPUT: &str = r"cargo:rustc-env=VERGEN_BUILD_DATE=2022-12-23
cargo:rustc-env=VERGEN_BUILD_TIMESTAMP=2022-12-23T15:29:20.000000000Z
cargo:rerun-if-changed=build.rs
cargo:rerun-if-env-changed=VERGEN_IDEMPOTENT
cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH
";

    const QUIET_IDEM_OUTPUT: &str = r"cargo:rustc-env=VERGEN_BUILD_DATE=VERGEN_IDEMPOTENT_OUTPUT
cargo:rustc-env=VERGEN_BUILD_TIMESTAMP=VERGEN_IDEMPOTENT_OUTPUT
cargo:rerun-if-changed=build.rs
cargo:rerun-if-env-changed=VERGEN_IDEMPOTENT
cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH
";

    #[test]
    #[serial]
    fn build_all_output() -> Result<()> {
        let mut stdout_buf = vec![];
        let build = BuildBuilder::all_build()?;
        Emitter::new()
            .add_instructions(&build)?
            .emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(BUILD_REGEX_INST.is_match(&output));
        Ok(())
    }

    #[test]
    #[serial]
    fn build_all_with_custom_output() -> Result<()> {
        let mut stdout_buf = vec![];
        let build = BuildBuilder::all_build()?;
        let cust_gen = CustomInsGen::default();
        Emitter::new()
            .add_instructions(&build)?
            .add_custom_instructions(&cust_gen)?
            .emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(BUILD_CUSTOM_REGEX_INST.is_match(&output));
        Ok(())
    }

    #[test]
    #[serial]
    fn build_all_with_custom_idempotent_output() -> Result<()> {
        let mut stdout_buf = vec![];
        let build = BuildBuilder::all_build()?;
        let cust_gen = CustomInsGen::default();
        Emitter::new()
            .idempotent()
            .add_instructions(&build)?
            .add_custom_instructions(&cust_gen)?
            .emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(BUILD_CUSTOM_IDEM_INST.is_match(&output));
        Ok(())
    }

    #[test]
    #[serial]
    fn build_all_with_custom_fail() -> Result<()> {
        let build = BuildBuilder::all_build()?;
        let cust_gen = CustomInsGenBuilder::default().fail(true).build()?;
        assert!(Emitter::new()
            .fail_on_error()
            .add_instructions(&build)?
            .add_custom_instructions(&cust_gen)
            .is_err());
        Ok(())
    }

    #[test]
    #[serial]
    fn build_all_with_custom_default() -> Result<()> {
        let mut stdout_buf = vec![];
        let build = BuildBuilder::all_build()?;
        let cust_gen = CustomInsGenBuilder::default().fail(true).build()?;
        Emitter::new()
            .add_instructions(&build)?
            .add_custom_instructions(&cust_gen)?
            .emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(BUILD_CUSTOM_FAIL_IDEM_INST.is_match(&output));
        Ok(())
    }

    #[test]
    #[serial]
    fn build_all_output_local() -> Result<()> {
        temp_env::with_var("SOURCE_DATE_EPOCH", Some("1671809360"), || {
            let result = || -> Result<()> {
                let mut stdout_buf = vec![];
                let build = BuildBuilder::default()
                    .build_date(true)
                    .build_timestamp(true)
                    .use_local(true)
                    .build()?;
                let result = Emitter::new()
                    .add_instructions(&build)?
                    .fail_on_error()
                    .emit_to(&mut stdout_buf);
                assert!(result.is_ok());
                let output = String::from_utf8_lossy(&stdout_buf);
                assert!(BUILD_REGEX_INST.is_match(&output));
                Ok(())
            }();
            assert!(result.is_ok());
        });
        Ok(())
    }

    #[test]
    #[serial]
    fn build_all_idempotent_output() -> Result<()> {
        temp_env::with_var_unset("SOURCE_DATE_EPOCH", || {
            let result = || -> Result<()> {
                let mut stdout_buf = vec![];
                let build = BuildBuilder::all_build()?;
                Emitter::new()
                    .idempotent()
                    .add_instructions(&build)?
                    .emit_to(&mut stdout_buf)?;
                let output = String::from_utf8_lossy(&stdout_buf);
                assert_eq!(IDEM_OUTPUT, output);
                Ok(())
            }();
            assert!(result.is_ok());
        });
        Ok(())
    }

    #[test]
    #[serial]
    fn build_all_idempotent_custom_buildrs_output() -> Result<()> {
        temp_env::with_var_unset("SOURCE_DATE_EPOCH", || {
            let result = || -> Result<()> {
                let mut stdout_buf = vec![];
                let build = BuildBuilder::all_build()?;
                Emitter::new()
                    .idempotent()
                    .custom_build_rs("a/custom_build.rs")
                    .add_instructions(&build)?
                    .emit_to(&mut stdout_buf)?;
                let output = String::from_utf8_lossy(&stdout_buf);
                assert_eq!(IDEM_OUTPUT_CUSTOM_BUILDRS, output);
                Ok(())
            }();
            assert!(result.is_ok());
        });
        Ok(())
    }

    #[test]
    #[serial]
    fn build_all_idempotent_output_quiet() -> Result<()> {
        temp_env::with_var_unset("SOURCE_DATE_EPOCH", || {
            let result = || -> Result<()> {
                let mut stdout_buf = vec![];
                let build = BuildBuilder::all_build()?;
                Emitter::new()
                    .idempotent()
                    .quiet()
                    .add_instructions(&build)?
                    .emit_to(&mut stdout_buf)?;
                let output = String::from_utf8_lossy(&stdout_buf);
                assert_eq!(QUIET_IDEM_OUTPUT, output);
                Ok(())
            }();
            assert!(result.is_ok());
        });
        Ok(())
    }

    #[test]
    #[serial]
    fn build_all_sde_output() -> Result<()> {
        temp_env::with_var("SOURCE_DATE_EPOCH", Some("1671809360"), || {
            let result = || -> Result<()> {
                let mut stdout_buf = vec![];
                let build = BuildBuilder::all_build()?;
                Emitter::new()
                    .add_instructions(&build)?
                    .emit_to(&mut stdout_buf)?;
                let output = String::from_utf8_lossy(&stdout_buf);
                assert_eq!(SOURCE_DATE_EPOCH_IDEM_OUTPUT, output);
                Ok(())
            }();
            assert!(result.is_ok());
        });
        Ok(())
    }
}
