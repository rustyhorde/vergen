// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use anyhow::{Context, Error, Result};
use derive_builder::Builder as DeriveBuilder;
use std::{
    env::{self, VarError},
    str::FromStr,
};
use time::{
    format_description::{self, well_known::Iso8601},
    OffsetDateTime,
};
use vergen_lib::{
    add_default_map_entry, add_map_entry,
    constants::{BUILD_DATE_NAME, BUILD_TIMESTAMP_NAME},
    AddEntries, CargoRerunIfChanged, CargoRustcEnvMap, CargoWarning, DefaultConfig, VergenKey,
};

/// The `VERGEN_BUILD_*` configuration features
///
/// | Variable | Sample |
/// | -------  | ------ |
/// | `VERGEN_BUILD_DATE` | 2021-02-25 |
/// | `VERGEN_BUILD_TIMESTAMP` | 2021-02-25T23:28:39.493201+00:00 |
///
/// # Example
/// Emit all of the build instructions
///
/// ```
/// # use anyhow::Result;
/// # use vergen::Emitter;
/// # use vergen::BuildBuilder;
/// #
/// # fn main() -> Result<()> {
/// let build = BuildBuilder::all_build()?;
/// Emitter::new().add_instructions(&build)?.emit()?;
/// #     Ok(())
/// # }
/// ```
///
/// Emit some of the build instructions
///
/// ```
/// # use anyhow::Result;
/// # use vergen::Emitter;
/// # use vergen::BuildBuilder;
/// #
/// # fn main() -> Result<()> {
/// let build = BuildBuilder::default().build_timestamp(true).build()?;
/// Emitter::new().add_instructions(&build)?.emit()?;
/// #     Ok(())
/// # }
/// ```
///
/// Override output with your own value
///
/// ```
/// # use anyhow::Result;
/// # use std::env;
/// # use vergen::Emitter;
/// # use vergen::BuildBuilder;
/// #
/// # fn main() -> Result<()> {
/// temp_env::with_var("VERGEN_BUILD_DATE", Some("01/01/2023"), || {
///     let result = || -> Result<()> {
///         let build = BuildBuilder::default().build_date(true).build()?;
///         Emitter::new().add_instructions(&build)?.emit()?;
///         Ok(())
///     }();
///     assert!(result.is_ok());
/// });
/// #     Ok(())
/// # }
/// ```
///
/// # Example
/// This feature can also be used in conjuction with the [`SOURCE_DATE_EPOCH`](https://reproducible-builds.org/docs/source-date-epoch/)
/// environment variable to generate deterministic timestamps based off the
/// last modification time of the source/package
///
/// ```
/// # use anyhow::Result;
/// # use std::env;
/// # use vergen::Emitter;
/// # use vergen::BuildBuilder;
/// #
/// # fn main() -> Result<()> {
/// temp_env::with_var("SOURCE_DATE_EPOCH", Some("1671809360"), || {
///     let result = || -> Result<()> {
///         let build = BuildBuilder::all_build()?;
///         Emitter::new().add_instructions(&build)?.emit()?;
///         Ok(())
///     }();
/// });
/// #   Ok(())
/// # }
/// ```
///
/// The above will always generate the following output for the timestamp
/// related instructions
///
/// ```text
/// cargo:rustc-env=VERGEN_BUILD_DATE=2022-12-23
/// cargo:rustc-env=VERGEN_BUILD_TIMESTAMP=2022-12-23T15:29:20.000000000Z
/// ```
///
/// # Example
/// This feature also recognizes the idempotent flag.
///
/// **NOTE** - `SOURCE_DATE_EPOCH` takes precedence over the idempotent flag. If you
/// use both, the output will be based off `SOURCE_DATE_EPOCH`.  This would still be
/// deterministic.
///
/// ```
/// # use anyhow::Result;
/// # use vergen::Emitter;
/// # use vergen::BuildBuilder;
/// #
/// # fn main() -> Result<()> {
/// let build = BuildBuilder::default().build()?;
/// Emitter::new().idempotent().add_instructions(&build)?.emit()?;
/// #   Ok(())
/// # }
/// ```
///
/// The above will always generate the following output for the timestamp
/// related instructions unless you also use quiet, then the warnings will
/// be suppressed.
///
/// ```text
/// cargo:rustc-env=VERGEN_BUILD_DATE=VERGEN_IDEMPOTENT_OUTPUT
/// cargo:rustc-env=VERGEN_BUILD_TIMESTAMP=VERGEN_IDEMPOTENT_OUTPUT
/// cargo:warning=VERGEN_BUILD_DATE set to default
/// cargo:warning=VERGEN_BUILD_TIMESTAMP set to default
/// cargo:rerun-if-changed=build.rs
/// cargo:rerun-if-env-changed=VERGEN_IDEMPOTENT
/// cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH
/// ```
///
#[derive(Clone, Copy, Debug, DeriveBuilder, PartialEq)]
#[allow(clippy::struct_field_names)]
pub struct Build {
    /// Enable the `VERGEN_BUILD_DATE` date output
    #[builder(default = "false")]
    build_date: bool,
    /// Enable the `VERGEN_BUILD_TIMESTAMP` date output
    #[builder(default = "false")]
    build_timestamp: bool,
    /// Enable local offset date/timestamp output
    #[builder(default = "false")]
    use_local: bool,
}

