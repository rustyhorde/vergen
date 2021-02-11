// Copyright (c) 2016, 2018, 2021 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Build time information.
use crate::constants::ConstantsFlags;
use crate::output::generate_build_info;
use std::{
    fs::{self, File},
    io::Write,
};
use std::{
    io::{self, Read},
    path::Path,
};
use std::{path::PathBuf, process::Command};

use super::Result;

/// Generate the `cargo:` key output
///
/// The keys that can be generated include:
/// * `cargo:rustc-env=<key>=<value>` where key/value pairs are controlled by the supplied `ConstantsFlags`.
/// * `cargo:rustc-rerun-if-changed=.git/HEAD`
/// * `cargo:rustc-rerun-if-changed=<file .git/HEAD points to>`
///
/// # Errors
/// * rustc may throw errors
/// * git commands may throw errors
///
/// # Example `build.rs`
///
/// ```
/// # use vergen::{ConstantsFlags, generate_cargo_keys};
/// #
/// # fn main() {
///     // Setup the flags, toggling off the 'SEMVER_FROM_CARGO_PKG' flag
///     let mut flags = ConstantsFlags::all();
///     flags.toggle(ConstantsFlags::SEMVER_FROM_CARGO_PKG);
///
///     // Generate the 'cargo:' key output
///     generate_cargo_keys(flags).expect("Unable to generate the cargo keys!");
/// # }
/// ```
pub fn generate_cargo_keys(flags: ConstantsFlags) -> Result<()> {
    let base = super::run_command(Command::new("git").args(&["rev-parse", "--show-toplevel"]));
    let mut git_dir_or_file = PathBuf::from(base);
    git_dir_or_file.push(".git");
    gen_cargo_keys(flags, git_dir_or_file, &mut io::stdout(), &mut io::stderr())
}

