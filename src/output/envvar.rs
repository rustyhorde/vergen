// Copyright (c) 2016, 2018 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Build time information.
use crate::constants::ConstantsFlags;
use crate::output::generate_build_info;
use failure::Fallible;
use std::fs::{self, File};
use std::io::Read;
use std::path::PathBuf;

/// Generate the `cargo:` key output
///
/// The keys that can be generated include:
/// * `cargo:rustc-env=<key>=<value>` where key/value pairs are controlled by the supplied `ConstantsFlags`.
/// * `cargo:rustc-rerun-if-changed=.git/HEAD`
/// * `cargo:rustc-rerun-if-changed=<file .git/HEAD points to>`
///
/// # Example `build.rs`
///
/// ```
/// extern crate vergen;
///
/// use vergen::{ConstantsFlags, generate_cargo_keys};
///
/// fn main() {
///     generate_cargo_keys(ConstantsFlags::all()).expect("Unable to generate cargo keys!");
/// }
/// ```
pub fn generate_cargo_keys(flags: ConstantsFlags) -> Fallible<()> {
    // Generate the build info map.
    let build_info = generate_build_info(flags)?;

    // Generate the 'cargo:' key output
    for (k, v) in build_info {
        println!("cargo:rustc-env={}={}", k.name(), v);
    }

    let git_dir_or_file = PathBuf::from(".git");
    let metadata = fs::metadata(&git_dir_or_file)?;

    let mut f = if metadata.is_dir() {
        let git_head_path = git_dir_or_file.join("HEAD");
        println!("cargo:rerun-if-changed={}", git_head_path.display());
        File::open(&git_head_path)?
    } else if metadata.is_file() {
        let mut git_file = File::open(&git_dir_or_file)?;
        let mut git_contents = String::new();
        let _ = git_file.read_to_string(&mut git_contents)?;
        let dir_vec: Vec<&str> = git_contents.split(": ").collect();
        let git_head_path = PathBuf::from(dir_vec[1].trim()).join("HEAD");
        println!("cargo:rerun-if-changed={}", git_head_path.display());
        File::open(&git_head_path)?
    } else {
        return Err(failure::err_msg("Invalid .git format"));
    };

    let mut git_head_contents = String::new();
    let _ = f.read_to_string(&mut git_head_contents)?;
    let ref_vec: Vec<&str> = git_head_contents.split(": ").collect();

    if ref_vec.len() == 2 {
        let current_head_file = ref_vec[1];
        let git_refs_path = PathBuf::from(".git").join(current_head_file);
        println!("cargo:rerun-if-changed={}", git_refs_path.display());
    }

    Ok(())
}
