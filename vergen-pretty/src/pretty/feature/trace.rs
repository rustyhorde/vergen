// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#[cfg(feature = "color")]
use super::color::{BOLD_BLUE, BOLD_GREEN};
use crate::{pretty::Pretty, Prefix, Suffix};
use tracing::{event, Level};

impl Pretty {
    /// Output the `vergen` environment variables that are set in table format to a tracing subscriber
    ///
    #[cfg_attr(docsrs, doc(cfg(feature = "trace")))]
    pub fn trace(mut self) {
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

    #[cfg(feature = "color")]
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

    #[cfg(not(feature = "color"))]
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

impl Prefix {
    /// Output the `vergen` environment variables that are set in table format to a tracing subscriber
    ///
    pub(crate) fn trace(&self) {
        self.inner_trace();
    }

    #[cfg(feature = "color")]
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

    #[cfg(not(feature = "color"))]
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

impl Suffix {
    /// Output the `vergen` environment variables that are set in table format to a tracing subscriber
    ///
    pub(crate) fn trace(&self) {
        self.inner_trace();
    }

    #[cfg(feature = "color")]
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

    #[cfg(not(feature = "color"))]
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

#[cfg(test)]
mod test {
    use crate::{
        utils::test_utils::TEST_PREFIX_SUFFIX, vergen_pretty_env, PrefixBuilder, PrettyBuilder,
        SuffixBuilder,
    };
    use anyhow::Result;
    #[cfg(feature = "color")]
    use console::Style;
    use std::sync::Once;
    use tracing::{metadata::LevelFilter, Level};
    use tracing_subscriber::{
        fmt, prelude::__tracing_subscriber_SubscriberExt, registry, util::SubscriberInitExt,
    };

    static INIT_TRACING: Once = Once::new();

    fn initialize_tracing() {
        INIT_TRACING.call_once(|| {
            let format = fmt::layer().compact().with_level(true).with_ansi(true);
            let filter_layer = LevelFilter::from(Level::INFO);
            registry()
                .with(format)
                .with(filter_layer)
                .try_init()
                .expect("unable to initialize tracing");
        });
    }

    #[test]
    fn default_trace_works() -> Result<()> {
        initialize_tracing();
        PrettyBuilder::default()
            .env(vergen_pretty_env!())
            .build()?
            .trace();
        Ok(())
    }

    #[test]
    fn trace_debug_works() -> Result<()> {
        initialize_tracing();
        let level = Level::DEBUG;
        let prefix = PrefixBuilder::default()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .level(level)
            .build()?;
        let suffix = SuffixBuilder::default()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .level(level)
            .build()?;
        PrettyBuilder::default()
            .env(vergen_pretty_env!())
            .level(level)
            .prefix(prefix)
            .suffix(suffix)
            .build()?
            .trace();
        Ok(())
    }

    #[test]
    fn default_trace_trace_works() -> Result<()> {
        initialize_tracing();
        let level = Level::TRACE;
        let prefix = PrefixBuilder::default()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .level(level)
            .build()?;
        let suffix = SuffixBuilder::default()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .level(level)
            .build()?;
        PrettyBuilder::default()
            .env(vergen_pretty_env!())
            .level(level)
            .prefix(prefix)
            .suffix(suffix)
            .build()?
            .trace();
        Ok(())
    }

    #[test]
    fn default_trace_error_works() -> Result<()> {
        initialize_tracing();
        let level = Level::ERROR;
        let prefix = PrefixBuilder::default()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .level(level)
            .build()?;
        let suffix = SuffixBuilder::default()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .level(level)
            .build()?;
        PrettyBuilder::default()
            .env(vergen_pretty_env!())
            .level(level)
            .prefix(prefix)
            .suffix(suffix)
            .build()?
            .trace();
        Ok(())
    }

    #[test]
    fn default_trace_warn_works() -> Result<()> {
        initialize_tracing();
        let level = Level::WARN;
        let prefix = PrefixBuilder::default()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .level(level)
            .build()?;
        let suffix = SuffixBuilder::default()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .level(level)
            .build()?;
        PrettyBuilder::default()
            .env(vergen_pretty_env!())
            .level(level)
            .prefix(prefix)
            .suffix(suffix)
            .build()?
            .trace();
        Ok(())
    }

    #[test]
    #[cfg(feature = "color")]
    fn trace_key_style_works() -> Result<()> {
        initialize_tracing();
        let red_bold = Style::new().bold().red();
        PrettyBuilder::default()
            .env(vergen_pretty_env!())
            .key_style(red_bold)
            .build()?
            .trace();
        Ok(())
    }

    #[test]
    #[cfg(feature = "color")]
    fn trace_value_style_works() -> Result<()> {
        initialize_tracing();
        let red_bold = Style::new().bold().red();
        PrettyBuilder::default()
            .env(vergen_pretty_env!())
            .value_style(red_bold)
            .build()?
            .trace();
        Ok(())
    }

    #[test]
    fn trace_prefix_works() -> Result<()> {
        initialize_tracing();
        let prefix = PrefixBuilder::default()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .build()?;
        PrettyBuilder::default()
            .env(vergen_pretty_env!())
            .prefix(prefix)
            .build()?
            .trace();
        Ok(())
    }

    #[cfg(feature = "color")]
    #[test]
    fn trace_prefix_with_style_works() -> Result<()> {
        initialize_tracing();
        let red_bold = Style::new().bold().red();
        let prefix = PrefixBuilder::default()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .style(red_bold)
            .build()?;
        PrettyBuilder::default()
            .env(vergen_pretty_env!())
            .prefix(prefix)
            .build()?
            .trace();
        Ok(())
    }

    #[test]
    fn trace_suffix_works() -> Result<()> {
        initialize_tracing();
        let suffix = SuffixBuilder::default()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .build()?;
        PrettyBuilder::default()
            .env(vergen_pretty_env!())
            .suffix(suffix)
            .build()?
            .trace();
        Ok(())
    }

    #[cfg(feature = "color")]
    #[test]
    fn trace_suffix_with_style_works() -> Result<()> {
        initialize_tracing();
        let red_bold = Style::new().bold().red();
        let suffix = SuffixBuilder::default()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .style(red_bold)
            .build()?;
        PrettyBuilder::default()
            .env(vergen_pretty_env!())
            .suffix(suffix)
            .build()?
            .trace();
        Ok(())
    }

    #[test]
    fn trace_with_filter_works() -> Result<()> {
        initialize_tracing();
        PrettyBuilder::default()
            .env(vergen_pretty_env!())
            .filter(vec!["VERGEN_GIT_BRANCH", "VERGEN_SYSINFO_USER"])
            .build()?
            .trace();
        Ok(())
    }
}
