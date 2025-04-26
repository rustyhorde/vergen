// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use anyhow::Result;
use bon::Builder;
#[cfg(feature = "color")]
use console::Style;
#[cfg(feature = "serde")]
use serde::Serialize;
use std::io::Write;
#[cfg(feature = "trace")]
use tracing::Level;

/// Configure prefix output for [`PrettyBuilder`](crate::PrettyBuilder)
#[derive(Builder, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Prefix {
    /// The prefix lines to output
    pub(crate) lines: Vec<String>,
    /// The [`Style`] to apply to the output lines
    #[cfg(feature = "color")]
    #[cfg_attr(feature = "serde", serde(skip))]
    pub(crate) style: Option<Style>,
    /// The tracing [`Level`] to output the prefix at
    #[cfg(feature = "trace")]
    #[builder(default = Level::INFO)]
    #[cfg_attr(feature = "serde", serde(skip))]
    pub(crate) level: Level,
}

impl Prefix {
    /// Output the `vergen` environment variables that are set in table format
    ///
    /// # Errors
    /// * The [`writeln!`](std::writeln!) macro can throw a [`std::io::Error`]
    ///
    pub(crate) fn display<T>(&self, writer: &mut T) -> Result<()>
    where
        T: Write + ?Sized,
    {
        self.inner_display(writer)?;
        writeln!(writer)?;
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
}

#[cfg(test)]
mod test {
    use crate::{Prefix, Pretty, utils::test_utils::TEST_PREFIX_SUFFIX, vergen_pretty_env};
    use anyhow::Result;
    use std::io::Write;

    #[test]
    #[allow(clippy::clone_on_copy, clippy::redundant_clone)]
    fn prefix_clone_works() {
        let prefix = Prefix::builder()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .build();
        let another = prefix.clone();
        assert_eq!(prefix, another);
    }

    #[test]
    fn prefix_debug_works() -> Result<()> {
        let prefix = Prefix::builder()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .build();
        let mut buf = vec![];
        write!(buf, "{prefix:?}")?;
        assert!(!buf.is_empty());
        Ok(())
    }

    #[test]
    fn display_prefix_works() -> Result<()> {
        let mut stdout = vec![];
        let map = vergen_pretty_env!();
        let prefix = Prefix::builder()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .build();
        let fmt = Pretty::builder().env(map).prefix(prefix).build();
        fmt.display(&mut stdout)?;
        assert!(!stdout.is_empty());
        Ok(())
    }
}
