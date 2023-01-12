// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#[cfg(all(test, feature = "cargo"))]
pub(crate) mod testutils {
    use std::env;

    pub(crate) fn setup() {
        env::set_var("CARGO_FEATURE_BUILD", "build");
        env::set_var("CARGO_FEATURE_GIT", "git");
        env::set_var("DEBUG", "true");
        env::set_var("OPT_LEVEL", "1");
        env::set_var("TARGET", "x86_64-unknown-linux-gnu");
    }

    pub(crate) fn teardown() {
        env::remove_var("CARGO_FEATURE_BUILD");
        env::remove_var("CARGO_FEATURE_GIT");
        env::remove_var("DEBUG");
        env::remove_var("OPT_LEVEL");
        env::remove_var("TARGET");
    }
}

#[cfg(any(
    feature = "build",
    feature = "cargo",
    all(
        feature = "git",
        any(feature = "gitcl", feature = "git2", feature = "gix")
    ),
    feature = "rustc",
    feature = "si",
))]
pub(crate) mod fns {
    use crate::{constants::VERGEN_IDEMPOTENT_DEFAULT, emitter::RustcEnvMap, key::VergenKey};

    pub(crate) fn add_default_map_entry(
        key: VergenKey,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
    ) {
        let _old = map.insert(key, VERGEN_IDEMPOTENT_DEFAULT.to_string());
        warnings.push(format!("{} set to default", key.name()));
    }

    pub(crate) fn add_map_entry<T>(key: VergenKey, value: T, map: &mut RustcEnvMap)
    where
        T: Into<String>,
    {
        let _old = map.insert(key, value.into());
    }
}

#[cfg(all(
    test,
    all(
        feature = "git",
        any(feature = "gitcl", feature = "git2", feature = "gix")
    )
))]
pub(crate) mod repo {
    use anyhow::Result;
    use git::{open, refs::transaction::PreviousValue};
    use git_repository as git;
    use std::{
        env,
        fs::{self, OpenOptions},
        io::BufWriter,
        io::Write,
        path::PathBuf,
        sync::Once,
    };

    const BARE_REPO_NAME: &str = "vergen_tmp1.git";
    const CLONE_NAME: &str = "vergen_tmp1";

    static CREATE_TEST_REPO: Once = Once::new();
    static CLONE_TEST_REPO: Once = Once::new();

    pub(crate) fn create_test_repo() {
        CREATE_TEST_REPO.call_once(|| {
            || -> Result<()> {
                let path = repo_path();
                // Always make sure to re-create repo in CI
                if let Ok(_ci) = env::var("CI") {
                    let _res = fs::remove_dir_all(&path);
                }
                if !path.exists() {
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
                        let entry = git::objs::tree::Entry {
                            mode: git::objs::tree::EntryMode::Blob,
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
                        let entry = git::objs::tree::Entry {
                            mode: git::objs::tree::EntryMode::Blob,
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
                }

                Ok(())
            }()
            .expect("unable to create test repository");
        });
    }

    pub(crate) fn clone_test_repo() {
        CLONE_TEST_REPO.call_once(|| {
            || -> Result<()> {
                // The bare repository path
                let bare_repo_path = repo_path();
                // The path we are cloning into
                let clone_path = clone_path();
                // Always make sure to clone a fresh directory in CI
                if let Ok(_ci) = env::var("CI") {
                    let _res = fs::remove_dir_all(&clone_path);
                }

                if !clone_path.exists() {
                    fs::create_dir_all(&clone_path)?;
                    let _res = git::interrupt::init_handler(|| {})?;
                    let url =
                        git::url::parse(git::path::os_str_into_bstr(bare_repo_path.as_os_str())?)?;
                    let opts = open::Options::isolated()
                        .config_overrides(["user.name=Vergen Test", "user.email=vergen@blah.com"]);
                    let mut prep = git::clone::PrepareFetch::new(
                        url,
                        &clone_path,
                        git::create::Kind::WithWorktree,
                        Default::default(),
                        opts,
                    )?;
                    let (mut prepare_checkout, _) = prep.fetch_then_checkout(
                        git::progress::Discard,
                        &git::interrupt::IS_INTERRUPTED,
                    )?;
                    let (_repo, _) = prepare_checkout
                        .main_worktree(git::progress::Discard, &git::interrupt::IS_INTERRUPTED)?;
                    let file_path = clone_path.join("foo.txt");
                    let foo_txt = OpenOptions::new().append(true).open(file_path)?;
                    let mut writer = BufWriter::new(foo_txt);
                    writeln!(writer, "another test line")?;
                }
                Ok(())
            }()
            .expect("unable to clone the test repository");
        });
    }

    pub(crate) fn repo_path() -> PathBuf {
        let clone_path = if let Ok(temp_path) = env::var("RUNNER_TEMP") {
            PathBuf::from(temp_path)
        } else {
            env::temp_dir()
        };
        clone_path.join(BARE_REPO_NAME)
    }

    pub(crate) fn clone_path() -> PathBuf {
        let clone_path = if let Ok(temp_path) = env::var("RUNNER_TEMP") {
            PathBuf::from(temp_path)
        } else {
            env::temp_dir()
        };
        clone_path.join(CLONE_NAME)
    }

    #[cfg(test)]
    mod test {
        use super::{clone_path, repo_path, BARE_REPO_NAME, CLONE_NAME};
        use std::env;

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
    }
}
