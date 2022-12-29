// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use crate::key::VergenKey;
use anyhow::Result;
use std::{
    collections::BTreeMap,
    env,
    io::{self, Write},
};

#[cfg(feature = "build")]
use crate::feature::build::Config as BuildConfig;
#[cfg(feature = "cargo")]
use crate::feature::cargo::Config as CargoConfig;
#[cfg(feature = "rustc")]
use crate::feature::rustc::Config as RustcConfig;
#[cfg(feature = "si")]
use crate::feature::si::Config as SysinfoConfig;

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
        let config = builder.build_config;
        let skip = builder.skip_if_error;
        let idem = builder.idempotent;
        builder
            .add_build_map_entries(idem, &mut self.cargo_rustc_env_map, &mut self.warnings)
            .or_else(|e| config.add_warnings(skip, e, &mut self.warnings))
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
        let config = builder.rustc_config;
        let skip = builder.skip_if_error;
        builder
            .add_rustc_map_entries(&mut self.cargo_rustc_env_map)
            .or_else(|e| config.add_warnings(skip, e, &mut self.warnings))
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

    #[cfg(feature = "cargo")]
    fn add_cargo_entries(&mut self, builder: &Builder) -> Result<()> {
        let config = builder.cargo_config;
        let skip = builder.skip_if_error;
        builder
            .add_cargo_map_entries(&mut self.cargo_rustc_env_map)
            .or_else(|e| config.add_warnings(skip, e, &mut self.warnings))
    }

    #[cfg(not(feature = "cargo"))]
    #[allow(
        clippy::unnecessary_wraps,
        clippy::trivially_copy_pass_by_ref,
        clippy::unused_self
    )]
    fn add_cargo_entries(&mut self, _builder: &Builder) -> Result<()> {
        Ok(())
    }

    #[cfg(feature = "si")]
    fn add_si_entries(&mut self, builder: &Builder) -> Result<()> {
        let config = builder.sysinfo_config;
        let idem = builder.idempotent;
        let skip = builder.skip_if_error;
        builder
            .add_sysinfo_map_entries(idem, &mut self.cargo_rustc_env_map, &mut self.warnings)
            .or_else(|e| config.add_warnings(skip, e, &mut self.warnings))
    }

    #[cfg(not(feature = "si"))]
    #[allow(
        clippy::unnecessary_wraps,
        clippy::trivially_copy_pass_by_ref,
        clippy::unused_self
    )]
    fn add_si_entries(&mut self, _builder: &Builder) -> Result<()> {
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
    #[cfg(feature = "cargo")]
    pub(crate) cargo_config: CargoConfig,
    #[cfg(feature = "rustc")]
    pub(crate) rustc_config: RustcConfig,
    #[cfg(feature = "si")]
    pub(crate) sysinfo_config: SysinfoConfig,
}

impl Default for Builder {
    fn default() -> Self {
        let idempotent = matches!(env::var("VERGEN_IDEMPOTENT"), Ok(_val));
        let skip_if_error = matches!(env::var("VERGEN_SKIP_IF_ERROR"), Ok(_val));
        Self {
            idempotent,
            skip_if_error,
            #[cfg(feature = "build")]
            build_config: BuildConfig::default(),
            #[cfg(feature = "cargo")]
            cargo_config: CargoConfig::default(),
            #[cfg(feature = "rustc")]
            rustc_config: RustcConfig::default(),
            #[cfg(feature = "si")]
            sysinfo_config: SysinfoConfig::default(),
        }
    }
}
impl Builder {
    /// Enable the `idempotent` feature
    ///
    /// **NOTE** - This feature can also be enabled via the `VERGEN_IDEMPOTENT`
    /// environment variable.
    ///
    /// When this feature is enabled, certain vergen output (i.e. timestamps, sysinfo)
    /// will be set to an idempotent default.  This will allow systems that
    /// depend on reproducible builds to override user requested vergen
    /// impurities.  This will mainly allow for package maintainers to build
    /// packages that depende on vergen in a reproducible manner.
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
Vergen::default().idempotent().all_build().gen()?;
"##
    )]
    /// // or
    /// env::set_var("VERGEN_IDEMPOTENT", "true");
    #[cfg_attr(
        feature = "build",
        doc = r##"
