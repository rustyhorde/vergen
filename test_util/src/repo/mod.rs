use anyhow::Result;
use git::{
    Id, ObjectId,
    clone::PrepareFetch,
    config::CommitAutoRollback,
    create::{Kind, Options},
    interrupt::IS_INTERRUPTED,
    objs::{
        Tree,
        tree::{Entry, EntryKind},
    },
    open,
    path::os_str_into_bstr,
    progress::Discard,
    refs::transaction::PreviousValue,
    remote::fetch::Shallow,
    url::parse,
};
use gix as git;
use rand::{Rng, distr::Alphanumeric, rng};
use std::{
    env,
    fs::{self, FileTimes, OpenOptions},
    io::{BufWriter, Write},
    path::PathBuf,
    sync::LazyLock,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use std::{fs::File, num::NonZeroU32};

const BARE_REPO_PREFIX: &str = "vergen_tmp";
const BARE_REPO_SUFFIX: &str = ".git";
const CLONE_NAME_PREFIX: &str = "vergen_tmp";
const RUNNER_TEMP_ENV: &str = "RUNNER_TEMP";
const MAGIC_MTIME: u64 = 1_234_567_890;

/// mtime to use for testing
pub static TEST_MTIME: LazyLock<SystemTime> =
    LazyLock::new(|| UNIX_EPOCH + Duration::from_secs(MAGIC_MTIME));

/// Utility to create a temporary bare repository and a repository cloned from the
/// bare repository.
///
/// # Example
/// ```
/// # use anyhow::Result;
/// # use std::path::PathBuf;
/// # use test_util::TestRepos;
/// # pub fn main() -> Result<()> {
/// let mut path = PathBuf::default();
/// {
///     let repo = TestRepos::new(false, false, false)?;
///     path = repo.path();
///     assert!(gix::discover(&path).is_ok());
///     assert!(path.exists());
/// }
/// // When dropped, the repositories will be removed.
/// assert!(!path.exists());
/// #     Ok(())
/// # }
/// ```
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
    /// Optionally, create a shallow clone
    ///
    /// # Example
    /// ```
    /// # use anyhow::Result;
    /// # use std::path::PathBuf;
    /// # use test_util::TestRepos;
    /// # pub fn main() -> Result<()> {
    /// let mut path = PathBuf::default();
    /// {
    ///     let repo = TestRepos::new(false, false, false)?;
    ///     path = repo.path();
    ///     assert!(gix::discover(&path).is_ok());
    ///     assert!(path.exists());
    /// }
    /// // When dropped, the repositories will be removed.
    /// assert!(!path.exists());
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// Many errors can occur mostly from the `gix` library
    pub fn new(modify_tracked: bool, include_untracked: bool, shallow_clone: bool) -> Result<Self> {
        let bare_repo_path = Self::repo_path();
        let clone_path = Self::clone_path();

        let mut test_repo = TestRepos {
            bare_repo_path,
            clone_path,
        };

        test_repo.create_repository()?;
        test_repo.clone_from_bare_repo(shallow_clone)?;

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
        let _old = config.set_raw_value(&"user.name", "Vergen Test")?;
        let _old = config.set_raw_value(&"user.email", "vergen@blah.com")?;

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

            // Create an annotated tag against the first commit
            let _tag_id = committer.tag(
                "0.1.0",
                first_commit_id,
                git::objs::Kind::Commit,
                None,
                "v0.1.0",
                PreviousValue::MustNotExist,
            )?;

            // Create a second commit
            let mut second_tree = Tree::empty();
            let second_commit_id = Self::create_commit(
                &mut second_tree,
                &committer,
                b"Hello, World!",
                "foo.txt",
                "such bad casing",
                first_commit_id.into(),
            )?;

            // Create a lightweight tag against the second commit
            let _tag_id = committer.tag_reference(
                "0.2.0-rc1",
                second_commit_id,
                PreviousValue::MustNotExist,
            )?;

            // Create a third commit
            let mut third_tree = Tree::empty();
            let _third_commit_id = Self::create_commit(
                &mut third_tree,
                &committer,
                b"this is my third commit",
                "foo.txt",
                "third commit",
                second_commit_id.into(),
            )?;
        }

        Ok(())
    }

    fn clone_from_bare_repo(&mut self, shallow_clone: bool) -> Result<()> {
        let bare_repo_path = &self.bare_repo_path;
        let clone_path = &self.clone_path;

        // Always make sure to clone a fresh directory in CI
        if let Ok(_ci) = env::var("CI") {
            let _res = fs::remove_dir_all(clone_path);
        }

        // Setup the directory
        fs::create_dir_all(clone_path)?;

        // Clone into the directory
        let url = parse(os_str_into_bstr(bare_repo_path.as_os_str())?)?;
        let opts = open::Options::isolated()
            .config_overrides(["user.name=Vergen Test", "user.email=vergen@blah.com"]);
        let mut prep = PrepareFetch::new(
            url,
            clone_path,
            Kind::WithWorktree,
            Options::default(),
            opts,
        )?;
        if shallow_clone && let Some(one) = NonZeroU32::new(1) {
            prep = prep.with_shallow(Shallow::DepthAtRemote(one));
        }
        let (mut prepare_checkout, _) = prep.fetch_then_checkout(Discard, &IS_INTERRUPTED)?;
        let (_repo, _) = prepare_checkout.main_worktree(Discard, &IS_INTERRUPTED)?;

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
    ///
    /// # Example
    /// ```
    /// # use anyhow::Result;
    /// # use test_util::TestRepos;
    /// #
    /// # pub fn main() -> Result<()> {
    /// let repo = TestRepos::new(false, false, false)?;
    /// assert!(repo.path().exists());
    /// #     Ok(())
    /// # }
    /// ```
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
        rng()
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

    /// Set mtime on .git index file
    ///
    /// # Errors
    /// * File open and modifiy errors
    ///
    pub fn set_index_magic_mtime(&self) -> Result<()> {
        let index_path = self.path().join(".git").join("index");
        File::open(&index_path)?.set_times(FileTimes::new().set_modified(*TEST_MTIME))?;
        Ok(())
    }

    /// Get mtime on the .git index file
    ///
    /// # Errors
    /// * File open and modifiy errors
    ///
    pub fn get_index_magic_mtime(&self) -> Result<SystemTime> {
        let index_path = self.path().join(".git").join("index");
        Ok(File::open(&index_path)?.metadata()?.modified()?)
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
    use super::{RUNNER_TEMP_ENV, TestRepos};
    use anyhow::Result;
    use serial_test::serial;

    #[test]
    #[serial]
    fn temp_dir_works() -> Result<()> {
        temp_env::with_var(RUNNER_TEMP_ENV, None::<String>, || {
            let repo = || -> Result<TestRepos> {
                let repo = TestRepos::new(false, false, false)?;
                Ok(repo)
            }();
            assert!(repo.is_ok());
        });
        Ok(())
    }
}
