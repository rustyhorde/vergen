// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use crate::{AddCustomEntries, AddEntries, CargoRustcEnvMap, DefaultConfig};
use anyhow::Result;
use std::{
    collections::BTreeMap,
    env,
    io::{self, Write},
};

/// The `Emitter` will emit cargo instructions (i.e. cargo:rustc-env=NAME=VALUE)
/// base on the configuration you enable.
#[derive(Clone, Debug, PartialEq)]
pub struct Emitter {
    idempotent: bool,
    fail_on_error: bool,
    quiet: bool,
    custom_buildrs: Option<&'static str>,
    #[doc(hidden)]
    cargo_rustc_env_map: CargoRustcEnvMap,
    #[doc(hidden)]
    cargo_rustc_env_map_custom: BTreeMap<String, String>,
    #[doc(hidden)]
    cargo_rerun_if_changed: Vec<String>,
    #[doc(hidden)]
    cargo_warning: Vec<String>,
}

impl Default for Emitter {
    fn default() -> Self {
        Self::new()
    }
}

impl Emitter {
    #[doc(hidden)]
    #[must_use]
    pub fn cargo_rustc_env_map(&self) -> &CargoRustcEnvMap {
        &self.cargo_rustc_env_map
    }
    #[doc(hidden)]
    #[must_use]
    pub fn cargo_rustc_env_map_custom(&self) -> &BTreeMap<String, String> {
        &self.cargo_rustc_env_map_custom
    }
    #[doc(hidden)]
    #[must_use]
    pub fn cargo_rerun_if_changed(&self) -> &Vec<String> {
        &self.cargo_rerun_if_changed
    }
    #[doc(hidden)]
    #[must_use]
    pub fn cargo_warning(&self) -> &Vec<String> {
        &self.cargo_warning
    }

    /// Instantiate the builder to configure the cargo instruction emits
    #[must_use]
    pub fn new() -> Self {
        Self {
            idempotent: matches!(env::var("VERGEN_IDEMPOTENT"), Ok(_val)),
            fail_on_error: false,
            quiet: false,
            custom_buildrs: None,
            cargo_rustc_env_map: CargoRustcEnvMap::default(),
            cargo_rustc_env_map_custom: BTreeMap::default(),
            cargo_rerun_if_changed: Vec::default(),
            cargo_warning: Vec::default(),
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
    /// # use vergen_lib::Emitter;
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
    /// # use vergen_lib::Emitter;
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
    /// # use vergen_lib::Emitter;
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
    /// # use vergen_lib::Emitter;
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
            &mut self.cargo_rerun_if_changed,
            &mut self.cargo_warning,
        )
        .or_else(|e| {
            let default_config = DefaultConfig::new(self.fail_on_error, e);
            gen.add_default_entries(
                &default_config,
                &mut self.cargo_rustc_env_map,
                &mut self.cargo_rerun_if_changed,
                &mut self.cargo_warning,
            )
        })?;
        Ok(self)
    }

    /// Add a set of custom instructions to the emitter output
    ///
    /// # Errors
    ///
    /// Errors may be generated if `fail_on_error` has been configured.
    ///
    pub fn add_custom_instructions<K, V>(
        &mut self,
        gen: &impl AddCustomEntries<K, V>,
    ) -> Result<&mut Self>
    where
        K: Into<String> + Ord,
        V: Into<String>,
    {
        let mut map = BTreeMap::default();
        gen.add_calculated_entries(
            self.idempotent,
            &mut map,
            &mut self.cargo_rerun_if_changed,
            &mut self.cargo_warning,
        )
        .or_else(|e| {
            let default_config = DefaultConfig::new(self.fail_on_error, e);
            gen.add_default_entries(
                &default_config,
                &mut map,
                &mut self.cargo_rerun_if_changed,
                &mut self.cargo_warning,
            )
        })?;
        self.cargo_rustc_env_map_custom.extend(Self::map_into(map));
        Ok(self)
    }

    fn map_into<K, V>(map: BTreeMap<K, V>) -> impl Iterator<Item = (String, String)>
    where
        K: Into<String> + Ord,
        V: Into<String>,
    {
        map.into_iter().map(|(k, v)| (k.into(), v.into()))
    }

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
            let sanitized_value = Self::filter_newlines(v);
            writeln!(stdout, "cargo:rustc-env={}={sanitized_value}", k.name())?;
        }

        // Emit the 'cargo:rustc-env' custom instructions
        for (k, v) in &self.cargo_rustc_env_map_custom {
            let sanitized_value = Self::filter_newlines(v);
            writeln!(stdout, "cargo:rustc-env={k}={sanitized_value}")?;
        }

        // Emit the `cargo:warning` instructions
        if !self.quiet {
            for warning in &self.cargo_warning {
                let sanitized_output = Self::filter_newlines(warning);
                writeln!(stdout, "cargo:warning={sanitized_output}")?;
            }
        }

        // Emit the 'cargo:rerun-if-changed' instructions for the git paths (if added)
        for path in &self.cargo_rerun_if_changed {
            let sanitized_output = Self::filter_newlines(path);
            writeln!(stdout, "cargo:rerun-if-changed={sanitized_output}")?;
        }

        // Emit the 'cargo:rerun-if-changed' instructions
        if !self.cargo_rustc_env_map.is_empty() || !self.cargo_warning.is_empty() {
            let buildrs = self.custom_buildrs.unwrap_or("build.rs");
            let sanitized_output = Self::filter_newlines(buildrs);
            writeln!(stdout, "cargo:rerun-if-changed={sanitized_output}")?;
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
    /// # use vergen_lib::Emitter;
    /// #
    /// # fn main() -> Result<()> {
    /// let emitter = Emitter::default().emit()?;
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
    /// # use vergen_lib::Emitter;
    /// #
    /// # fn main() -> Result<()> {
    /// Emitter::new().emit_and_set()?;
    /// #   Ok(())
    /// # }
    /// ```
    ///
    #[cfg(feature = "emit_and_set")]
    #[cfg_attr(coverage_nightly, coverage(off))]
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
    use std::io::Write;

    #[test]
    #[serial]
    #[allow(clippy::clone_on_copy, clippy::redundant_clone)]
    fn emitter_clone_works() {
        let emitter = Emitter::default();
        let another = emitter.clone();
        assert_eq!(another, emitter);
    }

    #[test]
    #[serial]
    fn emitter_debug_works() -> Result<()> {
        let emitter = Emitter::default();
        let mut buf = vec![];
        write!(buf, "{emitter:?}")?;
        assert!(!buf.is_empty());
        Ok(())
    }

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
}
