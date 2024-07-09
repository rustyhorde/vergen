// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use anyhow::Result;
use std::{
    env,
    io::{self, Write},
};

#[cfg(any(
    feature = "build",
    feature = "cargo",
    feature = "git",
    feature = "rustc",
    feature = "si"
))]
use {crate::key::VergenKey, std::collections::BTreeMap};

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
use std::path::PathBuf;

#[cfg(any(
    feature = "build",
    feature = "cargo",
    feature = "git",
    feature = "rustc",
    feature = "si"
))]
pub(crate) type RustcEnvMap = BTreeMap<VergenKey, String>;

// Everything that can be emitted as cargo build instructions
#[derive(Clone, Debug, Default)]
pub(crate) struct Emitter {
    pub(crate) failed: bool,
    #[cfg(any(
        feature = "build",
        feature = "cargo",
        feature = "git",
        feature = "rustc",
        feature = "si"
    ))]
    pub(crate) cargo_rustc_env_map: RustcEnvMap,
    #[cfg(any(
        feature = "build",
        feature = "cargo",
        feature = "git",
        feature = "rustc",
        feature = "si"
    ))]
    pub(crate) rerun_if_changed: Vec<String>,
    #[cfg(any(
        feature = "build",
        feature = "cargo",
        feature = "git",
        feature = "rustc",
        feature = "si"
    ))]
    pub(crate) warnings: Vec<String>,
}

