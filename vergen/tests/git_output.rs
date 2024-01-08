#[cfg(all(
    feature = "git",
    any(feature = "gitcl", feature = "git2", feature = "gix")
))]
mod test_git_git2 {
    use anyhow::Result;
    use git::{
        create::Options,
        objs::tree::{Entry, EntryKind},
        open,
        refs::transaction::PreviousValue,
    };
    #[cfg(feature = "git2")]
    use git2_rs::Repository;
    use gix as git;
    use lazy_static::lazy_static;
    use regex::Regex;
    use std::{
        env,
        fs::{self, File, OpenOptions},
        io::BufWriter,
        io::Write,
        path::PathBuf,
    };
    use vergen::EmitBuilder;

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

    const GIT_DIRTY_TRUE_OUTPUT: &str = r"cargo:rustc-env=VERGEN_GIT_DIRTY=true";
    const GIT_DIRTY_FALSE_OUTPUT: &str = r"cargo:rustc-env=VERGEN_GIT_DIRTY=false";

    const IDEM_QUIET_OUTPUT: &str = r"cargo:rustc-env=VERGEN_GIT_BRANCH=VERGEN_IDEMPOTENT_OUTPUT
cargo:rustc-env=VERGEN_GIT_COMMIT_AUTHOR_EMAIL=VERGEN_IDEMPOTENT_OUTPUT
cargo:rustc-env=VERGEN_GIT_COMMIT_AUTHOR_NAME=VERGEN_IDEMPOTENT_OUTPUT
cargo:rustc-env=VERGEN_GIT_COMMIT_COUNT=VERGEN_IDEMPOTENT_OUTPUT
cargo:rustc-env=VERGEN_GIT_COMMIT_DATE=VERGEN_IDEMPOTENT_OUTPUT
cargo:rustc-env=VERGEN_GIT_COMMIT_MESSAGE=VERGEN_IDEMPOTENT_OUTPUT
cargo:rustc-env=VERGEN_GIT_COMMIT_TIMESTAMP=VERGEN_IDEMPOTENT_OUTPUT
cargo:rustc-env=VERGEN_GIT_DESCRIBE=VERGEN_IDEMPOTENT_OUTPUT
cargo:rustc-env=VERGEN_GIT_SHA=VERGEN_IDEMPOTENT_OUTPUT
cargo:rustc-env=VERGEN_GIT_DIRTY=VERGEN_IDEMPOTENT_OUTPUT
cargo:rerun-if-changed=build.rs
cargo:rerun-if-env-changed=VERGEN_IDEMPOTENT
cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH
";

    const DISABLED_OUTPUT: &str = r"";

    const BARE_REPO_NAME: &str = "vergen_tmp.git";
    const CLONE_NAME: &str = "vergen_tmp";

    fn strip_reruns(output: &str) -> String {
        let lines: Vec<&str> = output
            .lines()
            .filter(|line| !line.starts_with("cargo:rerun-if"))
            .collect();

        lines.join("\n")
    }

