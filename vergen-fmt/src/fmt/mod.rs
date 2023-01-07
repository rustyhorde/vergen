// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use anyhow::Result;
use convert_case::{Case, Casing};
use std::{collections::BTreeMap, io::Write};
#[cfg(feature = "trace")]
use tracing::{event, Level};
use typed_builder::TypedBuilder;
#[cfg(feature = "color")]
use {crate::Style, lazy_static::lazy_static};

/// The `vergen-fmt` configuration
#[derive(Clone, Debug, TypedBuilder)]
pub struct Fmt {
    #[builder(setter(strip_option), default)]
    prefix: Option<Prefix>,
    env: BTreeMap<&'static str, Option<&'static str>>,
    #[builder(setter(skip), default)]
    vars: Vec<(String, String, String)>,
    #[cfg(feature = "color")]
    #[builder(setter(strip_option), default)]
    key_style: Option<Style>,
    #[cfg(feature = "color")]
    #[builder(setter(strip_option), default)]
    value_style: Option<Style>,
    #[builder(setter(skip), default)]
    max_label: usize,
    #[builder(setter(skip), default)]
    max_category: usize,
    #[builder(setter(strip_option), default)]
    suffix: Option<Suffix>,
    #[cfg(feature = "trace")]
    #[builder(default = Level::INFO)]
    level: Level,
}

impl Fmt {
    /// Output the `vergen` environment variables that are set in table format
    ///
    /// # Errors
    /// * The [`writeln!`](std::writeln!) macro can throw a [`std::io::Error`]
    ///
    pub fn display<T>(mut self, writer: &mut T) -> Result<()>
    where
        T: Write + ?Sized,
    {
        self.populate_fmt();

        if let Some(prefix) = &self.prefix {
            prefix.display(writer)?;
        }

        for (category, label, value) in &self.vars {
            let key = format!(
                "{label:>0$} ({category:>1$})",
                self.max_label, self.max_category
            );
            self.inner_display(writer, &key, value)?;
        }

        if let Some(suffix) = &self.suffix {
            suffix.display(writer)?;
        }

        Ok(())
    }

    fn populate_fmt(&mut self) {
        let vm_iter: Vec<(String, String, String)> = self
            .env
            .iter()
            .filter_map(has_value)
            .filter_map(split_key)
            .filter_map(split_kv)
            .collect();
        let max_label = vm_iter
            .iter()
            .map(|(_, label, _)| label.len())
            .max()
            .map_or_else(|| 16, |x| x);
        let max_category = vm_iter
            .iter()
            .map(|(category, _, _)| category.len())
            .max()
            .map_or_else(|| 7, |x| x);
        self.vars = vm_iter;
        self.max_label = max_label;
        self.max_category = max_category;
    }

    #[cfg(feature = "color")]
    fn inner_display<T>(&self, writer: &mut T, key: &str, value: &str) -> Result<()>
    where
        T: Write + ?Sized,
    {
        let key = if let Some(style) = &self.key_style {
            style
        } else {
            &*BOLD_BLUE
        }
        .apply_to(key);
        let value = if let Some(style) = &self.value_style {
            style
        } else {
            &*BOLD_GREEN
        }
        .apply_to(value);
        Ok(writeln!(writer, "{key}: {value}")?)
    }

    #[cfg(not(feature = "color"))]
    fn inner_display<T>(&self, writer: &mut T, key: &str, value: &str) -> Result<()>
    where
        T: Write + ?Sized,
    {
        Ok(writeln!(writer, "{key}: {value}")?)
    }

    /// Output the `vergen` environment variables that are set in table format to a tracing subscriber
    ///
    #[cfg(feature = "trace")]
    pub fn trace(&mut self) {
        self.populate_fmt();

        if let Some(prefix) = &self.prefix {
            prefix.trace();
        }

        for (category, label, value) in &self.vars {
            let key = format!(
                "{label:>0$} ({category:>1$})",
                self.max_label, self.max_category
            );
            self.inner_trace(&key, value);
        }

        if let Some(suffix) = &self.suffix {
            suffix.trace();
        }
    }

    #[cfg(all(feature = "trace", feature = "color"))]
    fn inner_trace(&self, key: &str, value: &str) {
        let key = if let Some(style) = &self.key_style {
            style
        } else {
            &*BOLD_BLUE
        }
        .apply_to(key);
        let value = if let Some(style) = &self.value_style {
            style
        } else {
            &*BOLD_GREEN
        }
        .apply_to(value);
        match self.level {
            Level::DEBUG => event!(Level::DEBUG, "{key}: {value}"),
            Level::ERROR => event!(Level::ERROR, "{key}: {value}"),
            Level::INFO => event!(Level::INFO, "{key}: {value}"),
            Level::TRACE => event!(Level::TRACE, "{key}: {value}"),
            Level::WARN => event!(Level::WARN, "{key}: {value}"),
        }
    }

