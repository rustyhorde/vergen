// Copyright (c) 2022 pud developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

// Header

use crate::{vergen_pretty_env, PrefixBuilder, PrettyBuilder};

use anyhow::Result;
use console::Style;
use rand::Rng;
use std::io::Write;

fn from_u8(val: u8) -> Style {
    let style = Style::new();
    match val {
        0 => style.green(),
        1 => style.yellow(),
        2 => style.blue(),
        3 => style.magenta(),
        4 => style.cyan(),
        5 => style.white(),
        _ => style.red(),
    }
}

/// Generate a pretty header
///
/// # Errors
///
pub fn header<T>(prefix: &'static str, writer: Option<&mut T>) -> Result<()>
where
    T: Write + ?Sized,
{
    let mut rng = rand::thread_rng();
    let app_style = from_u8(rng.gen_range(0..7));
    if let Some(writer) = writer {
        output_to_writer(writer, app_style.clone(), prefix)?;
    }
    trace(app_style, prefix)?;
    Ok(())
}

fn output_to_writer<T>(writer: &mut T, app_style: Style, prefix: &'static str) -> Result<()>
where
    T: Write + ?Sized,
{
    let prefix = PrefixBuilder::default()
        .lines(prefix.lines().map(str::to_string).collect())
        .style(app_style)
        .build()?;
    PrettyBuilder::default()
        .env(vergen_pretty_env!())
        .prefix(prefix)
        .build()?
        .display(writer)?;
    Ok(())
}

fn trace(app_style: Style, prefix: &'static str) -> Result<()> {
    let prefix = PrefixBuilder::default()
        .lines(prefix.lines().map(str::to_string).collect())
        .style(app_style)
        .build()?;
    PrettyBuilder::default()
        .env(vergen_pretty_env!())
        .prefix(prefix)
        .build()?
        .trace();
    Ok(())
}

#[cfg(test)]
mod test {
    use super::{from_u8, header};
    use console::Style;
    use lazy_static::lazy_static;
    use regex::Regex;

    const HEADER_PREFIX: &str = r#"██████╗ ██╗   ██╗██████╗ ██╗    ██╗
██╔══██╗██║   ██║██╔══██╗██║    ██║
██████╔╝██║   ██║██║  ██║██║ █╗ ██║
██╔═══╝ ██║   ██║██║  ██║██║███╗██║
██║     ╚██████╔╝██████╔╝╚███╔███╔╝
╚═╝      ╚═════╝ ╚═════╝  ╚══╝╚══╝ 

4a61736f6e204f7a696173
"#;

    lazy_static! {
        static ref BUILD_TIMESTAMP: Regex = Regex::new(r#"Timestamp \(  build\)"#).unwrap();
        static ref BUILD_SEMVER: Regex = Regex::new(r#"Semver \(  rustc\)"#).unwrap();
        static ref GIT_BRANCH: Regex = Regex::new(r#"Branch \(    git\)"#).unwrap();
    }

    #[test]
    fn from_u8_works() {
        assert_eq!(from_u8(0), Style::new().green());
        assert_eq!(from_u8(1), Style::new().yellow());
        assert_eq!(from_u8(2), Style::new().blue());
        assert_eq!(from_u8(3), Style::new().magenta());
        assert_eq!(from_u8(4), Style::new().cyan());
        assert_eq!(from_u8(5), Style::new().white());
        assert_eq!(from_u8(6), Style::new().red());
        assert_eq!(from_u8(7), Style::new().red());
    }

    #[test]
    #[cfg(debug_assertions)]
    fn header_writes() {
        let mut buf = vec![];
        assert!(header(HEADER_PREFIX, Some(&mut buf)).is_ok());
        assert!(!buf.is_empty());
        let header_str = String::from_utf8_lossy(&buf);
        println!("{header_str}");
        assert!(BUILD_TIMESTAMP.is_match(&header_str));
        assert!(BUILD_SEMVER.is_match(&header_str));
        assert!(GIT_BRANCH.is_match(&header_str));
    }

    #[test]
    #[cfg(not(debug_assertions))]
    fn header_writes() {
        let mut buf = vec![];
        assert!(header(&TestConfig::default(), HEADER_PREFIX, Some(&mut buf)).is_ok());
        assert!(!buf.is_empty());
        let header_str = String::from_utf8_lossy(&buf);
        assert!(BUILD_TIMESTAMP.is_match(&header_str));
        assert!(BUILD_SEMVER.is_match(&header_str));
        assert!(GIT_BRANCH.is_match(&header_str));
    }
}
