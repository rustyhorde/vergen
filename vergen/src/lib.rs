// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! # vergen - Emit cargo instructions from a build script
//! `vergen`, when used in conjunction with cargo [build scripts] can emit the following:
//!
//! - Will emit [`cargo:rustc-env=VAR=VALUE`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-envvarvalue)
//!   for each feature you have enabled.  These can be referenced with the [env!](std::env!) macro in your code.
//! - Will emit [`cargo:rerun-if-changed=.git/HEAD`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed)
//!   if the git feature is enabled.  This is done to ensure any git instructions are regenerated when commits are made.
//! - Will emit [`cargo:rerun-if-changed=.git/<path_to_ref>`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed)
//!   if the git feature is enabled.  This is done to ensure any git instructions are regenerated when commits are made.
//! - Can emit [`cargo:warning`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargo-warning) outputs if the
//!   [`fail_on_error`](EmitBuilder::fail_on_error) feature is not enabled and the requested variable is defaulted through error or
//!   the [`idempotent`](EmitBuilder::idempotent) flag.
//! - Will emit [`cargo:rerun-if-changed=build.rs`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed)
//!   to rerun instruction emission if the `build.rs` file changed.
//! - Will emit [`cargo:rerun-if-env-changed=VERGEN_IDEMPOTENT`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed)
//!   to rerun instruction emission if the `VERGEN_IDEMPOTENT` environment variable has changed.
//! - Will emit [`cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed)
//!   to rerun instruction emission if the `SOURCE_DATE_EPOCH` environment variable has changed.
//!
//! ## Usage
//!
//! 1. Ensure you have build scripts enabled via the `build` configuration in your `Cargo.toml`
//!
//! ```toml
//! [package]
//! #..
//! build = "build.rs"
//! ```
//!
//! 2. Add `vergen` as a build dependency in `Cargo.toml`, specifying the features you wish to enable.
//!
//! ```toml
//! [dependencies]
//! #..
//!
//! [build-dependencies]
//! # All features enabled
//! vergen = { version = "8.0.0", features = ["build", "cargo", "git", "gitcl", "rustc", "si"] }
//! # or
//! vergen = { version = "8.0.0", features = ["build", "git", "gitcl"] }
//! # if you wish to disable certain features
//! ```
//!
//! 3. Create a `build.rs` file that uses `vergen` to emit cargo instructions.  Configuration
//!    starts with [`EmitBuilder`].  Eventually you will call [`emit`](EmitBuilder::emit) to output the
//!    cargo instructions. See the [`emit`](EmitBuilder::emit) documentation for more robust examples.
//!
//! #### Generate all output
//!
//! ```
//! use anyhow::Result;
//! # use std::env;
//! use vergen::EmitBuilder;
//!
//! pub fn main() -> Result<()> {
#![cfg_attr(
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
    // NOTE: This will output everything, and requires all features enabled.
    // NOTE: See the EmitBuilder documentation for configuration options.
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
//!     Ok(())
//! }
//! ```
//!
//! #### Generate specific output
//!
//! ```
//! use anyhow::Result;
//! # use std::env;
//! use vergen::EmitBuilder;
//!
//! pub fn main() -> Result<()> {
#![cfg_attr(
    all(feature = "build", all(feature = "git", feature = "gitcl"),),
    doc = r##"
    // NOTE: This will output only a build timestamp and long SHA from git.
    // NOTE: This set requires the build and git features.
    // NOTE: See the EmitBuilder documentation for configuration options.
    EmitBuilder::builder()
        .build_timestamp()
        .git_sha(false)
        .emit()?;
