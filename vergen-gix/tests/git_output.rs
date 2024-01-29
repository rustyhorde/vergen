mod test_git_gix {
    use anyhow::Result;
    use lazy_static::lazy_static;
    use regex::Regex;
    use serial_test::serial;
    use std::env::{self, temp_dir};
    use temp_env::with_var;
    use vergen::Emitter;
    use vergen_gix::GixBuilder;

    use test_util::TestRepos;

    lazy_static! {
        static ref GIT_BRANCH_RE_STR: &'static str = r"cargo:rustc-env=VERGEN_GIT_BRANCH=.*";
        static ref GIT_CAE_RE_STR: &'static str =
            r"cargo:rustc-env=VERGEN_GIT_COMMIT_AUTHOR_EMAIL=\S+@\S+";
        static ref GIT_CAN_RE_STR: &'static str =
            r"cargo:rustc-env=VERGEN_GIT_COMMIT_AUTHOR_NAME=.*";
        static ref GIT_CC_RE_STR: &'static str =
            r"cargo:rustc-env=VERGEN_GIT_COMMIT_COUNT=([0-9]+)";
        static ref GIT_CD_RE_STR: &'static str = r"cargo:rustc-env=VERGEN_GIT_COMMIT_DATE=([0-9]+)-(0[1-9]|1[012])-(0[1-9]|[12][0-9]|3[01])";
        static ref GIT_CD_IDEM_RE_STR: &'static str =
            r"cargo:rustc-env=VERGEN_GIT_COMMIT_DATE=VERGEN_IDEMPOTENT_OUTPUT";
        static ref GIT_CM_RE_STR: &'static str =
            r"cargo:rustc-env=VERGEN_GIT_COMMIT_MESSAGE=[\s\S]+";
        static ref GIT_CT_RE_STR: &'static str = r"cargo:rustc-env=VERGEN_GIT_COMMIT_TIMESTAMP=([0-9]+)-(0[1-9]|1[012])-(0[1-9]|[12][0-9]|3[01])[Tt]([01][0-9]|2[0-3]):([0-5][0-9]):([0-5][0-9]|60)(\.[0-9]+)?(([Zz])|([\+|\-]([01][0-9]|2[0-3]):[0-5][0-9]))";
        static ref GIT_CT_IDEM_RE_STR: &'static str =
            r"cargo:rustc-env=VERGEN_GIT_COMMIT_TIMESTAMP=VERGEN_IDEMPOTENT_OUTPUT";
        static ref GIT_DESCRIBE_RE_STR: &'static str = r"cargo:rustc-env=VERGEN_GIT_DESCRIBE=.*";
        static ref GIT_SHA_RE_STR: &'static str = r"cargo:rustc-env=VERGEN_GIT_SHA=[0-9a-f]{40}";
        static ref GIT_SHORT_SHA_RE_STR: &'static str =
            r"cargo:rustc-env=VERGEN_GIT_SHA=[0-9a-f]{7}";
        static ref GIT_BRANCH_IDEM_RE_STR: &'static str =
            r"cargo:rustc-env=VERGEN_GIT_BRANCH=VERGEN_IDEMPOTENT_OUTPUT";
        static ref GIT_COMMIT_AUTHOR_EMAIL_IDEM_RE_STR: &'static str =
            r"cargo:rustc-env=VERGEN_GIT_COMMIT_AUTHOR_EMAIL=VERGEN_IDEMPOTENT_OUTPUT";
        static ref GIT_COMMIT_AUTHOR_NAME_IDEM_RE_STR: &'static str =
            r"cargo:rustc-env=VERGEN_GIT_COMMIT_AUTHOR_NAME=VERGEN_IDEMPOTENT_OUTPUT";
        static ref GIT_COMMIT_COUNT_IDEM_RE_STR: &'static str =
            r"cargo:rustc-env=VERGEN_GIT_COMMIT_COUNT=VERGEN_IDEMPOTENT_OUTPUT";
        static ref GIT_COMMIT_DATE_IDEM_RE_STR: &'static str =
            r"cargo:rustc-env=VERGEN_GIT_COMMIT_DATE=VERGEN_IDEMPOTENT_OUTPUT";
        static ref GIT_COMMIT_MESSAGE_IDEM_RE_STR: &'static str =
            r"cargo:rustc-env=VERGEN_GIT_COMMIT_MESSAGE=VERGEN_IDEMPOTENT_OUTPUT";
        static ref GIT_COMMIT_TIMESTAMP_IDEM_RE_STR: &'static str =
            r"cargo:rustc-env=VERGEN_GIT_COMMIT_TIMESTAMP=VERGEN_IDEMPOTENT_OUTPUT";
        static ref GIT_DESCRIBE_IDEM_RE_STR: &'static str =
            r"cargo:rustc-env=VERGEN_GIT_DESCRIBE=VERGEN_IDEMPOTENT_OUTPUT";
        static ref GIT_SHA_IDEM_RE_STR: &'static str =
            r"cargo:rustc-env=VERGEN_GIT_SHA=VERGEN_IDEMPOTENT_OUTPUT";
        static ref WARNINGS_RERUN_RE_STR: &'static str = r"cargo:warning=(.*?)
cargo:warning=VERGEN_GIT_BRANCH set to default
cargo:warning=VERGEN_GIT_COMMIT_AUTHOR_EMAIL set to default
cargo:warning=VERGEN_GIT_COMMIT_AUTHOR_NAME set to default
cargo:warning=VERGEN_GIT_COMMIT_COUNT set to default
cargo:warning=VERGEN_GIT_COMMIT_DATE set to default
cargo:warning=VERGEN_GIT_COMMIT_MESSAGE set to default
cargo:warning=VERGEN_GIT_COMMIT_TIMESTAMP set to default
cargo:warning=VERGEN_GIT_DESCRIBE set to default
cargo:warning=VERGEN_GIT_SHA set to default
cargo:rerun-if-changed=build.rs
cargo:rerun-if-env-changed=VERGEN_IDEMPOTENT
cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH";
        static ref GIT_REGEX_INST: Regex = {
            let re_str = [
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
        static ref GIT_REGEX_SHORT_INST: Regex = {
            let re_str = [
                *GIT_BRANCH_RE_STR,
                *GIT_CAE_RE_STR,
                *GIT_CAN_RE_STR,
                *GIT_CC_RE_STR,
                *GIT_CD_RE_STR,
                *GIT_CM_RE_STR,
                *GIT_CT_RE_STR,
                *GIT_DESCRIBE_RE_STR,
                *GIT_SHORT_SHA_RE_STR,
            ]
            .join("\n");
            Regex::new(&re_str).unwrap()
        };
        static ref GIT_REGEX_IDEM_INST: Regex = {
            let re_str = [
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
        static ref ALL_IDEM_OUTPUT: Regex = {
            let re_str = [
                *GIT_BRANCH_IDEM_RE_STR,
                *GIT_COMMIT_AUTHOR_EMAIL_IDEM_RE_STR,
                *GIT_COMMIT_AUTHOR_NAME_IDEM_RE_STR,
                *GIT_COMMIT_COUNT_IDEM_RE_STR,
                *GIT_COMMIT_DATE_IDEM_RE_STR,
                *GIT_COMMIT_MESSAGE_IDEM_RE_STR,
                *GIT_COMMIT_TIMESTAMP_IDEM_RE_STR,
                *GIT_DESCRIBE_IDEM_RE_STR,
                *GIT_SHA_IDEM_RE_STR,
                *WARNINGS_RERUN_RE_STR,
            ]
            .join("\n");
            Regex::new(&re_str).unwrap()
        };
    }

    fn repo_exists() -> Result<()> {
        let curr_dir = env::current_dir()?;
        let _repo = gix::discover(curr_dir)?;
        Ok(())
    }

    #[test]
    #[serial]
    fn git_all_output_idempotent() -> Result<()> {
        let mut stdout_buf = vec![];
        let mut gix = GixBuilder::all_git()?;
        let _ = gix.at_path(temp_dir());
        let failed = Emitter::default()
            .add_instructions(&gix)?
            .emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(!failed);
        assert!(ALL_IDEM_OUTPUT.is_match(&output));
        Ok(())
    }

    #[test]
    #[serial]
    fn git_all_output_default_dir() -> Result<()> {
        let mut stdout_buf = vec![];
        let gix = GixBuilder::all_git()?;
        let failed = Emitter::default()
            .add_instructions(&gix)?
            .emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(!failed);
        assert!(repo_exists().is_ok());
        assert!(GIT_REGEX_INST.is_match(&output));
        Ok(())
    }

    #[test]
    #[serial]
    fn git_all_flags_test_repo() -> Result<()> {
        let repo = TestRepos::new(true, false, false)?;
        let mut stdout_buf = vec![];
        let mut gix = GixBuilder::default()
            .all()
            .describe(true, false, None)
            .sha(true)
            .build()?;
        let _ = gix.at_path(repo.path());
        let failed = Emitter::default()
            .add_instructions(&gix)?
            .emit_to(&mut stdout_buf)?;
        assert!(!failed);
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(GIT_REGEX_SHORT_INST.is_match(&output));
        Ok(())
    }

    #[test]
    #[serial]
    fn git_all_flags_test_repo_local() -> Result<()> {
        let repo = TestRepos::new(true, false, false)?;
        let mut stdout_buf = vec![];
        let mut gix = GixBuilder::default()
            .all()
            .describe(true, false, None)
            .sha(true)
            .use_local(true)
            .build()?;
        let _ = gix.at_path(repo.path());
        let result = || -> Result<bool> {
            let failed = Emitter::default()
                .fail_on_error()
                .add_instructions(&gix)?
                .emit_to(&mut stdout_buf)?;
            Ok(failed)
        }();

        check_local_result(result, &stdout_buf);
        Ok(())
    }

    #[cfg(target_os = "windows")]
    fn check_local_result(result: Result<bool>, stdout_buf: &[u8]) {
        assert!(result.is_ok());
        let output = String::from_utf8_lossy(stdout_buf);
        assert!(GIT_REGEX_SHORT_INST.is_match(&output));
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn check_local_result(result: Result<bool>, _stdout_buf: &[u8]) {
        assert!(result.is_err());
    }

    #[test]
    #[serial]
    fn git_all_output_test_repo() -> Result<()> {
        let repo = TestRepos::new(true, true, false)?;
        let mut stdout_buf = vec![];
        let mut gix = GixBuilder::all_git()?;
        let _ = gix.at_path(repo.path());
        let failed = Emitter::default()
            .add_instructions(&gix)?
            .emit_to(&mut stdout_buf)?;
        assert!(!failed);
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(GIT_REGEX_INST.is_match(&output));
        Ok(())
    }

    #[test]
    #[serial]
    fn git_emit_at_test_repo() -> Result<()> {
        let repo = TestRepos::new(true, false, false)?;
        let mut gix = GixBuilder::default()
            .all()
            .describe(true, false, None)
            .sha(true)
            .build()?;
        let _ = gix.at_path(repo.path());
        assert!(Emitter::default().add_instructions(&gix)?.emit().is_ok());
        Ok(())
    }

    #[test]
    #[serial]
    fn git_all_idempotent_output() -> Result<()> {
        let mut stdout_buf = vec![];
        let gix = GixBuilder::all_git()?;
        let failed = Emitter::default()
            .idempotent()
            .add_instructions(&gix)?
            .emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(!failed);
        assert!(repo_exists().is_ok());
        assert!(GIT_REGEX_IDEM_INST.is_match(&output));
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn git_all_idempotent_output_quiet() -> Result<()> {
        let mut stdout_buf = vec![];
        let gix = GixBuilder::all_git()?;
        let failed = Emitter::default()
            .idempotent()
            .quiet()
            .add_instructions(&gix)?
            .emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(!failed);
        assert!(repo_exists().is_ok());
        assert!(GIT_REGEX_IDEM_INST.is_match(&output));
        Ok(())
    }

    #[test]
    #[serial]
    fn git_branch_override_works() {
        with_var("VERGEN_GIT_BRANCH", Some("this is a bad date"), || {
            let result = || -> Result<()> {
                let mut stdout_buf = vec![];
                let gix = GixBuilder::all_git()?;
                let _failed = Emitter::default()
                    .add_instructions(&gix)?
                    .emit_to(&mut stdout_buf)?;
                let output = String::from_utf8_lossy(&stdout_buf);
                assert!(output.contains("cargo:rustc-env=VERGEN_GIT_BRANCH=this is a bad date"));
                assert!(output.contains("cargo:warning=VERGEN_GIT_BRANCH overidden"));
                Ok(())
            }();
            assert!(result.is_ok());
        });
    }

    #[test]
    #[serial]
    fn git_commit_author_email_override_works() {
        with_var(
            "VERGEN_GIT_COMMIT_AUTHOR_EMAIL",
            Some("this is a bad date"),
            || {
                let result = || -> Result<()> {
                    let mut stdout_buf = vec![];
                    let gix = GixBuilder::all_git()?;
                    let _failed = Emitter::default()
                        .add_instructions(&gix)?
                        .emit_to(&mut stdout_buf)?;
                    let output = String::from_utf8_lossy(&stdout_buf);
                    assert!(output.contains(
                        "cargo:rustc-env=VERGEN_GIT_COMMIT_AUTHOR_EMAIL=this is a bad date"
                    ));
                    assert!(
                        output.contains("cargo:warning=VERGEN_GIT_COMMIT_AUTHOR_EMAIL overidden")
                    );
                    Ok(())
                }();
                assert!(result.is_ok());
            },
        );
    }

    #[test]
    #[serial]
    fn git_commit_author_name_override_works() {
        with_var(
            "VERGEN_GIT_COMMIT_AUTHOR_NAME",
            Some("this is a bad date"),
            || {
                let result = || -> Result<()> {
                    let mut stdout_buf = vec![];
                    let gix = GixBuilder::all_git()?;
                    let _failed = Emitter::default()
                        .add_instructions(&gix)?
                        .emit_to(&mut stdout_buf)?;
                    let output = String::from_utf8_lossy(&stdout_buf);
                    assert!(output.contains(
                        "cargo:rustc-env=VERGEN_GIT_COMMIT_AUTHOR_NAME=this is a bad date"
                    ));
                    assert!(
                        output.contains("cargo:warning=VERGEN_GIT_COMMIT_AUTHOR_NAME overidden")
                    );
                    Ok(())
                }();
                assert!(result.is_ok());
            },
        );
    }

    #[test]
    #[serial]
    fn git_commit_count_override_works() {
        with_var(
            "VERGEN_GIT_COMMIT_COUNT",
            Some("this is a bad date"),
            || {
                let result = || -> Result<()> {
                    let mut stdout_buf = vec![];
                    let gix = GixBuilder::all_git()?;
                    let _failed = Emitter::default()
                        .add_instructions(&gix)?
                        .emit_to(&mut stdout_buf)?;
                    let output = String::from_utf8_lossy(&stdout_buf);
                    assert!(output
                        .contains("cargo:rustc-env=VERGEN_GIT_COMMIT_COUNT=this is a bad date"));
                    assert!(output.contains("cargo:warning=VERGEN_GIT_COMMIT_COUNT overidden"));
                    Ok(())
                }();
                assert!(result.is_ok());
            },
        );
    }

    #[test]
    #[serial]
    fn git_commit_date_override_works() {
        with_var("VERGEN_GIT_COMMIT_DATE", Some("this is a bad date"), || {
            let result = || -> Result<()> {
                let mut stdout_buf = vec![];
                let gix = GixBuilder::all_git()?;
                let _failed = Emitter::default()
                    .add_instructions(&gix)?
                    .emit_to(&mut stdout_buf)?;
                let output = String::from_utf8_lossy(&stdout_buf);
                assert!(
                    output.contains("cargo:rustc-env=VERGEN_GIT_COMMIT_DATE=this is a bad date")
                );
                assert!(output.contains("cargo:warning=VERGEN_GIT_COMMIT_DATE overidden"));
                Ok(())
            }();
            assert!(result.is_ok());
        });
    }

    #[test]
    #[serial]
    fn git_commit_message_override_works() {
        with_var(
            "VERGEN_GIT_COMMIT_MESSAGE",
            Some("this is a bad date"),
            || {
                let result = || -> Result<()> {
                    let mut stdout_buf = vec![];
                    let gix = GixBuilder::all_git()?;
                    let _failed = Emitter::default()
                        .add_instructions(&gix)?
                        .emit_to(&mut stdout_buf)?;
                    let output = String::from_utf8_lossy(&stdout_buf);
                    assert!(output
                        .contains("cargo:rustc-env=VERGEN_GIT_COMMIT_MESSAGE=this is a bad date"));
                    assert!(output.contains("cargo:warning=VERGEN_GIT_COMMIT_MESSAGE overidden"));
                    Ok(())
                }();
                assert!(result.is_ok());
            },
        );
    }

    #[test]
    #[serial]
    fn git_commit_timestamp_override_works() {
        with_var(
            "VERGEN_GIT_COMMIT_TIMESTAMP",
            Some("this is a bad date"),
            || {
                let result = || -> Result<()> {
                    let mut stdout_buf = vec![];
                    let gix = GixBuilder::all_git()?;
                    let _failed = Emitter::default()
                        .add_instructions(&gix)?
                        .emit_to(&mut stdout_buf)?;
                    let output = String::from_utf8_lossy(&stdout_buf);
                    assert!(output.contains(
                        "cargo:rustc-env=VERGEN_GIT_COMMIT_TIMESTAMP=this is a bad date"
                    ));
                    assert!(output.contains("cargo:warning=VERGEN_GIT_COMMIT_TIMESTAMP overidden"));
                    Ok(())
                }();
                assert!(result.is_ok());
            },
        );
    }

    #[test]
    #[serial]
    fn git_describe_override_works() {
        with_var("VERGEN_GIT_DESCRIBE", Some("this is a bad date"), || {
            let result = || -> Result<()> {
                let mut stdout_buf = vec![];
                let gix = GixBuilder::all_git()?;
                let _failed = Emitter::default()
                    .add_instructions(&gix)?
                    .emit_to(&mut stdout_buf)?;
                let output = String::from_utf8_lossy(&stdout_buf);
                assert!(output.contains("cargo:rustc-env=VERGEN_GIT_DESCRIBE=this is a bad date"));
                assert!(output.contains("cargo:warning=VERGEN_GIT_DESCRIBE overidden"));
                Ok(())
            }();
            assert!(result.is_ok());
        });
    }

    #[test]
    #[serial]
    fn git_sha_override_works() {
        with_var("VERGEN_GIT_SHA", Some("this is a bad date"), || {
            let result = || -> Result<()> {
                let mut stdout_buf = vec![];
                let gix = GixBuilder::all_git()?;
                let _failed = Emitter::default()
                    .add_instructions(&gix)?
                    .emit_to(&mut stdout_buf)?;
                let output = String::from_utf8_lossy(&stdout_buf);
                assert!(output.contains("cargo:rustc-env=VERGEN_GIT_SHA=this is a bad date"));
                assert!(output.contains("cargo:warning=VERGEN_GIT_SHA overidden"));
                Ok(())
            }();
            assert!(result.is_ok());
        });
    }
}