Vergen::default().all_build().gen()?;
"##
    )]
    /// # env::remove_var("VERGEN_IDEMPOTENT");
    /// #   Ok(())
    /// # }
    /// ```
    ///
    /// This feature can also be used in conjuction with the [`SOURCE_DATE_EPOCH`](https://reproducible-builds.org/docs/source-date-epoch/)
    /// environment variable to generate reproducible timestamps based off the
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
Vergen::default().idempotent().all_build().gen()?;
"##
    )]
    /// # env::remove_var("SOURCE_DATE_EPOCH");
    /// #   Ok(())
    /// # }
    /// ```
    pub fn idempotent(&mut self) -> &mut Self {
        self.idempotent = true;
        self
    }

    /// Enable the `skip_if_error` feature
    ///
    /// **NOTE** - This feature can also be enabled via the `VERGEN_SKIP_IF_ERROR`
    /// environment variable.
    ///
    /// **NOTE** - The [`gen`](Self::gen) function can still potentially fail
    /// with an [`io`](std::io::Error) error writing to stdout, so the library
    /// still returns a [`Result`](anyhow::Result) that should be handled.
    ///
    /// This feature will cause `vergen` to skip any cargo instructions that would
    /// normally generate an error.  Instead, a [`cargo:warning`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargo-warning) instruction will
    /// be generated.  If you use this feature, you should use the [`option_env!`](std::option_env!)
    /// macro rather than the [`env!`](std::env!) macro when reading the variables
    /// in your code as they may not be set.
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
Vergen::default().skip_if_error().all_build().gen()?;
"##
    )]
    /// // or
    /// env::set_var("VERGEN_SKIP_IF_ERROR", "true");
    #[cfg_attr(
        feature = "build",
        doc = r##"
Vergen::default().all_build().gen()?;
"##
    )]
    /// # env::remove_var("VERGEN_SKIP_IF_ERROR");
    /// #   Ok(())
    /// # }
    /// ```
    pub fn skip_if_error(&mut self) -> &mut Self {
        self.skip_if_error = true;
        self
    }

    /// Generate the [`cargo:rustc-env=VAR=VALUE`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-envvarvalue)
    /// [build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html#outputs-of-the-build-script) outputs.
    ///
    /// **NOTE** - [`cargo:warning`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargo-warning) outputs
    /// may also be generated if the [`skip_if_error`](Self::skip_if_error) feature is enabled.
    ///
    /// # Errors
    /// * The [`writeln!`](std::writeln!) macro can generate a [`std::io::Error`]
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
        all(
            feature = "build",
            feature = "cargo",
            feature = "rustc",
            feature = "si"
        ),
        doc = r##"
# env::set_var("TARGET", "x86_64-unknown-linux-gnu");
# env::set_var("PROFILE", "build,rustc");
# env::set_var("CARGO_FEATURE_BUILD", "");
Vergen::default()
  .all_build()
  .all_cargo()
  .all_rustc()
  .all_sysinfo()
  .gen()?;
