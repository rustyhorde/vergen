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
use std::{collections::BTreeMap, io::Write};
#[cfg(feature = "trace")]
use tracing::Level;
use typed_builder::TypedBuilder;

pub(crate) mod feature;
pub(crate) mod prefix;
pub(crate) mod suffix;

/// Configure `vergen` environment variable pretty printing
#[derive(Clone, Debug, TypedBuilder)]
pub struct Pretty {
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

impl Pretty {
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
            let max_label = self.max_label;
            let max_category = self.max_category;
            let key = format!("{label:>max_label$} ({category:>max_category$})");
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

    #[cfg(not(feature = "color"))]
    fn inner_display<T>(&self, writer: &mut T, key: &str, value: &str) -> Result<()>
    where
        T: Write + ?Sized,
    {
        Ok(writeln!(writer, "{key}: {value}")?)
    }
}

#[cfg(test)]
mod tests {
    use crate::{utils::test_utils::is_empty, vergen_pretty_env, Pretty};
    use anyhow::Result;

    #[test]
    fn default_display_works() -> Result<()> {
        let mut stdout = vec![];
        let map = vergen_pretty_env!();
        let empty = is_empty(&map);
        let fmt = Pretty::builder().env(map).build();
        fmt.display(&mut stdout)?;
        if empty {
            assert!(stdout.is_empty());
        } else {
            assert!(!stdout.is_empty());
        }
        Ok(())
    }
}
