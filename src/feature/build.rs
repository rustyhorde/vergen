use crate::{
    emitter::{EmitBuilder, RustcEnvMap},
    key::VergenKey,
    utils::fns::{add_default_map_entry, add_map_entry},
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
    pub(crate) build_timestamp: bool,
}

/// The `VERGEN_BUILD_*` configuration features
///
/// | Variable | Sample |
/// | -------  | ------ |
/// | `VERGEN_BUILD_DATE` | 2021-02-25 |
/// | `VERGEN_BUILD_TIMESTAMP` | 2021-02-25T23:28:39.493201+00:00 |
///
/// # Example
///
/// ```
/// # use anyhow::Result;
/// # use vergen::EmitBuilder;
/// #
/// # fn main() -> Result<()> {
/// EmitBuilder::builder().all_build().emit()?;
/// #   Ok(())
/// # }
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "build")))]
impl EmitBuilder {
    /// Enable all of the `VERGEN_BUILD_*` options
    pub fn all_build(&mut self) -> &mut Self {
        self.build_date().build_timestamp()
    }

    /// Enable the `VERGEN_BUILD_DATE` date output
    pub fn build_date(&mut self) -> &mut Self {
        self.build_config.build_date = true;
        self
    }

    /// Enable the `VERGEN_BUILD_TIMESTAMP` date output
    pub fn build_timestamp(&mut self) -> &mut Self {
        self.build_config.build_timestamp = true;
        self
    }

    pub(crate) fn add_build_default(
        &self,
        e: Error,
        fail_on_error: bool,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
    ) -> Result<()> {
        if fail_on_error {
            Err(e)
        } else {
            if self.build_config.build_date {
                add_default_map_entry(VergenKey::BuildDate, map, warnings);
            }
            if self.build_config.build_timestamp {
                add_default_map_entry(VergenKey::BuildTimestamp, map, warnings);
            }
            Ok(())
        }
    }

    pub(crate) fn add_build_map_entries(
        &self,
        idempotent: bool,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
    ) -> Result<()> {
        self.add_timestamp_entries(idempotent, map, warnings)?;
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
                add_default_map_entry(VergenKey::BuildDate, map, warnings);
            } else {
                let format = format_description::parse("[year]-[month]-[day]")?;
                add_map_entry(VergenKey::BuildDate, ts.format(&format)?, map);
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
        if self.build_config.build_timestamp {
            if idempotent && !source_date_epoch {
                add_default_map_entry(VergenKey::BuildTimestamp, map, warnings);
            } else {
                add_map_entry(VergenKey::BuildTimestamp, ts.format(&Rfc3339)?, map);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{emitter::test::count_idempotent, EmitBuilder};
    use anyhow::Result;
    use std::env;

    #[test]
    #[serial_test::parallel]
    fn build_all_idempotent() -> Result<()> {
        let config = EmitBuilder::builder()
            .idempotent()
            .all_build()
            .test_emit()?;
        assert_eq!(2, config.cargo_rustc_env_map.len());
        assert_eq!(2, count_idempotent(config.cargo_rustc_env_map));
        assert_eq!(2, config.warnings.len());
        Ok(())
    }

    #[test]
    #[serial_test::parallel]
    fn build_all() -> Result<()> {
        let config = EmitBuilder::builder().all_build().test_emit()?;
        assert_eq!(2, config.cargo_rustc_env_map.len());
        assert_eq!(0, count_idempotent(config.cargo_rustc_env_map));
        assert_eq!(0, config.warnings.len());
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn source_date_epoch_works() -> Result<()> {
        env::set_var("SOURCE_DATE_EPOCH", "1671809360");
        let mut stdout_buf = vec![];
        EmitBuilder::builder()
            .idempotent()
            .all_build()
            .emit_to(&mut stdout_buf)?;
        let output = String::from_utf8_lossy(&stdout_buf);
        for (idx, line) in output.lines().enumerate() {
            if idx == 0 {
                assert_eq!("cargo:rustc-env=VERGEN_BUILD_DATE=2022-12-23", line);
            } else if idx == 1 {
                assert_eq!(
                    "cargo:rustc-env=VERGEN_BUILD_TIMESTAMP=2022-12-23T15:29:20Z",
                    line
                );
            }
        }
        env::remove_var("SOURCE_DATE_EPOCH");
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    #[cfg(unix)]
    fn bad_source_date_epoch_fails() -> Result<()> {
        use std::ffi::OsStr;
        use std::os::unix::prelude::OsStrExt;

        let source = [0x66, 0x6f, 0x80, 0x6f];
        let os_str = OsStr::from_bytes(&source[..]);
        env::set_var("SOURCE_DATE_EPOCH", os_str);

        let mut stdout_buf = vec![];
        assert!(EmitBuilder::builder()
            .idempotent()
            .fail_on_error()
            .all_build()
            .emit_to(&mut stdout_buf)
            .is_err());
        env::remove_var("SOURCE_DATE_EPOCH");
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    #[cfg(unix)]
    fn bad_source_date_epoch_defaults() -> Result<()> {
        use std::ffi::OsStr;
        use std::os::unix::prelude::OsStrExt;

        let source = [0x66, 0x6f, 0x80, 0x6f];
        let os_str = OsStr::from_bytes(&source[..]);
        env::set_var("SOURCE_DATE_EPOCH", os_str);

        let mut stdout_buf = vec![];
        assert!(EmitBuilder::builder()
            .idempotent()
            .all_build()
            .emit_to(&mut stdout_buf)
            .is_ok());
        env::remove_var("SOURCE_DATE_EPOCH");
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    #[cfg(windows)]
    fn bad_source_date_epoch_fails() -> Result<()> {
        use std::ffi::OsString;
        use std::os::windows::prelude::OsStringExt;

        let source = [0x0066, 0x006f, 0xD800, 0x006f];
        let os_string = OsString::from_wide(&source[..]);
        let os_str = os_string.as_os_str();
        env::set_var("SOURCE_DATE_EPOCH", os_str);

        let mut stdout_buf = vec![];
        assert!(EmitBuilder::builder()
            .idempotent()
            .fail_on_error()
            .all_build()
            .emit_to(&mut stdout_buf)
            .is_err());
        env::remove_var("SOURCE_DATE_EPOCH");
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    #[cfg(windows)]
    fn bad_source_date_epoch_defaults() -> Result<()> {
        use std::ffi::OsString;
        use std::os::windows::prelude::OsStringExt;

        let source = [0x0066, 0x006f, 0xD800, 0x006f];
        let os_string = OsString::from_wide(&source[..]);
        let os_str = os_string.as_os_str();
        env::set_var("SOURCE_DATE_EPOCH", os_str);

        let mut stdout_buf = vec![];
        assert!(EmitBuilder::builder()
            .idempotent()
            .all_build()
            .emit_to(&mut stdout_buf)
            .is_ok());
        env::remove_var("SOURCE_DATE_EPOCH");
        Ok(())
    }
}