"##
)]
//!     Ok(())
//! }
//! ```
//!
//! 4. Use the [`env!`](std::env!) macro in your code to read the environment variables.
//!
//! ```ignore
//! println!("Build Timestamp: {}", env!("VERGEN_BUILD_TIMESTAMP"));
//! println!("git describe: {}", env!("VERGEN_GIT_DESCRIBE"));
//! ```
//!
//! ## Features
//! `vergen` has five main feature toggles allowing you to customize your output. No features are enabled by default.  
//! You **must** specifically enable the features you wish to use.
//!
//! | Feature | Enables |
//! | ------- | ------- |
//! |  build  | `VERGEN_BUILD_*` instructions |
//! |  cargo  | `VERGEN_CARGO_*` instructions |
//! |   git   | `VERGEN_GIT_*` instructions and the `cargo:rerun-if-changed` instructions  |
//! |  rustc  | `VERGEN_RUSTC_*` instructions |
//! |   si    | `VERGEN_SYSINFO_*` instructions |
//!
//! #### Configuring the `git` feature
//! If you wish to use the git feature, you must also enable one of the git implementations.
//! The `gitcl` features is lightweight, but depends on `git` being on the path.  The other
//! implementations allow for git instructions to be emitted without a reliance on
//! the git binary.  The [`git2`](https://github.com/rust-lang/git2-rs) library are bindings over
//! the `libgit2` library, while [`gitoxide`](https://github.com/Byron/gitoxide) is entirely implemented in Rust.
//!
//! **NOTE** - These 3 features are mutually exclusive.  Only one can be chosen.  If you select
//! multiple, `vergen` intentionally will not compile.
//!
//! | Features | Enables |
//! | -------- | ------- |
//! | gitcl    | `VERGEN_GIT_` instructions emitted via the `git` binary at the command line |
//! | git2     | `VERGEN_GIT_` instructions emitted via git `git2` library |
//! | gitoxide | `VERGEN_GIT_` instructions emitted via the `gitoxide` library |
//!
//! A common configuration would be as follows:
//! ```toml
//! [build-dependencies]
//! vergen = { version = "8.0.0", features = [ "build", "git", "gitcl" ]}
//! # ...
//! ```
//! ### `Cargo` feature unification for `vergen` versions prior to 8.3.0
//! When a dependency is used by multiple packages, Cargo will [use the union](https://doc.rust-lang.org/cargo/reference/features.html#feature-unification) of all features enabled on that dependency when building it.  Prior to version **8.3.0**, `vergen` had a set of mutually exclusive features `gitcl`, `git2`, and `gitoxide` to enable to specific git backend you wished to use.  If your crate has a dependency on another crate that uses `vergen`, your crate may fail to compile if you select a different `git` backend then the crate you depend on.  For example, your crate depends on `fancy-lib`.   
//!
//! #### fancy-lib `Cargo.toml`
//! ```toml
//! [build-dependencies]
//! vergen = { version = "8.2.10", features = ["git","gitcl"] }
//! ```
//!
//! #### your crate `Cargo.toml`
//! ```toml
//! [dependencies]
//! fancy-lib = "0.1.0"
//!
//! [build-dependencies]
//! vergen = { version = "8.2.10", features = ["git","gitoxide"] }
//! ```
//!
//! Your crate will fail to compile because `cargo` unifies this to
//! ```toml
//! vergen = { version = "8.2.10", features = ["git","gitcl","gitoxide"] }
//! ```
//! and prior to **8.3.0** `vergen` will not compile with both `gitcl` and `gitoxide` as features.
//!
//! As a workaround, you can use `cargo tree -f "{p} {f}" | grep vergen` to determine the feature list cargo has set for `vergen`.  If
//! a `git` backend has already been determined you will be able to use that without declaring those features in your dependency list. This is not perfect
//! as this leaves you at the mercy of your dependency and the git feature they selected, but it's a workaround until version 9 comes out.
//!
//! #### fancy-lib `Cargo.toml`
//! ```toml
//! [build-dependencies]
//! vergen = { version = "8.2.10", features = ["git","gitcl"] }
//! ```
//!
//! #### your crate `Cargo.toml`
//! ```toml
//! [dependencies]
//! fancy-lib = "0.1.0"
//!
//! [build-dependencies]
//! vergen = "8.2.10"
//! ```
//! #### Unified
//! ```toml
//! vergen = { version = "8.2.10", features = ["git","gitcl"] }
//! ```
//! ### `Cargo` feature unification for `vergen` versions 8.3.0 and beyond
//! `vergen` will accept `gitcl`,`git2`, and `gitoxide` as features.  If more than one of them is included, `vergen` will select `gitcl` before `git2` and `git2` before `gitoxide`.
//!
//! ## Environment Variables
//! `vergen` currently recognizes the following environment variables
//!
//! | Variable | Functionality |
//! | -------- | ------------- |
//! | `VERGEN_IDEMPOTENT` | If this environment variable is set `vergen` will use the idempotent output feature regardless of the configuration set in `build.rs`.  This exists mainly to allow package maintainers to force idempotent output to generate deterministic binary output. |
//! | `SOURCE_DATE_EPOCH` | If this environment variable is set `vergen` will use the value (unix time since epoch) as the basis for a time based instructions.  This can help emit deterministic instructions. |
//! | `VERGEN_BUILD_*` | If this environment variable is set `vergen` will use the value you specify for the output rather than generating it. |
//! | `VERGEN_CARGO_*` | If this environment variable is set `vergen` will use the value you specify for the output rather than generating it. |
//! | `VERGEN_GIT_*` | If this environment variable is set `vergen` will use the value you specify for the output rather than generating it. |
//! | `VERGEN_RUSTC_*` | If this environment variable is set `vergen` will use the value you specify for the output rather than generating it. |
//! | `VERGEN_SYSINFO_*` | If this environment variable is set `vergen` will use the value you specify for the output rather than generating it. |
//!
//! ## Goals
//! I initially wrote `vergen` (**ver**sion **gen**erator, so original) so I could embed a some git information in my
//! personal projects.  Now, usage has grown to the point that `vergen` needs to fit better in the rust ecosystem.
//!   
//! The current goals are as follows:
//!
//! #### Minimize the tool footprint
//! - Adopt an opt-in, rather than opt-out strategy for the features.  The default feature set is empty
//!   and no instructions will be emitted.
//! - The instructions you have configured **will** be emitted.  If there are errors or idempotentcy
//!   has been configured, some of those instructions may be defaulted.
//! - Allow overriding configurtion set in `build.rs` through environment variables.  This will allow package
//!   maintainers to force sane defaults when packaging rust binaries for distribution.
//!
//! #### Minimize the compile time impact
//! - `git2` and `gitoxide` are large features.  These are opt-in now.  I've also added back support for
//!   generating git instructions via the `git` binary.
//! - I've removed some extraneous libraries.   Any libraries added in the future will be checked against
//!   the current standard compile times to ensure the impact is not too great.
//! - `vergen` should compile and test from a source tarball.
//!
//! #### Support deterministic output
//! Compilations run from the same source oftentimes need to generate identical binaries.  `vergen` now supports
//! this determinism in a few ways.
//! - An [`idempotent`](EmitBuilder::idempotent) configuration option has been added.  When this is enabled in a
//!   build script, each build via cargo against the same source code should generate identical binaries. Instructions
//!   that output information that may change between builds (i.e. timestamps, sysinfo) will be defaulted.
//! - Recognize common environment variables that support deterministic builds (i.e. [`SOURCE_DATE_EPOCH`](https://reproducible-builds.org/docs/source-date-epoch/))
//! - Allow `build.rs` configuration overrides though environment variables to allow users building a binary, but
//!   not controlling the source to generate deterministic binaries.
//!
//! # Use Cases
//! I generally use vergen for the following two cases
//!
//! 1. Generating verbose output describing a command line application.
//!
//! ```text
//! ~/p/r/app λ app -vv
//! app 0.1.0
//!
//! Build Timestamp:     2021-02-23T20:14:46.558472672+00:00
//! Describe:            0.1.0-9-g46f83e1
//! Commit SHA:          46f83e112520533338245862d366f6a02cef07d4
//! Commit Date:         2021-02-23T08:08:02-05:00
//! Commit Branch:       master
//! rustc Version:       1.52.0-nightly
//! rustc Channel:       nightly
//! rustc Host Triple:   x86_64-unknown-linux-gnu
//! rustc Commit SHA:    3f5aee2d5241139d808f4fdece0026603489afd1
//! cargo Target Triple: x86_64-unknown-linux-musl
//! cargo Profile:       release
//! ```
//!
//! 2. Information endpoints in web apis
//!
//! ```json
//! ~/p/r/app λ curl https://some.app.com/info | jq
//! {
//!   "build_timestamp": "2021-02-19T21:32:22.932833758+00:00",
//!   "git_describe": "0.0.0-7-gc96c096",
//!   "git_sha": "c96c0961c3b7b749eab92f6f588b67915889c4cd",
//!   "git_commit_date": "2021-02-19T16:29:06-05:00",
//!   "git_branch": "master",
//!   "rustc_semver": "1.52.0-nightly",
//!   "rustc_channel": "nightly",
//!   "rustc_host_triple": "x86_64-unknown-linux-gnu",
//!   "rustc_commit_sha": "3f5aee2d5241139d808f4fdece0026603489afd1",
//!   "cargo_target_triple": "x86_64-unknown-linux-musl",
//!   "cargo_profile": "release"
//! }
//! ```
//!
//! [build scripts]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#outputs-of-the-build-script
//! [cargo:rustc-env]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-env
//! [cargo:rerun-if-changed]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed
//!

