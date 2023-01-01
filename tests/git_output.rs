#[cfg(all(feature = "git", any(feature = "gitcl", feature = "git2")))]
mod test_git {
    use anyhow::Result;
    use lazy_static::lazy_static;
    use regex::Regex;
    use vergen::EmitBuilder;

    lazy_static! {
        static ref GIT_BRANCH_RE_STR: &'static str = r#"cargo:rustc-env=VERGEN_GIT_BRANCH=.*"#;
        static ref GIT_CAE_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_GIT_COMMIT_AUTHOR_EMAIL=\S+@\S+"#;
        static ref GIT_CAN_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_GIT_COMMIT_AUTHOR_NAME=.*"#;
        static ref GIT_CC_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_GIT_COMMIT_COUNT=([0-9]+)"#;
        static ref GIT_CD_RE_STR: &'static str = r#"cargo:rustc-env=VERGEN_GIT_COMMIT_DATE=([0-9]+)-(0[1-9]|1[012])-(0[1-9]|[12][0-9]|3[01])"#;
        static ref GIT_CD_IDEM_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_GIT_COMMIT_DATE=VERGEN_IDEMPOTENT_OUTPUT"#;
        static ref GIT_CM_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_GIT_COMMIT_MESSAGE=[\s\S]+"#;
        static ref GIT_CT_RE_STR: &'static str = r#"cargo:rustc-env=VERGEN_GIT_COMMIT_TIMESTAMP=([0-9]+)-(0[1-9]|1[012])-(0[1-9]|[12][0-9]|3[01])[Tt]([01][0-9]|2[0-3]):([0-5][0-9]):([0-5][0-9]|60)(\.[0-9]+)?(([Zz])|([\+|\-]([01][0-9]|2[0-3]):[0-5][0-9]))"#;
        static ref GIT_CT_IDEM_RE_STR: &'static str =
            r#"cargo:rustc-env=VERGEN_GIT_COMMIT_TIMESTAMP=VERGEN_IDEMPOTENT_OUTPUT"#;
        static ref GIT_DESCRIBE_RE_STR: &'static str = r#"cargo:rustc-env=VERGEN_GIT_DESCRIBE=.*"#;
        static ref GIT_SHA_RE_STR: &'static str = r#"cargo:rustc-env=VERGEN_GIT_SHA=[0-9a-f]{40}"#;
        static ref GIT_REGEX_INST: Regex = {
            let re_str = vec![
                *GIT_BRANCH_RE_STR,
                *GIT_CAE_RE_STR,
                *GIT_CAN_RE_STR,
                *GIT_CC_RE_STR,
                *GIT_CD_RE_STR,
                *GIT_CM_RE_STR,
                *GIT_CT_RE_STR,
                *GIT_DESCRIBE_RE_STR,
                *GIT_SHA_RE_STR,
            ]
            .join("\n");
            Regex::new(&re_str).unwrap()
        };
        static ref GIT_REGEX_IDEM_INST: Regex = {
            let re_str = vec![
                *GIT_BRANCH_RE_STR,
                *GIT_CAE_RE_STR,
                *GIT_CAN_RE_STR,
                *GIT_CC_RE_STR,
                *GIT_CD_IDEM_RE_STR,
                *GIT_CM_RE_STR,
                *GIT_CT_IDEM_RE_STR,
                *GIT_DESCRIBE_RE_STR,
                *GIT_SHA_RE_STR,
            ]
            .join("\n");
            Regex::new(&re_str).unwrap()
        };
    }

    #[test]
    fn git_all_output() -> Result<()> {
        let mut stdout_buf = vec![];
        EmitBuilder::builder().all_git().emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        println!("{output}");
        assert!(GIT_REGEX_INST.is_match(&output));
        Ok(())
    }

    #[test]
    fn git_all_idempotent_output() -> Result<()> {
        let mut stdout_buf = vec![];
        EmitBuilder::builder()
            .idempotent()
            .all_git()
            .emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(GIT_REGEX_IDEM_INST.is_match(&output));
        Ok(())
    }
}
