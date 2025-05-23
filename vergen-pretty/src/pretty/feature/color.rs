// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use crate::{Prefix, Suffix, pretty::Pretty};
use anyhow::Result;
use console::Style;
use std::{io::Write, sync::LazyLock};

pub(crate) static BOLD_BLUE: LazyLock<Style> = LazyLock::new(|| Style::new().bold().blue());
pub(crate) static BOLD_GREEN: LazyLock<Style> = LazyLock::new(|| Style::new().bold().green());

impl Pretty {
    #[cfg_attr(docsrs, doc(cfg(feature = "color")))]
    pub(crate) fn inner_display<T>(&self, writer: &mut T, key: &str, value: &str) -> Result<()>
    where
        T: Write + ?Sized,
    {
        let key_so = if let Some(style) = &self.key_style {
            style
        } else {
            &*BOLD_BLUE
        };
        let value_so = if let Some(style) = &self.value_style {
            style
        } else {
            &*BOLD_GREEN
        };
        let key = key_so.apply_to(key);
        let value = value_so.apply_to(value);
        Ok(writeln!(writer, "{key}: {value}")?)
    }
}

impl Prefix {
    pub(crate) fn inner_display<T>(&self, writer: &mut T) -> Result<()>
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
}

impl Suffix {
    pub(crate) fn inner_display<T>(&self, writer: &mut T) -> Result<()>
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
}

#[cfg(test)]
mod test {
    use crate::{
        Prefix, Pretty, Suffix,
        utils::test_utils::{TEST_PREFIX_SUFFIX, is_empty},
        vergen_pretty_env,
    };
    use anyhow::Result;
    use console::Style;

    #[test]
    fn display_key_style_works() -> Result<()> {
        let mut stdout = vec![];
        let red_bold = Style::new().bold().red();
        let map = vergen_pretty_env!();
        let empty = is_empty(&map);
        let fmt = Pretty::builder().env(map).key_style(red_bold).build();
        fmt.display(&mut stdout)?;
        if empty {
            assert!(stdout.is_empty());
        } else {
            assert!(!stdout.is_empty());
        }
        Ok(())
    }

    #[test]
    fn display_value_style_works() -> Result<()> {
        let mut stdout = vec![];
        let map = vergen_pretty_env!();
        let empty = is_empty(&map);
        let red_bold = Style::new().bold().red();
        let fmt = Pretty::builder().env(map).value_style(red_bold).build();
        fmt.display(&mut stdout)?;
        if empty {
            assert!(stdout.is_empty());
        } else {
            assert!(!stdout.is_empty());
        }
        Ok(())
    }

    #[test]
    fn display_prefix_with_style_works() -> Result<()> {
        let mut stdout = vec![];
        let map = vergen_pretty_env!();
        let red_bold = Style::new().bold().red();
        let prefix = Prefix::builder()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .style(red_bold)
            .build();
        let fmt = Pretty::builder().env(map).prefix(prefix).build();
        fmt.display(&mut stdout)?;
        assert!(!stdout.is_empty());
        Ok(())
    }

    #[test]
    fn display_suffix_with_style_works() -> Result<()> {
        let mut stdout = vec![];
        let map = vergen_pretty_env!();
        let red_bold = Style::new().bold().red();
        let suffix = Suffix::builder()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .style(red_bold)
            .build();
        let fmt = Pretty::builder().env(map).suffix(suffix).build();
        fmt.display(&mut stdout)?;
        assert!(!stdout.is_empty());
        Ok(())
    }
}