// rustc lints
#![cfg_attr(
    all(feature = "unstable", nightly),
    feature(
        multiple_supertrait_upcastable,
        must_not_suspend,
        non_exhaustive_omitted_patterns_lint,
        rustdoc_missing_doc_code_examples,
        strict_provenance,
    )
)]
#![cfg_attr(nightly, allow(single_use_lifetimes, unexpected_cfgs))]
#![cfg_attr(
    nightly,
    deny(
        absolute_paths_not_starting_with_crate,
        ambiguous_glob_imports,
        ambiguous_glob_reexports,
        ambiguous_wide_pointer_comparisons,
        anonymous_parameters,
        array_into_iter,
        asm_sub_register,
        async_fn_in_trait,
        bad_asm_style,
        bare_trait_objects,
        break_with_label_and_loop,
        byte_slice_in_packed_struct_with_derive,
        clashing_extern_declarations,
        coherence_leak_check,
        confusable_idents,
        const_eval_mutable_ptr_in_final_value,
        const_evaluatable_unchecked,
        const_item_mutation,
        dead_code,
        deprecated,
        deprecated_in_future,
        deprecated_where_clause_location,
        deref_into_dyn_supertrait,
        deref_nullptr,
        drop_bounds,
        dropping_copy_types,
        dropping_references,
        duplicate_macro_attributes,
        dyn_drop,
        elided_lifetimes_in_associated_constant,
        elided_lifetimes_in_paths,
        ellipsis_inclusive_range_patterns,
        explicit_outlives_requirements,
        exported_private_dependencies,
        ffi_unwind_calls,
        forbidden_lint_groups,
        forgetting_copy_types,
        forgetting_references,
        for_loops_over_fallibles,
        function_item_references,
        hidden_glob_reexports,
        improper_ctypes,
        improper_ctypes_definitions,
        inline_no_sanitize,
        internal_features,
        invalid_from_utf8,
        invalid_macro_export_arguments,
        invalid_nan_comparisons,
        invalid_value,
        irrefutable_let_patterns,
        keyword_idents_2018,
        keyword_idents_2024,
        large_assignments,
        late_bound_lifetime_arguments,
        legacy_derive_helpers,
        let_underscore_drop,
        macro_use_extern_crate,
        map_unit_fn,
        meta_variable_misuse,
        missing_abi,
        missing_copy_implementations,
        missing_debug_implementations,
        missing_docs,
        mixed_script_confusables,
        named_arguments_used_positionally,
        never_type_fallback_flowing_into_unsafe,
        no_mangle_generic_items,
        non_ascii_idents,
        non_camel_case_types,
        non_contiguous_range_endpoints,
        non_fmt_panics,
        non_local_definitions,
        non_shorthand_field_patterns,
        non_snake_case,
        non_upper_case_globals,
        noop_method_call,
        opaque_hidden_inferred_bound,
        overlapping_range_endpoints,
        path_statements,
        private_bounds,
        private_interfaces,
        redundant_lifetimes,
        redundant_semicolons,
        refining_impl_trait_internal,
        refining_impl_trait_reachable,
        renamed_and_removed_lints,
        repr_transparent_external_private_fields,
        rust_2021_incompatible_closure_captures,
        rust_2021_incompatible_or_patterns,
        rust_2021_prefixes_incompatible_syntax,
        rust_2021_prelude_collisions,
        semicolon_in_expressions_from_macros,
        special_module_name,
        stable_features,
        static_mut_refs,
        suspicious_double_ref_op,
        temporary_cstring_as_ptr,
        trivial_bounds,
        trivial_casts,
        trivial_numeric_casts,
        type_alias_bounds,
        tyvar_behind_raw_pointer,
        uncommon_codepoints,
        unconditional_recursion,
        uncovered_param_in_projection,
        undefined_naked_function_abi,
        ungated_async_fn_track_caller,
        uninhabited_static,
        unit_bindings,
        unknown_lints,
        unknown_or_malformed_diagnostic_attributes,
        unnameable_test_items,
        unnameable_types,
        unreachable_code,
        unreachable_patterns,
        unreachable_pub,
        unsafe_code,
        unsafe_op_in_unsafe_fn,
        unstable_name_collisions,
        unstable_syntax_pre_expansion,
        unsupported_calling_conventions,
        unused_allocation,
        unused_assignments,
        unused_associated_type_bounds,
        unused_attributes,
        unused_braces,
        unused_comparisons,
        unused_crate_dependencies,
        unused_doc_comments,
        unused_extern_crates,
        unused_features,
        unused_import_braces,
        unused_imports,
        unused_labels,
        unused_lifetimes,
        unused_macro_rules,
        unused_macros,
        unused_must_use,
        unused_mut,
        unused_parens,
        unused_qualifications,
        unused_results,
        unused_unsafe,
        unused_variables,
        useless_ptr_null_checks,
        variant_size_differences,
        wasm_c_abi,
        while_true,
        writes_through_immutable_pointer,
    )
)]
#![cfg_attr(all(nightly), allow(unstable_features))]
// If nightly and unstable, allow `incomplete_features` and `unstable_features`
#![cfg_attr(all(feature = "unstable", nightly), allow(incomplete_features))]
// If nightly and not unstable, deny `incomplete_features` and `unstable_features`
#![cfg_attr(
    all(not(feature = "unstable"), nightly),
    deny(incomplete_features, unstable_features)
)]
// The unstable lints
#![cfg_attr(
    all(feature = "unstable", nightly),
    deny(
        fuzzy_provenance_casts,
        lossy_provenance_casts,
        multiple_supertrait_upcastable,
        must_not_suspend,
        non_exhaustive_omitted_patterns,
        unfulfilled_lint_expectations,
    )
)]
// clippy lints
#![cfg_attr(nightly, deny(clippy::all, clippy::pedantic))]
// rustdoc lints
#![cfg_attr(
    nightly,
    deny(
        rustdoc::bare_urls,
        rustdoc::broken_intra_doc_links,
        rustdoc::invalid_codeblock_attributes,
        rustdoc::invalid_html_tags,
        rustdoc::missing_crate_level_docs,
        rustdoc::private_doc_tests,
        rustdoc::private_intra_doc_links,
    )
)]
#![cfg_attr(
    all(nightly, feature = "unstable"),
    deny(rustdoc::missing_doc_code_examples)
)]
#![cfg_attr(all(doc, nightly), feature(doc_auto_cfg))]
#![cfg_attr(all(docsrs, nightly), feature(doc_cfg))]
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