# env::remove_var("TARGET");
# env::remove_var("PROFILE");
# env::remove_var("CARGO_FEATURE_BUILD");
"##
    )]
    /// #   Ok(())
    /// # }
    /// ```
    ///
    /// # Sample Output
    /// ```text
    /// cargo:rustc-env=VERGEN_BUILD_DATE=2022-12-28
    /// cargo:rustc-env=VERGEN_BUILD_TIME=21:56:23
    /// cargo:rustc-env=VERGEN_BUILD_TIMESTAMP=2022-12-28T21:56:23.764785796Z
    /// cargo:rustc-env=VERGEN_BUILD_SEMVER=8.0.0
    /// cargo:rustc-env=VERGEN_CARGO_TARGET_TRIPLE=x86_64-unknown-linux-gnu
    /// cargo:rustc-env=VERGEN_CARGO_PROFILE=debug
    /// cargo:rustc-env=VERGEN_CARGO_FEATURES=git,build
    /// cargo:rustc-env=VERGEN_RUSTC_CHANNEL=nightly
    /// cargo:rustc-env=VERGEN_RUSTC_COMMIT_DATE=2022-12-27
    /// cargo:rustc-env=VERGEN_RUSTC_COMMIT_HASH=92c1937a90e5b6f20fa6e87016d6869da363972e
    /// cargo:rustc-env=VERGEN_RUSTC_HOST_TRIPLE=x86_64-unknown-linux-gnu
    /// cargo:rustc-env=VERGEN_RUSTC_LLVM_VERSION=15.0
    /// cargo:rustc-env=VERGEN_RUSTC_SEMVER=1.68.0-nightly
    /// cargo:rustc-env=VERGEN_SYSINFO_NAME=Arch Linux
    /// cargo:rustc-env=VERGEN_SYSINFO_OS_VERSION=Linux  Arch Linux
    /// cargo:rustc-env=VERGEN_SYSINFO_USER=jozias
    /// cargo:rustc-env=VERGEN_SYSINFO_TOTAL_MEMORY=31 GiB
    /// cargo:rustc-env=VERGEN_SYSINFO_CPU_VENDOR=AuthenticAMD
    /// cargo:rustc-env=VERGEN_SYSINFO_CPU_CORE_COUNT=8
    /// cargo:rustc-env=VERGEN_SYSINFO_CPU_NAME=cpu0,cpu1,cpu2,cpu3,cpu4,cpu5,cpu6,cpu7
    /// cargo:rustc-env=VERGEN_SYSINFO_CPU_BRAND=AMD Ryzen Threadripper 1900X 8-Core Processor
    /// cargo:rustc-env=VERGEN_SYSINFO_CPU_FREQUENCY=3792
    /// ```
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
    #[cfg(any(
        feature = "build",
        feature = "rustc",
        feature = "cargo",
        feature = "si"
    ))]
    pub fn test_gen_output<T>(self, stdout: &mut T) -> Result<()>
    where
        T: Write,
    {
        self.inner_gen().and_then(|x| x.gen_output(stdout))
    }

    fn inner_gen(self) -> Result<CargoOutput> {
        let mut config = CargoOutput::default();
        config.add_build_entries(&self)?;
        config.add_cargo_entries(&self)?;
        config.add_rustc_entries(&self)?;
        config.add_si_entries(&self)?;
        Ok(config)
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::Builder;
    use anyhow::Result;
    #[cfg(any(
        feature = "build",
        feature = "rustc",
        feature = "cargo",
        feature = "si"
    ))]
    use {super::RustcEnvMap, crate::constants::VERGEN_IDEMPOTENT_DEFAULT};

    #[cfg(any(
        feature = "build",
        feature = "rustc",
        feature = "cargo",
        feature = "si"
    ))]
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

    #[cfg(all(
        feature = "build",
        feature = "rustc",
        feature = "cargo",
        feature = "si"
    ))]
    #[test]
    fn everything_enabled() -> Result<()> {
        use crate::utils::testutils::{setup, teardown};

        setup();
        let mut stdout_buf = vec![];
        Builder::default()
            .idempotent()
            .skip_if_error()
            .all_build()
            .all_cargo()
            .all_rustc()
            .all_sysinfo()
            .test_gen_output(&mut stdout_buf)?;
        println!("{}", String::from_utf8_lossy(&stdout_buf));
        teardown();
        Ok(())
    }

    #[cfg(all(
        feature = "build",
        feature = "rustc",
        feature = "cargo",
        feature = "si"
    ))]
    #[test]
    fn all_output() -> Result<()> {
        use crate::utils::testutils::{setup, teardown};

        setup();
        let mut stdout_buf = vec![];
        Builder::default()
            .all_build()
            .all_cargo()
            .all_rustc()
            .all_sysinfo()
            .test_gen_output(&mut stdout_buf)?;
        println!("{}", String::from_utf8_lossy(&stdout_buf));
        teardown();
        Ok(())
    }
}