impl Emitter {
    #[cfg(feature = "build")]
    fn add_build_entries(&mut self, builder: &EmitBuilder) -> Result<()> {
        let idem = builder.idempotent;
        let fail_on_error = builder.fail_on_error;
        let mut empty = BTreeMap::new();
        let cargo_rustc_env_map = if builder.disable_build {
            &mut empty
        } else {
            &mut self.cargo_rustc_env_map
        };
        builder
            .add_build_map_entries(idem, cargo_rustc_env_map, &mut self.warnings)
            .or_else(|e| {
                builder.add_build_default(e, fail_on_error, cargo_rustc_env_map, &mut self.warnings)
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
        let mut empty = BTreeMap::new();
        let cargo_rustc_env_map = if builder.disable_cargo {
            &mut empty
        } else {
            &mut self.cargo_rustc_env_map
        };
        builder
            .add_cargo_map_entries(cargo_rustc_env_map)
            .or_else(|e| {
                builder.add_cargo_default(e, fail_on_error, cargo_rustc_env_map, &mut self.warnings)
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
    fn add_git_entries(&mut self, builder: &EmitBuilder, repo_path: Option<PathBuf>) -> Result<()> {
        let idem = builder.idempotent;
        let fail_on_error = builder.fail_on_error;
        let mut empty_cargo_rustc_env_map = BTreeMap::new();
        let mut empty_rerun_if_changed = vec![];
        let mut empty_warnings = vec![];
        let (cargo_rustc_env_map, rerun_if_changed, warnings) = if builder.disable_git {
            (
                &mut empty_cargo_rustc_env_map,
                &mut empty_rerun_if_changed,
                &mut empty_warnings,
            )
        } else {
            (
                &mut self.cargo_rustc_env_map,
                &mut self.rerun_if_changed,
                &mut self.warnings,
            )
        };
        builder
            .add_git_map_entries(
                repo_path,
                idem,
                cargo_rustc_env_map,
                warnings,
                rerun_if_changed,
            )
            .or_else(|e| {
                self.failed = true;
                builder.add_git_default(
                    e,
                    fail_on_error,
                    cargo_rustc_env_map,
                    warnings,
                    rerun_if_changed,
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
        clippy::unused_self,
        clippy::needless_pass_by_value
    )]
    fn add_git_entries(&mut self, _builder: &EmitBuilder, _path: Option<PathBuf>) -> Result<()> {
        Ok(())
    }

    #[cfg(feature = "rustc")]
    fn add_rustc_entries(&mut self, builder: &EmitBuilder) -> Result<()> {
        let fail_on_error = builder.fail_on_error;
        let mut empty = BTreeMap::new();
        let cargo_rustc_env_map = if builder.disable_rustc {
            &mut empty
        } else {
            &mut self.cargo_rustc_env_map
        };
        builder
            .add_rustc_map_entries(cargo_rustc_env_map, &mut self.warnings)
            .or_else(|e| {
                builder.add_rustc_default(e, fail_on_error, cargo_rustc_env_map, &mut self.warnings)
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
    fn add_si_entries(&mut self, builder: &EmitBuilder) {
        let idem = builder.idempotent;
        let mut empty = BTreeMap::new();
        let cargo_rustc_env_map = if builder.disable_sysinfo {
            &mut empty
        } else {
            &mut self.cargo_rustc_env_map
        };
        builder.add_sysinfo_map_entries(idem, cargo_rustc_env_map, &mut self.warnings);
    }

    #[cfg(not(feature = "si"))]
    #[allow(
        clippy::unnecessary_wraps,
        clippy::trivially_copy_pass_by_ref,
        clippy::unused_self
    )]
    fn add_si_entries(&mut self, _builder: &EmitBuilder) {}

    fn emit_output<T>(
        &self,
        quiet: bool,
        custom_buildrs: Option<&'static str>,
        stdout: &mut T,
    ) -> Result<()>
    where
        T: Write,
    {
        self.emit_instructions(quiet, custom_buildrs, stdout)
    }

    #[cfg(not(any(
        feature = "build",
        feature = "cargo",
        feature = "git",
        feature = "rustc",
        feature = "si"
    )))]
    #[allow(clippy::unused_self, clippy::unnecessary_wraps)]
    fn emit_instructions<T>(
        &self,
        _quiet: bool,
        _custom_buildrs: Option<&'static str>,
        _stdout: &mut T,
    ) -> Result<()>
    where
        T: Write,
    {
        Ok(())
    }

    #[cfg(any(
        feature = "build",
        feature = "cargo",
        feature = "git",
        feature = "rustc",
        feature = "si"
    ))]
    fn emit_instructions<T>(
        &self,
        quiet: bool,
        custom_buildrs: Option<&'static str>,
        stdout: &mut T,
    ) -> Result<()>
    where
        T: Write,
    {
        // Emit the 'cargo:rustc-env' instructions
        for (k, v) in &self.cargo_rustc_env_map {
            let output = Self::filter_newlines(v);
            writeln!(stdout, "cargo:rustc-env={}={output}", k.name())?;
        }

        // Emit the `cargo:warning` instructions
        if !quiet {
            for warning in &self.warnings {
                let output = Self::filter_newlines(warning);
                writeln!(stdout, "cargo:warning={output}")?;
            }
        }

        // Emit the 'cargo:rerun-if-changed' instructions for the git paths (if added)
        for path in &self.rerun_if_changed {
            let output = Self::filter_newlines(path);
            writeln!(stdout, "cargo:rerun-if-changed={output}")?;
        }

        // Emit the 'cargo:rerun-if-changed' instructions
        if !self.cargo_rustc_env_map.is_empty() || !self.warnings.is_empty() {
            let buildrs = custom_buildrs.unwrap_or("build.rs");
            let output = Self::filter_newlines(buildrs);
            writeln!(stdout, "cargo:rerun-if-changed={output}")?;
            writeln!(stdout, "cargo:rerun-if-env-changed=VERGEN_IDEMPOTENT")?;
            writeln!(stdout, "cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH")?;
        }
        Ok(())
    }

    #[cfg(any(
        feature = "build",
        feature = "cargo",
        feature = "git",
        feature = "rustc",
        feature = "si"
    ))]
    fn filter_newlines(s: &str) -> String {
        s.chars().filter(|c| *c != '\n').collect()
    }
}

