// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use anyhow::{Error, Result};
use derive_builder::Builder as DeriveBuilder;
use rustc_version::{version_meta, Channel, VersionMeta};
use std::env;
use vergen_lib::{
    add_default_map_entry, add_map_entry,
    constants::{
        RUSTC_CHANNEL_NAME, RUSTC_COMMIT_DATE, RUSTC_COMMIT_HASH, RUSTC_HOST_TRIPLE_NAME,
        RUSTC_LLVM_VERSION, RUSTC_SEMVER_NAME,
    },
    AddEntries, CargoRerunIfChanged, CargoRustcEnvMap, CargoWarning, DefaultConfig, VergenKey,
};

/// The `VERGEN_RUSTC_*` configuration features
///
/// **NOTE** - All rustc instructions are considered deterministic.  If you change
/// the version of rustc you are compiling with, these values should change if
/// being used in the generated binary.
///
/// | Variable | Sample |
/// | -------  | ------ |
/// | `VERGEN_RUSTC_CHANNEL` | nightly |
/// | `VERGEN_RUSTC_COMMIT_DATE` | 2021-02-24 |
/// | `VERGEN_RUSTC_COMMIT_HASH` | a8486b64b0c87dabd045453b6c81500015d122d6 |
/// | `VERGEN_RUSTC_HOST_TRIPLE` | apple-darwin |
/// | `VERGEN_RUSTC_LLVM_VERSION` | 11.0 |
/// | `VERGEN_RUSTC_SEMVER` | 1.52.0-nightly |
///
/// # Example
/// Emit all of the rustc instructions
///
/// ```
/// # use anyhow::Result;
/// # use vergen::Emitter;
/// # use vergen::RustcBuilder;
/// #
/// # fn main() -> Result<()> {
/// let rustc = RustcBuilder::all_rustc()?;
/// Emitter::default().add_instructions(&rustc)?.emit();
/// #   Ok(())
/// # }
/// ```
///
/// Emit some of the rustc instructions
///
/// ```
/// # use anyhow::Result;
/// # use vergen::Emitter;
/// # use vergen::RustcBuilder;
/// #
/// # fn main() -> Result<()> {
/// let rustc = RustcBuilder::default().channel(true).semver(true).build()?;
/// Emitter::default().add_instructions(&rustc)?.emit();
/// #   Ok(())
/// # }
/// ```
///
/// Override output with your own value
///
/// ```
/// # use anyhow::Result;
/// # use std::env;
/// # use vergen::Emitter;
/// # use vergen::RustcBuilder;
/// #
/// # fn main() -> Result<()> {
/// temp_env::with_var("VERGEN_RUSTC_CHANNEL", Some("this is the channel I want output"), || {
///     let result = || -> Result<()> {
///         let rustc = RustcBuilder::default().channel(true).semver(true).build()?;
///         Emitter::default().add_instructions(&rustc)?.emit();   
///         Ok(())  
///     }();
///     assert!(result.is_ok());
/// });
/// #   Ok(())
/// # }
/// ```
///
#[derive(Clone, Copy, Debug, DeriveBuilder, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub struct Rustc {
    /// Enable the rustc channel
    #[builder(default = "false")]
    channel: bool,
    /// Enable the rustc commit date
    #[builder(default = "false")]
    commit_date: bool,
    /// Enable the rustc SHA
    #[builder(default = "false")]
    commit_hash: bool,
    /// Enable rustc host triple
    #[builder(default = "false")]
    host_triple: bool,
    /// Enable rustc LLVM version
    #[builder(default = "false")]
    llvm_version: bool,
    /// Enable the rustc semver
    #[builder(default = "false")]
    semver: bool,
    #[cfg(test)]
    #[builder(default = "None", setter(skip))]
    str_to_test: Option<&'static str>,
}

impl RustcBuilder {
    /// Enable all of the `VERGEN_RUSTC_*` options
    ///
    /// # Errors
    /// The underlying build function can error
    ///
    pub fn all_rustc() -> Result<Rustc> {
        Self::default()
            .channel(true)
            .commit_date(true)
            .commit_hash(true)
            .host_triple(true)
            .llvm_version(true)
            .semver(true)
            .build()
            .map_err(Into::into)
    }
}

