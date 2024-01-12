use anyhow::Result;
use git::{
    config::CommitAutoRollback,
    create::Options,
    objs::{
        tree::{Entry, EntryKind},
        Tree,
    },
    open,
    refs::transaction::PreviousValue,
    Id, ObjectId,
};
use gix as git;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::fs::File;
use std::{
    env,
    fs::{self, OpenOptions},
    io::{BufWriter, Write},
    path::PathBuf,
};

const BARE_REPO_PREFIX: &str = "vergen_tmp";
const BARE_REPO_SUFFIX: &str = ".git";
const CLONE_NAME_PREFIX: &str = "vergen_tmp";
const RUNNER_TEMP_ENV: &str = "RUNNER_TEMP";

/// Utility to create a temporary bare repository and a repository cloned from the
/// bare repository.
#[derive(Clone, Debug)]
pub struct TestRepos {
    bare_repo_path: PathBuf,
    clone_path: PathBuf,
}

impl TestRepos {
    /// Create a new bare repository and a repository cloned from the bare repository.
    ///
    /// Optionally, modify a tracked file
    /// Optionally, include an untracked file
    ///
    /// # Errors
    ///
    pub fn new(modify_tracked: bool, include_untracked: bool) -> Result<Self> {
        let bare_repo_path = Self::repo_path();
        let clone_path = Self::clone_path();

        let mut test_repo = TestRepos {
            bare_repo_path,
            clone_path,
        };

        test_repo.create_repository()?;
        test_repo.clone_from_bare_repo()?;

        if modify_tracked {
            test_repo.modify_tracked()?;
        }

        if include_untracked {
            test_repo.create_untracked_file()?;
        }

        Ok(test_repo)
    }

    fn create_repository(&mut self) -> Result<()> {
        let path = &self.bare_repo_path;

        // Always make sure to re-create repo in CI
        if let Ok(_ci) = env::var("CI") {
            let _res = fs::remove_dir_all(path);
        }

        // Initialize a bare repository
        let mut repo = git::init_bare(path)?;

        // Create an empty tree for the initial commit
        let mut tree = Tree::empty();
        let empty_tree_id = repo.write_object(&tree)?.detach();

        // Setup the base configuration
        let mut config = repo.config_snapshot_mut();
        let _old = config.set_raw_value("user", None, "name", "Vergen Test")?;
        let _old = config.set_raw_value("user", None, "email", "vergen@blah.com")?;

        {
            // Create an empty commit with the initial empty tree
            let committer = config.commit_auto_rollback()?;

            // Make an initial empty commit
            let initial_commit_id = committer.commit(
                "HEAD",
                "initial commit",
                empty_tree_id,
                git::commit::NO_PARENT_IDS,
            )?;

            // Create a commit
            let first_commit_id = Self::create_commit(
                &mut tree,
                &committer,
                b"hello, world",
                "foo.txt",
                "foo commit",
                initial_commit_id.into(),
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

            // Create a second commit
            let mut second_tree = git::objs::Tree::empty();
            let _second_commit_id = Self::create_commit(
                &mut second_tree,
                &committer,
                b"Hello, World!",
                "foo.txt",
                "such bad casing",
                first_commit_id.into(),
            )?;
        }

        Ok(())
    }

    fn clone_from_bare_repo(&mut self) -> Result<()> {
        let bare_repo_path = &self.bare_repo_path;
        let clone_path = &self.clone_path;

        // Always make sure to clone a fresh directory in CI
        if let Ok(_ci) = env::var("CI") {
            let _res = fs::remove_dir_all(clone_path);
        }

        // Setup the directory
        fs::create_dir_all(clone_path)?;

        // Clone into the directory
        let url = git::url::parse(git::path::os_str_into_bstr(bare_repo_path.as_os_str())?)?;
        let opts = open::Options::isolated()
            .config_overrides(["user.name=Vergen Test", "user.email=vergen@blah.com"]);
        let mut prep = git::clone::PrepareFetch::new(
            url,
            clone_path,
            git::create::Kind::WithWorktree,
            Options::default(),
            opts,
        )?;
        let (mut prepare_checkout, _) =
            prep.fetch_then_checkout(git::progress::Discard, &git::interrupt::IS_INTERRUPTED)?;
        let (_repo, _) = prepare_checkout
            .main_worktree(git::progress::Discard, &git::interrupt::IS_INTERRUPTED)?;

        Ok(())
    }

    fn create_commit<'a>(
        tree: &mut Tree,
        committer: &'a CommitAutoRollback<'_>,
        blob: &[u8],
        filename: &str,
        message: &str,
        parent: ObjectId,
    ) -> Result<Id<'a>> {
        // Create a BLOB to commit, along with the corresponding tree entry
        let blob_id = committer.write_blob(blob)?.into();
        let entry = Entry {
            mode: EntryKind::Blob.into(),
            filename: filename.into(),
            oid: blob_id,
        };

        // Add everything to the tree
        tree.entries.push(entry);
        let tree_id = committer.write_object(&*tree)?;

        // Make the commit
        let commit_id = committer.commit("HEAD", message, tree_id, [parent])?;

        Ok(commit_id)
    }

    fn modify_tracked(&mut self) -> Result<()> {
        // "edit" a file to create a diffence between the index and worktree (i.e. dirty)
        let file_path = self.clone_path.join("foo.txt");
        let file = OpenOptions::new().append(true).open(file_path)?;
        let mut writer = BufWriter::new(file);
        writeln!(writer, "another test line")?;

        Ok(())
    }

    /// Get the path of the cloned repository
    #[must_use]
    pub fn path(&self) -> PathBuf {
        self.clone_path.clone()
    }

    // Create a new file that is not under git control
    fn create_untracked_file(&mut self) -> Result<()> {
        let file_path = self.clone_path.join("bar.txt");
        let bar = File::create(file_path)?;
        let mut writer = BufWriter::new(bar);
        writeln!(writer, "an uncontrolled test line")?;

        Ok(())
    }

    fn temp_path() -> PathBuf {
        if let Ok(temp_path) = env::var(RUNNER_TEMP_ENV) {
            PathBuf::from(temp_path)
        } else {
            env::temp_dir()
        }
    }

    fn rand_five() -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(5)
            .map(char::from)
            .collect()
    }

    fn repo_path() -> PathBuf {
        let temp_path = Self::temp_path();
        let rand_repo_path = format!("{BARE_REPO_PREFIX}_{}{BARE_REPO_SUFFIX}", Self::rand_five());
        temp_path.join(rand_repo_path)
    }

    fn clone_path() -> PathBuf {
        let temp_path = Self::temp_path();
        let rand_clone_path = format!("{CLONE_NAME_PREFIX}_{}", Self::rand_five());
        temp_path.join(rand_clone_path)
    }
}

impl Drop for TestRepos {
    fn drop(&mut self) {
        let _res = fs::remove_dir_all(&self.clone_path);
        let _res = fs::remove_dir_all(&self.bare_repo_path);
    }
}

#[cfg(test)]
mod test {
    use super::{TestRepos, RUNNER_TEMP_ENV};
    use anyhow::Result;

    #[test]
    #[serial_test::serial]
    fn temp_dir_works() -> Result<()> {
        temp_env::with_var(RUNNER_TEMP_ENV, None::<String>, || {
            let repo = || -> Result<TestRepos> {
                let repo = TestRepos::new(false, false)?;
                Ok(repo)
            }();
            assert!(repo.is_ok());
        });
        Ok(())
    }
}
