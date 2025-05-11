#[cfg(feature = "cargo")]
mod test_cargo {
    use std::sync::LazyLock;

    #[cfg(feature = "cargo_metadata")]
    use cargo_metadata::DependencyKind;
    use regex::Regex;
    use serial_test::serial;
    use test_util::with_cargo_vars;
    use vergen::Cargo;
    use vergen::Emitter;

    static CARGO_DEBUG_RE_STR: LazyLock<&'static str> =
        LazyLock::new(|| r"cargo:rustc-env=VERGEN_CARGO_DEBUG=(true|false)");
    static CARGO_FEA_RE_STR: LazyLock<&'static str> =
        LazyLock::new(|| r"cargo:rustc-env=VERGEN_CARGO_FEATURES=[a-zA-Z0-9-_]+,[a-zA-Z0-9-_]+");
    static CARGO_OPT_LEVEL_RE_STR: LazyLock<&'static str> =
        LazyLock::new(|| r"cargo:rustc-env=VERGEN_CARGO_OPT_LEVEL=\d{1}");
    static CARGO_TT_RE_STR: LazyLock<&'static str> =
        LazyLock::new(|| r"cargo:rustc-env=VERGEN_CARGO_TARGET_TRIPLE=[a-zA-Z0-9-_]+");
    #[cfg(feature = "cargo_metadata")]
    static CARGO_DEP_RE_STR: LazyLock<&'static str> =
        LazyLock::new(|| r"cargo:rustc-env=VERGEN_CARGO_DEPENDENCIES=.*");
    #[cfg(feature = "cargo_metadata")]
    static CARGO_DEP_NAME_RE_STR: LazyLock<&'static str> =
        LazyLock::new(|| r"(?m)^cargo:rustc-env=VERGEN_CARGO_DEPENDENCIES=anyhow 1\.0\.[0-9]{2,}$");
    #[cfg(feature = "cargo_metadata")]
    static CARGO_DEP_DK_RE_STR: LazyLock<&'static str> = LazyLock::new(
        || r"(?m)^cargo:rustc-env=VERGEN_CARGO_DEPENDENCIES=regex 1\.[0-9]{1,}\.[0-9]{1,}$",
    );
    #[cfg(feature = "cargo_metadata")]
    static CARGO_DEP_RV_RE_STR: LazyLock<&'static str> = LazyLock::new(
        || r"(?m)^cargo:rustc-env=VERGEN_CARGO_DEPENDENCIES=rustversion 1\.0\.[0-9]{2,}$",
    );
    static CARGO_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        let re_str = [
            *CARGO_DEBUG_RE_STR,
            *CARGO_FEA_RE_STR,
            *CARGO_OPT_LEVEL_RE_STR,
            *CARGO_TT_RE_STR,
            #[cfg(feature = "cargo_metadata")]
            *CARGO_DEP_RE_STR,
        ]
        .join("\n");
        Regex::new(&re_str).unwrap()
    });
    #[cfg(feature = "cargo_metadata")]
    static CARGO_REGEX_NO_DEP: LazyLock<Regex> = LazyLock::new(|| {
        let re_str = [
            *CARGO_DEBUG_RE_STR,
            *CARGO_FEA_RE_STR,
            *CARGO_OPT_LEVEL_RE_STR,
            *CARGO_TT_RE_STR,
        ]
        .join("\n");
        Regex::new(&re_str).unwrap()
    });
    #[cfg(feature = "cargo_metadata")]
    static CARGO_REGEX_NAME: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(&CARGO_DEP_NAME_RE_STR).unwrap());
    #[cfg(feature = "cargo_metadata")]
    static CARGO_REGEX_DK: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(&CARGO_DEP_DK_RE_STR).unwrap());
    #[cfg(feature = "cargo_metadata")]
    static CARGO_REGEX_RV: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(&CARGO_DEP_RV_RE_STR).unwrap());

    #[test]
    #[serial]
    fn cargo_all_output() {
        let result = with_cargo_vars(|| {
            let mut stdout_buf = vec![];
            let cargo = Cargo::all_cargo();
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
    #[serial]
    fn cargo_all_idempotent_output() {
        let result = with_cargo_vars(|| {
            let mut stdout_buf = vec![];
            let cargo = Cargo::all_cargo();
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
    #[cfg(feature = "cargo_metadata")]
    fn cargo_all_name_filter_none_output() {
        let result = with_cargo_vars(|| {
            let mut stdout_buf = vec![];
            let cargo = Cargo::all_cargo_builder().name_filter("blah").build();
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
    #[cfg(feature = "cargo_metadata")]
    fn cargo_all_name_filter_some_output() {
        let result = with_cargo_vars(|| {
            let mut stdout_buf = vec![];
            let cargo = Cargo::all_cargo_builder().name_filter("anyhow").build();
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
    #[cfg(feature = "cargo_metadata")]
    fn cargo_all_dep_kind_filter_output() {
        let result = with_cargo_vars(|| {
            let mut stdout_buf = vec![];
            let cargo = Cargo::all_cargo_builder()
                .dep_kind_filter(DependencyKind::Build)
                .build();
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
    #[cfg(feature = "cargo_metadata")]
    fn cargo_all_dep_kind_filter_with_name_filter_output() {
        let result = with_cargo_vars(|| {
            let mut stdout_buf = vec![];
            let cargo = Cargo::all_cargo_builder()
                .dep_kind_filter(DependencyKind::Development)
                .name_filter("regex")
                .build();
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
