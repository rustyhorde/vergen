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
#[cfg(all(
    feature = "git",
    any(feature = "git2", feature = "gitcl", feature = "gix")
))]
use crate::feature::git::Config as GitConfig;
#[cfg(feature = "rustc")]
use crate::feature::rustc::Config as RustcConfig;
#[cfg(feature = "si")]
use crate::feature::si::Config as SysinfoConfig;

pub(crate) type RustcEnvMap = BTreeMap<VergenKey, String>;

// Everything that can be emitted as cargo build instructions
#[derive(Clone, Debug, Default)]
pub(crate) struct Emitter {
    pub(crate) cargo_rustc_env_map: RustcEnvMap,
    pub(crate) rerun_if_changed: Vec<String>,
    pub(crate) warnings: Vec<String>,
}

impl Emitter {
    #[cfg(feature = "build")]
    fn add_build_entries(&mut self, builder: &EmitBuilder) -> Result<()> {
        let idem = builder.idempotent;
        let fail_on_error = builder.fail_on_error;
        builder
            .add_build_map_entries(idem, &mut self.cargo_rustc_env_map, &mut self.warnings)
            .or_else(|e| {
                builder.add_build_default(
                    e,
                    fail_on_error,
                    &mut self.cargo_rustc_env_map,
                    &mut self.warnings,
                )
            })
    }

    #[cfg(not(feature = "build"))]
    #[allow(
        clippy::unnecessary_wraps,
        clippy::trivially_copy_pass_by_ref,
        clippy::unused_self
    )]
    fn add_build_entries(&mut self, _builder: &EmitBuilder) -> Result<()> {
        Ok(())
    }

    #[cfg(feature = "cargo")]
    fn add_cargo_entries(&mut self, builder: &EmitBuilder) -> Result<()> {
        let fail_on_error = builder.fail_on_error;
        builder
            .add_cargo_map_entries(&mut self.cargo_rustc_env_map)
            .or_else(|e| {
                builder.add_cargo_default(
                    e,
                    fail_on_error,
                    &mut self.cargo_rustc_env_map,
                    &mut self.warnings,
                )
            })
    }

    #[cfg(not(feature = "cargo"))]
    #[allow(
        clippy::unnecessary_wraps,
        clippy::trivially_copy_pass_by_ref,
        clippy::unused_self
    )]
    fn add_cargo_entries(&mut self, _builder: &EmitBuilder) -> Result<()> {
        Ok(())
    }

    #[cfg(all(
        feature = "git",
        any(feature = "git2", feature = "gitcl", feature = "gix")
    ))]
    fn add_git_entries(&mut self, builder: &EmitBuilder) -> Result<()> {
        let idem = builder.idempotent;
        let fail_on_error = builder.fail_on_error;
        builder
            .add_git_map_entries(
                idem,
                &mut self.cargo_rustc_env_map,
                &mut self.rerun_if_changed,
            )
            .or_else(|e| {
                builder.add_git_default(
                    e,
                    fail_on_error,
                    &mut self.cargo_rustc_env_map,
                    &mut self.warnings,
                )
            })
    }

    #[cfg(not(all(
        feature = "git",
        any(feature = "git2", feature = "gitcl", feature = "gix")
    )))]
    #[allow(
        clippy::unnecessary_wraps,
        clippy::trivially_copy_pass_by_ref,
        clippy::unused_self
    )]
    fn add_git_entries(&mut self, _builder: &EmitBuilder) -> Result<()> {
        Ok(())
    }

    #[cfg(feature = "rustc")]
    fn add_rustc_entries(&mut self, builder: &EmitBuilder) -> Result<()> {
        let fail_on_error = builder.fail_on_error;
        builder
            .add_rustc_map_entries(&mut self.cargo_rustc_env_map, &mut self.warnings)
            .or_else(|e| {
                builder.add_rustc_default(
                    e,
                    fail_on_error,
                    &mut self.cargo_rustc_env_map,
                    &mut self.warnings,
                )
            })
    }

    #[cfg(not(feature = "rustc"))]
    #[allow(
        clippy::unnecessary_wraps,
        clippy::trivially_copy_pass_by_ref,
        clippy::unused_self
    )]
    fn add_rustc_entries(&mut self, _builder: &EmitBuilder) -> Result<()> {
        Ok(())
    }

    #[cfg(feature = "si")]
    fn add_si_entries(&mut self, builder: &EmitBuilder) -> Result<()> {
        let idem = builder.idempotent;
        let fail_on_error = builder.fail_on_error;
        builder
            .add_sysinfo_map_entries(idem, &mut self.cargo_rustc_env_map, &mut self.warnings)
            .or_else(|e| {
                builder.add_sysinfo_default(
                    e,
                    fail_on_error,
                    &mut self.cargo_rustc_env_map,
                    &mut self.warnings,
                )
            })
    }

    #[cfg(not(feature = "si"))]
    #[allow(
        clippy::unnecessary_wraps,
        clippy::trivially_copy_pass_by_ref,
        clippy::unused_self
    )]
    fn add_si_entries(&mut self, _builder: &EmitBuilder) -> Result<()> {
        Ok(())
    }

    fn emit_output<T>(&self, stdout: &mut T) -> Result<()>
    where
        T: Write,
    {
        self.emit_instructions(stdout)
    }

    #[cfg(not(any(
        feature = "build",
        feature = "cargo",
        feature = "git",
        feature = "rustc",
        feature = "si"
    )))]
    #[allow(clippy::unused_self, clippy::unnecessary_wraps)]
    fn emit_instructions<T>(&self, _stdout: &mut T) -> Result<()>
    where
        T: Write,
    {
        // Emit the 'cargo:rustc-env' instructions
        for _v in self.cargo_rustc_env_map.values() {}

        // Emit the `cargo:warning` instructions
        for _warning in &self.warnings {}

        // Emit the 'cargo:rerun-if-changed' instructions for the git paths (if added)
        for _path in &self.rerun_if_changed {}
        Ok(())
    }

    #[cfg(any(
        feature = "build",
        feature = "cargo",
        feature = "git",
        feature = "rustc",
        feature = "si"
    ))]
    fn emit_instructions<T>(&self, stdout: &mut T) -> Result<()>
    where
        T: Write,
    {
        // Emit the 'cargo:rustc-env' instructions
        for (k, v) in &self.cargo_rustc_env_map {
            writeln!(stdout, "cargo:rustc-env={}={v}", k.name())?;
        }

        // Emit the `cargo:warning` instructions
        for warning in &self.warnings {
            writeln!(stdout, "cargo:warning={warning}")?;
        }

        // Emit the 'cargo:rerun-if-changed' instructions for the git paths (if added)
        for path in &self.rerun_if_changed {
            writeln!(stdout, "cargo:rerun-if-changed={path}")?;
        }

        // Emit the 'cargo:rerun-if-changed' instructions
        if !self.cargo_rustc_env_map.is_empty() || !self.warnings.is_empty() {
            writeln!(stdout, "cargo:rerun-if-changed=build.rs")?;
            writeln!(stdout, "cargo:rerun-if-env-changed=VERGEN_IDEMPOTENT")?;
            writeln!(stdout, "cargo:rerun-if-env-changed=VERGEN_SKIP_IF_ERROR")?;
            writeln!(stdout, "cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH")?;
        }
        Ok(())
    }
}