    #[cfg(all(feature = "trace", not(feature = "color")))]
    fn inner_trace(&self, key: &str, value: &str) {
        match self.level {
            Level::DEBUG => event!(Level::DEBUG, "{key}: {value}"),
            Level::ERROR => event!(Level::ERROR, "{key}: {value}"),
            Level::INFO => event!(Level::INFO, "{key}: {value}"),
            Level::TRACE => event!(Level::TRACE, "{key}: {value}"),
            Level::WARN => event!(Level::WARN, "{key}: {value}"),
        }
    }
}

/// The prefix configuration
#[derive(Clone, Debug, TypedBuilder)]
pub struct Prefix {
    lines: Vec<String>,
    #[cfg(feature = "color")]
    #[builder(setter(strip_option, into), default)]
    style: Option<Style>,
    #[cfg(feature = "trace")]
    #[builder(default = Level::INFO)]
    level: Level,
}

impl Prefix {
    /// Output the `vergen` environment variables that are set in table format
    ///
    /// # Errors
    /// * The [`writeln!`](std::writeln!) macro can throw a [`std::io::Error`]
    ///
    fn display<T>(&self, writer: &mut T) -> Result<()>
    where
        T: Write + ?Sized,
    {
        self.inner_display(writer)?;
        writeln!(writer)?;
        Ok(())
    }

    #[cfg(feature = "color")]
    fn inner_display<T>(&self, writer: &mut T) -> Result<()>
    where
        T: Write + ?Sized,
    {
        for line in &self.lines {
            if let Some(style) = &self.style {
                writeln!(writer, "{}", style.apply_to(line))?;
            } else {
                writeln!(writer, "{line}")?;
            }
        }
        Ok(())
    }

    #[cfg(not(feature = "color"))]
    fn inner_display<T>(&self, writer: &mut T) -> Result<()>
    where
        T: Write + ?Sized,
    {
        for line in &self.lines {
            writeln!(writer, "{line}")?;
        }
        Ok(())
    }

    /// Output the `vergen` environment variables that are set in table format to a tracing subscriber
    ///
    #[cfg(feature = "trace")]
    fn trace(&self) {
        self.inner_trace();
    }

    #[cfg(all(feature = "trace", feature = "color"))]
    fn inner_trace(&self) {
        for line in &self.lines {
            let line = if let Some(style) = &self.style {
                format!("{}", style.apply_to(line))
            } else {
                line.clone()
            };
            match self.level {
                Level::DEBUG => event!(Level::DEBUG, "{line}"),
                Level::ERROR => event!(Level::ERROR, "{line}"),
                Level::INFO => event!(Level::INFO, "{line}"),
                Level::TRACE => event!(Level::TRACE, "{line}"),
                Level::WARN => event!(Level::WARN, "{line}"),
            }
        }
    }

    #[cfg(all(feature = "trace", not(feature = "color")))]
    fn inner_trace(&self) {
        for line in &self.lines {
            match self.level {
                Level::DEBUG => event!(Level::DEBUG, "{line}"),
                Level::ERROR => event!(Level::ERROR, "{line}"),
                Level::INFO => event!(Level::INFO, "{line}"),
                Level::TRACE => event!(Level::TRACE, "{line}"),
                Level::WARN => event!(Level::WARN, "{line}"),
            }
        }
    }
}

/// The suffix configuration
#[derive(Clone, Debug, TypedBuilder)]
pub struct Suffix {
    lines: Vec<String>,
    #[cfg(feature = "color")]
    #[builder(setter(strip_option), default)]
    style: Option<Style>,
    #[cfg(feature = "trace")]
    #[builder(default = Level::INFO)]
    level: Level,
}

impl Suffix {
    /// Output the `vergen` environment variables that are set in table format
    ///
    /// # Errors
    /// * The [`writeln!`](std::writeln!) macro can throw a [`std::io::Error`]
    ///
    fn display<T>(&self, writer: &mut T) -> Result<()>
    where
        T: Write + ?Sized,
    {
        self.inner_display(writer)
    }

    #[cfg(feature = "color")]
    fn inner_display<T>(&self, writer: &mut T) -> Result<()>
    where
        T: Write + ?Sized,
    {
        for line in &self.lines {
            if let Some(style) = &self.style {
                writeln!(writer, "{}", style.apply_to(line))?;
            } else {
                writeln!(writer, "{line}")?;
            }
        }
        Ok(())
    }

    #[cfg(not(feature = "color"))]
    fn inner_display<T>(&self, writer: &mut T) -> Result<()>
    where
        T: Write + ?Sized,
    {
        for line in &self.lines {
            writeln!(writer, "{line}")?;
        }
        Ok(())
    }

    /// Output the `vergen` environment variables that are set in table format to a tracing subscriber
    ///
    #[cfg(feature = "trace")]
    fn trace(&self) {
        self.inner_trace();
    }

    #[cfg(all(feature = "trace", feature = "color"))]
    fn inner_trace(&self) {
        for line in &self.lines {
            let line = if let Some(style) = &self.style {
                format!("{}", style.apply_to(line))
            } else {
                line.clone()
            };
            match self.level {
                Level::DEBUG => event!(Level::DEBUG, "{line}"),
                Level::ERROR => event!(Level::ERROR, "{line}"),
                Level::INFO => event!(Level::INFO, "{line}"),
                Level::TRACE => event!(Level::TRACE, "{line}"),
                Level::WARN => event!(Level::WARN, "{line}"),
            }
        }
    }

