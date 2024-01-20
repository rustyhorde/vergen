// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use anyhow::Result;
use getset::Getters;
use std::{
    env,
    io::{self, Write},
};
use vergen_lib::{AddEntries, CargoRustcEnvMap, DefaultConfig};

/// The `Emitter` will emit cargo instructions (i.e. cargo:rustc-env=NAME=VALUE)
/// base on the configuration you enable.
#[derive(Clone, Debug, Getters)]
pub struct Emitter {
    idempotent: bool,
    fail_on_error: bool,
    quiet: bool,
    custom_buildrs: Option<&'static str>,
    #[getset(get = "pub")]
    #[doc(hidden)]
    cargo_rustc_env_map: CargoRustcEnvMap,
    #[getset(get = "pub")]
    #[doc(hidden)]
    rerun_if_changed: Vec<String>,
    #[getset(get = "pub")]
    #[doc(hidden)]
    warnings: Vec<String>,
}

impl Default for Emitter {
    fn default() -> Self {
        Self::new()
    }
}

impl Emitter {
    /// Instantiate the builder to configure the cargo instruction emits
    #[must_use]
    pub fn new() -> Self {
        Self {
            idempotent: matches!(env::var("VERGEN_IDEMPOTENT"), Ok(_val)),
            fail_on_error: false,
            quiet: false,
            custom_buildrs: None,
            cargo_rustc_env_map: CargoRustcEnvMap::default(),
            rerun_if_changed: Vec::default(),
            warnings: Vec::default(),
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
    /// # use vergen::Emitter;
    /// #
    /// # fn main() -> Result<()> {
    /// Emitter::new().idempotent().emit()?;
    /// // or
    /// #     temp_env::with_var("VERGEN_IDEMPOTENT", Some("true"), || {
    /// #         let result = || -> Result<()> {
    /// // set::env("VERGEN_IDEMPOTENT", "true");
    /// Emitter::new().emit()?;
    /// #         Ok(())
    /// #         }();
    /// #     });
    /// #     Ok(())
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
    /// # use vergen::Emitter;
    /// #
    /// # fn main() -> Result<()> {
    /// Emitter::new().fail_on_error().emit()?;
    /// #   Ok(())
    /// # }
    /// ```
    pub fn fail_on_error(&mut self) -> &mut Self {
        self.fail_on_error = true;
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
    /// # use vergen::Emitter;
    /// #
    /// # fn main() -> Result<()> {
    /// Emitter::new().quiet().emit()?;
    /// #   Ok(())
    /// # }
    /// ```
    pub fn quiet(&mut self) -> &mut Self {
        self.quiet = true;
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
    /// # use vergen::Emitter;
    /// #
    /// # fn main() -> Result<()> {
    /// Emitter::new().custom_build_rs("my/custom/build.rs").emit()?;
    /// #   Ok(())
    /// # }
    /// ```
    pub fn custom_build_rs(&mut self, path: &'static str) -> &mut Self {
        self.custom_buildrs = Some(path);
        self
    }

    /// Add a set of instructions to the emitter output
    ///
    /// # Errors
    ///
    pub fn add_instructions(&mut self, gen: &dyn AddEntries) -> Result<&mut Self> {
        gen.add_map_entries(
            self.idempotent,
            &mut self.cargo_rustc_env_map,
            &mut self.rerun_if_changed,
            &mut self.warnings,
        )
        .or_else(|e| {
            let default_config = DefaultConfig::new(self.fail_on_error, e);
            gen.add_default_entries(
                &default_config,
                &mut self.cargo_rustc_env_map,
                &mut self.rerun_if_changed,
                &mut self.warnings,
            )
        })?;
        Ok(self)
    }

    // #[cfg(feature = "cargo")]
    // fn add_cargo_entries(&mut self, builder: &EmitBuilder) -> Result<()> {
    //     let fail_on_error = builder.fail_on_error;
    //     let mut empty = BTreeMap::new();
    //     let cargo_rustc_env_map = if builder.disable_cargo {
    //         &mut empty
    //     } else {
    //         &mut self.cargo_rustc_env_map
    //     };
    //     builder
    //         .add_cargo_map_entries(cargo_rustc_env_map)
    //         .or_else(|e| {
    //             builder.add_cargo_default(e, fail_on_error, cargo_rustc_env_map, &mut self.warnings)
    //         })
    // }

    // #[cfg(not(feature = "cargo"))]
    // #[allow(
    //     clippy::unnecessary_wraps,
    //     clippy::trivially_copy_pass_by_ref,
    //     clippy::unused_self
    // )]
    // fn add_cargo_entries(&mut self, _builder: &EmitBuilder) -> Result<()> {
    //     Ok(())
    // }

    // #[cfg(all(
    //     feature = "git",
    //     any(feature = "git2", feature = "gitcl", feature = "gix")
    // ))]
    // fn add_git_entries(&mut self, builder: &EmitBuilder, repo_path: Option<PathBuf>) -> Result<()> {
    //     let idem = builder.idempotent;
    //     let fail_on_error = builder.fail_on_error;
    //     let mut empty_cargo_rustc_env_map = BTreeMap::new();
    //     let mut empty_rerun_if_changed = vec![];
    //     let mut empty_warnings = vec![];
    //     let (cargo_rustc_env_map, rerun_if_changed, warnings) = if builder.disable_git {
    //         (
    //             &mut empty_cargo_rustc_env_map,
    //             &mut empty_rerun_if_changed,
    //             &mut empty_warnings,
    //         )
    //     } else {
    //         (
    //             &mut self.cargo_rustc_env_map,
    //             &mut self.rerun_if_changed,
    //             &mut self.warnings,
    //         )
    //     };
    //     builder
    //         .add_git_map_entries(
    //             repo_path,
    //             idem,
    //             cargo_rustc_env_map,
    //             warnings,
    //             rerun_if_changed,
    //         )
    //         .or_else(|e| {
    //             self.failed = true;
    //             builder.add_git_default(
    //                 e,
    //                 fail_on_error,
    //                 cargo_rustc_env_map,
    //                 warnings,
    //                 rerun_if_changed,
    //             )
    //         })
    // }

    // #[cfg(not(all(
    //     feature = "git",
    //     any(feature = "git2", feature = "gitcl", feature = "gix")
    // )))]
    // #[allow(
    //     clippy::unnecessary_wraps,
    //     clippy::trivially_copy_pass_by_ref,
    //     clippy::unused_self,
    //     clippy::needless_pass_by_value
    // )]
    // fn add_git_entries(&mut self, _builder: &EmitBuilder, _path: Option<PathBuf>) -> Result<()> {
    //     Ok(())
    // }

    // #[cfg(feature = "rustc")]
    // fn add_rustc_entries(&mut self, builder: &EmitBuilder) -> Result<()> {
    //     let fail_on_error = builder.fail_on_error;
    //     let mut empty = BTreeMap::new();
    //     let cargo_rustc_env_map = if builder.disable_rustc {
    //         &mut empty
    //     } else {
    //         &mut self.cargo_rustc_env_map
    //     };
    //     builder
    //         .add_rustc_map_entries(cargo_rustc_env_map, &mut self.warnings)
    //         .or_else(|e| {
    //             builder.add_rustc_default(e, fail_on_error, cargo_rustc_env_map, &mut self.warnings)
    //         })
    // }

    // #[cfg(not(feature = "rustc"))]
    // #[allow(
    //     clippy::unnecessary_wraps,
    //     clippy::trivially_copy_pass_by_ref,
    //     clippy::unused_self
    // )]
    // fn add_rustc_entries(&mut self, _builder: &EmitBuilder) -> Result<()> {
    //     Ok(())
    // }

    // #[cfg(feature = "si")]
    // fn add_si_entries(&mut self, builder: &EmitBuilder) {
    //     let idem = builder.idempotent;
    //     let mut empty = BTreeMap::new();
    //     let cargo_rustc_env_map = if builder.disable_sysinfo {
    //         &mut empty
    //     } else {
    //         &mut self.cargo_rustc_env_map
    //     };
    //     builder.add_sysinfo_map_entries(idem, cargo_rustc_env_map, &mut self.warnings);
    // }

    // #[cfg(not(feature = "si"))]
    // #[allow(
    //     clippy::unnecessary_wraps,
    //     clippy::trivially_copy_pass_by_ref,
    //     clippy::unused_self
    // )]
    // fn add_si_entries(&mut self, _builder: &EmitBuilder) {}

    fn emit_output<T>(&self, stdout: &mut T) -> Result<()>
    where
        T: Write,
    {
        self.emit_instructions(stdout)
    }

    fn emit_instructions<T>(&self, stdout: &mut T) -> Result<()>
    where
        T: Write,
    {
        // Emit the 'cargo:rustc-env' instructions
        for (k, v) in &self.cargo_rustc_env_map {
            let output = Self::filter_newlines(v);
            writeln!(stdout, "cargo:rustc-env={}={output}", k.name())?;
        }

        // Emit the `cargo:warning` instructions
        if !self.quiet {
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
            let buildrs = if let Some(path) = self.custom_buildrs {
                path
            } else {
                "build.rs"
            };
            let output = Self::filter_newlines(buildrs);
            writeln!(stdout, "cargo:rerun-if-changed={output}")?;
            writeln!(stdout, "cargo:rerun-if-env-changed=VERGEN_IDEMPOTENT")?;
            writeln!(stdout, "cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH")?;
        }
        Ok(())
    }

    fn filter_newlines(s: &str) -> String {
        s.chars().filter(|c| *c != '\n').collect()
    }

    /// Emit cargo instructions from your build script
    ///
    /// - Will emit [`cargo:rustc-env=VAR=VALUE`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-envvarvalue) for each feature you have enabled.
    /// - Will emit [`cargo:rerun-if-changed=PATH`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed) if the git feature is enabled.  This is done to ensure any git variables are regenerated when commits are made.
    /// - Can emit [`cargo:warning`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargo-warning) outputs if the
    /// [`fail_on_error`](Self::fail_on_error) feature is not enabled and the requested variable is defaulted through error or
    /// the [`idempotent`](Self::idempotent) flag.
    ///
    /// # Errors
    /// * The [`writeln!`](std::writeln!) macro can throw a [`std::io::Error`]
    ///
    /// # Example
    ///
    /// ```
    /// # use anyhow::Result;
    /// # use std::env;
    /// # use vergen::Emitter;
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
    pub fn emit(&self) -> Result<()> {
        self.emit_output(&mut io::stdout())
    }

    /// Emit cargo instructions from your build script and set environment variables for use in `build.rs`
    ///
    /// - Will emit [`cargo:rustc-env=VAR=VALUE`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-envvarvalue) for each feature you have enabled.
    /// - Will emit [`cargo:rerun-if-changed=PATH`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed) if the git feature is enabled.  This is done to ensure any git variables are regenerated when commits are made.
    /// - Can emit [`cargo:warning`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargo-warning) outputs if the
    /// [`fail_on_error`](Self::fail_on_error) feature is not enabled and the requested variable is defaulted through error or
    /// the [`idempotent`](Self::idempotent) flag.
    ///
    /// # Errors
    /// * The [`writeln!`](std::writeln!) macro can throw a [`std::io::Error`]
    ///
    /// # Example
    ///
    /// ```
    /// # use anyhow::Result;
    /// # use std::env;
    /// # use vergen::Emitter;
    /// #
    /// # fn main() -> Result<()> {
    /// Emitter::new().emit_and_set()?;
    /// #   Ok(())
    /// # }
    /// ```
    pub fn emit_and_set(&self) -> Result<()> {
        self.emit_output(&mut io::stdout()).map(|()| {
            for (k, v) in &self.cargo_rustc_env_map {
                if env::var(k.name()).is_err() {
                    env::set_var(k.name(), v);
                }
            }
        })
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
    pub fn emit_to<T>(&self, stdout: &mut T) -> Result<bool>
    where
        T: Write,
    {
        self.emit_output(stdout).map(|()| false)
    }

    #[doc(hidden)]
    #[must_use]
    pub fn test_emit(&self) -> Emitter {
        self.clone()
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::Emitter;
    use anyhow::Result;
    use serial_test::serial;

    #[test]
    #[serial]
    fn default_is_no_emit() -> Result<()> {
        let mut stdout_buf = vec![];
        _ = Emitter::new().emit_to(&mut stdout_buf)?;
        assert!(stdout_buf.is_empty());
        Ok(())
    }

    #[test]
    #[serial]
    fn default_emit_is_ok() {
        assert!(Emitter::new().emit().is_ok());
    }

    // #[cfg(all(
    //     feature = "build",
    //     feature = "rustc",
    //     feature = "cargo",
    //     feature = "si"
    // ))]
    // #[test]
    // #[serial_test::serial]
    // fn everything_enabled() -> Result<()> {
    //     use crate::utils::testutils::{setup, teardown};

    //     setup();
    //     let mut stdout_buf = vec![];
    //     _ = EmitBuilder::builder()
    //         .idempotent()
    //         .fail_on_error()
    //         .all_build()
    //         .all_cargo()
    //         .all_rustc()
    //         .all_sysinfo()
    //         .emit_to(&mut stdout_buf)?;
    //     teardown();
    //     Ok(())
    // }

    // #[cfg(all(
    //     feature = "build",
    //     feature = "rustc",
    //     feature = "cargo",
    //     feature = "si"
    // ))]
    // #[test]
    // #[serial_test::serial]
    // fn all_output_non_git() -> Result<()> {
    //     use crate::utils::testutils::{setup, teardown};

    //     setup();
    //     let mut stdout_buf = vec![];
    //     _ = EmitBuilder::builder()
    //         .all_build()
    //         .all_cargo()
    //         .all_rustc()
    //         .all_sysinfo()
    //         .emit_to(&mut stdout_buf)?;
    //     assert!(!stdout_buf.is_empty());
    //     teardown();
    //     Ok(())
    // }

    // #[cfg(all(
    //     feature = "build",
    //     feature = "rustc",
    //     all(
    //         feature = "git",
    //         any(feature = "gitcl", feature = "git2", feature = "gix")
    //     ),
    //     feature = "cargo",
    //     feature = "si"
    // ))]
    // #[test]
    // #[serial_test::serial]
    // fn all_output() -> Result<()> {
    //     use crate::utils::testutils::{setup, teardown};

    //     setup();
    //     let mut stdout_buf = vec![];
    //     _ = EmitBuilder::builder()
    //         .all_build()
    //         .all_cargo()
    //         .all_git()
    //         .all_rustc()
    //         .all_sysinfo()
    //         .emit_to(&mut stdout_buf)?;
    //     assert!(!stdout_buf.is_empty());
    //     teardown();
    //     Ok(())
    // }

    // #[cfg(all(
    //     feature = "build",
    //     feature = "rustc",
    //     all(
    //         feature = "git",
    //         any(feature = "gitcl", feature = "git2", feature = "gix")
    //     ),
    //     feature = "cargo",
    //     feature = "si"
    // ))]
    // #[test]
    // #[serial_test::serial]
    // fn all_features_no_output() -> Result<()> {
    //     use crate::utils::testutils::{setup, teardown};

    //     setup();
    //     let mut stdout_buf = vec![];
    //     _ = EmitBuilder::builder().emit_to(&mut stdout_buf)?;
    //     assert!(stdout_buf.is_empty());
    //     teardown();
    //     Ok(())
    // }
}