fn gen_cargo_keys<P, T, E>(
    flags: ConstantsFlags,
    git_path: P,
    stdout: &mut T,
    stderr: &mut E,
) -> Result<()>
where
    P: AsRef<Path>,
    T: Write,
    E: Write,
{
    // Generate the build info map.
    let build_info = generate_build_info(flags)?;

    // Generate the 'cargo:' key output
    for (k, v) in build_info {
        writeln!(stdout, "cargo:rustc-env={}={}", k.name(), v)?;
    }

    if let Ok(metadata) = fs::symlink_metadata(&git_path) {
        if metadata.is_dir() {
            // Echo the HEAD path
            let gp = git_path.as_ref().to_path_buf();
            let git_head_path = gp.join("HEAD");
            writeln!(stdout, "cargo:rerun-if-changed={}", git_head_path.display())?;

            // Determine where HEAD points and echo that path also.
            let mut f = File::open(&git_head_path)?;
            let mut git_head_contents = String::new();
            let _ = f.read_to_string(&mut git_head_contents)?;
            writeln!(stderr, "HEAD contents: {}", git_head_contents)?;
            let ref_vec: Vec<&str> = git_head_contents.split(": ").collect();

            if ref_vec.len() == 2 {
                let current_head_file = ref_vec[1].trim();
                let git_refs_path = gp.join(current_head_file);
                writeln!(stdout, "cargo:rerun-if-changed={}", git_refs_path.display())?;
            } else {
                writeln!(stderr, "You are most likely in a detached HEAD state")?;
            }
        } else if metadata.is_file() {
            // We are in a worktree, so find out where the actual worktrees/<name>/HEAD file is.
            let mut git_file = File::open(&git_path)?;
            let mut git_contents = String::new();
            let _ = git_file.read_to_string(&mut git_contents)?;
            let dir_vec: Vec<&str> = git_contents.split(": ").collect();
            writeln!(stderr, ".git contents: {}", git_contents)?;
            let wt_git_path = dir_vec[1].trim();

            // Echo the HEAD path
            let git_head_path = PathBuf::from(wt_git_path).join("HEAD");
            writeln!(stdout, "cargo:rerun-if-changed={}", git_head_path.display())?;

            // Find out what the full path to the .git dir is.
            let mut actual_git_dir = PathBuf::from(wt_git_path);
            let _ = actual_git_dir.pop();
            let _ = actual_git_dir.pop();

            // Determine where HEAD points and echo that path also.
            let mut f = File::open(&git_head_path)?;
            let mut git_head_contents = String::new();
            let _ = f.read_to_string(&mut git_head_contents)?;
            writeln!(stderr, "HEAD contents: {}", git_head_contents)?;
            let ref_vec: Vec<&str> = git_head_contents.split(": ").collect();

            if ref_vec.len() == 2 {
                let current_head_file = ref_vec[1].trim();
                let git_refs_path = actual_git_dir.join(current_head_file);
                writeln!(stdout, "cargo:rerun-if-changed={}", git_refs_path.display())?;
            } else {
                writeln!(stderr, "You are most likely in a detached HEAD state")?;
            }
        } else {
            return Err("Invalid .git format (Not a directory or a file)".into());
        };
    } else {
        writeln!(stderr, "Unable to generate 'cargo:rerun-if-changed'")?;
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::{gen_cargo_keys, generate_cargo_keys};
    use crate::constants::ConstantsFlags;
    use std::path::PathBuf;

    #[test]
    fn pub_api() {
        assert!(generate_cargo_keys(ConstantsFlags::all()).is_ok());
    }

    #[test]
    fn gitdir() {
        let mut buf_stdout = Vec::new();
        let mut buf_stderr = Vec::new();

        let mut git_path = PathBuf::from("testdata");
        git_path.push("gitdir");

        assert!(gen_cargo_keys(
            ConstantsFlags::all(),
            git_path,
            &mut buf_stdout,
            &mut buf_stderr,
        )
        .is_ok());
        let stdout = String::from_utf8_lossy(&buf_stdout);
        #[cfg(target_family = "unix")]
        assert!(stdout.contains("cargo:rerun-if-changed=testdata/gitdir/HEAD"));
        #[cfg(target_family = "windows")]
        assert!(stdout.contains(r"cargo:rerun-if-changed=testdata\gitdir\HEAD"));
        #[cfg(target_family = "unix")]
        assert!(stdout.contains("cargo:rerun-if-changed=testdata/gitdir/kcov"));
        #[cfg(target_family = "windows")]
        assert!(stdout.contains(r"cargo:rerun-if-changed=testdata\gitdir\kcov"));
    }

    #[test]
    fn detached_gitdir() {
        let mut buf_stdout = Vec::new();
        let mut buf_stderr = Vec::new();

        let mut git_path = PathBuf::from("testdata");
        git_path.push("gitdir2");

        assert!(gen_cargo_keys(
            ConstantsFlags::all(),
            git_path,
            &mut buf_stdout,
            &mut buf_stderr,
        )
        .is_ok());
        let stdout = String::from_utf8_lossy(&buf_stdout);
        let stderr = String::from_utf8_lossy(&buf_stderr);
        #[cfg(target_family = "unix")]
        assert!(stdout.contains("cargo:rerun-if-changed=testdata/gitdir2/HEAD"));
        #[cfg(target_family = "windows")]
        assert!(stdout.contains(r"cargo:rerun-if-changed=testdata\gitdir2\HEAD"));
        assert!(stderr.contains("You are most likely in a detached HEAD state"));
    }

    #[test]
    fn worktree() {
        let mut buf_stdout = Vec::new();
        let mut buf_stderr = Vec::new();

        let mut git_path = PathBuf::from("testdata");
        git_path.push("blahgit");

        assert!(gen_cargo_keys(
            ConstantsFlags::all(),
            git_path,
            &mut buf_stdout,
            &mut buf_stderr,
        )
        .is_ok());

        let stdout = String::from_utf8_lossy(&buf_stdout);
        #[cfg(target_family = "unix")]
        assert!(stdout.contains("cargo:rerun-if-changed=testdata/blah/worktrees/vergen-1/HEAD"));
        #[cfg(target_family = "windows")]
        assert!(stdout.contains(r"cargo:rerun-if-changed=testdata/blah/worktrees/vergen-1\HEAD"));
        #[cfg(target_family = "unix")]
        assert!(stdout.contains("cargo:rerun-if-changed=testdata/blah/refs/heads/vergen-1"));
        #[cfg(target_family = "windows")]
        assert!(stdout.contains(r"cargo:rerun-if-changed=testdata/blah\refs/heads/vergen-1"));
    }

    #[test]
    fn detached_worktree() {
        let mut buf_stdout = Vec::new();
        let mut buf_stderr = Vec::new();

        let mut git_path = PathBuf::from("testdata");
        git_path.push("blahgit2");

        assert!(gen_cargo_keys(
            ConstantsFlags::all(),
            git_path,
            &mut buf_stdout,
            &mut buf_stderr,
        )
        .is_ok());

        let stdout = String::from_utf8_lossy(&buf_stdout);
        let stderr = String::from_utf8_lossy(&buf_stderr);
        println!("STDOUT: {}", stdout);
        #[cfg(target_family = "unix")]
        assert!(stdout.contains("cargo:rerun-if-changed=testdata/blah2/worktrees/vergen-1/HEAD"));
        #[cfg(target_family = "windows")]
        assert!(stdout.contains(r"cargo:rerun-if-changed=testdata/blah2/worktrees/vergen-1\HEAD"));
        assert!(stderr.contains("You are most likely in a detached HEAD state"));
    }

    #[test]
    #[cfg(target_family = "unix")]
    fn error_on_symlink() {
        let mut buf_stdout = Vec::new();
        let mut buf_stderr = Vec::new();

        let mut git_path = PathBuf::from("testdata");
        git_path.push("badgit");

        assert!(gen_cargo_keys(
            ConstantsFlags::all(),
            git_path,
            &mut buf_stdout,
            &mut buf_stderr,
        )
        .is_err());
    }

    #[test]
    fn invalid_file() {
        let mut buf_stdout = Vec::new();
        let mut buf_stderr = Vec::new();

        assert!(gen_cargo_keys(
            ConstantsFlags::all(),
            "xxxxzzzyyy",
            &mut buf_stdout,
            &mut buf_stderr,
        )
        .is_ok());

        let stderr = String::from_utf8_lossy(&buf_stderr);
        assert!(stderr.contains("Unable to generate 'cargo:rerun-if-changed'"));
    }
}