/// Build the `vergen` configuration to enable specific cargo instruction
/// output
#[derive(Clone, Copy, Debug)]
#[allow(clippy::struct_excessive_bools)]
pub struct EmitBuilder {
    idempotent: bool,
    fail_on_error: bool,
    quiet: bool,
    custom_buildrs: Option<&'static str>,
    #[cfg(feature = "build")]
    disable_build: bool,
    #[cfg(feature = "build")]
    pub(crate) build_config: BuildConfig,
    #[cfg(feature = "cargo")]
    disable_cargo: bool,
    #[cfg(feature = "cargo")]
    pub(crate) cargo_config: CargoConfig,
    #[cfg(all(
        feature = "git",
        any(feature = "git2", feature = "gitcl", feature = "gix")
    ))]
    disable_git: bool,
    #[cfg(all(
        feature = "git",
        any(feature = "git2", feature = "gitcl", feature = "gix")
    ))]
    pub(crate) git_config: GitConfig,
    #[cfg(feature = "rustc")]
    disable_rustc: bool,
    #[cfg(feature = "rustc")]
    pub(crate) rustc_config: RustcConfig,
    #[cfg(feature = "si")]
    disable_sysinfo: bool,
    #[cfg(feature = "si")]
    pub(crate) sysinfo_config: SysinfoConfig,
}

impl EmitBuilder {
    /// Instantiate the builder to configure the cargo instruction emits
    #[must_use]
    pub fn builder() -> Self {
        let idempotent = matches!(env::var("VERGEN_IDEMPOTENT"), Ok(_val));
        Self {
            idempotent,
            fail_on_error: false,
            quiet: false,
            custom_buildrs: None,
            #[cfg(feature = "build")]
            disable_build: false,
            #[cfg(feature = "build")]
            build_config: BuildConfig::default(),
            #[cfg(feature = "cargo")]
            disable_cargo: false,
            #[cfg(feature = "cargo")]
            cargo_config: CargoConfig::default(),
            #[cfg(all(
                feature = "git",
                any(feature = "git2", feature = "gitcl", feature = "gix")
            ))]
            disable_git: false,
            #[cfg(all(
                feature = "git",
                any(feature = "git2", feature = "gitcl", feature = "gix")
            ))]
            git_config: GitConfig::default(),
            #[cfg(feature = "rustc")]
            disable_rustc: false,
            #[cfg(feature = "rustc")]
            rustc_config: RustcConfig::default(),
            #[cfg(feature = "si")]
            disable_sysinfo: false,
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
    /// depend on deterministic builds to override user requested `vergen`
    /// impurities.  This will mainly allow for package maintainers to build
    /// packages that depend on `vergen` in a deterministic manner.
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

    /// Set a custom build.rs path if you are using a non-standard path
    ///
    /// By default `vergen` will use `build.rs` as the build path for the
    /// `cargo:rerun-if-changed` emit.  You can specify a custom `build.rs`
    /// path here if you have changed this default
    ///
    /// # Example
    ///
    /// ```
    /// # use anyhow::Result;
    /// # use vergen::EmitBuilder;
    /// #
    /// # fn main() -> Result<()> {
    #[cfg_attr(
        feature = "build",
        doc = r##"
EmitBuilder::builder().custom_build_rs("my/custom/build.rs").all_build().emit()?;
"##
    )]
    /// #   Ok(())
    /// # }
    /// ```
    pub fn custom_build_rs(&mut self, path: &'static str) -> &mut Self {
        self.custom_buildrs = Some(path);
        self
    }

    /// Enable the quiet feature
    ///
    /// Suppress the emission of the `cargo:warning` instructions.
    ///
    /// # Example
    ///
    /// ```
    /// # use anyhow::Result;
    /// # use vergen::EmitBuilder;
    /// #
    /// # fn main() -> Result<()> {
    #[cfg_attr(
        feature = "build",
        doc = r##"
EmitBuilder::builder().quiet().all_build().emit()?;
"##
    )]
    /// #   Ok(())
    /// # }
    /// ```
    pub fn quiet(&mut self) -> &mut Self {
        self.quiet = true;
        self
    }

    /// Disable the build output, even when the build feature is enabled.
    ///
    /// # Example
    ///
    /// ```
    /// # use anyhow::Result;
    /// # use vergen::EmitBuilder;
    /// #
    /// # fn main() -> Result<()> {
    #[cfg_attr(
        feature = "build",
        doc = r##"
EmitBuilder::builder().all_build().disable_build().emit()?;
"##
    )]
    /// #   Ok(())
    /// # }
    /// ```
    #[cfg(feature = "build")]
    pub fn disable_build(&mut self) -> &mut Self {
        self.disable_build = true;
        self
    }

    /// Disable the cargo output, even when the cargo feature is enabled.
    ///
    /// # Example
    ///
    /// ```
    /// # use anyhow::Result;
    /// # use vergen::EmitBuilder;
    /// #
    /// # fn main() -> Result<()> {
    #[cfg_attr(
        feature = "cargo",
        doc = r##"