    fn create_test_repo(path: &PathBuf) -> Result<()> {
        // Always make sure to re-create repo in CI
        if let Ok(_ci) = env::var("CI") {
            let _res = fs::remove_dir_all(&path);
        }
        if !path.exists() {
            let create_repo_result = || -> Result<()> {
                // Initialize a bare repository
                let mut repo = git::init_bare(&path)?;

                // Create an empty tree for the initial commit
                let mut tree = git::objs::Tree::empty();
                let empty_tree_id = repo.write_object(&tree)?.detach();

                // Setup the base configuration
                let mut config = repo.config_snapshot_mut();
                let _old = config.set_raw_value("user", None, "name", "Vergen Test")?;
                let _old = config.set_raw_value("user", None, "email", "vergen@blah.com")?;
                {
                    // Create an empty commit with the initial empty tree
                    let committer = config.commit_auto_rollback()?;
                    let initial_commit_id = committer.commit(
                        "HEAD",
                        "initial commit",
                        empty_tree_id,
                        git::commit::NO_PARENT_IDS,
                    )?;

                    // Create a BLOB to commit, along with the corresponding tree entry
                    let first_blob_id = committer.write_blob("hello, world")?.into();
                    let entry = Entry {
                        mode: EntryKind::Blob.into(),
                        filename: "foo.txt".into(),
                        oid: first_blob_id,
                    };

                    // Add everything to the empty tree
                    tree.entries.push(entry);
                    let first_tree_id = committer.write_object(&tree)?;

                    // Make the commit
                    let first_commit_id = committer.commit(
                        "HEAD",
                        "foo commit",
                        first_tree_id,
                        [initial_commit_id],
                    )?;

                    // Tag the previous commit
                    let _tag_id = committer.tag(
                        "0.1.0",
                        first_commit_id,
                        git::objs::Kind::Commit,
                        None,
                        "v0.1.0",
                        PreviousValue::MustNotExist,
                    )?;

                    // Create a new BLOB to commit
                    let second_blob_id = committer.write_blob("Hello, World!")?.into();
                    let entry = Entry {
                        mode: EntryKind::Blob.into(),
                        oid: second_blob_id,
                        filename: "foo.txt".into(),
                    };

                    // Setup a new tree for this commit
                    let mut second_tree = git::objs::Tree::empty();
                    second_tree.entries.push(entry);
                    let second_tree_id = committer.write_object(&second_tree)?;

                    // Make the commit
                    let _second_commit_id = committer.commit(
                        "HEAD",
                        "such bad casing",
                        second_tree_id,
                        [first_commit_id],
                    )?;
                }

                Ok(())
            }();

            if create_repo_result.is_err() {
                let _res = fs::remove_dir_all(&path);
            }
            create_repo_result
        } else {
            Err(anyhow::anyhow!(
                "The temp repo path exists which it should not. \
                Please remove and try again: {:?}",
                path
            ))
        }
    }

    fn clone_test_repo(bare_repo_path: &PathBuf, clone_path: &PathBuf) -> Result<()> {
        // Always make sure to clone a fresh directory in CI
        if let Ok(_ci) = env::var("CI") {
            let _res = fs::remove_dir_all(&clone_path);
        }

        if !clone_path.exists() {
            let clone_result = || -> Result<()> {
                // Setup the directory
                fs::create_dir_all(&clone_path)?;

                // Clone into the directory
                let url =
                    git::url::parse(git::path::os_str_into_bstr(bare_repo_path.as_os_str())?)?;
                let opts = open::Options::isolated()
                    .config_overrides(["user.name=Vergen Test", "user.email=vergen@blah.com"]);
                let mut prep = git::clone::PrepareFetch::new(
                    url,
                    &clone_path,
                    git::create::Kind::WithWorktree,
                    Options::default(),
                    opts,
                )?;
                let (mut prepare_checkout, _) = prep
                    .fetch_then_checkout(git::progress::Discard, &git::interrupt::IS_INTERRUPTED)?;
                let (_repo, _) = prepare_checkout
                    .main_worktree(git::progress::Discard, &git::interrupt::IS_INTERRUPTED)?;

                Ok(())
            }();

            if clone_result.is_err() {
                let _res = fs::remove_dir_all(&clone_path);
            }

            clone_result
        } else {
            Err(anyhow::anyhow!(
                "The temp path exists which it should not. \
                Please remove and try again: {:?}",
                clone_path
            ))
        }
    }

    struct TestRepo {
        bare_repo_path: PathBuf,
        clone_path: PathBuf,
    }

    impl TestRepo {
        fn new(make_dirty: bool) -> Result<Self> {
            let bare_repo_path = repo_path();
            let clone_path = clone_path();

            create_test_repo(&bare_repo_path)?;
            clone_test_repo(&bare_repo_path, &clone_path)?;

            let mut test_repo = TestRepo {
                bare_repo_path,
                clone_path,
            };

            if make_dirty {
                test_repo.make_dirty()?;
            }

            Ok(test_repo)
        }

        fn path(&self) -> PathBuf {
            self.clone_path.clone()
        }

        fn make_dirty(&mut self) -> Result<()> {
            // "edit" a file to mark the repository describe as dirty
            let file_path = self.clone_path.join("foo.txt");
            let foo = OpenOptions::new().append(true).open(file_path)?;
            let mut writer = BufWriter::new(foo);
            writeln!(writer, "another test line")?;

            Ok(())
        }

        /// Create a new file that is not under git control
        fn create_uncontrolled_file(&mut self) -> Result<()> {
            let file_path = self.clone_path.join("bar.txt");
            let bar = File::create(file_path)?;
            let mut writer = BufWriter::new(bar);
            writeln!(writer, "an uncontrolled test line")?;

            Ok(())
        }
    }

