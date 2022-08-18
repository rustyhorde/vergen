// Copyright (c) 2016, 2018, 2021 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `vergen` build feature implementation

use crate::config::{Config, Instructions};
use anyhow::Result;
#[cfg(feature = "build")]
use {
    crate::{
        config::VergenKey,
        feature::{add_entry, TimeZone, TimestampKind},
    },
    getset::{Getters, MutGetters},
    std::env,
    time::{format_description, OffsetDateTime},
};

/// Configuration for the `VERGEN_BUILD_*` instructions
///
/// # Instructions
/// The following instructions can be generated:
///
/// | Instruction | Default |
/// | ----------- | :-----: |
/// | `cargo:rustc-env=VERGEN_BUILD_DATE=2021-02-12` | |
/// | `cargo:rustc-env=VERGEN_BUILD_TIME=11:22:34` | |
/// | `cargo:rustc-env=VERGEN_BUILD_TIMESTAMP=2021-02-12T01:54:15.134750+00:00` | * |
/// | `cargo:rustc-env=VERGEN_BUILD_SEMVER=4.2.0` | * |
///
/// * If the `timestamp` field is false, the date/time instructions will not be generated.
/// * If the `semver` field is false, the semver instruction will not be generated.
/// * **NOTE** - By default, the date/time related instructions will use [`UTC`](TimeZone::Utc).
/// * **NOTE** - The date/time instruction output is determined by the [`kind`](TimestampKind) field and can be any combination of the three.
/// * **NOTE** - To keep processing other sections if an Error occurs in this one, set
///     [`Build::skip_if_error`](Build::skip_if_error_mut()) to true.
///
/// # Example
///
/// ```
/// # use anyhow::Result;
/// use vergen::{vergen, Config};
#[cfg_attr(feature = "build", doc = r##"use vergen::TimestampKind;"##)]
#[cfg_attr(
    all(feature = "build", feature = "local_offset"),
    doc = r##"use vergen::TimeZone;"##
)]
///
/// # pub fn main() -> Result<()> {
/// let mut config = Config::default();
#[cfg_attr(
    feature = "build",
    doc = r##"
// Generate all three date/time instructions
*config.build_mut().kind_mut() = TimestampKind::All;

// Generate the instructions
vergen(config)?;
"##
)]
#[cfg_attr(
    all(feature = "build", feature = "local_offset"),
    doc = r##"
// Generate all three date/time instructions
*config.build_mut().kind_mut() = TimestampKind::All;
// Generate the output time in the local timezone
*config.build_mut().timezone_mut() = TimeZone::Local;

// Generate the instructions
vergen(config)?;
"##
)]
/// # Ok(())
/// # }
#[cfg(feature = "build")]
#[derive(Clone, Copy, Debug, Getters, MutGetters)]
#[getset(get = "pub(crate)", get_mut = "pub")]
pub struct Build {
    /// Enable/Disable the build output
    enabled: bool,
    /// Enable/Disable the `VERGEN_BUILD_DATE`, `VERGEN_BUILD_TIME`, and `VERGEN_BUILD_TIMESTAMP` instructions.
    timestamp: bool,
    /// The timezone to use for the date/time instructions.
    timezone: TimeZone,
    /// The kind of date/time instructions to output.
    kind: TimestampKind,
    /// Enable/Disable the `VERGEN_BUILD_SEMVER` instruction.
    semver: bool,
    /// Enable/Disable skipping [`Build`] if an Error occurs.
    /// Use [`option_env!`](std::option_env!) to read the generated environment variables.
    skip_if_error: bool,
}

#[cfg(feature = "build")]
impl Default for Build {
    fn default() -> Self {
        Self {
            enabled: true,
            timestamp: true,
            timezone: TimeZone::Utc,
            kind: TimestampKind::Timestamp,
            semver: true,
            skip_if_error: false,
        }
    }
}

#[cfg(feature = "build")]
impl Build {
    pub(crate) fn has_enabled(self) -> bool {
        self.enabled && (self.timestamp || self.semver)
    }
}