EmitBuilder::builder().all_cargo().disable_cargo().emit()?;
"##
    )]
    /// #   Ok(())
    /// # }
    /// ```
    #[cfg(feature = "cargo")]
    pub fn disable_cargo(&mut self) -> &mut Self {
        self.disable_cargo = true;
        self
    }

    /// Disable the git output, even when the git feature is enabled.
    ///
    /// # Example
    ///
    /// ```
    /// # use anyhow::Result;
    /// # use vergen::EmitBuilder;
    /// #
    /// # fn main() -> Result<()> {
    #[cfg_attr(
        feature = "git",
        doc = r##"
EmitBuilder::builder().all_git().disable_git().emit()?;
"##
    )]
    /// #   Ok(())
    /// # }
    /// ```
    #[cfg(all(
        feature = "git",
        any(feature = "git2", feature = "gitcl", feature = "gix")
    ))]
    pub fn disable_git(&mut self) -> &mut Self {
        self.disable_git = true;
        self
    }

    /// Disable the rustc output, even when the rustc feature is enabled.
    ///
    /// # Example
    ///
    /// ```
    /// # use anyhow::Result;
    /// # use vergen::EmitBuilder;
    /// #
    /// # fn main() -> Result<()> {
    #[cfg_attr(
        feature = "rustc",
        doc = r##"
EmitBuilder::builder().all_rustc().disable_rustc().emit()?;
"##
    )]
    /// #   Ok(())
    /// # }
    /// ```
    #[cfg(feature = "rustc")]
    pub fn disable_rustc(&mut self) -> &mut Self {
        self.disable_rustc = true;
        self
    }

    /// Disable the sysinfo output, even when the sysinfo feature is enabled.
    ///
    /// # Example
    ///
    /// ```
    /// # use anyhow::Result;
    /// # use vergen::EmitBuilder;
    /// #
    /// # fn main() -> Result<()> {
    #[cfg_attr(
        feature = "si",
        doc = r##"
EmitBuilder::builder().all_sysinfo().disable_sysinfo().emit()?;
"##
    )]
    /// #   Ok(())
    /// # }
    /// ```
    #[cfg(feature = "si")]
    pub fn disable_sysinfo(&mut self) -> &mut Self {
        self.disable_sysinfo = true;
        self
    }

    /// Emit cargo instructions from your build script
    ///
    /// - Will emit [`cargo:rustc-env=VAR=VALUE`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-envvarvalue) for each feature you have enabled.
    #[cfg_attr(
        feature = "git",
        doc = r##" - Will emit [`cargo:rerun-if-changed=PATH`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed) if the git feature