mod constants;
mod emitter;
mod feature;
mod key;
mod utils;

#[cfg(feature = "git")]
cfg_if::cfg_if! {
    if #[cfg(all(feature = "gitcl", feature = "git2", feature = "gitoxide"))] {
        use gix as _;
        use git2_rs as _;
    } else if #[cfg(all(feature = "gitcl", feature = "gitoxide"))] {
        use gix as _;
    } else if #[cfg(all(feature = "gitcl", feature = "git2"))] {
        use git2_rs as _;
    } else if #[cfg(all(feature = "git2", feature = "gitoxide"))] {
        use gix as _;
    }
}

// This is here to appease the `unused_crate_dependencies` lint
#[cfg(test)]
use {gix as _, lazy_static as _, regex as _, repo_util as _, serial_test as _, temp_env as _};

pub use crate::emitter::EmitBuilder;
#[cfg(feature = "cargo")]
pub use cargo_metadata::DependencyKind;
#[cfg(feature = "si")]
pub use sysinfo::CpuRefreshKind;
#[cfg(feature = "si")]
pub use sysinfo::MemoryRefreshKind;
#[cfg(feature = "si")]
pub use sysinfo::ProcessRefreshKind;
#[cfg(feature = "si")]
pub use sysinfo::RefreshKind;
