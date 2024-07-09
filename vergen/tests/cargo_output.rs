#[cfg(feature = "cargo")]
mod test_build {
    use anyhow::Result;
    use cargo_metadata::DependencyKind;
    use lazy_static::lazy_static;
    use regex::Regex;
    use std::env;
    use vergen::EmitBuilder;

    lazy_static! {
        static ref CARGO_DEBUG_RE_STR: &'static str =
            r"cargo:rustc-env=VERGEN_CARGO_DEBUG=(true|false)";
        static ref CARGO_FEA_RE_STR: &'static str =
            r"cargo:rustc-env=VERGEN_CARGO_FEATURES=[a-zA-Z0-9-_]+,[a-zA-Z0-9-_]+";
        static ref CARGO_OPT_LEVEL_RE_STR: &'static str =
            r"cargo:rustc-env=VERGEN_CARGO_OPT_LEVEL=\d{1}";
        static ref CARGO_TT_RE_STR: &'static str =
            r"cargo:rustc-env=VERGEN_CARGO_TARGET_TRIPLE=[a-zA-Z0-9-_]+";
        static ref CARGO_DEP_RE_STR: &'static str = r"cargo:rustc-env=VERGEN_CARGO_DEPENDENCIES=.*";
        static ref CARGO_DEP_NAME_RE_STR: &'static str =
            r"(?m)^cargo:rustc-env=VERGEN_CARGO_DEPENDENCIES=anyhow 1\.0\.[0-9]{2,}$";
        static ref CARGO_DEP_DK_RE_STR: &'static str =
            r"(?m)^cargo:rustc-env=VERGEN_CARGO_DEPENDENCIES=gix 0\.63\.[0-9]{1,}$";
        static ref CARGO_DEP_RV_RE_STR: &'static str =
            r"(?m)^cargo:rustc-env=VERGEN_CARGO_DEPENDENCIES=rustversion 1\.0\.[0-9]{2,}$";
        static ref CARGO_REGEX: Regex = {
            let re_str = [
                *CARGO_DEBUG_RE_STR,
                *CARGO_FEA_RE_STR,
                *CARGO_OPT_LEVEL_RE_STR,
                *CARGO_TT_RE_STR,
                *CARGO_DEP_RE_STR,
            ]
            .join("\n");
            Regex::new(&re_str).unwrap()
        };
        static ref CARGO_REGEX_NO_DEP: Regex = {
            let re_str = [
                *CARGO_DEBUG_RE_STR,
                *CARGO_FEA_RE_STR,
                *CARGO_OPT_LEVEL_RE_STR,
                *CARGO_TT_RE_STR,
            ]
            .join("\n");
            Regex::new(&re_str).unwrap()
        };
        static ref CARGO_REGEX_NAME: Regex = Regex::new(&CARGO_DEP_NAME_RE_STR).unwrap();
        static ref CARGO_REGEX_DK: Regex = Regex::new(&CARGO_DEP_DK_RE_STR).unwrap();
        static ref CARGO_REGEX_RV: Regex = Regex::new(&CARGO_DEP_RV_RE_STR).unwrap();
    }

    fn setup() {
        env::set_var("CARGO_FEATURE_BUILD", "build");
        env::set_var("CARGO_FEATURE_GIT", "git");
        env::set_var("DEBUG", "true");
        env::set_var("OPT_LEVEL", "1");
        env::set_var("TARGET", "x86_64-unknown-linux-gnu");
    }

    fn teardown() {
        env::remove_var("CARGO_FEATURE_BUILD");
        env::remove_var("CARGO_FEATURE_GIT");
        env::remove_var("DEBUG");
        env::remove_var("OPT_LEVEL");
        env::remove_var("TARGET");
    }

    const DISABLED_OUTPUT: &str = r"";

    #[test]
    #[serial_test::serial]
    fn cargo_all_output() -> Result<()> {
        setup();
        let mut stdout_buf = vec![];
        EmitBuilder::builder()
            .all_cargo()
            .emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(CARGO_REGEX.is_match(&output));
        teardown();
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn cargo_disabled_output() -> Result<()> {
        setup();
        let mut stdout_buf = vec![];
        EmitBuilder::builder()
            .all_cargo()
            .disable_cargo()
            .emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert_eq!(DISABLED_OUTPUT, output);
        teardown();
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn cargo_all_idempotent_output() -> Result<()> {
        setup();
        let mut stdout_buf = vec![];
        EmitBuilder::builder()
            .idempotent()
            .all_cargo()
            .emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(CARGO_REGEX.is_match(&output));
        teardown();
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn cargo_all_name_filter_none_output() -> Result<()> {
        setup();
        let mut stdout_buf = vec![];
        EmitBuilder::builder()
            .all_cargo()
            .cargo_dependencies_name_filter(Some("blah"))
            .emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(CARGO_REGEX_NO_DEP.is_match(&output));
        teardown();
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn cargo_all_name_filter_some_output() -> Result<()> {
        setup();
        let mut stdout_buf = vec![];
        EmitBuilder::builder()
            .all_cargo()
            .cargo_dependencies_name_filter(Some("anyhow"))
            .emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(CARGO_REGEX.is_match(&output));
        assert!(CARGO_REGEX_NAME.is_match(&output));
        teardown();
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn cargo_all_dep_kind_filter_none_output() -> Result<()> {
        setup();
        let mut stdout_buf = vec![];
        EmitBuilder::builder()
            .all_cargo()
            .cargo_dependencies_dep_kind_filter(Some(DependencyKind::Build))
            .emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(CARGO_REGEX_NO_DEP.is_match(&output));
        assert!(CARGO_REGEX_RV.is_match(&output));
        teardown();
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn cargo_all_dep_kind_filter_some_output() -> Result<()> {
        setup();
        let mut stdout_buf = vec![];
        EmitBuilder::builder()
            .all_cargo()
            .cargo_dependencies_name_filter(Some("gix"))
            .cargo_dependencies_dep_kind_filter(Some(DependencyKind::Development))
            .emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(CARGO_REGEX.is_match(&output));
        assert!(CARGO_REGEX_DK.is_match(&output));
        teardown();
        Ok(())
    }
}
