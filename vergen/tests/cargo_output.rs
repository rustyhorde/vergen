#[cfg(feature = "cargo")]
mod test_cargo {
    use cargo_metadata::DependencyKind;
    use lazy_static::lazy_static;
    use regex::Regex;
    use serial_test::serial;
    use test_util::with_cargo_vars;
    use vergen::CargoBuilder;
    use vergen::Emitter;

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
            r"(?m)^cargo:rustc-env=VERGEN_CARGO_DEPENDENCIES=regex 1\.[0-9]{1,}\.[0-9]{1,}$";
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

    #[test]
    #[serial]
    fn cargo_all_output() {
        let result = with_cargo_vars(|| {
            let mut stdout_buf = vec![];
            let cargo = CargoBuilder::all_cargo()?;
            Emitter::default()
                .add_instructions(&cargo)?
                .emit_to(&mut stdout_buf)?;
            let output = String::from_utf8_lossy(&stdout_buf);
            assert!(CARGO_REGEX.is_match(&output));
            Ok(())
        });
        assert!(result.is_ok());
    }

    #[test]
    #[serial_test::serial]
    fn cargo_all_idempotent_output() {
        let result = with_cargo_vars(|| {
            let mut stdout_buf = vec![];
            let cargo = CargoBuilder::all_cargo()?;
            Emitter::default()
                .idempotent()
                .add_instructions(&cargo)?
                .emit_to(&mut stdout_buf)?;
            let output = String::from_utf8_lossy(&stdout_buf);
            assert!(CARGO_REGEX.is_match(&output));
            Ok(())
        });
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn cargo_all_name_filter_none_output() {
        let result = with_cargo_vars(|| {
            let mut stdout_buf = vec![];
            let mut cargo = CargoBuilder::all_cargo()?;
            cargo.set_name_filter(Some("blah"));
            Emitter::default()
                .add_instructions(&cargo)?
                .emit_to(&mut stdout_buf)?;
            let output = String::from_utf8_lossy(&stdout_buf);
            assert!(CARGO_REGEX_NO_DEP.is_match(&output));
            Ok(())
        });
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn cargo_all_name_filter_some_output() {
        let result = with_cargo_vars(|| {
            let mut stdout_buf = vec![];
            let mut cargo = CargoBuilder::all_cargo()?;
            cargo.set_name_filter(Some("anyhow"));
            Emitter::default()
                .add_instructions(&cargo)?
                .emit_to(&mut stdout_buf)?;
            let output = String::from_utf8_lossy(&stdout_buf);
            assert!(CARGO_REGEX.is_match(&output));
            assert!(CARGO_REGEX_NAME.is_match(&output));
            Ok(())
        });
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn cargo_all_dep_kind_filter_output() {
        let result = with_cargo_vars(|| {
            let mut stdout_buf = vec![];
            let mut cargo = CargoBuilder::all_cargo()?;
            cargo.set_dep_kind_filter(Some(DependencyKind::Build));
            Emitter::default()
                .add_instructions(&cargo)?
                .emit_to(&mut stdout_buf)?;
            let output = String::from_utf8_lossy(&stdout_buf);
            assert!(CARGO_REGEX_NO_DEP.is_match(&output));
            assert!(CARGO_REGEX_RV.is_match(&output));
            Ok(())
        });
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn cargo_all_dep_kind_filter_with_name_filter_output() {
        let result = with_cargo_vars(|| {
            let mut stdout_buf = vec![];
            let mut cargo = CargoBuilder::all_cargo()?;
            cargo.set_dep_kind_filter(Some(DependencyKind::Development));
            cargo.set_name_filter(Some("regex"));
            Emitter::default()
                .add_instructions(&cargo)?
                .emit_to(&mut stdout_buf)?;
            let output = String::from_utf8_lossy(&stdout_buf);
            assert!(CARGO_REGEX.is_match(&output));
            assert!(CARGO_REGEX_DK.is_match(&output));
            Ok(())
        });
        assert!(result.is_ok());
    }
}
