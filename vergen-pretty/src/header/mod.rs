// Copyright (c) 2022 pud developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

// Header

use crate::{Prefix, Pretty, Suffix};

use anyhow::Result;
use bon::Builder;
use console::Style;
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

/// Convenience configuration around [`crate::Pretty`] to ease output generation.
///
/// # Example
/// ```
/// # use anyhow::Result;
/// # use vergen_pretty::{Config, header, vergen_pretty_env};
#[cfg_attr(feature = "color", doc = r"use vergen_pretty::Style;")]
/// #
/// # pub fn main() -> Result<()> {
/// let mut buf = vec![];
/// let config = Config::builder()
#[cfg_attr(feature = "color", doc = r"    .style(Style::new().green())")]
///     .prefix("HEADER_PREFIX")
///     .env(vergen_pretty_env!())
///     .suffix("HEADER_SUFFIX")
///     .build();
/// assert!(header(&config, Some(&mut buf)).is_ok());
/// assert!(!buf.is_empty());
/// #     Ok(())
/// # }
/// ```
///
#[derive(Builder, Clone, Debug, Default, PartialEq)]
pub struct Config {
    #[cfg(feature = "color")]
    #[builder(default = false)]
    /// Use a random [`Style`] color for the output
    random_style: bool,
    #[cfg(feature = "color")]
    /// Use the given [`Style`] for the output (mutually exclusive with `random_style`)
    style: Option<Style>,
    /// An optional prefix string
    #[builder(into)]
    prefix: Option<&'static str>,
    /// The vergen env (generated with the [`vergen_pretty_env`](crate::vergen_pretty_env) macro)
    env: Env,
    /// An optional suffix string
    #[builder(into)]
    suffix: Option<&'static str>,
}

/// Generate a pretty header based off your emitted `vergen` variables.
///
/// # Example
/// ```
/// # use anyhow::Result;
/// # use vergen_pretty::{Config, header, vergen_pretty_env};
/// #
/// # pub fn main() -> Result<()> {
/// let mut buf = vec![];
/// let config = Config::builder()
///     .prefix("HEADER_PREFIX")
///     .env(vergen_pretty_env!())
///     .suffix("HEADER_SUFFIX")
///     .build();
/// assert!(header(&config, Some(&mut buf)).is_ok());
/// assert!(!buf.is_empty());
/// #     Ok(())
/// # }
/// ```
///
/// # Errors
///
/// The errors are generally passed up from [`PrettyBuilder`]
///
pub fn header<T>(config: &Config, writer: Option<&mut T>) -> Result<()>
where
    T: Write + ?Sized,
{
    if let Some(writer) = writer {
        output_to_writer(writer, config)?;
    }
    trace(config);
    Ok(())
}

#[cfg(feature = "color")]
fn output_to_writer<T>(writer: &mut T, config: &Config) -> Result<()>
where
    T: Write + ?Sized,
{
    let app_style = get_style(config.random_style, config.style.clone());
    Pretty::builder()
        .env(config.env.clone())
        .prefix(get_prefix(config.prefix, &app_style))
        .suffix(get_suffix(config.suffix, &app_style))
        .build()
        .display(writer)?;
    Ok(())
}

#[cfg(not(feature = "color"))]
fn output_to_writer<T>(writer: &mut T, config: &Config) -> Result<()>
where
    T: Write + ?Sized,
{
    let app_style = get_style(false, None);
    Pretty::builder()
        .env(config.env.clone())
        .prefix(get_prefix(config.prefix, &app_style))
        .suffix(get_suffix(config.suffix, &app_style))
        .build()
        .display(writer)?;
    Ok(())
}

#[cfg(all(feature = "trace", feature = "color"))]
fn trace(config: &Config) {
    let app_style = get_style(config.random_style, config.style.clone());
    Pretty::builder()
        .env(config.env.clone())
        .prefix(get_prefix(config.prefix, &app_style))
        .suffix(get_suffix(config.suffix, &app_style))
        .build()
        .trace();
}

