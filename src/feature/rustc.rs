use crate::{
    emitter::{EmitBuilder, RustcEnvMap},
    key::VergenKey,
    utils::fns::{add_default_map_entry, add_map_entry},
};
use anyhow::{Error, Result};
use rustc_version::{version_meta, Channel, VersionMeta};

#[derive(Clone, Copy, Debug, Default)]
#[allow(clippy::struct_excessive_bools)]
pub(crate) struct Config {
    pub(crate) rustc_channel: bool,
    pub(crate) rustc_commit_date: bool,
    pub(crate) rustc_commit_hash: bool,
    pub(crate) rustc_host_triple: bool,
    pub(crate) rustc_llvm_version: bool,
    pub(crate) rustc_semver: bool,
    #[cfg(test)]
    rustc_str_to_test: Option<&'static str>,
}

/// The `VERGEN_RUSTC_*` configuration features
///
/// | Variable | Sample |
/// | -------  | ------ |
/// | `VERGEN_RUSTC_CHANNEL` | nightly |
/// | `VERGEN_RUSTC_COMMIT_DATE` | 2021-02-24 |
/// | `VERGEN_RUSTC_COMMIT_HASH` | a8486b64b0c87dabd045453b6c81500015d122d6 |
/// | `VERGEN_RUSTC_HOST_TRIPLE` | x86_64-apple-darwin |
/// | `VERGEN_RUSTC_LLVM_VERSION` | 11.0 |
/// | `VERGEN_RUSTC_SEMVER` | 1.52.0-nightly |
///
/// # Example
///
/// ```
/// # use anyhow::Result;
/// # use vergen::EmitBuilder;
/// #
/// # fn main() -> Result<()> {
/// EmitBuilder::builder().all_rustc().emit()?;
/// #   Ok(())
/// # }
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "rustc")))]
impl EmitBuilder {
    /// Enable all of the `VERGEN_RUSTC_*` options
    pub fn all_rustc(&mut self) -> &mut Self {
        self.rustc_channel()
            .rustc_commit_date()
            .rustc_commit_hash()
            .rustc_host_triple()
            .rustc_llvm_version()
            .rustc_semver()
    }

    /// Enable the rustc channel
    pub fn rustc_channel(&mut self) -> &mut Self {
        self.rustc_config.rustc_channel = true;
        self
    }

    /// Enable the rustc commit date
    pub fn rustc_commit_date(&mut self) -> &mut Self {
        self.rustc_config.rustc_commit_date = true;
        self
    }

    /// Enable the rustc SHA
    pub fn rustc_commit_hash(&mut self) -> &mut Self {
        self.rustc_config.rustc_commit_hash = true;
        self
    }

    /// Enable rustc host triple
    pub fn rustc_host_triple(&mut self) -> &mut Self {
        self.rustc_config.rustc_host_triple = true;
        self
    }

    /// Enable rustc LLVM version
    pub fn rustc_llvm_version(&mut self) -> &mut Self {
        self.rustc_config.rustc_llvm_version = true;
        self
    }

    /// Enable the rustc semver
    pub fn rustc_semver(&mut self) -> &mut Self {
        self.rustc_config.rustc_semver = true;
        self
    }

    pub(crate) fn add_rustc_default(
        &self,
        e: Error,
        fail_on_error: bool,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
    ) -> Result<()> {
        if fail_on_error {
            Err(e)
        } else {
            if self.rustc_config.rustc_channel {
                add_default_map_entry(VergenKey::RustcChannel, map, warnings);
            }
            if self.rustc_config.rustc_commit_date {
                add_default_map_entry(VergenKey::RustcCommitDate, map, warnings);
            }
            if self.rustc_config.rustc_commit_hash {
                add_default_map_entry(VergenKey::RustcCommitHash, map, warnings);
            }
            if self.rustc_config.rustc_host_triple {
                add_default_map_entry(VergenKey::RustcHostTriple, map, warnings);
            }
            if self.rustc_config.rustc_llvm_version {
                add_default_map_entry(VergenKey::RustcLlvmVersion, map, warnings);
            }
            if self.rustc_config.rustc_semver {
                add_default_map_entry(VergenKey::RustcSemver, map, warnings);
            }

            Ok(())
        }
    }

    #[cfg(not(test))]
    pub(crate) fn add_rustc_map_entries(
        &self,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
    ) -> Result<()> {
        self.add_rustc_to_map(version_meta(), map, warnings)
    }

    #[cfg(test)]
    pub(crate) fn add_rustc_map_entries(
        &self,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
    ) -> Result<()> {
        use rustc_version::version_meta_for;

        let vm = if let Some(rustc_str) = self.rustc_config.rustc_str_to_test {
            version_meta_for(rustc_str)
        } else {
            version_meta()
        };
        self.add_rustc_to_map(vm, map, warnings)
    }