impl Rustc {
    fn any(self) -> bool {
        self.channel
            || self.commit_date
            || self.commit_hash
            || self.host_triple
            || self.llvm_version
            || self.semver
    }

    #[cfg(not(test))]
    fn add_rustc_map_entries(
        self,
        cargo_rustc_env: &mut CargoRustcEnvMap,
        cargo_warning: &mut CargoWarning,
    ) -> Result<()> {
        self.add_rustc_to_map(version_meta(), cargo_rustc_env, cargo_warning)
    }

    #[cfg(test)]
    fn add_rustc_map_entries(
        self,
        cargo_rustc_env: &mut CargoRustcEnvMap,
        cargo_warning: &mut CargoWarning,
    ) -> Result<()> {
        use rustc_version::version_meta_for;

        let vm = if let Some(rustc_str) = self.str_to_test {
            version_meta_for(rustc_str)
        } else {
            version_meta()
        };
        self.add_rustc_to_map(vm, cargo_rustc_env, cargo_warning)
    }

    fn add_rustc_to_map(
        self,
        rustc_res: std::result::Result<VersionMeta, rustc_version::Error>,
        cargo_rustc_env: &mut CargoRustcEnvMap,
        cargo_warning: &mut CargoWarning,
    ) -> Result<()> {
        let rustc = rustc_res?;

        if self.channel {
            if let Ok(_value) = env::var(RUSTC_CHANNEL_NAME) {
                add_default_map_entry(VergenKey::RustcChannel, cargo_rustc_env, cargo_warning);
            } else {
                let channel = match rustc.channel {
                    Channel::Dev => "dev",
                    Channel::Nightly => "nightly",
                    Channel::Beta => "beta",
                    Channel::Stable => "stable",
                };
                add_map_entry(VergenKey::RustcChannel, channel, cargo_rustc_env);
            }
        }

        if self.commit_date {
            if let Ok(_value) = env::var(RUSTC_COMMIT_DATE) {
                add_default_map_entry(VergenKey::RustcCommitDate, cargo_rustc_env, cargo_warning);
            } else if let Some(commit_date) = rustc.commit_date {
                add_map_entry(VergenKey::RustcCommitDate, commit_date, cargo_rustc_env);
            } else {
                add_default_map_entry(VergenKey::RustcCommitDate, cargo_rustc_env, cargo_warning);
            }
        }

        if self.commit_hash {
            if let Ok(_value) = env::var(RUSTC_COMMIT_HASH) {
                add_default_map_entry(VergenKey::RustcCommitHash, cargo_rustc_env, cargo_warning);
            } else if let Some(commit_hash) = rustc.commit_hash {
                add_map_entry(VergenKey::RustcCommitHash, commit_hash, cargo_rustc_env);
            } else {
                add_default_map_entry(VergenKey::RustcCommitHash, cargo_rustc_env, cargo_warning);
            }
        }

        if self.host_triple {
            if let Ok(_value) = env::var(RUSTC_HOST_TRIPLE_NAME) {
                add_default_map_entry(VergenKey::RustcHostTriple, cargo_rustc_env, cargo_warning);
            } else {
                add_map_entry(VergenKey::RustcHostTriple, rustc.host, cargo_rustc_env);
            }
        }

        if self.llvm_version {
            if let Ok(_value) = env::var(RUSTC_LLVM_VERSION) {
                add_default_map_entry(VergenKey::RustcLlvmVersion, cargo_rustc_env, cargo_warning);
            } else if let Some(llvm_version) = rustc.llvm_version {
                add_map_entry(
                    VergenKey::RustcLlvmVersion,
                    format!("{llvm_version}"),
                    cargo_rustc_env,
                );
            } else {
                add_default_map_entry(VergenKey::RustcLlvmVersion, cargo_rustc_env, cargo_warning);
            }
        }

        if self.semver {
            if let Ok(_value) = env::var(RUSTC_SEMVER_NAME) {
                add_default_map_entry(VergenKey::RustcSemver, cargo_rustc_env, cargo_warning);
            } else {
                add_map_entry(
                    VergenKey::RustcSemver,
                    format!("{}", rustc.semver),
                    cargo_rustc_env,
                );
            }
        }

        Ok(())
    }