#[cfg(all(feature = "trace", not(feature = "color")))]
fn trace(config: &Config) {
    let app_style = get_style(false, None);
    Pretty::builder()
        .env(config.env.clone())
        .prefix(get_prefix(config.prefix, &app_style))
        .suffix(get_suffix(config.suffix, &app_style))
        .build()
        .trace();
}

#[cfg(not(feature = "trace"))]
fn trace(_config: &Config) {}

#[cfg(feature = "color")]
fn get_style(random_style: bool, style_opt: Option<Style>) -> Style {
    if random_style {
        let mut rng = rand::rng();
        from_u8(rng.random_range(0..7))
    } else if let Some(style) = style_opt {
        style
    } else {
        Style::new()
    }
}

#[cfg(not(feature = "color"))]
#[allow(clippy::needless_pass_by_value)]
fn get_style(_random_style: bool, _style_opt: Option<Style>) -> Style {
    Style::new()
}

#[cfg(feature = "color")]
fn get_prefix(prefix_opt: Option<&'static str>, app_style: &Style) -> Prefix {
    if let Some(prefix) = prefix_opt {
        Prefix::builder()
            .lines(prefix.lines().map(str::to_string).collect())
            .style(app_style.clone())
            .build()
    } else {
        Prefix::builder().lines(vec![]).build()
    }
}

#[cfg(not(feature = "color"))]
fn get_prefix(prefix_opt: Option<&'static str>, _app_style: &Style) -> Prefix {
    if let Some(prefix) = prefix_opt {
        Prefix::builder()
            .lines(prefix.lines().map(str::to_string).collect())
            .build()
    } else {
        Prefix::builder().lines(vec![]).build()
    }
}

#[cfg(feature = "color")]
fn get_suffix(suffix_opt: Option<&'static str>, app_style: &Style) -> Suffix {
    if let Some(suffix) = suffix_opt {
        Suffix::builder()
            .lines(suffix.lines().map(str::to_string).collect())
            .style(app_style.clone())
            .build()
    } else {
        Suffix::builder().lines(vec![]).build()
    }
}

#[cfg(not(feature = "color"))]
fn get_suffix(suffix_opt: Option<&'static str>, _app_style: &Style) -> Suffix {
    if let Some(suffix) = suffix_opt {
        Suffix::builder()
            .lines(suffix.lines().map(str::to_string).collect())
            .build()
    } else {
        Suffix::builder().lines(vec![]).build()
    }
}

#[cfg(test)]
mod test {
    use super::Config;
    #[cfg(feature = "color")]
    use super::from_u8;
    #[cfg(feature = "__vergen_test")]
    use super::header;
    use anyhow::Result;
    #[cfg(feature = "color")]
    use console::Style;
    #[cfg(feature = "__vergen_test")]
    use regex::Regex;
    use std::io::Write;
    #[cfg(feature = "__vergen_test")]
    use std::sync::LazyLock;

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

