#[cfg(feature = "cargo")]
mod test_build {
    use anyhow::Result;
    use lazy_static::lazy_static;
    use regex::Regex;
    use std::env;
    use vergen::Vergen;

    lazy_static! {
        static ref CARGO_TT_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_CARGO_TARGET_TRIPLE=[a-zA-Z0-9-_]+"#;
        static ref CARGO_PROF_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_CARGO_PROFILE=[a-zA-Z0-9-_]+"#;
        static ref CARGO_FEA_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_CARGO_FEATURES=[a-zA-Z0-9-_]+,[a-zA-Z0-9-_]+"#;
        static ref CARGO_REGEX: Regex = {
            let re_str = vec![*CARGO_TT_RE_STR, *CARGO_PROF_RE_STR, *CARGO_FEA_RE_STR].join("\n");
            Regex::new(&re_str).unwrap()
        };
    }

    fn setup() {
        env::set_var("TARGET", "x86_64-unknown-linux-gnu");
        env::set_var("PROFILE", "debug");
        env::set_var("CARGO_FEATURE_GIT", "git");
        env::set_var("CARGO_FEATURE_BUILD", "build");
    }

    fn teardown() {
        env::remove_var("TARGET");
        env::remove_var("PROFILE");
        env::remove_var("CARGO_FEATURE_GIT");
        env::remove_var("CARGO_FEATURE_BUILD");
    }

    #[test]
    #[serial_test::serial]
    fn cargo_all_output() -> Result<()> {
        setup();
        let mut stdout_buf = vec![];
        Vergen::default()
            .all_cargo()
            .test_gen_output(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(CARGO_REGEX.is_match(&output));
        teardown();
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn cargo_all_idempotent_output() -> Result<()> {
        setup();
        let mut stdout_buf = vec![];
        Vergen::default()
            .idempotent()
            .all_cargo()
            .test_gen_output(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(CARGO_REGEX.is_match(&output));
        teardown();
        Ok(())
    }
}