    impl Drop for TestRepo {
        fn drop(&mut self) {
            let _res = fs::remove_dir_all(&self.clone_path);
            let _res = fs::remove_dir_all(&self.bare_repo_path);
        }
    }

    fn repo_path() -> PathBuf {
        let clone_path = if let Ok(temp_path) = env::var("RUNNER_TEMP") {
            PathBuf::from(temp_path)
        } else {
            env::temp_dir()
        };
        clone_path.join(BARE_REPO_NAME)
    }

    fn clone_path() -> PathBuf {
        let clone_path = if let Ok(temp_path) = env::var("RUNNER_TEMP") {
            PathBuf::from(temp_path)
        } else {
            env::temp_dir()
        };
        clone_path.join(CLONE_NAME)
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
        let _repo = gix::discover(curr_dir)?;
        Ok(())
    }

    #[cfg(feature = "gitcl")]
    fn repo_exists() -> Result<()> {
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn git_all_output_default_dir() -> Result<()> {
        let mut stdout_buf = vec![];
        let failed = EmitBuilder::builder().all_git().emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        if repo_exists().is_ok() && !failed {
            assert!(GIT_REGEX_INST.is_match(&output));
        } else {
            assert!(ALL_IDEM_OUTPUT.is_match(&output));
        }
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn git_all_flags_test_repo() -> Result<()> {
        let repo = TestRepo::new(true)?;
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
        let repo = TestRepo::new(true)?;
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
        let repo = TestRepo::new(true)?;
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
        let repo = TestRepo::new(true)?;
        assert!(EmitBuilder::builder()
            .all_git()
            .git_describe(true, true, None)
            .git_sha(true)
            .emit_at(repo.path())
            .is_ok());
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn git_dirty_set() -> Result<()> {
        let repo = TestRepo::new(true)?;

        let mut stdout_buf = vec![];
        EmitBuilder::builder()
            .git_dirty(false)
            .emit_to_at(&mut stdout_buf, Some(repo.path()))?;

        let output = String::from_utf8_lossy(&stdout_buf);
        let stripped_output = strip_reruns(&output);
        assert_eq!(GIT_DIRTY_TRUE_OUTPUT, stripped_output);

        // Should also work with untracked files turned on but no
        // untracked files included.
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
    fn git_dirty_set_uncontrolled_file() -> Result<()> {
        // We defensively create a clean repo and check that first.
        let mut repo = TestRepo::new(false)?;

        let mut stdout_buf = vec![];
        EmitBuilder::builder()
            .git_dirty(false)
            .emit_to_at(&mut stdout_buf, Some(repo.path()))?;

        let output = String::from_utf8_lossy(&stdout_buf);
        let stripped_output = strip_reruns(&output);
        assert_eq!(GIT_DIRTY_FALSE_OUTPUT, stripped_output);

        // Now add an uncontrolled file to make it dirty
        repo.create_uncontrolled_file()?;

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
    fn git_dirty_unset() -> Result<()> {
        let repo = TestRepo::new(false)?;

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
            assert!(ALL_IDEM_OUTPUT.is_match(&output));
        }
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
        if repo_exists().is_ok() && !failed {
            assert!(GIT_REGEX_IDEM_INST.is_match(&output));
        } else {
            assert_eq!(IDEM_QUIET_OUTPUT, output);
        }
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn repo_path_temp_dir_works() {
        if let Ok(runner_temp) = env::var("RUNNER_TEMP") {
            env::remove_var("RUNNER_TEMP");
            assert!(repo_path().ends_with(BARE_REPO_NAME));
            env::set_var("RUNNER_TEMP", runner_temp);
        } else {
            assert!(repo_path().ends_with(BARE_REPO_NAME));
        }
    }

    #[test]
    #[serial_test::serial]
    fn clone_path_temp_dir_works() {
        if let Ok(runner_temp) = env::var("RUNNER_TEMP") {
            env::remove_var("RUNNER_TEMP");
            assert!(clone_path().ends_with(CLONE_NAME));
            env::set_var("RUNNER_TEMP", runner_temp);
        } else {
            assert!(clone_path().ends_with(CLONE_NAME));
        }
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
        if repo_exists().is_ok() && !failed {
            assert!(GIT_REGEX_INST.is_match(&output));
        } else {
            assert!(ALL_IDEM_OUTPUT.is_match(&output));
        }
        Ok(())
    }
}
