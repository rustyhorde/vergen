// Copyright (c) 2016, 2018, 2021 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `vergen` cargo flag generation

use crate::{
    config::{Config, VergenKey},
    constants::ConstantsFlags,
    error::Result,
};
use std::{
    io::{self, Write},
    path::Path,
};

/// Some Docs
///
/// # Errors
///
#[cfg(not(feature = "git"))]
pub fn gen(flags: ConstantsFlags) -> Result<()> {
    // This is here to help with type inference
    let no_repo: Option<&'static str> = None;
    gen_cargo_instructions(flags, no_repo, &mut io::stdout(), &mut io::stderr())
}

/// Some Docs
///
/// # Errors
///
#[cfg(feature = "git")]
pub fn gen(flags: ConstantsFlags) -> Result<()> {
    gen_cargo_instructions(flags, Some("."), &mut io::stdout(), &mut io::stderr())
}

fn gen_cargo_instructions<T, U, V>(
    flags: ConstantsFlags,
    repo: Option<V>,
    stdout: &mut T,
    _stderr: &mut U,
) -> Result<()>
where
    T: Write,
    U: Write,
    V: AsRef<Path>,
{
    // Generate the config to drive 'cargo:' instruction output
    let config = Config::build(flags, repo)?;

    // Generate the 'cargo:' instruction output
    for (k, v) in config.cfg_map().iter().filter_map(some_vals) {
        writeln!(stdout, "cargo:rustc-env={}={}", k.name(), v)?;
    }

    if flags.contains(ConstantsFlags::REBUILD_ON_HEAD_CHANGE) {
        // Add the HEAD path to cargo:rerun-if-changed
        if let Some(head_path) = config.head_path() {
            writeln!(stdout, "cargo:rerun-if-changed={}", head_path.display())?;
        }

        // Add the resolved ref path to cargo:rerun-if-changed
        if let Some(ref_path) = config.ref_path() {
            writeln!(stdout, "cargo:rerun-if-changed={}", ref_path.display())?;
        }
    }

    Ok(())
}

fn some_vals<'a>(tuple: (&'a VergenKey, &'a Option<String>)) -> Option<(&VergenKey, &String)> {
    if tuple.1.is_some() {
        Some((tuple.0, tuple.1.as_ref().unwrap()))
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::{gen, gen_cargo_instructions};
    use crate::{constants::ConstantsFlags, error::Result};
    use std::{io, path::PathBuf};

    #[test]
    fn gen_works() -> Result<()> {
        assert!(gen(ConstantsFlags::all()).is_ok());
        Ok(())
    }

    #[test]
    fn describe_falls_back() -> Result<()> {
        let no_tags_path = PathBuf::from("testdata").join("notagsrepo");
        assert!(gen_cargo_instructions(
            ConstantsFlags::all(),
            Some(no_tags_path),
            &mut io::stdout(),
            &mut io::stderr(),
        )
        .is_ok());
        Ok(())
    }

    #[test]
    fn describe() -> Result<()> {
        let no_tags_path = PathBuf::from("testdata").join("tagsrepo");
        assert!(gen_cargo_instructions(
            ConstantsFlags::all(),
            Some(no_tags_path),
            &mut io::stdout(),
            &mut io::stderr(),
        )
        .is_ok());
        Ok(())
    }
}
