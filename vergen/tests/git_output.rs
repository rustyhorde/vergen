#[cfg(all(
    feature = "git",
    any(feature = "gitcl", feature = "git2", feature = "gix")
))]
mod test_git_git2 {
    use anyhow::Result;
    use lazy_static::lazy_static;
    use regex::Regex;
    use std::env;
    use vergen::EmitBuilder;

    use repo_util::TestRepos;

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
        static ref GIT_DIRTY_RE_STR: &'static str =
            r"cargo:rustc-env=VERGEN_GIT_DIRTY=(true|false)";
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
        static ref GIT_DIRTY_IDEM_RE_STR: &'static str =
            r"cargo:rustc-env=VERGEN_GIT_DIRTY=VERGEN_IDEMPOTENT_OUTPUT";
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
cargo:warning=VERGEN_GIT_DIRTY set to default
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
                *GIT_DIRTY_RE_STR,
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
                *GIT_DIRTY_RE_STR,
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
                *GIT_DIRTY_RE_STR,
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
                *GIT_DIRTY_IDEM_RE_STR,
                *WARNINGS_RERUN_RE_STR,
            ]
            .join("\n");
            Regex::new(&re_str).unwrap()
        };
    }

    const DISABLED_OUTPUT: &str = r"";

    cfg_if::cfg_if! {
        if #[cfg(feature = "gitcl")] {
            fn repo_exists() -> Result<()> {
                Ok(())
            }
        } else if #[cfg(feature = "git2")] {
            use git2_rs::Repository;

            fn repo_exists() -> Result<()> {
                let curr_dir = env::current_dir()?;
                let _repo = Repository::discover(curr_dir)?;
                Ok(())
            }
        } else if #[cfg(feature = "gix")] {
            fn repo_exists() -> Result<()> {
                let curr_dir = env::current_dir()?;
                let _repo = gix::discover(curr_dir)?;
                Ok(())
            }
        } else {
            fn repo_exists() -> Result<()> {
                Ok(())
            }
        }
    }

    #[test]
    #[serial_test::serial]
    fn git_all_output_idempotent() -> Result<()> {
        let mut stdout_buf = vec![];
        let failed = EmitBuilder::builder()
            .all_git()
            .emit_to_at(&mut stdout_buf, Some(env::temp_dir()))?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(failed);
        assert!(ALL_IDEM_OUTPUT.is_match(&output));
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn git_all_output_default_dir() -> Result<()> {
        let mut stdout_buf = vec![];
        let failed = EmitBuilder::builder().all_git().emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(!failed);
        assert!(repo_exists().is_ok());
        assert!(GIT_REGEX_INST.is_match(&output));
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn git_all_flags_test_repo() -> Result<()> {
        let repo = TestRepos::new(true, false, false)?;
        let mut stdout_buf = vec![];
        let failed = EmitBuilder::builder()
            .all_git()
            .git_describe(true, true, Some("0.1*"))
            .git_sha(true)
            .emit_to_at(&mut stdout_buf, Some(repo.path()))?;
        assert!(!failed);
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(GIT_REGEX_SHORT_INST.is_match(&output));
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn git_all_flags_test_repo_local() -> Result<()> {
        let repo = TestRepos::new(true, false, false)?;
        let mut stdout_buf = vec![];
        let result = EmitBuilder::builder()
            .all_git()
            .git_describe(true, true, Some("0.1*"))
            .git_sha(true)
            .use_local_git()
            .fail_on_error()
            .emit_to_at(&mut stdout_buf, Some(repo.path()));
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
    #[serial_test::serial]
    fn git_all_output_test_repo() -> Result<()> {
        let repo = TestRepos::new(true, true, false)?;
        let mut stdout_buf = vec![];
        let failed = EmitBuilder::builder()
            .all_git()
            .git_describe(true, true, Some("0.1*"))
            .emit_to_at(&mut stdout_buf, Some(repo.path()))?;
        assert!(!failed);
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(GIT_REGEX_INST.is_match(&output));
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn git_disabled_output() -> Result<()> {
        let mut stdout_buf = vec![];
        EmitBuilder::builder()
            .all_git()
            .disable_git()
            .emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert_eq!(DISABLED_OUTPUT, output);
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn git_emit_at_test_repo() -> Result<()> {
        let repo = TestRepos::new(true, false, false)?;
        assert!(EmitBuilder::builder()
            .all_git()
            .git_describe(true, true, None)
            .git_sha(true)
            .emit_at(repo.path())
            .is_ok());
        Ok(())
    }

    #[cfg(all(feature = "git", any(feature = "gitcl", feature = "git2")))]
    #[cfg(test)]
    mod git_dirty {
        use anyhow::Result;
        use vergen::EmitBuilder;

        use repo_util::TestRepos;

        const GIT_DIRTY_TRUE_OUTPUT: &str = r"cargo:rustc-env=VERGEN_GIT_DIRTY=true";
        const GIT_DIRTY_FALSE_OUTPUT: &str = r"cargo:rustc-env=VERGEN_GIT_DIRTY=false";

        fn strip_reruns(output: &str) -> String {
            let lines: Vec<&str> = output
                .lines()
                .filter(|line| !line.starts_with("cargo:rerun-if"))
                .collect();

            lines.join("\n")
        }

        #[test]
        #[serial_test::serial]
        fn git_dirty_ignore_untracked_no_modified_no_untracked() -> Result<()> {
            // On a repository with no modified files and no untracked files,
            // dirty should be false.
            let repo = TestRepos::new(false, false, false)?;

            let mut stdout_buf = vec![];
            EmitBuilder::builder()
                .git_dirty(false)
                .emit_to_at(&mut stdout_buf, Some(repo.path()))?;

            let output = String::from_utf8_lossy(&stdout_buf);
            let stripped_output = strip_reruns(&output);
            assert_eq!(GIT_DIRTY_FALSE_OUTPUT, stripped_output);
            Ok(())
        }

        #[test]
        #[serial_test::serial]
        fn git_dirty_include_untracked_no_modified_no_untracked() -> Result<()> {
            // On a repository with no modified files and no untracked files,
            // dirty should be false.
            let repo = TestRepos::new(false, false, false)?;

            let mut stdout_buf = vec![];
            EmitBuilder::builder()
                .git_dirty(true)
                .emit_to_at(&mut stdout_buf, Some(repo.path()))?;

            let output = String::from_utf8_lossy(&stdout_buf);
            let stripped_output = strip_reruns(&output);
            assert_eq!(GIT_DIRTY_FALSE_OUTPUT, stripped_output);
            Ok(())
        }

        #[test]
        #[serial_test::serial]
        fn git_dirty_ignore_untracked_modified_no_untracked() -> Result<()> {
            // On a repository with modified files and no untracked files,
            // dirty should be true.
            let repo = TestRepos::new(true, false, false)?;

            let mut stdout_buf = vec![];
            EmitBuilder::builder()
                .git_dirty(false)
                .emit_to_at(&mut stdout_buf, Some(repo.path()))?;

            let output = String::from_utf8_lossy(&stdout_buf);
            let stripped_output = strip_reruns(&output);
            assert_eq!(GIT_DIRTY_TRUE_OUTPUT, stripped_output);
            Ok(())
        }

        #[test]
        #[serial_test::serial]
        fn git_dirty_include_untracked_modified_no_untracked() -> Result<()> {
            // On a repository with modified files and no untracked files,
            // dirty should be true.
            let repo = TestRepos::new(true, false, false)?;

            let mut stdout_buf = vec![];
            EmitBuilder::builder()
                .git_dirty(true)
                .emit_to_at(&mut stdout_buf, Some(repo.path()))?;

            let output = String::from_utf8_lossy(&stdout_buf);
            let stripped_output = strip_reruns(&output);
            assert_eq!(GIT_DIRTY_TRUE_OUTPUT, stripped_output);
            Ok(())
        }

        #[test]
        #[serial_test::serial]
        fn git_dirty_ignore_untracked_no_modified_untracked() -> Result<()> {
            // On a repository with no modified files and untracked files,
            // dirty should be false when include_untracked is false.
            let repo = TestRepos::new(false, true, false)?;

            let mut stdout_buf = vec![];
            EmitBuilder::builder()
                .git_dirty(false)
                .emit_to_at(&mut stdout_buf, Some(repo.path()))?;

            let output = String::from_utf8_lossy(&stdout_buf);
            let stripped_output = strip_reruns(&output);
            assert_eq!(GIT_DIRTY_FALSE_OUTPUT, stripped_output);
            Ok(())
        }

        #[test]
        #[serial_test::serial]
        fn git_dirty_include_untracked_no_modified_untracked() -> Result<()> {
            // On a repository with no modified files and untracked files,
            // dirty should be true when include_untracked is true.
            let repo = TestRepos::new(false, true, false)?;

            let mut stdout_buf = vec![];
            EmitBuilder::builder()
                .git_dirty(true)
                .emit_to_at(&mut stdout_buf, Some(repo.path()))?;

            let output = String::from_utf8_lossy(&stdout_buf);
            let stripped_output = strip_reruns(&output);
            assert_eq!(GIT_DIRTY_TRUE_OUTPUT, stripped_output);
            Ok(())
        }

        #[test]
        #[serial_test::serial]
        fn git_dirty_ignore_untracked_modified_untracked() -> Result<()> {
            // On a repository with modified files and untracked files,
            // dirty should be true.
            let repo = TestRepos::new(true, true, false)?;

            let mut stdout_buf = vec![];
            EmitBuilder::builder()
                .git_dirty(false)
                .emit_to_at(&mut stdout_buf, Some(repo.path()))?;

            let output = String::from_utf8_lossy(&stdout_buf);
            let stripped_output = strip_reruns(&output);
            assert_eq!(GIT_DIRTY_TRUE_OUTPUT, stripped_output);
            Ok(())
        }

        #[test]
        #[serial_test::serial]
        fn git_dirty_include_untracked_modified_untracked() -> Result<()> {
            // On a repository with modified files and untracked files,
            // dirty should be true.
            let repo = TestRepos::new(true, true, false)?;

            let mut stdout_buf = vec![];
            EmitBuilder::builder()
                .git_dirty(true)
                .emit_to_at(&mut stdout_buf, Some(repo.path()))?;

            let output = String::from_utf8_lossy(&stdout_buf);
            let stripped_output = strip_reruns(&output);
            assert_eq!(GIT_DIRTY_TRUE_OUTPUT, stripped_output);
            Ok(())
        }
    }

    #[test]
    #[serial_test::serial]
    fn git_all_idempotent_output() -> Result<()> {
        let mut stdout_buf = vec![];
        let failed = EmitBuilder::builder()
            .idempotent()
            .all_git()
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
        let failed = EmitBuilder::builder()
            .idempotent()
            .quiet()
            .all_git()
            .emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(!failed);
        assert!(repo_exists().is_ok());
        assert!(GIT_REGEX_IDEM_INST.is_match(&output));
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn git_branch_override_works() -> Result<()> {
        env::set_var("VERGEN_GIT_BRANCH", "this is a bad date");
        let mut stdout_buf = vec![];
        let failed = EmitBuilder::builder().all_git().emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(output.contains("cargo:rustc-env=VERGEN_GIT_BRANCH=this is a bad date"));
        if failed {
            assert!(output.contains("cargo:warning=VERGEN_GIT_BRANCH overidden"));
        }
        env::remove_var("VERGEN_GIT_BRANCH");
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn git_commit_author_email_override_works() -> Result<()> {
        env::set_var("VERGEN_GIT_COMMIT_AUTHOR_EMAIL", "this is a bad date");
        let mut stdout_buf = vec![];
        let failed = EmitBuilder::builder().all_git().emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(
            output.contains("cargo:rustc-env=VERGEN_GIT_COMMIT_AUTHOR_EMAIL=this is a bad date")
        );
        if failed {
            assert!(output.contains("cargo:warning=VERGEN_GIT_COMMIT_AUTHOR_EMAIL overidden"));
        }
        env::remove_var("VERGEN_GIT_COMMIT_AUTHOR_EMAIL");
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn git_commit_author_name_override_works() -> Result<()> {
        env::set_var("VERGEN_GIT_COMMIT_AUTHOR_NAME", "this is a bad date");
        let mut stdout_buf = vec![];
        let failed = EmitBuilder::builder().all_git().emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(output.contains("cargo:rustc-env=VERGEN_GIT_COMMIT_AUTHOR_NAME=this is a bad date"));
        if failed {
            assert!(output.contains("cargo:warning=VERGEN_GIT_COMMIT_AUTHOR_NAME overidden"));
        }
        env::remove_var("VERGEN_GIT_COMMIT_AUTHOR_NAME");
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn git_commit_count_override_works() -> Result<()> {
        env::set_var("VERGEN_GIT_COMMIT_COUNT", "this is a bad date");
        let mut stdout_buf = vec![];
        let failed = EmitBuilder::builder().all_git().emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(output.contains("cargo:rustc-env=VERGEN_GIT_COMMIT_COUNT=this is a bad date"));
        if failed {
            assert!(output.contains("cargo:warning=VERGEN_GIT_COMMIT_COUNT overidden"));
        }
        env::remove_var("VERGEN_GIT_COMMIT_COUNT");
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn git_commit_date_override_works() -> Result<()> {
        env::set_var("VERGEN_GIT_COMMIT_DATE", "this is a bad date");
        let mut stdout_buf = vec![];
        let failed = EmitBuilder::builder().all_git().emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(output.contains("cargo:rustc-env=VERGEN_GIT_COMMIT_DATE=this is a bad date"));
        if failed {
            assert!(output.contains("cargo:warning=VERGEN_GIT_COMMIT_DATE overidden"));
        }
        env::remove_var("VERGEN_GIT_COMMIT_DATE");
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn git_commit_message_override_works() -> Result<()> {
        env::set_var("VERGEN_GIT_COMMIT_MESSAGE", "this is a bad date");
        let mut stdout_buf = vec![];
        let failed = EmitBuilder::builder().all_git().emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(output.contains("cargo:rustc-env=VERGEN_GIT_COMMIT_MESSAGE=this is a bad date"));
        if failed {
            assert!(output.contains("cargo:warning=VERGEN_GIT_COMMIT_MESSAGE overidden"));
        }
        env::remove_var("VERGEN_GIT_COMMIT_MESSAGE");
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn git_commit_timestamp_override_works() -> Result<()> {
        env::set_var("VERGEN_GIT_COMMIT_TIMESTAMP", "this is a bad date");
        let mut stdout_buf = vec![];
        let failed = EmitBuilder::builder().all_git().emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(output.contains("cargo:rustc-env=VERGEN_GIT_COMMIT_TIMESTAMP=this is a bad date"));
        if failed {
            assert!(output.contains("cargo:warning=VERGEN_GIT_COMMIT_TIMESTAMP overidden"));
        }
        env::remove_var("VERGEN_GIT_COMMIT_TIMESTAMP");
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn git_describe_override_works() -> Result<()> {
        env::set_var("VERGEN_GIT_DESCRIBE", "this is a bad date");
        let mut stdout_buf = vec![];
        let failed = EmitBuilder::builder().all_git().emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(output.contains("cargo:rustc-env=VERGEN_GIT_DESCRIBE=this is a bad date"));
        if failed {
            assert!(output.contains("cargo:warning=VERGEN_GIT_DESCRIBE overidden"));
        }
        env::remove_var("VERGEN_GIT_DESCRIBE");
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn git_sha_override_works() -> Result<()> {
        env::set_var("VERGEN_GIT_SHA", "this is a bad date");
        let mut stdout_buf = vec![];
        let failed = EmitBuilder::builder().all_git().emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(output.contains("cargo:rustc-env=VERGEN_GIT_SHA=this is a bad date"));
        if failed {
            assert!(output.contains("cargo:warning=VERGEN_GIT_SHA overidden"));
        }
        env::remove_var("VERGEN_GIT_SHA");
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn git_dirty_override_works() -> Result<()> {
        env::set_var("VERGEN_GIT_DIRTY", "this is a bad date");
        let mut stdout_buf = vec![];
        let failed = EmitBuilder::builder().all_git().emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(output.contains("cargo:rustc-env=VERGEN_GIT_DIRTY=this is a bad date"));
        if failed {
            assert!(output.contains("cargo:warning=VERGEN_GIT_DIRTY overidden"));
        }
        env::remove_var("VERGEN_GIT_DIRTY");
        Ok(())
    }

    #[cfg(feature = "gitcl")]
    #[test]
    #[serial_test::serial]
    fn git_cmd_override_works() -> Result<()> {
        let mut stdout_buf = vec![];
        let failed = EmitBuilder::builder()
            .all_git()
            .git_cmd(Some("git -v"))
            .emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        assert!(!failed);
        assert!(repo_exists().is_ok());
        assert!(GIT_REGEX_INST.is_match(&output));
        Ok(())
    }
}
