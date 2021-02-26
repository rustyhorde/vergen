// Copyright (c) 2016, 2018, 2021 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `vergen` build feature implementation

use crate::{
    config::{Config, Instructions},
    constants::ConstantsFlags,
};
#[cfg(feature = "build")]
use {
    crate::{
        config::VergenKey,
        feature::{add_entry, TimeZone, TimestampKind},
    },
    chrono::{DateTime, Local, Utc},
    getset::{Getters, MutGetters},
    std::env,
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
///
/// # Example
///
/// ```
/// # use vergen::Error;
/// use vergen::{vergen, Config};
#[cfg_attr(feature = "build", doc = r##"use vergen::{TimestampKind, TimeZone};"##)]
///
/// # pub fn main() -> Result<(), Error> {
/// let mut config = Config::default();
#[cfg_attr(
    feature = "build",
    doc = r##"
// Generate all three date/time instructions
*config.build_mut().kind_mut() = TimestampKind::All;
// Change the date/time instructions to show `Local` time
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
    /// Enable/Disable the `VERGEN_BUILD_DATE`, `VERGEN_BUILD_TIME`, and `VERGEN_BUILD_TIMESTAMP` instructions.
    timestamp: bool,
    /// The timezone to use for the date/time instructions.
    timezone: TimeZone,
    /// The kind of date/time instructions to output.
    kind: TimestampKind,
    /// Enable/Disable the `VERGEN_BUILD_SEMVER` instruction.
    semver: bool,
}

#[cfg(feature = "build")]
impl Default for Build {
    fn default() -> Self {
        Self {
            timestamp: true,
            timezone: TimeZone::Utc,
            kind: TimestampKind::Timestamp,
            semver: true,
        }
    }
}

#[cfg(feature = "build")]
impl Build {
    pub(crate) fn has_enabled(self) -> bool {
        self.timestamp || self.semver
    }
}

#[cfg(feature = "build")]
pub(crate) fn add_build_config(flags: ConstantsFlags, config: &mut Config) {
    // Setup datetime information
    let now = Utc::now();
    if flags.contains(ConstantsFlags::BUILD_TIMESTAMP) {
        add_timestamp_entry(config, &now);
    }

    if flags.contains(ConstantsFlags::BUILD_DATE) {
        add_date_entry(config, &now);
    }

    if flags.contains(ConstantsFlags::SEMVER_FROM_CARGO_PKG) {
        add_entry(
            config.cfg_map_mut(),
            VergenKey::Semver,
            env::var("CARGO_PKG_VERSION").ok(),
        );
    }
}

#[cfg(feature = "build")]
pub(crate) fn configure_build(instructions: Instructions, config: &mut Config) {
    let build_config = instructions.build();

    if build_config.has_enabled() {
        if *build_config.timestamp() {
            match build_config.timezone() {
                TimeZone::Utc => add_config_entries(config, *build_config, &Utc::now()),
                TimeZone::Local => add_config_entries(config, *build_config, &Local::now()),
            };
        }

        if *build_config.semver() {
            add_entry(
                config.cfg_map_mut(),
                VergenKey::BuildSemver,
                env::var("CARGO_PKG_VERSION").ok(),
            );
        }
    }
}

#[cfg(feature = "build")]
fn add_config_entries<T>(config: &mut Config, build_config: Build, now: &DateTime<T>)
where
    T: chrono::TimeZone,
    T::Offset: std::fmt::Display,
{
    match build_config.kind() {
        TimestampKind::DateOnly => add_date_entry(config, now),
        TimestampKind::TimeOnly => add_time_entry(config, now),
        TimestampKind::DateAndTime => {
            add_date_entry(config, now);
            add_time_entry(config, now);
        }
        TimestampKind::Timestamp => add_timestamp_entry(config, now),
        TimestampKind::All => {
            add_date_entry(config, now);
            add_time_entry(config, now);
            add_timestamp_entry(config, now);
        }
    }
}

#[cfg(feature = "build")]
fn add_date_entry<T>(config: &mut Config, now: &DateTime<T>)
where
    T: chrono::TimeZone,
    T::Offset: std::fmt::Display,
{
    add_entry(
        config.cfg_map_mut(),
        VergenKey::BuildDate,
        Some(now.format("%Y-%m-%d").to_string()),
    );
}

