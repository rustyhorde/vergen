// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use self::{prefix::Prefix, suffix::Suffix};
use crate::utils::{has_value, split_key, split_kv};
use anyhow::Result;
#[cfg(feature = "color")]
use console::Style;
use derive_builder::Builder;
use std::{collections::BTreeMap, io::Write};
#[cfg(feature = "trace")]
use tracing::Level;

pub(crate) mod feature;
pub(crate) mod prefix;
pub(crate) mod suffix;

/// Configuration for `vergen` environment variable pretty printing
///
/// Build this with [`PrettyBuilder`]
///
/// # Display
/// ```
/// # use anyhow::Result;
/// # use vergen_pretty::{vergen_pretty_env, PrettyBuilder};
/// #
/// # pub fn main() -> Result<()> {
/// let mut stdout = vec![];
/// PrettyBuilder::default()
///     .env(vergen_pretty_env!())
///     .build()?
///     .display(&mut stdout)?;
/// #     Ok(())
/// # }
/// ```
///
#[cfg_attr(
    feature = "trace",
    doc = r##"
# Trace

Generate [`tracing`] output

```
# use anyhow::Result;
# use vergen_pretty::{vergen_pretty_env, PrettyBuilder};
# 
# pub fn main() -> Result<()> {
PrettyBuilder::default()
    .env(vergen_pretty_env!())
    .build()?
    .trace();
#     Ok(())
# }
```
"##
)]
///
/// # Output a prefix/suffix
///
/// [`Prefix`] and [`Suffix`] also have configurable styles if you enable
/// the `color` feature
///
/// ```
/// # use anyhow::Result;
/// # use vergen_pretty::{vergen_pretty_env, PrettyBuilder, PrefixBuilder, SuffixBuilder};
/// #
/// const TEST_PREFIX: &str = r#"A wonderful introduction
/// "#;
/// const TEST_SUFFIX: &str = r#"An outro"#;
///
/// # pub fn main() -> Result<()> {
/// let mut stdout = vec![];
/// let prefix = PrefixBuilder::default()
///     .lines(TEST_PREFIX.lines().map(str::to_string).collect())
///     .build()?;
/// let suffix = SuffixBuilder::default()
///     .lines(TEST_SUFFIX.lines().map(str::to_string).collect())
///     .build()?;
/// PrettyBuilder::default()
///     .env(vergen_pretty_env!())
///     .prefix(prefix)
///     .suffix(suffix)
///     .build()?
///     .display(&mut stdout)?;
/// #     Ok(())
/// # }
/// ```
///
#[cfg_attr(
    feature = "color",
    doc = r##"
# Customize Colorized Output

Uses [`Style`](console::Style) to colorize output

```
# use anyhow::Result;
# use vergen_pretty::{vergen_pretty_env, PrettyBuilder, Style};
# 
# pub fn main() -> Result<()> {
let mut stdout = vec![];
let red_bold = Style::new().bold().red();
let yellow_bold = Style::new().bold().yellow();
PrettyBuilder::default()
    .env(vergen_pretty_env!())
    .key_style(red_bold)
    .value_style(yellow_bold)
    .build()?
    .display(&mut stdout)?;
#     Ok(())
# }
```
"##
)]
///
#[derive(Builder, Clone, Debug, PartialEq)]
pub struct Pretty {
    /// Set the optional [`Prefix`] configuration
    #[builder(setter(strip_option), default)]
    prefix: Option<Prefix>,
    /// The `vergen` env variables.  Should be set with [`vergen_pretty_env!`](crate::vergen_pretty_env) macro.
    env: BTreeMap<&'static str, Option<&'static str>>,
    /// A set of `vergen` env variable names that should be filtered regardless if they are set or not.
    #[builder(setter(strip_option), default)]
    filter: Option<Vec<&'static str>>,
    /// Include category output.  Default: true
    #[builder(default = "true")]
    category: bool,
    #[builder(setter(skip), default)]
    vars: Vec<(String, String, String)>,
    /// The [`Style`] to apply to the keys in the output
    #[cfg(feature = "color")]
    #[builder(setter(strip_option), default)]
    key_style: Option<Style>,
    /// The [`Style`] to apply to the values in the output
    #[cfg(feature = "color")]
    #[builder(setter(strip_option), default)]
    value_style: Option<Style>,
    #[builder(setter(skip), default)]
    max_label: usize,
    #[builder(setter(skip), default)]
    max_category: usize,
    /// Set the optional [`Suffix`] configuration
    #[builder(setter(strip_option), default)]
    suffix: Option<Suffix>,
    /// Set the tracing [`Level`]
    #[cfg(feature = "trace")]
    #[builder(default = "Level::INFO")]
    level: Level,
    /// Flatten the serde output if no prefix/suffix are defined. Default: false
    #[cfg(feature = "serde")]
    #[builder(default = "false")]
    flatten: bool,
}

impl Pretty {
    /// Write the `vergen` environment variables that are set in table format to
    /// the given [`writer`](std::io::Write)
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
            let max_label = self.max_label;
            let max_category = self.max_category;
            let key = if self.category {
                format!("{label:>max_label$} ({category:>max_category$})")
            } else {
                format!("{label:>max_label$}")
            };
            self.inner_display(writer, &key, value)?;
        }

        if let Some(suffix) = &self.suffix {
            suffix.display(writer)?;
        }

        Ok(())
    }

    fn populate_fmt(&mut self) {
        let filter_fn = |tuple: &(&'static str, &'static str)| -> bool {
            let (key, _) = tuple;
            if let Some(filter) = &self.filter {
                !filter.contains(key)
            } else {
                true
            }
        };
        let vm_iter: Vec<(String, String, String)> = self
            .env
            .iter()
            .filter_map(has_value)
            .filter(filter_fn)
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

    #[cfg(not(feature = "color"))]
    #[allow(clippy::unused_self)]
    fn inner_display<T>(&self, writer: &mut T, key: &str, value: &str) -> Result<()>
    where
        T: Write + ?Sized,
    {
        Ok(writeln!(writer, "{key}: {value}")?)
    }
}

#[cfg(test)]
mod tests {
    use super::PrettyBuilder;
    use crate::vergen_pretty_env;
    use anyhow::Result;
    use std::io::Write;

    #[test]
    #[allow(clippy::clone_on_copy, clippy::redundant_clone)]
    fn pretty_clone_works() -> Result<()> {
        let map = vergen_pretty_env!();
        let pretty = PrettyBuilder::default().env(map).build()?;
        let another = pretty.clone();
        assert_eq!(pretty, another);
        Ok(())
    }

    #[test]
    fn pretty_debug_works() -> Result<()> {
        let map = vergen_pretty_env!();
        let pretty = PrettyBuilder::default().env(map).build()?;
        let mut buf = vec![];
        write!(buf, "{pretty:?}")?;
        assert!(!buf.is_empty());
        Ok(())
    }

    #[test]
    fn default_display_works() -> Result<()> {
        let mut stdout = vec![];
        let map = vergen_pretty_env!();
        let fmt = PrettyBuilder::default().env(map).build()?;
        fmt.display(&mut stdout)?;
        assert!(!stdout.is_empty());
        Ok(())
    }

    #[test]
    fn no_category_works() -> Result<()> {
        let mut stdout = vec![];
        let map = vergen_pretty_env!();
        let fmt = PrettyBuilder::default().env(map).category(false).build()?;
        fmt.display(&mut stdout)?;
        assert!(!stdout.is_empty());
        Ok(())
    }
}