/// Build the `vergen` configuration to enable specific cargo instruction
/// output
#[derive(Clone, Copy, Debug)]
pub struct EmitBuilder {
    idempotent: bool,
    fail_on_error: bool,
    #[cfg(feature = "build")]
    pub(crate) build_config: BuildConfig,
    #[cfg(feature = "cargo")]
    pub(crate) cargo_config: CargoConfig,
    #[cfg(all(
        feature = "git",
        any(feature = "git2", feature = "gitcl", feature = "gix")
    ))]
    pub(crate) git_config: GitConfig,
    #[cfg(feature = "rustc")]
    pub(crate) rustc_config: RustcConfig,
    #[cfg(feature = "si")]
    pub(crate) sysinfo_config: SysinfoConfig,
}

impl EmitBuilder {
    /// Construct a new [`EmitBuilder`] builder to configure the cargo instruction emits
    #[must_use]
    pub fn builder() -> Self {
        let idempotent = matches!(env::var("VERGEN_IDEMPOTENT"), Ok(_val));
        Self {
            idempotent,
            fail_on_error: false,
            #[cfg(feature = "build")]
            build_config: BuildConfig::default(),
            #[cfg(feature = "cargo")]
            cargo_config: CargoConfig::default(),
            #[cfg(all(
                feature = "git",
                any(feature = "git2", feature = "gitcl", feature = "gix")
            ))]
            git_config: GitConfig::default(),
            #[cfg(feature = "rustc")]
            rustc_config: RustcConfig::default(),
            #[cfg(feature = "si")]
            sysinfo_config: SysinfoConfig::default(),
        }
    }

    /// Enable the `idempotent` feature
    ///
    /// **NOTE** - This feature can also be enabled via the `VERGEN_IDEMPOTENT`
    /// environment variable.
    ///
    /// When this feature is enabled, certain vergen output (i.e. timestamps, sysinfo)
    /// will be set to an idempotent default.  This will allow systems that
    /// depend on reproducible builds to override user requested vergen
    /// impurities.  This will mainly allow for package maintainers to build
    /// packages that depend on vergen in a reproducible manner.
    ///
    /// See [this issue](https://github.com/rustyhorde/vergen/issues/141) for
    /// more details
    ///
    /// | Variable | Sample |
    /// | -------  | ------ |
    /// | `VERGEN_BUILD_DATE` | `VERGEN_IDEMPOTENT_OUTPUT` |
    /// | `VERGEN_BUILD_TIMESTAMP` | `VERGEN_IDEMPOTENT_OUTPUT` |
    ///
    /// # Example
    ///
    /// ```
    /// # use anyhow::Result;
    /// # use std::env;
    /// # use vergen::EmitBuilder;
    /// #
    /// # fn main() -> Result<()> {
    #[cfg_attr(
        feature = "build",
        doc = r##"
