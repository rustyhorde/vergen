#[cfg(all(
    feature = "git",
    any(feature = "gitcl", feature = "git2", feature = "gix")
))]
mod test_git_git2 {
    use anyhow::Result;
    use git::refs::transaction::PreviousValue;
    #[cfg(feature = "git2")]
    use git2_rs::Repository;
    use git_repository as git;
    use lazy_static::lazy_static;
    use regex::Regex;
    use std::{env, path::PathBuf, sync::Once};
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

    const ALL_IDEM_OUTPUT: &str = r#"cargo:rustc-env=VERGEN_GIT_BRANCH=VERGEN_IDEMPOTENT_OUTPUT
cargo:rustc-env=VERGEN_GIT_COMMIT_AUTHOR_EMAIL=VERGEN_IDEMPOTENT_OUTPUT
cargo:rustc-env=VERGEN_GIT_COMMIT_AUTHOR_NAME=VERGEN_IDEMPOTENT_OUTPUT
cargo:rustc-env=VERGEN_GIT_COMMIT_COUNT=VERGEN_IDEMPOTENT_OUTPUT
cargo:rustc-env=VERGEN_GIT_COMMIT_DATE=VERGEN_IDEMPOTENT_OUTPUT
cargo:rustc-env=VERGEN_GIT_COMMIT_MESSAGE=VERGEN_IDEMPOTENT_OUTPUT
cargo:rustc-env=VERGEN_GIT_COMMIT_TIMESTAMP=VERGEN_IDEMPOTENT_OUTPUT
cargo:rustc-env=VERGEN_GIT_DESCRIBE=VERGEN_IDEMPOTENT_OUTPUT
cargo:rustc-env=VERGEN_GIT_SHA=VERGEN_IDEMPOTENT_OUTPUT
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
cargo:rerun-if-env-changed=VERGEN_SKIP_IF_ERROR
cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH
"#;

    static CREATE_TEST_REPO: Once = Once::new();

    fn create_test_repo() {
        CREATE_TEST_REPO.call_once(|| {
            || -> Result<()> {
                let mut path = if let Ok(temp_path) = env::var("RUNNER_TEMP") {
                    PathBuf::from(temp_path)
                } else {
                    env::temp_dir()
                };
                path.push("vergen_tmp.git");
                println!("Creating repo at '{}'", path.display());

                if !path.exists() {
                    let mut repo = git::init_bare(&path)?;
                    let mut tree = git::objs::Tree::empty();
                    let empty_tree_id = repo.write_object(&tree)?.detach();

                    let mut config = repo.config_snapshot_mut();
                    config.set_raw_value("author", None, "name", "Vergen Test")?;
                    config.set_raw_value("author", None, "email", "vergen@blah.com")?;
                    {
                        let committer = config.commit_auto_rollback()?;
                        let initial_commit_id = committer.commit(
                            "HEAD",
                            "initial commit",
                            empty_tree_id,
                            git::commit::NO_PARENT_IDS,
                        )?;

                        let blob_id = committer.write_blob("hello, world")?.into();
                        let entry = git::objs::tree::Entry {
                            mode: git::objs::tree::EntryMode::Blob,
                            filename: "foo.txt".into(),
                            oid: blob_id,
                        };

                        tree.entries.push(entry);
                        let hello_tree_id = committer.write_object(&tree)?;

                        let blob_commit_id = committer.commit(
                            "HEAD",
                            "foo commit",
                            hello_tree_id,
                            [initial_commit_id],
                        )?;

                        let _tag_id = committer.tag(
                            "0.1.0",
                            blob_commit_id,
                            git::objs::Kind::Commit,
                            None,
                            "v0.1.0",
                            PreviousValue::MustNotExist,
                        )?;

                        let blob_id = committer.write_blob("Hello, World!")?.into();
                        let entry = git::objs::tree::Entry {
                            mode: git::objs::tree::EntryMode::Blob,
                            oid: blob_id,
                            filename: "foo.txt".into(),
                        };

                        let mut tree = git::objs::Tree::empty();
                        tree.entries.push(entry);
                        let commit_2 = committer.write_object(&tree)?;

                        let _blob_commit_id = committer.commit(
                            "HEAD",
                            "such bad casing",
                            commit_2,
                            [blob_commit_id],
                        )?;
                    }
                }

                Ok(())
            }()
            .expect("unable to create test repository");
        });
    }

    #[cfg(feature = "git2")]
    fn repo_exists() -> Result<()> {
        let curr_dir = env::current_dir()?;
        let _repo = Repository::discover(curr_dir)?;
        Ok(())
    }

    #[cfg(feature = "gix")]
    fn repo_exists() -> Result<()> {
        let curr_dir = env::current_dir()?;
        let _repo = git_repository::discover(curr_dir)?;
        Ok(())
    }

    #[cfg(feature = "gitcl")]
    fn repo_exists() -> Result<()> {
        Ok(())
    }

    #[test]
    fn git_all_output() -> Result<()> {
        create_test_repo();
        let mut stdout_buf = vec![];
        let failed = EmitBuilder::builder().all_git().emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        if repo_exists().is_ok() && !failed {
            assert!(GIT_REGEX_INST.is_match(&output));
        } else {
            assert_eq!(ALL_IDEM_OUTPUT, output);
        }
        Ok(())
    }

    #[test]
    fn git_all_idempotent_output() -> Result<()> {
        let mut stdout_buf = vec![];
        let failed = EmitBuilder::builder()
            .idempotent()
            .all_git()
            .emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        if repo_exists().is_ok() && !failed {
            assert!(GIT_REGEX_IDEM_INST.is_match(&output));
        } else {
            assert_eq!(ALL_IDEM_OUTPUT, output);
        }
        Ok(())
    }
}