    fn add_rustc_to_map(
        &self,
        rustc_res: std::result::Result<VersionMeta, rustc_version::Error>,
        map: &mut RustcEnvMap,
        warnings: &mut Vec<String>,
    ) -> Result<()> {
        let rustc = rustc_res?;
        if self.rustc_config.rustc_channel {
            let channel = match rustc.channel {
                Channel::Dev => "dev",
                Channel::Nightly => "nightly",
                Channel::Beta => "beta",
                Channel::Stable => "stable",
            };
            add_map_entry(VergenKey::RustcChannel, channel, map);
        }

        if self.rustc_config.rustc_commit_date {
            if let Some(commit_date) = rustc.commit_date {
                add_map_entry(VergenKey::RustcCommitDate, commit_date, map);
            } else {
                add_default_map_entry(VergenKey::RustcCommitDate, map, warnings);
            }
        }

        if self.rustc_config.rustc_commit_hash {
            if let Some(commit_hash) = rustc.commit_hash {
                add_map_entry(VergenKey::RustcCommitHash, commit_hash, map);
            } else {
                add_default_map_entry(VergenKey::RustcCommitHash, map, warnings);
            }
        }

        if self.rustc_config.rustc_host_triple {
            add_map_entry(VergenKey::RustcHostTriple, rustc.host, map);
        }

        if self.rustc_config.rustc_llvm_version {
            if let Some(llvm_version) = rustc.llvm_version {
                add_map_entry(VergenKey::RustcLlvmVersion, format!("{llvm_version}"), map);
            } else {
                add_default_map_entry(VergenKey::RustcLlvmVersion, map, warnings);
            }
        }

        if self.rustc_config.rustc_semver {
            add_map_entry(VergenKey::RustcSemver, format!("{}", rustc.semver), map);
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{emitter::test::count_idempotent, EmitBuilder};
    use anyhow::Result;

    #[test]
    #[serial_test::parallel]
    fn rustc_all_idempotent() -> Result<()> {
        let config = EmitBuilder::builder()
            .idempotent()
            .all_rustc()
            .test_emit()?;
        assert_eq!(6, config.cargo_rustc_env_map.len());
        assert_eq!(0, count_idempotent(&config.cargo_rustc_env_map));
        assert_eq!(0, config.warnings.len());
        Ok(())
    }

    #[test]
    #[serial_test::parallel]
    fn rustc_all() -> Result<()> {
        let config = EmitBuilder::builder().all_rustc().test_emit()?;
        assert_eq!(6, config.cargo_rustc_env_map.len());
        assert_eq!(0, count_idempotent(&config.cargo_rustc_env_map));
        assert_eq!(0, config.warnings.len());
        Ok(())
    }

    const NO_LLVM: &str = r#"rustc 1.68.0-nightly (270c94e48 2022-12-28)
binary: rustc
commit-hash: 270c94e484e19764a2832ef918c95224eb3f17c7
commit-date: 2022-12-28
host: x86_64-unknown-linux-gnu
release: 1.68.0-nightly
"#;

    #[test]
    #[serial_test::parallel]
    fn no_llvm_in_rustc() -> Result<()> {
        let mut config = EmitBuilder::builder();
        let _ = config.fail_on_error();
        let _ = config.all_rustc();
        config.rustc_config.rustc_str_to_test = Some(NO_LLVM);
        let emitter = config.test_emit()?;
        assert_eq!(6, emitter.cargo_rustc_env_map.len());
        assert_eq!(1, count_idempotent(&emitter.cargo_rustc_env_map));
        assert_eq!(1, emitter.warnings.len());
        Ok(())
    }

    const DEV_BUILD: &str = r#"rustc 1.68.0-nightly (270c94e48 2022-12-28)
binary: rustc
commit-hash: 270c94e484e19764a2832ef918c95224eb3f17c7
commit-date: 2022-12-28
host: x86_64-unknown-linux-gnu
release: 1.68.0-dev
LLVM version: 15.0.6
"#;

    #[test]
    #[serial_test::parallel]
    fn rustc_dev_build() -> Result<()> {
        let mut config = EmitBuilder::builder();
        let _ = config.fail_on_error();
        let _ = config.all_rustc();
        config.rustc_config.rustc_str_to_test = Some(DEV_BUILD);
        let emitter = config.test_emit()?;
        assert_eq!(6, emitter.cargo_rustc_env_map.len());
        assert_eq!(0, count_idempotent(&emitter.cargo_rustc_env_map));
        assert_eq!(0, emitter.warnings.len());
        Ok(())
    }

    const UNKNOWN_BITS: &str = r#"rustc 1.68.0-nightly (270c94e48 2022-12-28)
binary: rustc
commit-hash: unknown
commit-date: unknown
host: x86_64-unknown-linux-gnu
release: 1.68.0-dev
LLVM version: 15.0.6
"#;

    #[test]
    #[serial_test::parallel]
    fn rustc_unknown_bits() -> Result<()> {
        let mut config = EmitBuilder::builder();
        let _ = config.fail_on_error();
        let _ = config.all_rustc();
        config.rustc_config.rustc_str_to_test = Some(UNKNOWN_BITS);
        let emitter = config.test_emit()?;
        assert_eq!(6, emitter.cargo_rustc_env_map.len());
        assert_eq!(2, count_idempotent(&emitter.cargo_rustc_env_map));
        assert_eq!(2, emitter.warnings.len());
        Ok(())
    }

    #[test]
    #[serial_test::parallel]
    fn rustc_fails_on_bad_input() -> Result<()> {
        let mut config = EmitBuilder::builder();
        let _ = config.fail_on_error();
        let _ = config.all_rustc();
        config.rustc_config.rustc_str_to_test = Some("a_bad_rustcvv_string");
        assert!(config.test_emit().is_err());
        Ok(())
    }

    #[test]
    #[serial_test::parallel]
    fn rustc_defaults_on_bad_input() -> Result<()> {
        let mut config = EmitBuilder::builder();
        let _ = config.all_rustc();
        config.rustc_config.rustc_str_to_test = Some("a_bad_rustcvv_string");
        let emitter = config.test_emit()?;
        assert_eq!(6, emitter.cargo_rustc_env_map.len());
        assert_eq!(6, count_idempotent(&emitter.cargo_rustc_env_map));
        assert_eq!(6, emitter.warnings.len());
        Ok(())
    }
}
