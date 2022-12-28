use crate::{
    builder::{Builder, RustcEnvMap},
    constants::VERGEN_IDEMPOTENT_DEFAULT,
    key::VergenKey,
};
use anyhow::{Error, Result};
use std::{env, str::FromStr};
use time::{
    format_description::{self, well_known::Rfc3339},
    OffsetDateTime,
};

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Config {
    pub(crate) build_date: bool,
    pub(crate) build_time: bool,
    pub(crate) build_timestamp: bool,
    pub(crate) build_semver: bool,
}

impl Config {
    pub(crate) fn add_warnings(
        self,
        skip_if_error: bool,
        e: Error,
        warnings: &mut Vec<String>,
    ) -> Result<()> {
        if skip_if_error {
            if self.build_date {
                warnings.push(format!(
                    "Unable to add {} to output",
                    VergenKey::BuildDate.name()
                ));
            }
            if self.build_time {
                warnings.push(format!(
                    "Unable to add {} to output",
                    VergenKey::BuildTime.name()
                ));
            }
            if self.build_timestamp {
                warnings.push(format!(
                    "Unable to add {} to output",
                    VergenKey::BuildTimestamp.name()
                ));
            }
            if self.build_semver {
                warnings.push(format!(
                    "Unable to add {} to output",
                    VergenKey::BuildSemver.name()
                ));
            }
            Ok(())
        } else {
            Err(e)
        }
    }
}

/// The `VERGEN_BUILD_*` configuration features
///
/// | Variable | Sample |
/// | -------  | ------ |
/// | `VERGEN_BUILD_DATE` | 2021-02-25 |
/// | `VERGEN_BUILD_TIME` | 23:28:39 |
/// | `VERGEN_BUILD_TIMESTAMP` | 2021-02-25T23:28:39.493201+00:00 |
/// | `VERGEN_BUILD_SEMVER` | 8.0.0 |
///
/// # Example
///
/// ```
/// # use anyhow::Result;
/// # use vergen::Vergen;
/// #
/// # fn main() -> Result<()> {
/// Vergen::default().all_build().gen()?;
/// #   Ok(())
/// # }
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "build")))]
impl Builder {
    /// Enable all of the `VERGEN_BUILD_*` options
    pub fn all_build(&mut self) -> &mut Self {
        self.build_date()
            .build_semver()
            .build_time()
            .build_timestamp()
    }

    /// Enable the `VERGEN_BUILD_DATE` date output
    pub fn build_date(&mut self) -> &mut Self {
        self.build_config.build_date = true;
        self
    }

    /// Enable the `VERGEN_BUILD_TIME` date output
    pub fn build_time(&mut self) -> &mut Self {
        self.build_config.build_time = true;
        self
    }

    /// Enable the `VERGEN_BUILD_TIMESTAMP` date output
    pub fn build_timestamp(&mut self) -> &mut Self {
        self.build_config.build_timestamp = true;
        self
    }

    /// Enable the `VERGEN_BUILD_SEMVER` date output
    pub fn build_semver(&mut self) -> &mut Self {
        self.build_config.build_semver = true;
        self
    }

    pub(crate) fn add_build_map_entries(
        &self,
        idempotent: bool,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
    ) -> Result<()> {
        self.add_semver_entry(map)?;
        self.add_timestamp_entries(idempotent, map, warnings)?;
        Ok(())
    }

    fn add_semver_entry(&self, map: &mut RustcEnvMap) -> Result<()> {
        if self.build_config.build_semver {
            let _old = map.insert(VergenKey::BuildSemver, env::var("CARGO_PKG_VERSION")?);
        }
        Ok(())
    }

    fn add_timestamp_entries(
        &self,
        idempotent: bool,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
    ) -> Result<()> {
        let (sde, ts) = match env::var("SOURCE_DATE_EPOCH") {
            Ok(v) => (
                true,
                OffsetDateTime::from_unix_timestamp(i64::from_str(&v)?)?,
            ),
            Err(std::env::VarError::NotPresent) => (false, OffsetDateTime::now_utc()),
            Err(e) => return Err(e.into()),
        };

        self.add_date_entry(idempotent, sde, &ts, map, warnings)?;
        self.add_time_entry(idempotent, sde, &ts, map, warnings)?;
        self.add_timestamp_entry(idempotent, sde, &ts, map, warnings)?;
        Ok(())
    }

