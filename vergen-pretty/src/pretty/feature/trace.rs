// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#[cfg(feature = "color")]
use super::color::{BOLD_BLUE, BOLD_GREEN};
use crate::{Prefix, Suffix, pretty::Pretty};
use tracing::{Level, event};

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
            &BOLD_BLUE
        }
        .apply_to(key);
        let value = if let Some(style) = &self.value_style {
            style
        } else {
            &BOLD_GREEN
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
    use crate::{Prefix, Pretty, Suffix, utils::test_utils::TEST_PREFIX_SUFFIX, vergen_pretty_env};
    #[cfg(feature = "color")]
    use console::Style;
    use std::sync::Once;
    use tracing::{Level, metadata::LevelFilter};
    use tracing_subscriber::{
        fmt, prelude::__tracing_subscriber_SubscriberExt, registry, util::SubscriberInitExt,
    };

    static INIT_TRACING: Once = Once::new();

    fn initialize_tracing() {
        INIT_TRACING.call_once(|| {
            let format = fmt::layer().compact().with_level(true).with_ansi(true);
            let filter_layer = LevelFilter::from(Level::TRACE);
            registry()
                .with(format)
                .with(filter_layer)
                .try_init()
                .expect("unable to initialize tracing");
        });
    }

    #[test]
    fn default_trace_works() {
        initialize_tracing();
        Pretty::builder().env(vergen_pretty_env!()).build().trace();
    }

    #[test]
    fn trace_debug_works() {
        initialize_tracing();
        let level = Level::DEBUG;
        let prefix = Prefix::builder()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .level(level)
            .build();
        let suffix = Suffix::builder()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .level(level)
            .build();
        Pretty::builder()
            .env(vergen_pretty_env!())
            .level(level)
            .prefix(prefix)
            .suffix(suffix)
            .build()
            .trace();
    }

    #[test]
    fn default_trace_trace_works() {
        initialize_tracing();
        let level = Level::TRACE;
        let prefix = Prefix::builder()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .level(level)
            .build();
        let suffix = Suffix::builder()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .level(level)
            .build();
        Pretty::builder()
            .env(vergen_pretty_env!())
            .level(level)
            .prefix(prefix)
            .suffix(suffix)
            .build()
            .trace();
    }

    #[test]
    fn default_trace_error_works() {
        initialize_tracing();
        let level = Level::ERROR;
        let prefix = Prefix::builder()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .level(level)
            .build();
        let suffix = Suffix::builder()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .level(level)
            .build();
        Pretty::builder()
            .env(vergen_pretty_env!())
            .level(level)
            .prefix(prefix)
            .suffix(suffix)
            .build()
            .trace();
    }

    #[test]
    fn default_trace_warn_works() {
        initialize_tracing();
        let level = Level::WARN;
        let prefix = Prefix::builder()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .level(level)
            .build();
        let suffix = Suffix::builder()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .level(level)
            .build();
        Pretty::builder()
            .env(vergen_pretty_env!())
            .level(level)
            .prefix(prefix)
            .suffix(suffix)
            .build()
            .trace();
    }

    #[test]
    #[cfg(feature = "color")]
    fn trace_key_style_works() {
        initialize_tracing();
        let red_bold = Style::new().bold().red();
        Pretty::builder()
            .env(vergen_pretty_env!())
            .key_style(red_bold)
            .build()
            .trace();
    }

    #[test]
    #[cfg(feature = "color")]
    fn trace_value_style_works() {
        initialize_tracing();
        let red_bold = Style::new().bold().red();
        Pretty::builder()
            .env(vergen_pretty_env!())
            .value_style(red_bold)
            .build()
            .trace();
    }

    #[test]
    fn trace_prefix_works() {
        initialize_tracing();
        let prefix = Prefix::builder()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .build();
        Pretty::builder()
            .env(vergen_pretty_env!())
            .prefix(prefix)
            .build()
            .trace();
    }

    #[cfg(feature = "color")]
    #[test]
    fn trace_prefix_with_style_works() {
        initialize_tracing();
        let red_bold = Style::new().bold().red();
        let prefix = Prefix::builder()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .style(red_bold)
            .build();
        Pretty::builder()
            .env(vergen_pretty_env!())
            .prefix(prefix)
            .build()
            .trace();
    }

    #[test]
    fn trace_suffix_works() {
        initialize_tracing();
        let suffix = Suffix::builder()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .build();
        Pretty::builder()
            .env(vergen_pretty_env!())
            .suffix(suffix)
            .build()
            .trace();
    }

    #[cfg(feature = "color")]
    #[test]
    fn trace_suffix_with_style_works() {
        initialize_tracing();
        let red_bold = Style::new().bold().red();
        let suffix = Suffix::builder()
            .lines(TEST_PREFIX_SUFFIX.lines().map(str::to_string).collect())
            .style(red_bold)
            .build();
        Pretty::builder()
            .env(vergen_pretty_env!())
            .suffix(suffix)
            .build()
            .trace();
    }

    #[test]
    fn trace_with_filter_works() {
        initialize_tracing();
        Pretty::builder()
            .env(vergen_pretty_env!())
            .filter(vec!["VERGEN_GIT_BRANCH", "VERGEN_SYSINFO_USER"])
            .build()
            .trace();
    }
}
