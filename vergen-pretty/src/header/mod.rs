// Copyright (c) 2022 pud developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

// Header

use crate::{Prefix, PrefixBuilder, PrettyBuilder, Suffix, SuffixBuilder};

use anyhow::Result;
use console::Style;
use derive_builder::Builder;
#[cfg(feature = "color")]
use rand::Rng;
use std::{collections::BTreeMap, io::Write};

#[cfg(feature = "color")]
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

/// Environment tree type alias
pub type Env = BTreeMap<&'static str, Option<&'static str>>;

/// Header Configuration
#[derive(Builder, Clone, Debug, Default)]
pub struct Config {
    #[cfg(feature = "color")]
    #[builder(default)]
    /// Use a random [`Style`] color for the output
    random_style: bool,
    #[cfg(feature = "color")]
    #[builder(setter(into, strip_option), default)]
    /// Use the given [`Style`] for the output (mutually exclusive with `random_style`)
    style: Option<Style>,
    /// An optional prefix string
    #[builder(setter(into, strip_option), default)]
    prefix: Option<&'static str>,
    /// The vergen env (generated with the [`vergen_pretty_env`](crate::vergen_pretty_env) macro)
    env: Env,
    /// An optional suffix string
    #[builder(setter(into, strip_option), default)]
    suffix: Option<&'static str>,
}

/// Generate a pretty header
///
/// # Errors
///
pub fn header<T>(config: &Config, writer: Option<&mut T>) -> Result<()>
where
    T: Write + ?Sized,
{
    if let Some(writer) = writer {
        output_to_writer(writer, config)?;
    }
    trace(config)?;
    Ok(())
}

#[cfg(feature = "color")]
fn output_to_writer<T>(writer: &mut T, config: &Config) -> Result<()>
where
    T: Write + ?Sized,
{
    let app_style = get_style(config.random_style, config.style.clone());
    PrettyBuilder::default()
        .env(config.env.clone())
        .prefix(get_prefix(config.prefix, &app_style)?)
        .suffix(get_suffix(config.suffix, &app_style)?)
        .build()?
        .display(writer)?;
    Ok(())
}

#[cfg(not(feature = "color"))]
fn output_to_writer<T>(writer: &mut T, config: &Config) -> Result<()>
where
    T: Write + ?Sized,
{
    let app_style = get_style(false, None);
    PrettyBuilder::default()
        .env(config.env.clone())
        .prefix(get_prefix(config.prefix, &app_style)?)
        .suffix(get_suffix(config.suffix, &app_style)?)
        .build()?
        .display(writer)?;
    Ok(())
}

#[cfg(all(feature = "trace", feature = "color"))]
fn trace(config: &Config) -> Result<()> {
    let app_style = get_style(config.random_style, config.style.clone());
    PrettyBuilder::default()
        .env(config.env.clone())
        .prefix(get_prefix(config.prefix, &app_style)?)
        .suffix(get_suffix(config.suffix, &app_style)?)
        .build()?
        .trace();
    Ok(())
}

#[cfg(all(feature = "trace", not(feature = "color")))]
fn trace(config: &Config) -> Result<()> {
    let app_style = get_style(false, None);
    PrettyBuilder::default()
        .env(config.env.clone())
        .prefix(get_prefix(config.prefix, &app_style)?)
        .suffix(get_suffix(config.suffix, &app_style)?)
        .build()?
        .trace();
    Ok(())
}

#[cfg(not(feature = "trace"))]
fn trace(_config: &Config) -> Result<()> {
    Ok(())
}

#[cfg(feature = "color")]
fn get_style(random_style: bool, style_opt: Option<Style>) -> Style {
    if random_style {
        let mut rng = rand::thread_rng();
        from_u8(rng.gen_range(0..7))
    } else if let Some(style) = style_opt {
        style
    } else {
        Style::new()
    }
}

#[cfg(not(feature = "color"))]
fn get_style(_random_style: bool, _style_opt: Option<Style>) -> Style {
    Style::new()
}