is enabled.  This is done to ensure any git variables are regenerated when commits are made.
"##
    )]
    /// - Can emit [`cargo:warning`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargo-warning) outputs if the
    ///   [`fail_on_error`](Self::fail_on_error) feature is not enabled and the requested variable is defaulted through error or
    ///   the [`idempotent`](Self::idempotent) flag.
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
            all(feature = "git", feature = "gitcl"),
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
  .all_git()
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
    ///
    /// **NOTE** - You won't see this output unless you invoke cargo with the `-vv` flag.
    /// The instruction output is not displayed by default.
    ///
    /// ```text
    /// cargo:rustc-env=VERGEN_BUILD_DATE=2023-01-04
    /// cargo:rustc-env=VERGEN_BUILD_TIMESTAMP=2023-01-04T15:38:11.097507114Z
    /// cargo:rustc-env=VERGEN_CARGO_DEBUG=true
    /// cargo:rustc-env=VERGEN_CARGO_FEATURES=build,git
    /// cargo:rustc-env=VERGEN_CARGO_OPT_LEVEL=1
    /// cargo:rustc-env=VERGEN_CARGO_TARGET_TRIPLE=x86_64-unknown-linux-gnu
    /// cargo:rustc-env=VERGEN_GIT_BRANCH=feature/version8
    /// cargo:rustc-env=VERGEN_GIT_COMMIT_AUTHOR_EMAIL=your@email.com
    /// cargo:rustc-env=VERGEN_GIT_COMMIT_AUTHOR_NAME=Yoda
    /// cargo:rustc-env=VERGEN_GIT_COMMIT_COUNT=476
    /// cargo:rustc-env=VERGEN_GIT_COMMIT_DATE=2023-01-03
    /// cargo:rustc-env=VERGEN_GIT_COMMIT_MESSAGE=The best message
    /// cargo:rustc-env=VERGEN_GIT_COMMIT_TIMESTAMP=2023-01-03T14:08:12.000000000-05:00
    /// cargo:rustc-env=VERGEN_GIT_DESCRIBE=7.4.4-103-g53ae8a6
    /// cargo:rustc-env=VERGEN_GIT_SHA=53ae8a69ab7917a2909af40f2e5d015f5b29ae28
    /// cargo:rustc-env=VERGEN_RUSTC_CHANNEL=nightly
    /// cargo:rustc-env=VERGEN_RUSTC_COMMIT_DATE=2023-01-03
    /// cargo:rustc-env=VERGEN_RUSTC_COMMIT_HASH=c7572670a1302f5c7e245d069200e22da9df0316
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
    /// cargo:rerun-if-changed=.git/HEAD
    /// cargo:rerun-if-changed=.git/refs/heads/feature/version8
    /// cargo:rerun-if-changed=build.rs
    /// cargo:rerun-if-env-changed=VERGEN_IDEMPOTENT
    /// cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH
    /// ```
    ///
    pub fn emit(self) -> Result<()> {
        self.inner_emit(None)
            .and_then(|x| x.emit_output(self.quiet, self.custom_buildrs, &mut io::stdout()))
    }

    /// Emit cargo instructions from your build script and set environment variables for use in `build.rs`
    ///
    /// - Will emit [`cargo:rustc-env=VAR=VALUE`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-envvarvalue) for each feature you have enabled.
    #[cfg_attr(
        feature = "git",
        doc = r##" - Will emit [`cargo:rerun-if-changed=PATH`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed) if the git feature
