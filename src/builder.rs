use crate::key::VergenKey;
use anyhow::Result;
use std::{
    collections::BTreeMap,
    env,
    io::{self, Write},
};

#[cfg(feature = "build")]
use crate::build::Config as BuildConfig;
#[cfg(feature = "rustc")]
use crate::rustc::Config as RustcConfig;

pub(crate) type RustcEnvMap = BTreeMap<VergenKey, String>;

// Holds the base cargo instructions
#[derive(Clone, Debug, Default)]
pub(crate) struct CargoOutput {
    pub(crate) cargo_rustc_env_map: RustcEnvMap,
    pub(crate) warnings: Vec<String>,
}

impl CargoOutput {
    #[cfg(feature = "build")]
    fn add_build_entries(&mut self, builder: &Builder) -> Result<()> {
        let bc = builder.build_config;
        let skip = builder.skip_if_error;
        let idem = builder.idempotent;
        builder
            .add_build_map_entries(idem, &mut self.cargo_rustc_env_map, &mut self.warnings)
            .or_else(|e| bc.add_warnings(skip, e, &mut self.warnings))
    }

    #[cfg(not(feature = "build"))]
    #[allow(
        clippy::unnecessary_wraps,
        clippy::trivially_copy_pass_by_ref,
        clippy::unused_self
    )]
    fn add_build_entries(&mut self, _builder: &Builder) -> Result<()> {
        Ok(())
    }

    #[cfg(feature = "rustc")]
    fn add_rustc_entries(&mut self, builder: &Builder) -> Result<()> {
        let rc = builder.rustc_config;
        let skip = builder.skip_if_error;
        builder
            .add_rustc_map_entries(&mut self.cargo_rustc_env_map)
            .or_else(|e| rc.add_warnings(skip, e, &mut self.warnings))
    }

    #[cfg(not(feature = "rustc"))]
    #[allow(
        clippy::unnecessary_wraps,
        clippy::trivially_copy_pass_by_ref,
        clippy::unused_self
    )]
    fn add_rustc_entries(&mut self, _builder: &Builder) -> Result<()> {
        Ok(())
    }

    fn gen_output<T>(&self, stdout: &mut T) -> Result<()>
    where
        T: Write,
    {
        // Generate the 'cargo:' instruction output
        for (k, v) in &self.cargo_rustc_env_map {
            writeln!(stdout, "cargo:rustc-env={}={v}", k.name())?;
        }

        // Emit a cargo:warning if a section was skipped over
        for warning in &self.warnings {
            writeln!(stdout, "cargo:warning={warning}")?;
        }
        Ok(())
    }
}

/// Build the `vergen` configuration to enable specific cargo instruction
/// output
#[derive(Clone, Copy, Debug)]
pub struct Builder {
    idempotent: bool,
    skip_if_error: bool,
    #[cfg(feature = "build")]
    pub(crate) build_config: BuildConfig,
    #[cfg(feature = "rustc")]
    pub(crate) rustc_config: RustcConfig,
}