EmitBuilder::builder().idempotent().all_build().emit()?;
"##
    )]
    /// // or
    /// env::set_var("VERGEN_IDEMPOTENT", "true");
    #[cfg_attr(
        feature = "build",
        doc = r##"
EmitBuilder::builder().all_build().emit()?;
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
    /// # use vergen::EmitBuilder;
    /// #
    /// # fn main() -> Result<()> {
    /// env::set_var("SOURCE_DATE_EPOCH", "1671809360");
    #[cfg_attr(
        feature = "build",
        doc = r##"
EmitBuilder::builder().idempotent().all_build().emit()?;
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

    /// Enable the `fail_on_error` feature
    ///
    /// By default `vergen` will emit the instructions you requested.  If for some
    /// reason those instructions cannot be generated correctly, placeholder values
    /// will be used instead.   `vergen` will also emit [`cargo:warning`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargo-warning)
    /// instructions notifying you this has happened.
    ///
    /// For example, if you configure `vergen` to emit `VERGEN_GIT_*` instructions and
    /// you run a build from a source tarball with no `.git` directory, the instructions
    /// will be populated with placeholder values, rather than information gleaned through git.
    ///
    /// You can turn off this behavior by enabling `fail_on_error`.
    ///
    /// # Example
    ///
    /// ```
    /// # use anyhow::Result;
    /// # use std::env;
    /// # use vergen::EmitBuilder;
    /// #
    /// # fn main() -> Result<()> {
    #[cfg_attr(
        feature = "build",
        doc = r##"
EmitBuilder::builder().fail_on_error().all_build().emit()?;
"##
    )]
    /// #   Ok(())
    /// # }
    /// ```
    pub fn fail_on_error(&mut self) -> &mut Self {
        self.fail_on_error = true;
        self
    }

    /// Emit the [`cargo:rustc-env=VAR=VALUE`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-envvarvalue)
    /// [build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html#outputs-of-the-build-script) outputs.
    ///
    /// **NOTE** - [`cargo:warning`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargo-warning) outputs
    /// may also be emitted if the [`fail_on_error`](Self::fail_on_error) feature is not enabled.
    ///
    /// # Errors
    /// * The [`writeln!`](std::writeln!) macro can throw a [`std::io::Error`]
    ///
    /// # Example
    ///
    /// ```
    /// # use anyhow::Result;
    /// # use std::env;
    /// # use vergen::EmitBuilder;
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
# env::set_var("CARGO_FEATURE_BUILD", "build");
# env::set_var("CARGO_FEATURE_GIT", "git");
# env::set_var("DEBUG", "true");
# env::set_var("OPT_LEVEL", "1");
# env::set_var("TARGET", "x86_64-unknown-linux-gnu");
EmitBuilder::builder()
  .all_build()
  .all_cargo()
  .all_rustc()
  .all_sysinfo()
  .emit()?;
