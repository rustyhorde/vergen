// Copyright (c) 2016, 2018, 2021 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `vergen` cargo flag generation

use crate::{config::Config, constants::ConstantsFlags, error::Result, output::VergenKey};
#[cfg(feature = "git")]
use git2::Repository;
use std::io::{self, Write};

/// Some Docs
///
/// # Errors
///
#[cfg(feature = "git")]
pub fn gen(flags: ConstantsFlags) -> Result<()> {
    gen_cargo_instructions(
        flags,
        &Repository::discover(".")?,
        &mut io::stdout(),
        &mut io::stderr(),
    )
}

/// Some Docs
///
/// # Errors
///
#[cfg(not(feature = "git"))]
pub fn gen(flags: ConstantsFlags) -> Result<()> {
    gen_cargo_instructions(flags, &mut io::stdout(), &mut io::stderr())
}

#[cfg(feature = "git")]
fn gen_cargo_instructions<T, U>(
    flags: ConstantsFlags,
    repo: &Repository,
    stdout: &mut T,
    _stderr: &mut U,
) -> Result<()>
where
    T: Write,
    U: Write,
{
    // Generate the config to drive 'cargo:' instruction output
    let config = Config::build(flags, repo)?;

    // Generate the 'cargo:' instruction output
    for (k, v) in config.cfg_map().iter().filter_map(some_vals) {
        writeln!(stdout, "cargo:rustc-env={}={}", k.name(), v)?;
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

#[cfg(not(feature = "git"))]
fn gen_cargo_instructions<T, U>(
    flags: ConstantsFlags,
    _stdout: &mut T,
    _stderr: &mut U,
) -> Result<()>
where
    T: Write,
    U: Write,
{
    let _config = Config::build(flags)?;

    Ok(())
}

#[cfg(all(test, feature = "git"))]
mod test {
    use super::{gen, gen_cargo_instructions};
    use crate::{constants::ConstantsFlags, error::Result};
    use git2::Repository;

    #[test]
    fn gen_works() -> Result<()> {
        assert!(gen(ConstantsFlags::all()).is_ok());
        Ok(())
    }

    #[test]
    fn describe_falls_back() -> Result<()> {
        use std::io;
        let repo = Repository::open("testdata/notagsrepo")?;
        assert!(gen_cargo_instructions(
            ConstantsFlags::all(),
            &repo,
            &mut io::stdout(),
            &mut io::stderr(),
        )
        .is_ok());
        Ok(())
    }
}

#[cfg(all(test, not(feature = "git")))]
mod test {
    use super::gen;
    use crate::{constants::ConstantsFlags, error::Result};

    #[test]
    fn gen_works() -> Result<()> {
        assert!(gen(ConstantsFlags::all()).is_ok());
        Ok(())
    }
}