is enabled.  This is done to ensure any git variables are regenerated when commits are made.
"##
    )]
    /// - Can emit [`cargo:warning`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargo-warning) outputs if the
    ///   [`fail_on_error`](Self::fail_on_error) feature is not enabled and the requested variable is defaulted through error or
    ///   the [`idempotent`](Self::idempotent) flag.
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
            all(feature = "git", feature = "gitcl"),
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
  .all_git()
  .all_rustc()
  .all_sysinfo()
  .emit_and_set()?;
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
    #[cfg(any(
        feature = "build",
        feature = "cargo",
        feature = "git",
        feature = "rustc",
        feature = "si"
    ))]
    pub fn emit_and_set(self) -> Result<()> {
        self.inner_emit(None)
            .and_then(|x| {
                x.emit_output(self.quiet, self.custom_buildrs, &mut io::stdout())
                    .map(|()| x)
            })
            .map(|x| {
                for (k, v) in &x.cargo_rustc_env_map {
                    if env::var(k.name()).is_err() {
                        env::set_var(k.name(), v);
                    }
                }
            })
    }

    /// Emit instructions from the given repository path.
    ///
    /// # Errors
    ///
    #[cfg(all(
        feature = "git",
        any(feature = "gitcl", feature = "git2", feature = "gix")
    ))]
    pub fn emit_at(self, repo_path: PathBuf) -> Result<()> {
        self.inner_emit(Some(repo_path))
            .and_then(|x| x.emit_output(self.quiet, self.custom_buildrs, &mut io::stdout()))
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
        self.inner_emit(None)
    }

    #[cfg(all(
        test,
        all(
            feature = "git",
            any(feature = "gitcl", feature = "git2", feature = "gix")
        ),
    ))]
    pub(crate) fn test_emit_at(self, repo_path: Option<PathBuf>) -> Result<Emitter> {
        self.inner_emit(repo_path)
    }

    #[doc(hidden)]
    /// Emit the cargo build script instructions to the given [`Write`](std::io::Write).
    ///
    /// **NOTE** - This is generally only used for testing and probably shouldn't be used
    /// within a `build.rs` file.
    ///
    /// # Errors
    /// * The [`writeln!`](std::writeln!) macro can throw a [`std::io::Error`]
    ///
    pub fn emit_to<T>(self, stdout: &mut T) -> Result<bool>
    where
        T: Write,
    {
        self.inner_emit(None).and_then(|x| {
            x.emit_output(self.quiet, self.custom_buildrs, stdout)
                .map(|()| x.failed)
        })
    }

    #[doc(hidden)]
    /// Emit the cargo build script instructions to the given [`Write`](std::io::Write) at
    /// the given repository path for git instructions
    ///
    /// **NOTE** - This is generally only used for testing and probably shouldn't be used
    /// within a `build.rs` file.
    ///
    /// # Errors
    /// * The [`writeln!`](std::writeln!) macro can throw a [`std::io::Error`]
    ///
    pub fn emit_to_at<T>(self, stdout: &mut T, path: Option<PathBuf>) -> Result<bool>
    where
        T: Write,
    {
        self.inner_emit(path).and_then(|x| {
            x.emit_output(self.quiet, self.custom_buildrs, stdout)
                .map(|()| x.failed)
        })
    }

    fn inner_emit(self, path: Option<PathBuf>) -> Result<Emitter> {
        let mut config = Emitter::default();
        config.add_build_entries(&self)?;
        config.add_cargo_entries(&self)?;
        config.add_git_entries(&self, path)?;
        config.add_rustc_entries(&self)?;
        config.add_si_entries(&self);
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
    pub(crate) fn count_idempotent(map: &RustcEnvMap) -> usize {
        map.values()
            .filter(|x| *x == VERGEN_IDEMPOTENT_DEFAULT)
            .count()
    }

    #[test]
    #[serial_test::serial]
    fn default_is_no_emit() -> Result<()> {
        let mut stdout_buf = vec![];
        _ = EmitBuilder::builder().emit_to(&mut stdout_buf)?;
        assert!(stdout_buf.is_empty());
        Ok(())
    }

    #[test]
    #[serial_test::serial]
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
        _ = EmitBuilder::builder()
            .idempotent()
            .fail_on_error()
            .all_build()
            .all_cargo()
            .all_rustc()
            .all_sysinfo()
            .emit_to(&mut stdout_buf)?;
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
    fn all_output_non_git() -> Result<()> {
        use crate::utils::testutils::{setup, teardown};

        setup();
        let mut stdout_buf = vec![];
        _ = EmitBuilder::builder()
            .all_build()
            .all_cargo()
            .all_rustc()
            .all_sysinfo()
            .emit_to(&mut stdout_buf)?;
        assert!(!stdout_buf.is_empty());
        teardown();
        Ok(())
    }

    #[cfg(all(
        feature = "build",
        feature = "rustc",
        all(
            feature = "git",
            any(feature = "gitcl", feature = "git2", feature = "gix")
        ),
        feature = "cargo",
        feature = "si"
    ))]
    #[test]
    #[serial_test::serial]
    fn all_output() -> Result<()> {
        use crate::utils::testutils::{setup, teardown};

        setup();
        let mut stdout_buf = vec![];
        _ = EmitBuilder::builder()
            .all_build()
            .all_cargo()
            .all_git()
            .all_rustc()
            .all_sysinfo()
            .emit_to(&mut stdout_buf)?;
        assert!(!stdout_buf.is_empty());
        teardown();
        Ok(())
    }

    #[cfg(all(
        feature = "build",
        feature = "rustc",
        all(
            feature = "git",
            any(feature = "gitcl", feature = "git2", feature = "gix")
        ),
        feature = "cargo",
        feature = "si"
    ))]
    #[test]
    #[serial_test::serial]
    fn all_features_no_output() -> Result<()> {
        use crate::utils::testutils::{setup, teardown};

        setup();
        let mut stdout_buf = vec![];
        _ = EmitBuilder::builder().emit_to(&mut stdout_buf)?;
        assert!(stdout_buf.is_empty());
        teardown();
        Ok(())
    }
}
