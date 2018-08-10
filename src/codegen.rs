// Copyright (c) 2016, 2018 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! The build information implementation related to generating code to use with
//! the `include!` macro within your project.
use constants::{ConstantsFlags, CONST_PREFIX, CONST_TYPE};
use envvar::Vergen;
use error::Result;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn gen_const<W: Write>(f: &mut W, comment: &str, name: &str, value: &str) -> Result<()> {
    writeln!(
        f,
        "{}\n{}{}{}\"{}\";",
        comment, CONST_PREFIX, name, CONST_TYPE, value
    )?;
    Ok(())
}

/// Create a `version.rs` file in `OUT_DIR` and write the toggled on constants
/// to the file.
///
/// # Example build.rs
/// ```
/// # extern crate vergen;
/// #
/// # use std::env;
/// # use vergen::{ConstantsFlags, Result, vergen};
/// #
/// fn main() {
/// #   env::set_var("OUT_DIR", "target");
///     let mut flags = ConstantsFlags::all();
///     flags.toggle(ConstantsFlags::BUILD_TIMESTAMP);
///     vergen(flags).expect("Unable to generate constants!");
/// }
/// ```
///
/// # Example Output (All Flags Enabled)
/// ```
/// /// Build Timestamp (UTC)
/// pub const VERGEN_BUILD_TIMESTAMP: &str = "2018-08-09T15:15:57.282334589+00:00";
///
/// /// Build Date - Short (UTC)
/// pub const VERGEN_BUILD_DATE: &str = "2018-08-09";
///
/// /// Commit SHA
/// pub const VERGEN_SHA: &str = "75b390dc6c05a6a4aa2791cc7b3934591803bc22";
///
/// /// Commit SHA - Short
/// pub const VERGEN_SHA_SHORT: &str = "75b390d";
///
/// /// Commit Date
/// pub const VERGEN_COMMIT_DATE: &str = "'2018-08-08'";
///
/// /// Target Triple
/// pub const VERGEN_TARGET_TRIPLE: &str = "x86_64-unknown-linux-gnu";
///
/// /// Semver
/// pub const VERGEN_SEMVER: &str = "v0.1.0-pre.0";
///
/// /// Semver (Lightweight)
/// pub const VERGEN_SEMVER_LIGHTWEIGHT: &str = "v0.1.0-pre.0";
/// ```
pub fn vergen(flags: ConstantsFlags) -> Result<()> {
    let dst = PathBuf::from(env::var("OUT_DIR")?);
    let mut f = File::create(&dst.join("version.rs"))?;
    let vergen = Vergen::new(flags)?;

    for (k, v) in vergen.build_info() {
        gen_const(&mut f, k.comment(), k.name(), v)?;
        writeln!(f)?;
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::gen_const;
    use constants::ConstantsFlags;
    use envvar::Vergen;
    use regex::Regex;
    use std::io::Cursor;

    lazy_static! {
        static ref CONST_RE: Regex =
            Regex::new(r#"^/// .*[\r\n]+pub const [A-Z_]+: \&str = ".*";[\r\n]+$"#)
                .expect("Unable to create const regex");
    }

    #[test]
    fn gen_const_output() {
        let flags = ConstantsFlags::all();
        let vergen = Vergen::new(flags).expect("Unable to generate build information map");

        for (k, v) in vergen.build_info() {
            let buffer = Vec::new();
            let mut cursor = Cursor::new(buffer);
            gen_const(&mut cursor, k.comment(), k.name(), v)
                .expect("Unable to generate const string");
            let const_str = String::from_utf8_lossy(&cursor.get_ref());
            assert!(CONST_RE.is_match(&const_str));
        }
    }
}