impl BuildBuilder {
    /// Enable all of the `VERGEN_BUILD_*` options
    ///
    /// # Errors
    /// The build function can error
    ///
    pub fn all_build() -> Result<Build> {
        Self::default()
            .build_date(true)
            .build_timestamp(true)
            .build()
            .map_err(Into::into)
    }
}

impl Build {
    fn any(self) -> bool {
        self.build_date || self.build_timestamp
    }

    fn add_timestamp_entries(
        self,
        idempotent: bool,
        cargo_rustc_env: &mut CargoRustcEnvMap,
        cargo_warning: &mut CargoWarning,
    ) -> Result<()> {
        let (sde, ts) = match env::var("SOURCE_DATE_EPOCH") {
            Ok(v) => (
                true,
                OffsetDateTime::from_unix_timestamp(i64::from_str(&v)?)?,
            ),
            Err(VarError::NotPresent) => {
                if self.use_local {
                    (false, OffsetDateTime::now_local()?)
                } else {
                    (false, OffsetDateTime::now_utc())
                }
            }
            Err(e) => return Err(e.into()),
        };

        self.add_date_entry(idempotent, sde, &ts, cargo_rustc_env, cargo_warning)?;
        self.add_timestamp_entry(idempotent, sde, &ts, cargo_rustc_env, cargo_warning)?;
        Ok(())
    }

    fn add_date_entry(
        self,
        idempotent: bool,
        source_date_epoch: bool,
        ts: &OffsetDateTime,
        cargo_rustc_env: &mut CargoRustcEnvMap,
        cargo_warning: &mut CargoWarning,
    ) -> Result<()> {
        if self.build_date {
            if let Ok(value) = env::var(BUILD_DATE_NAME) {
                add_map_entry(VergenKey::BuildDate, value, cargo_rustc_env);
            } else if idempotent && !source_date_epoch {
                add_default_map_entry(VergenKey::BuildDate, cargo_rustc_env, cargo_warning);
            } else {
                let format = format_description::parse("[year]-[month]-[day]")?;
                add_map_entry(VergenKey::BuildDate, ts.format(&format)?, cargo_rustc_env);
            }
        }
        Ok(())
    }

    fn add_timestamp_entry(
        self,
        idempotent: bool,
        source_date_epoch: bool,
        ts: &OffsetDateTime,
        cargo_rustc_env: &mut CargoRustcEnvMap,
        cargo_warning: &mut CargoWarning,
    ) -> Result<()> {
        if self.build_timestamp {
            if let Ok(value) = env::var(BUILD_TIMESTAMP_NAME) {
                add_map_entry(VergenKey::BuildTimestamp, value, cargo_rustc_env);
            } else if idempotent && !source_date_epoch {
                add_default_map_entry(VergenKey::BuildTimestamp, cargo_rustc_env, cargo_warning);
            } else {
                add_map_entry(
                    VergenKey::BuildTimestamp,
                    ts.format(&Iso8601::DEFAULT)?,
                    cargo_rustc_env,
                );
            }
        }
        Ok(())
    }
}