    #[cfg(feature = "__vergen_test")]
    static BUILD_TIMESTAMP: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"Timestamp \(  build\)").unwrap());
    #[cfg(feature = "__vergen_test")]
    static BUILD_SEMVER: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"Semver \(  rustc\)").unwrap());
    #[cfg(feature = "__vergen_test")]
    static GIT_BRANCH: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"Branch \(    git\)").unwrap());

    #[test]
    #[allow(clippy::clone_on_copy, clippy::redundant_clone)]
    fn header_clone_works() {
        let config = Config::default();
        let another = config.clone();
        assert_eq!(another, config);
    }

    #[test]
    fn builder_debug_works() -> Result<()> {
        let config = Config::default();
        let mut buf = vec![];
        write!(buf, "{config:?}")?;
        assert!(!buf.is_empty());
        Ok(())
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
    fn header_default() {
        use super::Config;
        use crate::vergen_pretty_env;

        let mut buf = vec![];
        let config = Config::builder().env(vergen_pretty_env!()).build();
        assert!(header(&config, Some(&mut buf)).is_ok());
        assert!(!buf.is_empty());
        let header_str = String::from_utf8_lossy(&buf);
        assert!(BUILD_TIMESTAMP.is_match(&header_str));
        assert!(BUILD_SEMVER.is_match(&header_str));
        assert!(GIT_BRANCH.is_match(&header_str));
    }

    #[test]
    #[cfg(feature = "__vergen_test")]
    fn header_no_writer() {
        use super::Config;
        use crate::vergen_pretty_env;

        let buf: Vec<u8> = vec![];
        let config = Config::builder().env(vergen_pretty_env!()).build();
        assert!(header(&config, None::<&mut Vec<u8>>).is_ok());
        assert!(buf.is_empty());
    }

    #[test]
    #[cfg(feature = "__vergen_test")]
    fn header_all() {
        use super::Config;
        use crate::vergen_pretty_env;

        let mut buf = vec![];
        let config = Config::builder()
            .prefix(HEADER_PREFIX)
            .env(vergen_pretty_env!())
            .suffix(HEADER_SUFFIX)
            .build();
        assert!(header(&config, Some(&mut buf)).is_ok());
        assert!(!buf.is_empty());
        let header_str = String::from_utf8_lossy(&buf);
        assert!(BUILD_TIMESTAMP.is_match(&header_str));
        assert!(BUILD_SEMVER.is_match(&header_str));
        assert!(GIT_BRANCH.is_match(&header_str));
    }

    #[test]
    #[cfg(all(feature = "__vergen_test", feature = "color"))]
    fn header_all_color_random() {
        use super::Config;
        use crate::vergen_pretty_env;

        let mut buf = vec![];
        let config = Config::builder()
            .random_style(true)
            .prefix(HEADER_PREFIX)
            .env(vergen_pretty_env!())
            .suffix(HEADER_SUFFIX)
            .build();
        assert!(header(&config, Some(&mut buf)).is_ok());
        assert!(!buf.is_empty());
        let header_str = String::from_utf8_lossy(&buf);
        assert!(BUILD_TIMESTAMP.is_match(&header_str));
        assert!(BUILD_SEMVER.is_match(&header_str));
        assert!(GIT_BRANCH.is_match(&header_str));
    }

    #[test]
    #[cfg(all(feature = "__vergen_test", feature = "color"))]
    fn header_all_color_specific() {
        use super::Config;
        use crate::vergen_pretty_env;

        let mut buf = vec![];
        let config = Config::builder()
            .style(Style::new().green())
            .prefix(HEADER_PREFIX)
            .env(vergen_pretty_env!())
            .suffix(HEADER_SUFFIX)
            .build();
        assert!(header(&config, Some(&mut buf)).is_ok());
        assert!(!buf.is_empty());
        let header_str = String::from_utf8_lossy(&buf);
        assert!(BUILD_TIMESTAMP.is_match(&header_str));
        assert!(BUILD_SEMVER.is_match(&header_str));
        assert!(GIT_BRANCH.is_match(&header_str));
    }

    #[test]
    #[cfg(debug_assertions)]
    #[cfg(feature = "__vergen_test")]
    fn header_writes() {
        use super::Config;
        use crate::vergen_pretty_env;

        let mut buf = vec![];
        let config = Config::builder()
            .prefix(HEADER_PREFIX)
            .env(vergen_pretty_env!())
            .build();
        assert!(header(&config, Some(&mut buf)).is_ok());
        assert!(!buf.is_empty());
        let header_str = String::from_utf8_lossy(&buf);
        assert!(BUILD_TIMESTAMP.is_match(&header_str));
        assert!(BUILD_SEMVER.is_match(&header_str));
        assert!(GIT_BRANCH.is_match(&header_str));
    }

    #[test]
    #[cfg(not(debug_assertions))]
    #[cfg(feature = "__vergen_test")]
    fn header_writes() {
        use super::Config;
        use crate::vergen_pretty_env;

        let mut buf = vec![];
        let config = Config::builder()
            .prefix(HEADER_PREFIX)
            .env(vergen_pretty_env!())
            .build();
        assert!(header(&config, Some(&mut buf)).is_ok());
        assert!(!buf.is_empty());
        let header_str = String::from_utf8_lossy(&buf);
        assert!(BUILD_TIMESTAMP.is_match(&header_str));
        assert!(BUILD_SEMVER.is_match(&header_str));
        assert!(GIT_BRANCH.is_match(&header_str));
    }
}