    #[cfg(test)]
    fn with_rustc_str(&mut self, rustc_str: &'static str) -> &mut Self {
        self.str_to_test = Some(rustc_str);
        self
    }
}

impl AddEntries for Rustc {
    fn add_map_entries(
        &self,
        _idempotent: bool,
        cargo_rustc_env: &mut CargoRustcEnvMap,
        _cargo_rerun_if_changed: &mut CargoRerunIfChanged,
        cargo_warning: &mut CargoWarning,
    ) -> Result<()> {
        if self.any() {
            self.add_rustc_map_entries(cargo_rustc_env, cargo_warning)
        } else {
            Ok(())
        }
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
            if self.channel {
                add_default_map_entry(VergenKey::RustcChannel, cargo_rustc_env_map, cargo_warning);
            }
            if self.commit_date {
                add_default_map_entry(
                    VergenKey::RustcCommitDate,
                    cargo_rustc_env_map,
                    cargo_warning,
                );
            }
            if self.commit_hash {
                add_default_map_entry(
                    VergenKey::RustcCommitHash,
                    cargo_rustc_env_map,
                    cargo_warning,
                );
            }
            if self.host_triple {
                add_default_map_entry(
                    VergenKey::RustcHostTriple,
                    cargo_rustc_env_map,
                    cargo_warning,
                );
            }
            if self.llvm_version {
                add_default_map_entry(
                    VergenKey::RustcLlvmVersion,
                    cargo_rustc_env_map,
                    cargo_warning,
                );
            }
            if self.semver {
                add_default_map_entry(VergenKey::RustcSemver, cargo_rustc_env_map, cargo_warning);
            }

            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::RustcBuilder;
    use crate::Emitter;
    use anyhow::Result;
    use serial_test::serial;
    use std::io::Write;
    use temp_env::with_var;
    use vergen_lib::count_idempotent;

    #[test]
    #[serial]
    #[allow(clippy::clone_on_copy, clippy::redundant_clone)]
    fn rustc_clone_works() -> Result<()> {
        let rustc = RustcBuilder::all_rustc()?;
        let another = rustc.clone();
        assert_eq!(another, rustc);
        Ok(())
    }

    #[test]
    #[serial]
    fn rustc_debug_works() -> Result<()> {
        let rustc = RustcBuilder::all_rustc()?;
        let mut buf = vec![];
        write!(buf, "{rustc:?}")?;
        assert!(!buf.is_empty());
        Ok(())
    }

    #[test]
    #[serial]
    fn rustc_default() -> Result<()> {
        let rustc = RustcBuilder::default().build()?;
        let emitter = Emitter::default().add_instructions(&rustc)?.test_emit();
        assert_eq!(0, emitter.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(0, emitter.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn rustc_all_idempotent() -> Result<()> {
        let rustc = RustcBuilder::all_rustc()?;
        let config = Emitter::default()
            .idempotent()
            .add_instructions(&rustc)?
            .test_emit();
        assert_eq!(6, config.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(config.cargo_rustc_env_map()));
        assert_eq!(0, config.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn rustc_all() -> Result<()> {
        let rustc = RustcBuilder::all_rustc()?;
        let config = Emitter::default().add_instructions(&rustc)?.test_emit();
        assert_eq!(6, config.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(config.cargo_rustc_env_map()));
        assert_eq!(0, config.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn rustc_commit_date() -> Result<()> {
        let rustc = RustcBuilder::default().commit_date(true).build()?;
        let config = Emitter::default().add_instructions(&rustc)?.test_emit();
        assert_eq!(1, config.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(config.cargo_rustc_env_map()));
        assert_eq!(0, config.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn rustc_commit_hash() -> Result<()> {
        let rustc = RustcBuilder::default().commit_hash(true).build()?;
        let config = Emitter::default().add_instructions(&rustc)?.test_emit();
        assert_eq!(1, config.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(config.cargo_rustc_env_map()));
        assert_eq!(0, config.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn rustc_host_triple() -> Result<()> {
        let rustc = RustcBuilder::default().host_triple(true).build()?;
        let config = Emitter::default().add_instructions(&rustc)?.test_emit();
        assert_eq!(1, config.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(config.cargo_rustc_env_map()));
        assert_eq!(0, config.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn rustc_llvm_version() -> Result<()> {
        let rustc = RustcBuilder::default().llvm_version(true).build()?;
        let config = Emitter::default().add_instructions(&rustc)?.test_emit();
        assert_eq!(1, config.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(config.cargo_rustc_env_map()));
        assert_eq!(0, config.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn rustc_semver() -> Result<()> {
        let rustc = RustcBuilder::default().semver(true).build()?;
        let config = Emitter::default().add_instructions(&rustc)?.test_emit();
        assert_eq!(1, config.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(config.cargo_rustc_env_map()));
        assert_eq!(0, config.cargo_warning().len());
        Ok(())
    }

    const NO_LLVM: &str = r"rustc 1.68.0-nightly (270c94e48 2022-12-28)
binary: rustc
commit-hash: 270c94e484e19764a2832ef918c95224eb3f17c7
commit-date: 2022-12-28
host: x86_64-unknown-linux-gnu
release: 1.68.0-nightly
    ";

    #[test]
    #[serial]
    fn no_llvm_in_rustc() -> Result<()> {
        let mut rustc = RustcBuilder::all_rustc()?;
        let _ = rustc.with_rustc_str(NO_LLVM);
        let emitter = Emitter::default()
            .fail_on_error()
            .add_instructions(&rustc)?
            .test_emit();
        assert_eq!(6, emitter.cargo_rustc_env_map().len());
        assert_eq!(1, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(1, emitter.cargo_warning().len());
        Ok(())
    }

    const DEV_BUILD: &str = r"rustc 1.68.0-nightly (270c94e48 2022-12-28)
binary: rustc
commit-hash: 270c94e484e19764a2832ef918c95224eb3f17c7
commit-date: 2022-12-28
host: x86_64-unknown-linux-gnu
release: 1.68.0-dev
LLVM version: 15.0.6
    ";

    #[test]
    #[serial]
    fn rustc_dev_build() -> Result<()> {
        let mut rustc = RustcBuilder::all_rustc()?;
        let _ = rustc.with_rustc_str(DEV_BUILD);
        let emitter = Emitter::default()
            .fail_on_error()
            .add_instructions(&rustc)?
            .test_emit();
        assert_eq!(6, emitter.cargo_rustc_env_map().len());
        assert_eq!(0, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(0, emitter.cargo_warning().len());
        Ok(())
    }

    const UNKNOWN_BITS: &str = r"rustc 1.68.0-nightly (270c94e48 2022-12-28)
binary: rustc
commit-hash: unknown
commit-date: unknown
host: x86_64-unknown-linux-gnu
release: 1.68.0-dev
LLVM version: 15.0.6
    ";

    #[test]
    #[serial]
    fn rustc_unknown_bits() -> Result<()> {
        let mut rustc = RustcBuilder::all_rustc()?;
        let _ = rustc.with_rustc_str(UNKNOWN_BITS);
        let emitter = Emitter::default()
            .fail_on_error()
            .add_instructions(&rustc)?
            .test_emit();
        assert_eq!(6, emitter.cargo_rustc_env_map().len());
        assert_eq!(2, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(2, emitter.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn rustc_fails_on_bad_input() -> Result<()> {
        let mut rustc = RustcBuilder::all_rustc()?;
        let _ = rustc.with_rustc_str("a_bad_rustcvv_string");
        assert!(Emitter::default()
            .fail_on_error()
            .add_instructions(&rustc)
            .is_err());
        Ok(())
    }

    #[test]
    #[serial]
    fn rustc_defaults_on_bad_input() -> Result<()> {
        let mut rustc = RustcBuilder::all_rustc()?;
        let _ = rustc.with_rustc_str("a_bad_rustcvv_string");
        let emitter = Emitter::default().add_instructions(&rustc)?.test_emit();
        assert_eq!(6, emitter.cargo_rustc_env_map().len());
        assert_eq!(6, count_idempotent(emitter.cargo_rustc_env_map()));
        assert_eq!(6, emitter.cargo_warning().len());
        Ok(())
    }

    #[test]
    #[serial]
    fn rustc_channel_override_works() {
        with_var("VERGEN_RUSTC_CHANNEL", Some("this is a bad date"), || {
            let result = || -> Result<()> {
                let mut stdout_buf = vec![];
                let rustc = RustcBuilder::all_rustc()?;
                assert!(Emitter::default()
                    .add_instructions(&rustc)?
                    .emit_to(&mut stdout_buf)
                    .is_ok());
                let output = String::from_utf8_lossy(&stdout_buf);
                assert!(output.contains("cargo:rustc-env=VERGEN_RUSTC_CHANNEL=this is a bad date"));
                Ok(())
            }();
            assert!(result.is_ok());
        });
    }

    #[test]
    #[serial]
    fn rustc_commit_date_override_works() {
        with_var(
            "VERGEN_RUSTC_COMMIT_DATE",
            Some("this is a bad date"),
            || {
                let result = || -> Result<()> {
                    let mut stdout_buf = vec![];
                    let rustc = RustcBuilder::all_rustc()?;
                    assert!(Emitter::default()
                        .add_instructions(&rustc)?
                        .emit_to(&mut stdout_buf)
                        .is_ok());
                    let output = String::from_utf8_lossy(&stdout_buf);
                    assert!(output
                        .contains("cargo:rustc-env=VERGEN_RUSTC_COMMIT_DATE=this is a bad date"));
                    Ok(())
                }();
                assert!(result.is_ok());
            },
        );
    }

    #[test]
    #[serial]
    fn rustc_commit_hash_override_works() {
        with_var(
            "VERGEN_RUSTC_COMMIT_HASH",
            Some("this is a bad date"),
            || {
                let result = || -> Result<()> {
                    let mut stdout_buf = vec![];
                    let rustc = RustcBuilder::all_rustc()?;
                    assert!(Emitter::default()
                        .add_instructions(&rustc)?
                        .emit_to(&mut stdout_buf)
                        .is_ok());
                    let output = String::from_utf8_lossy(&stdout_buf);
                    assert!(output
                        .contains("cargo:rustc-env=VERGEN_RUSTC_COMMIT_HASH=this is a bad date"));
                    Ok(())
                }();
                assert!(result.is_ok());
            },
        );
    }

    #[test]
    #[serial]
    fn rustc_host_triple_override_works() {
        with_var(
            "VERGEN_RUSTC_HOST_TRIPLE",
            Some("this is a bad date"),
            || {
                let result = || -> Result<()> {
                    let mut stdout_buf = vec![];
                    let rustc = RustcBuilder::all_rustc()?;
                    assert!(Emitter::default()
                        .add_instructions(&rustc)?
                        .emit_to(&mut stdout_buf)
                        .is_ok());
                    let output = String::from_utf8_lossy(&stdout_buf);
                    assert!(output
                        .contains("cargo:rustc-env=VERGEN_RUSTC_HOST_TRIPLE=this is a bad date"));
                    Ok(())
                }();
                assert!(result.is_ok());
            },
        );
    }

    #[test]
    #[serial]
    fn rustc_llvm_version_override_works() {
        with_var(
            "VERGEN_RUSTC_LLVM_VERSION",
            Some("this is a bad date"),
            || {
                let result = || -> Result<()> {
                    let mut stdout_buf = vec![];
                    let rustc = RustcBuilder::all_rustc()?;
                    assert!(Emitter::default()
                        .add_instructions(&rustc)?
                        .emit_to(&mut stdout_buf)
                        .is_ok());
                    let output = String::from_utf8_lossy(&stdout_buf);
                    assert!(output
                        .contains("cargo:rustc-env=VERGEN_RUSTC_LLVM_VERSION=this is a bad date"));
                    Ok(())
                }();
                assert!(result.is_ok());
            },
        );
    }

    #[test]
    #[serial]
    fn rustc_semver_override_works() {
        with_var("VERGEN_RUSTC_SEMVER", Some("this is a bad date"), || {
            let result = || -> Result<()> {
                let mut stdout_buf = vec![];
                let rustc = RustcBuilder::all_rustc()?;
                assert!(Emitter::default()
                    .add_instructions(&rustc)?
                    .emit_to(&mut stdout_buf)
                    .is_ok());
                let output = String::from_utf8_lossy(&stdout_buf);
                assert!(output.contains("cargo:rustc-env=VERGEN_RUSTC_SEMVER=this is a bad date"));
                Ok(())
            }();
            assert!(result.is_ok());
        });
    }
}