#[cfg(feature = "build")]
pub(crate) fn configure_build(instructions: &Instructions, config: &mut Config) -> Result<()> {
    let build_config = instructions.build();

    let mut add_entries = || {
        if *build_config.timestamp() {
            match build_config.timezone() {
                TimeZone::Utc => {
                    add_config_entries(config, *build_config, &OffsetDateTime::now_utc())?;
                }
                #[cfg(feature = "local_offset")]
                TimeZone::Local => {
                    add_config_entries(config, *build_config, &OffsetDateTime::now_local()?)?;
                }
            };
        }

        if *build_config.semver() {
            add_entry(
                config.cfg_map_mut(),
                VergenKey::BuildSemver,
                env::var("CARGO_PKG_VERSION").ok(),
            );
        }
        Ok(())
    };

    if build_config.has_enabled() {
        if build_config.skip_if_error {
            // hide errors, but emit a warning
            let result = add_entries();
            if result.is_err() {
                let warning = format!(
                    "An Error occurred during processing of {}. \
                    VERGEN_{}_* may be incomplete.",
                    "Build", "BUILD"
                );
                config.warnings_mut().push(warning);
            }
            Ok(())
        } else {
            add_entries()
        }
    } else {
        Ok(())
    }
}

#[cfg(feature = "build")]
fn add_config_entries(
    config: &mut Config,
    build_config: Build,
    now: &OffsetDateTime,
) -> Result<()> {
    match build_config.kind() {
        TimestampKind::DateOnly => add_date_entry(config, now)?,
        TimestampKind::TimeOnly => add_time_entry(config, now)?,
        TimestampKind::DateAndTime => {
            add_date_entry(config, now)?;
            add_time_entry(config, now)?;
        }
        TimestampKind::Timestamp => add_timestamp_entry(config, now)?,
        TimestampKind::All => {
            add_date_entry(config, now)?;
            add_time_entry(config, now)?;
            add_timestamp_entry(config, now)?;
        }
    }
    Ok(())
}

#[cfg(feature = "build")]
fn add_date_entry(config: &mut Config, now: &OffsetDateTime) -> Result<()> {
    let format = format_description::parse("[year]-[month]-[day]")?;
    add_entry(
        config.cfg_map_mut(),
        VergenKey::BuildDate,
        Some(now.format(&format)?),
    );
    Ok(())
}

#[cfg(feature = "build")]
fn add_time_entry(config: &mut Config, now: &OffsetDateTime) -> Result<()> {
    let format = format_description::parse("[hour]:[minute]:[second]")?;
    add_entry(
        config.cfg_map_mut(),
        VergenKey::BuildTime,
        Some(now.format(&format)?),
    );
    Ok(())
}

#[cfg(feature = "build")]
fn add_timestamp_entry(config: &mut Config, now: &OffsetDateTime) -> Result<()> {
    use time::format_description::well_known::Rfc3339;

    add_entry(
        config.cfg_map_mut(),
        VergenKey::BuildTimestamp,
        Some(now.format(&Rfc3339)?),
    );
    Ok(())
}

#[cfg(not(feature = "build"))]
pub(crate) fn configure_build(_instructions: &Instructions, _config: &mut Config) -> Result<()> {
    Ok(())
}

#[cfg(all(test, feature = "build"))]
mod test {
    use crate::{
        config::Instructions,
        feature::{TimeZone, TimestampKind},
    };

    #[test]
    fn build_config() {
        let mut config = Instructions::default();
        assert!(config.build().timestamp());
        assert_eq!(config.build().timezone(), &TimeZone::Utc);
        assert_eq!(config.build().kind(), &TimestampKind::Timestamp);
        *config.build_mut().kind_mut() = TimestampKind::All;
        assert_eq!(config.build().kind(), &TimestampKind::All);
    }

    #[test]
    fn not_enabled() {
        let mut config = Instructions::default();
        *config.build_mut().enabled_mut() = false;
        assert!(!config.build().has_enabled());
    }

    #[test]
    fn no_timestamp() {
        let mut config = Instructions::default();
        *config.build_mut().timestamp_mut() = false;
        assert!(config.build().has_enabled());
    }

    #[test]
    fn nothing() {
        let mut config = Instructions::default();
        *config.build_mut().timestamp_mut() = false;
        *config.build_mut().semver_mut() = false;
        assert!(!config.build().has_enabled());
    }
}

#[cfg(all(test, not(feature = "build")))]
mod test {}