#[cfg(feature = "color")]
fn get_prefix(prefix_opt: Option<&'static str>, app_style: &Style) -> Result<Prefix> {
    Ok(if let Some(prefix) = prefix_opt {
        PrefixBuilder::default()
            .lines(prefix.lines().map(str::to_string).collect())
            .style(app_style.clone())
            .build()?
    } else {
        PrefixBuilder::default().lines(vec![]).build()?
    })
}

#[cfg(not(feature = "color"))]
fn get_prefix(prefix_opt: Option<&'static str>, _app_style: &Style) -> Result<Prefix> {
    Ok(if let Some(prefix) = prefix_opt {
        PrefixBuilder::default()
            .lines(prefix.lines().map(str::to_string).collect())
            .build()?
    } else {
        PrefixBuilder::default().lines(vec![]).build()?
    })
}

#[cfg(feature = "color")]
fn get_suffix(suffix_opt: Option<&'static str>, app_style: &Style) -> Result<Suffix> {
    Ok(if let Some(suffix) = suffix_opt {
        SuffixBuilder::default()
            .lines(suffix.lines().map(str::to_string).collect())
            .style(app_style.clone())
            .build()?
    } else {
        SuffixBuilder::default().lines(vec![]).build()?
    })
}

#[cfg(not(feature = "color"))]
fn get_suffix(suffix_opt: Option<&'static str>, _app_style: &Style) -> Result<Suffix> {
    Ok(if let Some(suffix) = suffix_opt {
        SuffixBuilder::default()
            .lines(suffix.lines().map(str::to_string).collect())
            .build()?
    } else {
        SuffixBuilder::default().lines(vec![]).build()?
    })
}

#[cfg(test)]
mod test {
    #[cfg(feature = "color")]
    use super::from_u8;
    #[cfg(feature = "__vergen_test")]
    use super::header;
    #[cfg(feature = "__vergen_test")]
    use anyhow::Result;
    #[cfg(feature = "color")]
    use console::Style;
    use lazy_static::lazy_static;
    use regex::Regex;

    #[cfg(feature = "__vergen_test")]
    const HEADER_PREFIX: &str = r"██████╗ ██╗   ██╗██████╗ ██╗    ██╗
██╔══██╗██║   ██║██╔══██╗██║    ██║
██████╔╝██║   ██║██║  ██║██║ █╗ ██║
██╔═══╝ ██║   ██║██║  ██║██║███╗██║
██║     ╚██████╔╝██████╔╝╚███╔███╔╝
╚═╝      ╚═════╝ ╚═════╝  ╚══╝╚══╝ 

4a61736f6e204f7a696173
";

    #[cfg(feature = "__vergen_test")]
    const HEADER_SUFFIX: &str = r"
4a61736f6e204f7a696173
";

    lazy_static! {
        static ref BUILD_TIMESTAMP: Regex = Regex::new(r"Timestamp \(  build\)").unwrap();
        static ref BUILD_SEMVER: Regex = Regex::new(r"Semver \(  rustc\)").unwrap();
        static ref GIT_BRANCH: Regex = Regex::new(r"Branch \(    git\)").unwrap();
    }

    #[test]
    #[cfg(feature = "color")]
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
    #[cfg(feature = "__vergen_test")]
    fn header_default() -> Result<()> {
        use super::ConfigBuilder;
        use crate::vergen_pretty_env;

        let mut buf = vec![];
        let config = ConfigBuilder::default().env(vergen_pretty_env!()).build()?;
        assert!(header(&config, Some(&mut buf)).is_ok());
        assert!(!buf.is_empty());
        let header_str = String::from_utf8_lossy(&buf);
        assert!(BUILD_TIMESTAMP.is_match(&header_str));
        assert!(BUILD_SEMVER.is_match(&header_str));
        assert!(GIT_BRANCH.is_match(&header_str));
        Ok(())
    }