    #[cfg(all(feature = "trace", not(feature = "color")))]
    fn inner_trace(&self) {
        for line in &self.lines {
            match self.level {
                Level::DEBUG => event!(Level::DEBUG, "{line}"),
                Level::ERROR => event!(Level::ERROR, "{line}"),
                Level::INFO => event!(Level::INFO, "{line}"),
                Level::TRACE => event!(Level::TRACE, "{line}"),
                Level::WARN => event!(Level::WARN, "{line}"),
            }
        }
    }
}

#[cfg(feature = "color")]
lazy_static! {
    pub(crate) static ref BOLD_BLUE: Style = Style::new().bold().blue();
    pub(crate) static ref BOLD_GREEN: Style = Style::new().bold().green();
}

#[allow(clippy::ref_option_ref)]
fn has_value(
    tuple: (&&'static str, &Option<&'static str>),
) -> Option<(&'static str, &'static str)> {
    let (key, value) = tuple;
    if value.is_some() {
        Some((*key, value.unwrap_or_default()))
    } else {
        None
    }
}

fn split_key(tuple: (&str, &str)) -> Option<(Vec<String>, String)> {
    let (key, value) = tuple;
    let key = key.to_ascii_lowercase();
    if key.starts_with("vergen") {
        let kv_vec: Vec<String> = key.split('_').filter_map(not_vergen).collect();
        Some((kv_vec, value.to_string()))
    } else {
        None
    }
}
fn split_kv(tuple: (Vec<String>, String)) -> Option<(String, String, String)> {
    let (mut kv, v) = tuple;
    if kv.len() >= 2 {
        let category = kv.remove(0);
        let label = kv.join(" ").to_case(Case::Title);
        Some((category, label, v))
    } else {
        None
    }
}

fn not_vergen(part: &str) -> Option<String> {
    if part == "vergen" {
        None
    } else {
        Some(part.to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::has_value;
    use crate::{vergen_fmt_env, Fmt, Prefix, Suffix};
    use anyhow::Result;
    #[cfg(feature = "color")]
    use console::Style;

    const TEST_PREFIX_SUFFIX: &str = r#"██████╗ ██╗   ██╗██████╗ ██╗    ██╗
██╔══██╗██║   ██║██╔══██╗██║    ██║
██████╔╝██║   ██║██║  ██║██║ █╗ ██║
██╔═══╝ ██║   ██║██║  ██║██║███╗██║
██║     ╚██████╔╝██████╔╝╚███╔███╔╝
╚═╝      ╚═════╝ ╚═════╝  ╚══╝╚══╝ 

4a61736f6e204f7a696173
"#;

    fn is_empty(map: &BTreeMap<&'static str, Option<&'static str>>) -> bool {
        map.iter().filter_map(has_value).count() == 0
    }

    #[test]
    fn default_works() -> Result<()> {
        let mut stdout = vec![];
        let map = vergen_fmt_env!();
        let empty = is_empty(&map);
        let fmt = Fmt::builder().env(map).build();
        fmt.display(&mut stdout)?;
        if empty {
            assert!(stdout.is_empty());
        } else {
            assert!(!stdout.is_empty());
        }
        Ok(())
    }

    #[test]
    #[cfg(feature = "color")]
    fn key_style_works() -> Result<()> {
        let mut stdout = vec![];
        let red_bold = Style::new().bold().red();
        let map = vergen_fmt_env!();
        let empty = is_empty(&map);
        let fmt = Fmt::builder().env(map).key_style(red_bold).build();
        fmt.display(&mut stdout)?;
        if empty {
            assert!(stdout.is_empty());
        } else {
            assert!(!stdout.is_empty());
        }
        Ok(())
    }

    #[test]
    #[cfg(feature = "color")]
    fn value_style_works() -> Result<()> {
        let mut stdout = vec![];
        let map = vergen_fmt_env!();
        let empty = is_empty(&map);
        let red_bold = Style::new().bold().red();
        let fmt = Fmt::builder().env(map).value_style(red_bold).build();
        fmt.display(&mut stdout)?;
        if empty {
            assert!(stdout.is_empty());
        } else {
            assert!(!stdout.is_empty());
        }
        Ok(())
    }

    #[test]
    fn prefix_works() -> Result<()> {
        let mut stdout = vec![];
        let map = vergen_fmt_env!();
        let prefix = Prefix::builder()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .build();
        let fmt = Fmt::builder().env(map).prefix(prefix).build();
        fmt.display(&mut stdout)?;
        assert!(!stdout.is_empty());
        Ok(())
    }

    #[test]
    fn suffix_works() -> Result<()> {
        let mut stdout = vec![];
        let map = vergen_fmt_env!();
        let suffix = Suffix::builder()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .build();
        let fmt = Fmt::builder().env(map).suffix(suffix).build();
        fmt.display(&mut stdout)?;
        assert!(!stdout.is_empty());
        Ok(())
    }
}