# env::remove_var("CARGO_FEATURE_BUILD");
# env::remove_var("CARGO_FEATURE_GIT");
# env::remove_var("DEBUG");
# env::remove_var("OPT_LEVEL");
# env::remove_var("TARGET");
"##
    )]
    /// #   Ok(())
    /// # }
    /// ```
    ///
    /// # Sample Output
    /// ```text
    /// cargo:rustc-env=VERGEN_BUILD_DATE=2022-12-28
    /// cargo:rustc-env=VERGEN_BUILD_TIMESTAMP=2022-12-28T21:56:23.764785796Z
    /// cargo:rustc-env=VERGEN_CARGO_TARGET_TRIPLE=x86_64-unknown-linux-gnu
    /// cargo:rustc-env=VERGEN_CARGO_FEATURES=build,cargo,git,gitcl,rustc,si
    /// cargo:rustc-env=VERGEN_GIT_BRANCH=feature/version8
    /// cargo:rustc-env=VERGEN_GIT_COMMIT_AUTHOR_EMAIL=your@email.com
    /// cargo:rustc-env=VERGEN_GIT_COMMIT_AUTHOR_NAME=Yoda
    /// cargo:rustc-env=VERGEN_GIT_COMMIT_COUNT=389
    /// cargo:rustc-env=VERGEN_GIT_COMMIT_DATE=2022-12-29
    /// cargo:rustc-env=VERGEN_GIT_COMMIT_MESSAGE=Fix git framework
    /// cargo:rustc-env=VERGEN_GIT_COMMIT_TIMESTAMP=2022-12-29T10:48:31-05:00
    /// cargo:rustc-env=VERGEN_GIT_DESCRIBE=7.4.4-16-g2f35555
    /// cargo:rustc-env=VERGEN_GIT_SHA=2f35555f4d02dc44a60bf5854d5aad8b36494230
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
    pub fn emit(self) -> Result<()> {
        self.inner_emit()
            .and_then(|x| x.emit_output(&mut io::stdout()))
    }

    #[cfg(all(
        test,
        any(
            feature = "build",
            feature = "cargo",
            all(
                feature = "git",
                any(feature = "gitcl", feature = "git2", feature = "gix")
            ),
            feature = "rustc",
            feature = "si"
        )
    ))]
    pub(crate) fn test_emit(self) -> Result<Emitter> {
        self.inner_emit()
    }

    /// Emit the cargo build script instructions to the given [`Write`](std::io::Write).
    ///
    /// **NOTE** - This is genarally only used for testing and probably shouldn't be used
    /// withing a `build.rs` file.
    ///
    /// # Errors
    /// * The [`writeln!`](std::writeln!) macro can throw a [`std::io::Error`]
    ///
    pub fn emit_to<T>(self, stdout: &mut T) -> Result<()>
    where
        T: Write,
    {
        self.inner_emit().and_then(|x| x.emit_output(stdout))
    }

    fn inner_emit(self) -> Result<Emitter> {
        let mut config = Emitter::default();
        config.add_build_entries(&self)?;
        config.add_cargo_entries(&self)?;
        config.add_git_entries(&self)?;
        config.add_rustc_entries(&self)?;
        config.add_si_entries(&self)?;
        Ok(config)
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::EmitBuilder;
    use anyhow::Result;
    #[cfg(any(
        feature = "build",
        feature = "cargo",
        all(
            feature = "git",
            any(feature = "git2", feature = "gitcl", feature = "gix")
        ),
        feature = "rustc",
        feature = "si",
    ))]
    use {super::RustcEnvMap, crate::constants::VERGEN_IDEMPOTENT_DEFAULT};

    #[cfg(any(
        feature = "build",
        feature = "cargo",
        all(
            feature = "git",
            any(feature = "git2", feature = "gitcl", feature = "gix")
        ),
        feature = "rustc",
        feature = "si",
    ))]
    pub(crate) fn count_idempotent(map: RustcEnvMap) -> usize {
        map.values()
            .filter(|x| *x == VERGEN_IDEMPOTENT_DEFAULT)
            .count()
    }

    #[test]
    #[serial_test::parallel]
    fn default_is_no_emit() -> Result<()> {
        let mut stdout_buf = vec![];
        EmitBuilder::builder().emit_to(&mut stdout_buf)?;
        assert!(stdout_buf.is_empty());
        Ok(())
    }

    #[test]
    #[serial_test::parallel]
    fn gen_is_ok() -> Result<()> {
        assert!(EmitBuilder::builder().emit().is_ok());
        Ok(())
    }

    #[cfg(all(
        feature = "build",
        feature = "rustc",
        feature = "cargo",
        feature = "si"
    ))]
    #[test]
    #[serial_test::serial]
    fn everything_enabled() -> Result<()> {
        use crate::utils::testutils::{setup, teardown};
        
        setup();
        let mut stdout_buf = vec![];
        EmitBuilder::builder()
            .idempotent()
            .fail_on_error()
            .all_build()
            .all_cargo()
            .all_rustc()
            .all_sysinfo()
            .emit_to(&mut stdout_buf)?;
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
    #[serial_test::serial]
    fn all_output() -> Result<()> {
        use crate::utils::testutils::{setup, teardown};

        setup();
        let mut stdout_buf = vec![];
        EmitBuilder::builder()
            .all_build()
            .all_cargo()
            .all_rustc()
            .all_sysinfo()
            .emit_to(&mut stdout_buf)?;
        println!("{}", String::from_utf8_lossy(&stdout_buf));
        teardown();
        Ok(())
    }
}