    #[test]
    #[cfg(feature = "__vergen_test")]
    fn header_all() -> Result<()> {
        use super::ConfigBuilder;
        use crate::vergen_pretty_env;

        let mut buf = vec![];
        let config = ConfigBuilder::default()
            .prefix(HEADER_PREFIX)
            .env(vergen_pretty_env!())
            .suffix(HEADER_SUFFIX)
            .build()?;
        assert!(header(&config, Some(&mut buf)).is_ok());
        assert!(!buf.is_empty());
        let header_str = String::from_utf8_lossy(&buf);
        assert!(BUILD_TIMESTAMP.is_match(&header_str));
        assert!(BUILD_SEMVER.is_match(&header_str));
        assert!(GIT_BRANCH.is_match(&header_str));
        Ok(())
    }

    #[test]
    #[cfg(all(feature = "__vergen_test", feature = "color"))]
    fn header_all_color_random() -> Result<()> {
        use super::ConfigBuilder;
        use crate::vergen_pretty_env;

        let mut buf = vec![];
        let config = ConfigBuilder::default()
            .random_style(true)
            .prefix(HEADER_PREFIX)
            .env(vergen_pretty_env!())
            .suffix(HEADER_SUFFIX)
            .build()?;
        assert!(header(&config, Some(&mut buf)).is_ok());
        assert!(!buf.is_empty());
        let header_str = String::from_utf8_lossy(&buf);
        assert!(BUILD_TIMESTAMP.is_match(&header_str));
        assert!(BUILD_SEMVER.is_match(&header_str));
        assert!(GIT_BRANCH.is_match(&header_str));
        Ok(())
    }

    #[test]
    #[cfg(all(feature = "__vergen_test", feature = "color"))]
    fn header_all_color_specific() -> Result<()> {
        use super::ConfigBuilder;
        use crate::vergen_pretty_env;

        let mut buf = vec![];
        let config = ConfigBuilder::default()
            .style(Style::new().green())
            .prefix(HEADER_PREFIX)
            .env(vergen_pretty_env!())
            .suffix(HEADER_SUFFIX)
            .build()?;
        assert!(header(&config, Some(&mut buf)).is_ok());
        assert!(!buf.is_empty());
        let header_str = String::from_utf8_lossy(&buf);
        assert!(BUILD_TIMESTAMP.is_match(&header_str));
        assert!(BUILD_SEMVER.is_match(&header_str));
        assert!(GIT_BRANCH.is_match(&header_str));
        Ok(())
    }

    #[test]
    #[cfg(debug_assertions)]
    #[cfg(feature = "__vergen_test")]
    fn header_writes() -> Result<()> {
        use super::ConfigBuilder;
        use crate::vergen_pretty_env;

        let mut buf = vec![];
        let config = ConfigBuilder::default()
            .prefix(HEADER_PREFIX)
            .env(vergen_pretty_env!())
            .build()?;
        assert!(header(&config, Some(&mut buf)).is_ok());
        assert!(!buf.is_empty());
        let header_str = String::from_utf8_lossy(&buf);
        assert!(BUILD_TIMESTAMP.is_match(&header_str));
        assert!(BUILD_SEMVER.is_match(&header_str));
        assert!(GIT_BRANCH.is_match(&header_str));
        Ok(())
    }

    #[test]
    #[cfg(not(debug_assertions))]
    #[cfg(feature = "__vergen_test")]
    fn header_writes() -> Result<()> {
        use super::ConfigBuilder;
        use crate::vergen_pretty_env;

        let mut buf = vec![];
        let config = ConfigBuilder::default()
            .prefix(HEADER_PREFIX)
            .env(vergen_pretty_env!())
            .build()?;
        assert!(header(&config, Some(&mut buf)).is_ok());
        assert!(!buf.is_empty());
        let header_str = String::from_utf8_lossy(&buf);
        assert!(BUILD_TIMESTAMP.is_match(&header_str));
        assert!(BUILD_SEMVER.is_match(&header_str));
        assert!(GIT_BRANCH.is_match(&header_str));
        Ok(())
    }
}
