#[cfg(feature = "si")]
mod test_sysinfo {
    use anyhow::Result;
    use lazy_static::lazy_static;
    use regex::Regex;
    use vergen::Vergen;

    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    lazy_static! {
        static ref NAME_RE_STR: &'static str = r#"cargo:rustc-env=VERGEN_SYSINFO_NAME=.*"#;
        static ref NAME_IDEM_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_SYSINFO_NAME=VERGEN_IDEMPOTENT_OUTPUT"#;
        static ref OS_VERSION_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_SYSINFO_OS_VERSION=.*"#;
        static ref OS_VERSION_IDEM_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_SYSINFO_OS_VERSION=VERGEN_IDEMPOTENT_OUTPUT"#;
        static ref USER_RE_STR: &'static str = r#"cargo:rustc-env=VERGEN_SYSINFO_USER=.*"#;
        static ref USER_IDEM_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_SYSINFO_USER=VERGEN_IDEMPOTENT_OUTPUT"#;
        static ref TOTAL_MEMORY_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_SYSINFO_TOTAL_MEMORY=.*"#;
        static ref TOTAL_MEMORY_IDEM_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_SYSINFO_TOTAL_MEMORY=VERGEN_IDEMPOTENT_OUTPUT"#;
        static ref CPU_VENDOR_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_SYSINFO_CPU_VENDOR=.*"#;
        static ref CPU_VENDOR_IDEM_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_SYSINFO_CPU_VENDOR=VERGEN_IDEMPOTENT_OUTPUT"#;
        static ref CPU_CORE_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_SYSINFO_CPU_CORE_COUNT=.*"#;
        static ref CPU_CORE_IDEM_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_SYSINFO_CPU_CORE_COUNT=VERGEN_IDEMPOTENT_OUTPUT"#;
        static ref SYSINFO_REGEX_INST: Regex = {
            let re_str = vec![
                *NAME_RE_STR,
                *OS_VERSION_RE_STR,
                *USER_RE_STR,
                *TOTAL_MEMORY_RE_STR,
                *CPU_VENDOR_RE_STR,
                *CPU_CORE_RE_STR,
            ]
            .join("\n");
            Regex::new(&re_str).unwrap()
        };
        static ref SYSINFO_IDEM_REGEX_INST: Regex = {
            let re_str = vec![
                *NAME_IDEM_RE_STR,
                *OS_VERSION_IDEM_RE_STR,
                *USER_IDEM_RE_STR,
                *TOTAL_MEMORY_IDEM_RE_STR,
                *CPU_VENDOR_IDEM_RE_STR,
                *CPU_CORE_IDEM_RE_STR,
            ]
            .join("\n");
            Regex::new(&re_str).unwrap()
        };
    }

    #[cfg(target_os = "macos")]
    lazy_static! {
        static ref NAME_RE_STR: &'static str = r#"cargo:rustc-env=VERGEN_SYSINFO_NAME=.*"#;
        static ref OS_VERSION_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_SYSINFO_OS_VERSION=.*"#;
        static ref TOTAL_MEMORY_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_SYSINFO_TOTAL_MEMORY=.*"#;
        static ref CPU_VENDOR_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_SYSINFO_CPU_VENDOR=.*"#;
        static ref CPU_CORE_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_SYSINFO_CPU_CORE_COUNT=.*"#;
        static ref SYSINFO_REGEX_INST: Regex = {
            let re_str = vec![
                *NAME_RE_STR,
                *OS_VERSION_RE_STR,
                *TOTAL_MEMORY_RE_STR,
                *CPU_VENDOR_RE_STR,
                *CPU_CORE_RE_STR,
            ]
            .join("\n");
            Regex::new(&re_str).unwrap()
        };
        static ref SYSINFO_IDEM_REGEX_INST: Regex = {
            let re_str = vec![
                *NAME_RE_STR,
                *OS_VERSION_RE_STR,
                *TOTAL_MEMORY_RE_STR,
                *CPU_VENDOR_RE_STR,
                *CPU_CORE_RE_STR,
            ]
            .join("\n");
            Regex::new(&re_str).unwrap()
        };
    }