    fn add_date_entry(
        &self,
        idempotent: bool,
        source_date_epoch: bool,
        ts: &OffsetDateTime,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
    ) -> Result<()> {
        if self.build_config.build_date {
            if idempotent && !source_date_epoch {
                warnings.push(format!(
                    "{} set to idempotent default",
                    VergenKey::BuildDate.name()
                ));
                let _old = map.insert(VergenKey::BuildDate, VERGEN_IDEMPOTENT_DEFAULT.to_string());
            } else {
                let format = format_description::parse("[year]-[month]-[day]")?;
                let _old = map.insert(VergenKey::BuildDate, ts.format(&format)?);
            }
        }
        Ok(())
    }

    fn add_time_entry(
        &self,
        idempotent: bool,
        source_date_epoch: bool,
        ts: &OffsetDateTime,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
    ) -> Result<()> {
        if self.build_config.build_time {
            if idempotent && !source_date_epoch {
                warnings.push(format!(
                    "{} set to idempotent default",
                    VergenKey::BuildTime.name()
                ));
                let _old = map.insert(VergenKey::BuildTime, VERGEN_IDEMPOTENT_DEFAULT.to_string());
            } else {
                let format = format_description::parse("[hour]:[minute]:[second]")?;
                let _old = map.insert(VergenKey::BuildTime, ts.format(&format)?);
            }
        }
        Ok(())
    }

    fn add_timestamp_entry(
        &self,
        idempotent: bool,
        source_date_epoch: bool,
        ts: &OffsetDateTime,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
    ) -> Result<()> {
        if self.build_config.build_time {
            if idempotent && !source_date_epoch {
                warnings.push(format!(
                    "{} set to idempotent default",
                    VergenKey::BuildTimestamp.name()
                ));
                let _old = map.insert(
                    VergenKey::BuildTimestamp,
                    VERGEN_IDEMPOTENT_DEFAULT.to_string(),
                );
            } else {
                let _old = map.insert(VergenKey::BuildTimestamp, ts.format(&Rfc3339)?);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{builder::test::count_idempotent, Vergen};
    use anyhow::Result;
    use std::env;

    #[test]
    #[serial_test::parallel]
    fn build_all_idempotent() -> Result<()> {
        let config = Vergen::default().idempotent().all_build().test_gen()?;
        assert_eq!(4, config.cargo_rustc_env_map.len());
        assert_eq!(3, count_idempotent(config.cargo_rustc_env_map));
        assert_eq!(3, config.warnings.len());
        Ok(())
    }

    #[test]
    #[serial_test::parallel]
    fn build_all() -> Result<()> {
        let config = Vergen::default().all_build().test_gen()?;
        assert_eq!(4, config.cargo_rustc_env_map.len());
        assert_eq!(0, count_idempotent(config.cargo_rustc_env_map));
        assert_eq!(0, config.warnings.len());
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn source_date_epoch_works() -> Result<()> {
        env::set_var("SOURCE_DATE_EPOCH", "1671809360");
        let mut stdout_buf = vec![];
        Vergen::default()
            .idempotent()
            .all_build()
            .test_gen_output(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        for (idx, line) in output.lines().enumerate() {
            if idx == 0 {
                assert_eq!("cargo:rustc-env=VERGEN_BUILD_DATE=2022-12-23", line);
            } else if idx == 1 {
                assert_eq!("cargo:rustc-env=VERGEN_BUILD_TIME=15:29:20", line);
            } else if idx == 2 {
                assert_eq!(
                    "cargo:rustc-env=VERGEN_BUILD_TIMESTAMP=2022-12-23T15:29:20Z",
                    line
                );
            }
        }
        env::remove_var("SOURCE_DATE_EPOCH");
        Ok(())
    }
}
