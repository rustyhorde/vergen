// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use super::determine_maxes;
#[cfg(feature = "color")]
use super::{BOLD_BLUE, BOLD_GREEN};
use anyhow::Result;
use std::io::Write;

/// Output the `vergen` environment variables that are set in table format
///
/// # Errors
/// * The [`writeln!`](std::writeln!) macro can throw a [`std::io::Error`]
///
pub fn display<T>(writer: &mut T) -> Result<()>
where
    T: Write,
{
    let (vm_iter, max_prefix, max_kind) = determine_maxes();
    for (prefix, kind, value) in vm_iter {
        let key = format!("{prefix:>max_prefix$} ({kind:>max_kind$})");
        inner_write(writer, key, value)?;
    }
    Ok(())
}

#[cfg(feature = "color")]
fn inner_write<T>(writer: &mut T, key: String, value: &str) -> Result<()>
where
    T: Write,
{
    let blue_key = (*BOLD_BLUE).apply_to(key);
    let green_val = (*BOLD_GREEN).apply_to(value);
    Ok(writeln!(writer, "{blue_key}: {green_val}")?)
}

#[cfg(not(feature = "color"))]
fn inner_write<T>(writer: &mut T, key: String, value: &str) -> Result<()>
where
    T: Write,
{
    Ok(writeln!(writer, "{key}: {value}")?)
}

#[cfg(test)]
mod tests {
    use super::display;
    use anyhow::Result;

    #[test]
    fn display_works() -> Result<()> {
        let mut stdout = vec![];
        display(&mut stdout)?;
        assert!(!stdout.is_empty());
        println!("{}", String::from_utf8_lossy(&stdout));
        Ok(())
    }
}
