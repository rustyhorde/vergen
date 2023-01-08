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
use std::io::Write;
#[cfg(feature = "trace")]
use tracing::Level;
use typed_builder::TypedBuilder;

/// Configure prefix output for [`Pretty`](crate::Pretty)
#[derive(Clone, Debug, TypedBuilder)]
pub struct Prefix {
    pub(crate) lines: Vec<String>,
    #[cfg(feature = "color")]
    #[builder(setter(strip_option, into), default)]
    pub(crate) style: Option<Style>,
    #[cfg(feature = "trace")]
    #[builder(default = Level::INFO)]
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
    use crate::{utils::test_utils::TEST_PREFIX_SUFFIX, vergen_pretty_env, Prefix, Pretty};
    use anyhow::Result;

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