#[cfg(feature = "build")]
fn add_time_entry<T>(config: &mut Config, now: &DateTime<T>)
where
    T: chrono::TimeZone,
    T::Offset: std::fmt::Display,
{
    add_entry(
        config.cfg_map_mut(),
        VergenKey::BuildTime,
        Some(now.format("%H:%M:%S").to_string()),
    );
}

#[cfg(feature = "build")]
fn add_timestamp_entry<T>(config: &mut Config, now: &DateTime<T>)
where
    T: chrono::TimeZone,
    T::Offset: std::fmt::Display,
{
    add_entry(
        config.cfg_map_mut(),
        VergenKey::BuildTimestamp,
        Some(now.to_rfc3339()),
    );
}

#[cfg(not(feature = "build"))]
pub(crate) fn add_build_config(_flags: ConstantsFlags, _config: &mut Config) {}

#[cfg(not(feature = "build"))]
pub(crate) fn configure_build(_instructions: Instructions, _config: &mut Config) {}

#[cfg(all(test, feature = "build"))]
mod test {
    use super::add_build_config;
    use crate::{
        config::{Config, Instructions, VergenKey},
        constants::ConstantsFlags,
        feature::{TimeZone, TimestampKind},
        test::get_map_value,
    };
    use lazy_static::lazy_static;
    use regex::Regex;
    use std::collections::BTreeMap;

    lazy_static! {
        static ref DATE_REGEX: Regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
        static ref RFC3339_REGEX: Regex = Regex::new(r"^([0-9]+)-(0[1-9]|1[012])-(0[1-9]|[12][0-9]|3[01])[Tt]([01][0-9]|2[0-3]):([0-5][0-9]):([0-5][0-9]|60)(\.[0-9]+)?(([Zz])|([\+|\-]([01][0-9]|2[0-3]):[0-5][0-9]))$").unwrap();
        static ref SEMVER_REGEX: Regex = Regex::new(r"^(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$").unwrap();
    }

    fn check_build_instructions(cfg_map: &BTreeMap<VergenKey, Option<String>>) {
        assert!(DATE_REGEX.is_match(&get_map_value(VergenKey::BuildDate, cfg_map)));
        assert!(RFC3339_REGEX.is_match(&get_map_value(VergenKey::BuildTimestamp, cfg_map)));
        assert!(SEMVER_REGEX.is_match(&get_map_value(VergenKey::Semver, cfg_map)));
    }

    fn check_build_keys(cfg_map: &BTreeMap<VergenKey, Option<String>>) {
        let mut count = 0;
        for (k, v) in cfg_map {
            match *k {
                VergenKey::BuildDate | VergenKey::BuildTimestamp | VergenKey::Semver => {
                    assert!(v.is_some());
                    count += 1;
                }
                _ => {}
            }
        }
        assert_eq!(count, 3);
    }

    #[test]
    fn add_build_config_works() {
        let mut config = Config::default();
        add_build_config(ConstantsFlags::all(), &mut config);
        check_build_keys(config.cfg_map());
        check_build_instructions(config.cfg_map());
    }

    #[test]
    fn build_config() {
        let mut config = Instructions::default();
        assert!(config.build().timestamp());
        assert_eq!(config.build().timezone(), &TimeZone::Utc);
        assert_eq!(config.build().kind(), &TimestampKind::Timestamp);
        *config.build_mut().kind_mut() = TimestampKind::All;
        assert_eq!(config.build().kind(), &TimestampKind::All);
    }
}

#[cfg(all(test, not(feature = "build")))]
mod test {
    use super::add_build_config;
    use crate::{
        config::{Config, VergenKey},
        constants::ConstantsFlags,
        error::Result,
    };
    use std::collections::BTreeMap;

    fn check_build_keys(cfg_map: &BTreeMap<VergenKey, Option<String>>) {
        let mut count = 0;
        for (k, v) in cfg_map {
            match *k {
                VergenKey::BuildDate | VergenKey::BuildTimestamp | VergenKey::Semver => {
                    assert!(v.is_none());
                    count += 1;
                }
                _ => {}
            }
        }
        assert_eq!(count, 3);
    }

    #[test]
    fn add_build_config_works() -> Result<()> {
        let mut config = Config::default();
        add_build_config(ConstantsFlags::all(), &mut config);
        check_build_keys(config.cfg_map());
        Ok(())
    }
}
