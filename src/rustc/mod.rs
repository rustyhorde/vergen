use crate::{builder::Builder, key::VergenKey};
use anyhow::{Error, Result};
use rustc_version::{version_meta, Channel};
use std::collections::BTreeMap;

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
/// # use vergen::Vergen;
/// #
/// # fn main() -> Result<()> {
/// Vergen::default().enable_all_rustc().gen()?;
/// #   Ok(())
/// # }
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "rustc")))]
impl Builder {
    /// Enable all of the `VERGEN_RUSTC_*` options
    pub fn enable_all_rustc(&mut self) -> &mut Self {
        self.enable_rustc_channel()
            .enable_rustc_commit_date()
            .enable_rustc_host_triple()
            .enable_rustc_llvm_version()
            .enable_rustc_semver()
            .enable_rustc_sha()
    }

    /// Enable the rustc channel
    pub fn enable_rustc_channel(&mut self) -> &mut Self {
        self.rustc_config.rustc_channel = true;
        self
    }

    /// Enable the rustc commit date
    pub fn enable_rustc_commit_date(&mut self) -> &mut Self {
        self.rustc_config.rustc_commit_date = true;
        self
    }

    /// Enable rustc host triple
    pub fn enable_rustc_host_triple(&mut self) -> &mut Self {
        self.rustc_config.rustc_host_triple = true;
        self
    }

    /// Enable rustc LLVM version
    pub fn enable_rustc_llvm_version(&mut self) -> &mut Self {
        self.rustc_config.rustc_llvm_version = true;
        self
    }

    /// Enable the rustc semver
    pub fn enable_rustc_semver(&mut self) -> &mut Self {
        self.rustc_config.rustc_semver = true;
        self
    }

    /// Enable the rustc SHA
    pub fn enable_rustc_sha(&mut self) -> &mut Self {
        self.rustc_config.rustc_sha = true;
        self
    }

    pub(crate) fn add_rustc_map_entries(
        &self,
        map: &mut BTreeMap<VergenKey, String>,
    ) -> Result<()> {
        let rustc = version_meta()?;

        if self.rustc_config.rustc_channel {
            let _ = map.insert(
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
            let _ = map.insert(
                VergenKey::RustcCommitDate,
                rustc.commit_date.unwrap_or_default(),
            );
        }

        if self.rustc_config.rustc_host_triple {
            let _ = map.insert(VergenKey::RustcHostTriple, rustc.host);
        }

        if self.rustc_config.rustc_llvm_version {
            let _ = map.insert(
                VergenKey::RustcLlvmVersion,
                if let Some(llvmver) = rustc.llvm_version {
                    format!("{llvmver}")
                } else {
                    "unknown".to_string()
                },
            );
        }

        if self.rustc_config.rustc_semver {
            let _ = map.insert(VergenKey::RustcSemver, format!("{}", rustc.semver));
        }

        if self.rustc_config.rustc_sha {
            let _ = map.insert(
                VergenKey::RustcCommitHash,
                rustc.commit_hash.unwrap_or_default(),
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{builder::test::count_idempotent, Vergen};
    use anyhow::Result;

    #[test]
    #[serial_test::parallel]
    fn rustc_all_idempotent() -> Result<()> {
        let config = Vergen::default()
            .enable_idempotent()
            .enable_all_rustc()
            .test_gen()?;
        assert_eq!(6, config.cargo_rustc_env_map.len());
        assert_eq!(0, count_idempotent(config.cargo_rustc_env_map));
        assert_eq!(0, config.warnings.len());
        Ok(())
    }

    #[test]
    #[serial_test::parallel]
    fn rustc_all() -> Result<()> {
        let config = Vergen::default().enable_all_rustc().test_gen()?;
        assert_eq!(6, config.cargo_rustc_env_map.len());
        assert_eq!(0, count_idempotent(config.cargo_rustc_env_map));
        assert_eq!(0, config.warnings.len());
        Ok(())
    }
}