impl Default for Builder {
    fn default() -> Self {
        let idempotent = matches!(env::var("VERGEN_IDEMPOTENT"), Ok(_val));
        let skip_if_error = matches!(env::var("VERGEN_SKIP_IF_ERROR"), Ok(_val));
        Self {
            idempotent,
            skip_if_error,
            #[cfg(feature = "build")]
            build_config: Default::default(),
            #[cfg(feature = "rustc")]
            rustc_config: Default::default(),
        }
    }
}
impl Builder {
    /// Enable idempotent output
    ///
    /// **NOTE** - This feature can also be enabled via the `VERGEN_IDEMPOTENT`
    /// environment variable.
    ///
    /// When this feature is enabled, certain vergen output (i.e. timestamps, sysinfo)
    /// will be set to an idempotent default.  This will allow systems that
    /// depend on reproducible builds to override user requested vergen
    /// impurities.  This will mainly allow for package maintainer to build
    /// package that depende on vergen in a reproducible manner
    ///
    /// See [this issue](https://github.com/rustyhorde/vergen/issues/141) for
    /// more details
    ///
    /// | Variable | Sample |
    /// | -------  | ------ |
    /// | `VERGEN_BUILD_DATE` | `VERGEN_IDEMPOTENT_OUTPUT` |
    /// | `VERGEN_BUILD_TIME` | `VERGEN_IDEMPOTENT_OUTPUT` |
    /// | `VERGEN_BUILD_TIMESTAMP` | `VERGEN_IDEMPOTENT_OUTPUT` |
    /// | `VERGEN_BUILD_SEMVER` | 8.0.0 |
    ///
    /// # Example
    ///
    /// ```
    /// # use anyhow::Result;
    /// # use std::env;
    /// # use vergen::Vergen;
    /// #
    /// # fn main() -> Result<()> {
    #[cfg_attr(
        feature = "build",
        doc = r##"
Vergen::default().enable_idempotent().enable_all_build().gen()?;
"##
    )]
    /// // or
    /// env::set_var("VERGEN_IDEMPOTENT", "true");
    #[cfg_attr(
        feature = "build",
        doc = r##"
Vergen::default().enable_all_build().gen()?;
"##
    )]
    /// # env::remove_var("VERGEN_IDEMPOTENT");
    /// #   Ok(())
    /// # }
    /// ```
    ///
    /// This feature can also be used in conjuction with the [`SOURCE_DATE_EPOCH`](https://reproducible-builds.org/docs/source-date-epoch/)
    /// environment variable to generate reproducible timestamps based of the
    /// last modification time of the source/package
    ///
    /// # Example
    ///
    /// ```
    /// # use anyhow::Result;
    /// # use std::env;
    /// # use vergen::Vergen;
    /// #
    /// # fn main() -> Result<()> {
    /// env::set_var("SOURCE_DATE_EPOCH", "1671809360");
    #[cfg_attr(
        feature = "build",
        doc = r##"
Vergen::default().enable_idempotent().enable_all_build().gen()?;
"##
    )]
    /// # env::remove_var("SOURCE_DATE_EPOCH");
    /// #   Ok(())
    /// # }
    /// ```
    pub fn enable_idempotent(&mut self) -> &mut Self {
        self.idempotent = true;
        self
    }

    /// Enable skip cargo output on error, rather than failing
    pub fn enable_skip_if_error(&mut self) -> &mut Self {
        self.skip_if_error = true;
        self
    }

    /// Generate the vergen output
    ///
    /// # Errors
    ///
    pub fn gen(self) -> Result<()> {
        self.inner_gen()
            .and_then(|x| x.gen_output(&mut io::stdout()))
    }

    #[cfg(test)]
    pub(crate) fn test_gen(self) -> Result<CargoOutput> {
        self.inner_gen()
    }

    #[doc(hidden)]
    #[cfg(any(feature = "build", feature = "rustc"))]
    pub fn test_gen_output<T>(self, stdout: &mut T) -> Result<()>
    where
        T: Write,
    {
        self.inner_gen().and_then(|x| x.gen_output(stdout))
    }

    fn inner_gen(self) -> Result<CargoOutput> {
        let mut config = CargoOutput::default();
        config.add_build_entries(&self)?;
        config.add_rustc_entries(&self)?;
        Ok(config)
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::Builder;
    use anyhow::Result;
    #[cfg(any(feature = "build", feature = "rustc"))]
    use {super::RustcEnvMap, crate::constants::VERGEN_IDEMPOTENT_DEFAULT};

    #[cfg(any(feature = "build", feature = "rustc"))]
    pub(crate) fn count_idempotent(map: RustcEnvMap) -> usize {
        map.values()
            .filter(|x| *x == VERGEN_IDEMPOTENT_DEFAULT)
            .count()
    }

    #[test]
    fn default_is_disabled() -> Result<()> {
        let config = Builder::default().test_gen()?;
        assert!(config.cargo_rustc_env_map.is_empty());
        assert!(config.warnings.is_empty());
        Ok(())
    }
}
