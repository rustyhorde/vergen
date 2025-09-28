// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use anyhow::Result;
#[cfg(feature = "color")]
use console::Style;
use derive_builder::Builder;
#[cfg(feature = "serde")]
use serde::Serialize;
use std::io::Write;
#[cfg(feature = "trace")]
use tracing::Level;

/// Configure suffix output for [`PrettyBuilder`](crate::PrettyBuilder)
#[derive(Builder, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Suffix {
    /// The suffix lines to output
    pub(crate) lines: Vec<String>,
    /// The [`Style`] to apply to the output lines
    #[cfg(feature = "color")]
    #[builder(setter(strip_option), default)]
    #[cfg_attr(feature = "serde", serde(skip))]
    pub(crate) style: Option<Style>,
    /// The tracing [`Level`] to output the prefix at
    #[cfg(feature = "trace")]
    #[builder(default = "Level::INFO")]
    #[cfg_attr(feature = "serde", serde(skip))]
    pub(crate) level: Level,
}

impl Suffix {
    /// Output the `vergen` environment variables that are set in table format
    ///
    /// # Errors
    /// * The [`writeln!`](std::writeln!) macro can throw a [`std::io::Error`]
    ///
    pub(crate) fn display<T>(&self, writer: &mut T) -> Result<()>
    where
        T: Write + ?Sized,
    {
        self.inner_display(writer)
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
}

#[cfg(test)]
mod test {
    use crate::{
        PrettyBuilder, SuffixBuilder, utils::test_utils::TEST_PREFIX_SUFFIX, vergen_pretty_env,
    };
    use anyhow::Result;

    #[test]
    fn display_suffix_works() -> Result<()> {
        let mut stdout = vec![];
        let map = vergen_pretty_env!();
        let suffix = SuffixBuilder::default()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .build()?;
        let fmt = PrettyBuilder::default().env(map).suffix(suffix).build()?;
        fmt.display(&mut stdout)?;
        assert!(!stdout.is_empty());
        Ok(())
    }
}