impl AddEntries for Build {
    fn add_map_entries(
        &self,
        idempotent: bool,
        cargo_rustc_env: &mut CargoRustcEnvMap,
        _cargo_rerun_if_changed: &mut CargoRerunIfChanged,
        cargo_warning: &mut CargoWarning,
    ) -> Result<()> {
        if self.any() {
            self.add_timestamp_entries(idempotent, cargo_rustc_env, cargo_warning)
                .with_context(|| "Error adding build timestamp entries")?;
        }
        Ok(())
    }

    fn add_default_entries(
        &self,
        config: &DefaultConfig,
        cargo_rustc_env_map: &mut CargoRustcEnvMap,
        _cargo_rerun_if_changed: &mut CargoRerunIfChanged,
        cargo_warning: &mut CargoWarning,
    ) -> Result<()> {
        if *config.fail_on_error() {
            let error = Error::msg(format!("{:?}", config.error()));
            Err(error)
        } else {
            if self.build_date {
                add_default_map_entry(VergenKey::BuildDate, cargo_rustc_env_map, cargo_warning);
            }
            if self.build_timestamp {
                add_default_map_entry(
                    VergenKey::BuildTimestamp,
                    cargo_rustc_env_map,
                    cargo_warning,
                );
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::BuildBuilder;
    use crate::Emitter;
    use anyhow::Result;
    use serial_test::serial;
    use std::io::Write;
    use vergen_lib::{count_idempotent, CustomInsGen};

    #[test]
    #[serial]
    #[allow(clippy::clone_on_copy, clippy::redundant_clone)]
    fn builder_clone() -> Result<()> {
        let build = BuildBuilder::all_build()?;
        let another = build.clone();
        assert_eq!(another, build);
        Ok(())
    }

    #[test]
    #[serial]
    #[allow(clippy::clone_on_copy, clippy::redundant_clone)]
    fn build_clone() -> Result<()> {
        let build = BuildBuilder::all_build()?;
        let another = build.clone();
        assert_eq!(another, build);
        Ok(())
    }

    #[test]
    #[serial]
    fn builder_debug() -> Result<()> {
        let builder = BuildBuilder::all_build()?;
        let mut buf = vec![];
        write!(buf, "{builder:?}")?;
        assert!(!buf.is_empty());
        Ok(())
    }

    #[test]
    #[serial]
    fn build_debug() -> Result<()> {
        let build = BuildBuilder::all_build()?;
        let mut buf = vec![];
        write!(buf, "{build:?}")?;
        assert!(!buf.is_empty());
        Ok(())
    }

    #[test]
    #[serial]
    fn build_default() -> Result<()> {
        let build = BuildBuilder::default().build()?;
        let emitter = Emitter::default().add_instructions(&build)?.test_emit();
        assert_eq!(0, emitter.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(0, emitter.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn build_all_idempotent() -> Result<()> {
        let build = BuildBuilder::all_build()?;
        let emitter = Emitter::new()
            .idempotent()
            .add_instructions(&build)?
            .test_emit();
        assert_eq!(2, emitter.cargo_rustc_env_map().len());
        assert_eq!(2, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(2, emitter.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn build_all() -> Result<()> {
        let build = BuildBuilder::all_build()?;
        let emitter = Emitter::new().add_instructions(&build)?.test_emit();
        assert_eq!(2, emitter.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(0, emitter.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn build_date() -> Result<()> {
        let build = BuildBuilder::default().build_date(true).build()?;
        let emitter = Emitter::new().add_instructions(&build)?.test_emit();
        assert_eq!(1, emitter.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(0, emitter.cargo_warning().len());
        Ok(())
    }

    #[cfg(any(unix, target_os = "macos"))]
    #[test]
    #[serial]
    fn build_date_local() -> Result<()> {
        // use local is unsound on nix
        let build = BuildBuilder::default()
            .build_date(true)
            .use_local(true)
            .build()?;
        let emitter = Emitter::new().add_instructions(&build)?.test_emit();
        assert_eq!(1, emitter.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(0, emitter.cargo_warning().len());
        Ok(())
    }

    #[cfg(windows)]
    #[test]
    #[serial]
    fn build_date_local() -> Result<()> {
        let build = BuildBuilder::default()
            .build_date(true)
            .use_local(true)
            .build()?;
        let emitter = Emitter::new().add_instructions(&build)?.test_emit();
        assert_eq!(1, emitter.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(0, emitter.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn build_timestamp() -> Result<()> {
        let build = BuildBuilder::default().build_timestamp(true).build()?;
        let emitter = Emitter::new().add_instructions(&build)?.test_emit();
        assert_eq!(1, emitter.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(0, emitter.cargo_warning().len());
        Ok(())
    }

    #[cfg(any(unix, target_os = "macos"))]
    #[test]
    #[serial]
    fn build_timestamp_local() -> Result<()> {
        // use local is unsound on nix
        let build = BuildBuilder::default()
            .build_timestamp(true)
            .use_local(true)
            .build()?;
        let emitter = Emitter::new().add_instructions(&build)?.test_emit();
        assert_eq!(1, emitter.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(0, emitter.cargo_warning().len());
        Ok(())
    }

    #[cfg(windows)]
    #[test]
    #[serial]
    fn build_timestamp_local() -> Result<()> {
        let build = BuildBuilder::default()
            .build_timestamp(true)
            .use_local(true)
            .build()?;
        let emitter = Emitter::new().add_instructions(&build)?.test_emit();
        assert_eq!(1, emitter.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(0, emitter.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn source_date_epoch_works() {
        temp_env::with_var("SOURCE_DATE_EPOCH", Some("1671809360"), || {
            let result = || -> Result<()> {
                let mut stdout_buf = vec![];
                let build = BuildBuilder::all_build()?;
                _ = Emitter::new()
                    .idempotent()
                    .add_instructions(&build)?
                    .emit_to(&mut stdout_buf)?;
                let output = String::from_utf8_lossy(&stdout_buf);
                for (idx, line) in output.lines().enumerate() {
                    if idx == 0 {
                        assert_eq!("cargo:rustc-env=VERGEN_BUILD_DATE=2022-12-23", line);
                    } else if idx == 1 {
                        assert_eq!(
                            "cargo:rustc-env=VERGEN_BUILD_TIMESTAMP=2022-12-23T15:29:20.000000000Z",
                            line
                        );
                    }
                }
                Ok(())
            }();
            assert!(result.is_ok());
        });
    }

    #[test]
    #[serial]
    #[cfg(unix)]
    fn bad_source_date_epoch_fails() {
        use std::ffi::OsStr;
        use std::os::unix::prelude::OsStrExt;

        let source = [0x66, 0x6f, 0x80, 0x6f];
        let os_str = OsStr::from_bytes(&source[..]);
        temp_env::with_var("SOURCE_DATE_EPOCH", Some(os_str), || {
            let result = || -> Result<bool> {
                let mut stdout_buf = vec![];
                let build = BuildBuilder::all_build()?;
                Emitter::new()
                    .idempotent()
                    .fail_on_error()
                    .add_instructions(&build)?
                    .emit_to(&mut stdout_buf)
            }();
            assert!(result.is_err());
        });
    }

    #[test]
    #[serial]
    #[cfg(unix)]
    fn bad_source_date_epoch_defaults() {
        use std::ffi::OsStr;
        use std::os::unix::prelude::OsStrExt;

        let source = [0x66, 0x6f, 0x80, 0x6f];
        let os_str = OsStr::from_bytes(&source[..]);
        temp_env::with_var("SOURCE_DATE_EPOCH", Some(os_str), || {
            let result = || -> Result<bool> {
                let mut stdout_buf = vec![];
                let build = BuildBuilder::all_build()?;
                Emitter::new()
                    .idempotent()
                    .add_instructions(&build)?
                    .emit_to(&mut stdout_buf)
            }();
            assert!(result.is_ok());
        });
    }

    #[test]
    #[serial]
    #[cfg(windows)]
    fn bad_source_date_epoch_fails() {
        use std::{ffi::OsString, os::windows::prelude::OsStringExt};

        let source = [0x0066, 0x006f, 0xD800, 0x006f];
        let os_string = OsString::from_wide(&source[..]);
        let os_str = os_string.as_os_str();
        temp_env::with_var("SOURCE_DATE_EPOCH", Some(os_str), || {
            let result = || -> Result<bool> {
                let mut stdout_buf = vec![];
                let build = BuildBuilder::all_build()?;
                Emitter::new()
                    .fail_on_error()
                    .idempotent()
                    .add_instructions(&build)?
                    .emit_to(&mut stdout_buf)
            }();
            assert!(result.is_err());
        });
    }

    #[test]
    #[serial]
    #[cfg(windows)]
    fn bad_source_date_epoch_defaults() {
        use std::{ffi::OsString, os::windows::prelude::OsStringExt};

        let source = [0x0066, 0x006f, 0xD800, 0x006f];
        let os_string = OsString::from_wide(&source[..]);
        let os_str = os_string.as_os_str();
        temp_env::with_var("SOURCE_DATE_EPOCH", Some(os_str), || {
            let result = || -> Result<bool> {
                let mut stdout_buf = vec![];
                let build = BuildBuilder::all_build()?;
                Emitter::new()
                    .idempotent()
                    .add_instructions(&build)?
                    .emit_to(&mut stdout_buf)
            }();
            assert!(result.is_ok());
        });
    }

    #[test]
    #[serial]
    fn build_date_override_works() {
        temp_env::with_var("VERGEN_BUILD_DATE", Some("this is a bad date"), || {
            let result = || -> Result<()> {
                let mut stdout_buf = vec![];
                let build = BuildBuilder::all_build()?;
                assert!(Emitter::default()
                    .add_instructions(&build)?
                    .emit_to(&mut stdout_buf)
                    .is_ok());
                let output = String::from_utf8_lossy(&stdout_buf);
                assert!(output.contains("cargo:rustc-env=VERGEN_BUILD_DATE=this is a bad date"));
                Ok(())
            }();
            assert!(result.is_ok());
        });
    }

    #[test]
    #[serial]
    fn build_timestamp_override_works() {
        temp_env::with_var(
            "VERGEN_BUILD_TIMESTAMP",
            Some("this is a bad timestamp"),
            || {
                let result = || -> Result<()> {
                    let mut stdout_buf = vec![];
                    let build = BuildBuilder::all_build()?;
                    assert!(Emitter::default()
                        .add_instructions(&build)?
                        .emit_to(&mut stdout_buf)
                        .is_ok());
                    let output = String::from_utf8_lossy(&stdout_buf);
                    assert!(output.contains(
                        "cargo:rustc-env=VERGEN_BUILD_TIMESTAMP=this is a bad timestamp"
                    ));
                    Ok(())
                }();
                assert!(result.is_ok());
            },
        );
    }

    #[test]
    #[serial]
    fn custom_emit_works() -> Result<()> {
        let cust_gen = CustomInsGen::default();
        let build = BuildBuilder::all_build()?;
        let emitter = Emitter::default()
            .add_instructions(&build)?
            .add_custom_instructions(&cust_gen)?
            .test_emit();
        assert_eq!(2, emitter.cargo_rustc_env_map().len());
        assert_eq!(1, emitter.cargo_rustc_env_map_custom().len());
        assert_eq!(0, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(0, emitter.cargo_warning().len());
        Ok(())
    }
}
