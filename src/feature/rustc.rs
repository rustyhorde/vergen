use crate::{
    emitter::{EmitBuilder, RustcEnvMap},
    key::VergenKey,
};
use anyhow::{Error, Result};
use rustc_version::{version_meta, Channel, VersionMeta};

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Config {
    pub(crate) rustc_channel: bool,
    pub(crate) rustc_commit_date: bool,
    pub(crate) rustc_host_triple: bool,
    pub(crate) rustc_llvm_version: bool,
    pub(crate) rustc_semver: bool,
    pub(crate) rustc_sha: bool,
}

impl Config {
    #[cfg(test)]
    fn enable_all(&mut self) {
        self.rustc_channel = true;
        self.rustc_commit_date = true;
        self.rustc_host_triple = true;
        self.rustc_llvm_version = true;
        self.rustc_semver = true;
        self.rustc_sha = true;
    }

    pub(crate) fn add_warnings(
        self,
        skip_if_error: bool,
        e: Error,
        warnings: &mut Vec<String>,
    ) -> Result<()> {
        if skip_if_error {
            if self.rustc_channel {
                warnings.push(format!(
                    "Unable to add {} to output",
                    VergenKey::RustcChannel.name()
                ));
            }
            if self.rustc_commit_date {
                warnings.push(format!(
                    "Unable to add {} to output",
                    VergenKey::RustcCommitDate.name()
                ));
            }
            if self.rustc_host_triple {
                warnings.push(format!(
                    "Unable to add {} to output",
                    VergenKey::RustcHostTriple.name()
                ));
            }
            if self.rustc_llvm_version {
                warnings.push(format!(
                    "Unable to add {} to output",
                    VergenKey::RustcLlvmVersion.name()
                ));
            }
            if self.rustc_semver {
                warnings.push(format!(
                    "Unable to add {} to output",
                    VergenKey::RustcSemver.name()
                ));
            }
            if self.rustc_sha {
                warnings.push(format!(
                    "Unable to add {} to output",
                    VergenKey::RustcCommitHash.name()
                ));
            }
            Ok(())
        } else {
            Err(e)
        }
    }
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
            .rustc_host_triple()
            .rustc_llvm_version()
            .rustc_semver()
            .rustc_sha()
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

    /// Enable the rustc SHA
    pub fn rustc_sha(&mut self) -> &mut Self {
        self.rustc_config.rustc_sha = true;
        self
    }

    pub(crate) fn add_rustc_map_entries(&self, map: &mut RustcEnvMap) -> Result<()> {
        self.add_rustc_to_map(map, version_meta()?)
    }

    fn add_rustc_to_map(&self, map: &mut RustcEnvMap, rustc: VersionMeta) -> Result<()> {
        if self.rustc_config.rustc_channel {
            let _old = map.insert(
                VergenKey::RustcChannel,
                match rustc.channel {
                    Channel::Dev => "dev",
                    Channel::Nightly => "nightly",
                    Channel::Beta => "beta",
                    Channel::Stable => "stable",
                }
                .to_string(),
            );
        }

        if self.rustc_config.rustc_commit_date {
            let _old = map.insert(
                VergenKey::RustcCommitDate,
                rustc.commit_date.unwrap_or_default(),
            );
        }

        if self.rustc_config.rustc_host_triple {
            let _old = map.insert(VergenKey::RustcHostTriple, rustc.host);
        }

        if self.rustc_config.rustc_llvm_version {
            let _old = map.insert(
                VergenKey::RustcLlvmVersion,
                if let Some(llvmver) = rustc.llvm_version {
                    format!("{llvmver}")
                } else {
                    "unknown".to_string()
                },
            );
        }

        if self.rustc_config.rustc_semver {
            let _old = map.insert(VergenKey::RustcSemver, format!("{}", rustc.semver));
        }

        if self.rustc_config.rustc_sha {
            let _old = map.insert(
                VergenKey::RustcCommitHash,
                rustc.commit_hash.unwrap_or_default(),
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use super::Config;
    use crate::{emitter::test::count_idempotent, EmitBuilder};
    use anyhow::{anyhow, Result};
    use rustc_version::version_meta_for;

    #[test]
    #[serial_test::parallel]
    fn add_warnings_is_err() -> Result<()> {
        let config = Config::default();
        let mut warnings = vec![];
        assert!(config
            .add_warnings(false, anyhow!("test"), &mut warnings)
            .is_err());
        Ok(())
    }

    #[test]
    #[serial_test::parallel]
    fn add_warnings_adds_warnings() -> Result<()> {
        let mut config = Config::default();
        config.enable_all();

        let mut warnings = vec![];
        assert!(config
            .add_warnings(true, anyhow!("test"), &mut warnings)
            .is_ok());
        assert_eq!(6, warnings.len());
        Ok(())
    }

    #[test]
    #[serial_test::parallel]
    fn rustc_all_idempotent() -> Result<()> {
        let config = EmitBuilder::builder()
            .idempotent()
            .all_rustc()
            .test_emit()?;
        assert_eq!(6, config.cargo_rustc_env_map.len());
        assert_eq!(0, count_idempotent(config.cargo_rustc_env_map));
        assert_eq!(0, config.warnings.len());
        Ok(())
    }

    #[test]
    #[serial_test::parallel]
    fn rustc_all() -> Result<()> {
        let config = EmitBuilder::builder().all_rustc().test_emit()?;
        assert_eq!(6, config.cargo_rustc_env_map.len());
        assert_eq!(0, count_idempotent(config.cargo_rustc_env_map));
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
        let mut map = BTreeMap::new();
        let vm = version_meta_for(NO_LLVM)?;
        let mut config = EmitBuilder::builder();
        let _ = config.all_rustc();
        config.add_rustc_to_map(&mut map, vm)?;
        let blah = config.test_emit()?;
        assert_eq!(6, blah.cargo_rustc_env_map.len());
        assert_eq!(0, count_idempotent(blah.cargo_rustc_env_map));
        assert_eq!(0, blah.warnings.len());
        Ok(())
    }

    const DEV_BUILD: &str = r#"rustc 1.68.0-nightly (270c94e48 2022-12-28)
binary: rustc
commit-hash: 270c94e484e19764a2832ef918c95224eb3f17c7
commit-date: 2022-12-28
host: x86_64-unknown-linux-gnu
release: 1.68.0-dev
"#;

    #[test]
    #[serial_test::parallel]
    fn rustc_dev_build() -> Result<()> {
        let mut map = BTreeMap::new();
        let vm = version_meta_for(DEV_BUILD)?;
        let mut config = EmitBuilder::builder();
        let _ = config.all_rustc();
        config.add_rustc_to_map(&mut map, vm)?;
        let blah = config.test_emit()?;
        assert_eq!(6, blah.cargo_rustc_env_map.len());
        assert_eq!(0, count_idempotent(blah.cargo_rustc_env_map));
        assert_eq!(0, blah.warnings.len());
        Ok(())
    }
}