    const IDEM_OUTPUT: &str = r#"cargo:rustc-env=VERGEN_SYSINFO_NAME=VERGEN_IDEMPOTENT_OUTPUT
cargo:rustc-env=VERGEN_SYSINFO_OS_VERSION=VERGEN_IDEMPOTENT_OUTPUT
cargo:rustc-env=VERGEN_SYSINFO_USER=VERGEN_IDEMPOTENT_OUTPUT
cargo:rustc-env=VERGEN_SYSINFO_TOTAL_MEMORY=VERGEN_IDEMPOTENT_OUTPUT
cargo:rustc-env=VERGEN_SYSINFO_CPU_VENDOR=VERGEN_IDEMPOTENT_OUTPUT
cargo:rustc-env=VERGEN_SYSINFO_CPU_CORE_COUNT=VERGEN_IDEMPOTENT_OUTPUT
cargo:rustc-env=VERGEN_SYSINFO_CPU_NAME=VERGEN_IDEMPOTENT_OUTPUT
cargo:rustc-env=VERGEN_SYSINFO_CPU_BRAND=VERGEN_IDEMPOTENT_OUTPUT
cargo:rustc-env=VERGEN_SYSINFO_CPU_FREQUENCY=VERGEN_IDEMPOTENT_OUTPUT
cargo:warning=VERGEN_SYSINFO_NAME set to idempotent default
cargo:warning=VERGEN_SYSINFO_OS_VERSION set to idempotent default
cargo:warning=VERGEN_SYSINFO_USER set to idempotent default
cargo:warning=VERGEN_SYSINFO_TOTAL_MEMORY set to idempotent default
cargo:warning=VERGEN_SYSINFO_CPU_VENDOR set to idempotent default
cargo:warning=VERGEN_SYSINFO_CPU_CORE_COUNT set to idempotent default
cargo:warning=VERGEN_SYSINFO_CPU_NAME set to idempotent default
cargo:warning=VERGEN_SYSINFO_CPU_BRAND set to idempotent default
cargo:warning=VERGEN_SYSINFO_CPU_FREQUENCY set to idempotent default
cargo:rerun-if-changed=build.rs
cargo:rerun-if-env-changed=VERGEN_IDEMPOTENT
cargo:rerun-if-env-changed=VERGEN_SKIP_IF_ERROR
cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH
"#;

    #[cfg(target_os = "windows")]
    lazy_static! {
        static ref NAME_RE_STR: &'static str = r#"cargo:rustc-env=VERGEN_SYSINFO_NAME=.*"#;
        static ref OS_VERSION_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_SYSINFO_OS_VERSION=.*"#;
        static ref TOTAL_MEMORY_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_SYSINFO_TOTAL_MEMORY=.*"#;
        static ref CPU_VENDOR_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_SYSINFO_CPU_VENDOR=.*"#;
        static ref CPU_CORE_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_SYSINFO_CPU_CORE_COUNT=.*"#;
        static ref SYSINFO_REGEX_INST: Regex = {
            let re_str = vec![
                *NAME_RE_STR,
                *OS_VERSION_RE_STR,
                *TOTAL_MEMORY_RE_STR,
                *CPU_VENDOR_RE_STR,
                *CPU_CORE_RE_STR,
            ]
            .join("\n");
            Regex::new(&re_str).unwrap()
        };
        static ref SYSINFO_IDEM_REGEX_INST: Regex = {
            let re_str = vec![
                *NAME_RE_STR,
                *OS_VERSION_RE_STR,
                *TOTAL_MEMORY_RE_STR,
                *CPU_VENDOR_RE_STR,
                *CPU_CORE_RE_STR,
            ]
            .join("\n");
            Regex::new(&re_str).unwrap()
        };
    }

    #[test]
    fn sysinfo_all_output() -> Result<()> {
        let mut stdout_buf = vec![];
        Vergen::default()
            .all_sysinfo()
            .test_gen_output(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(SYSINFO_REGEX_INST.is_match(&output));
        Ok(())
    }

    #[test]
    fn sysinfo_all_idempotent_output() -> Result<()> {
        let mut stdout_buf = vec![];
        Vergen::default()
            .idempotent()
            .all_sysinfo()
            .test_gen_output(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert_eq!(IDEM_OUTPUT, output);
        Ok(())
    }
}
