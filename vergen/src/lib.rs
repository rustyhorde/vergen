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
//! for each feature you have enabled.  These can be referenced with the [env!](std::env!) macro in your code.
//! - Can emit [`cargo:warning`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargo-warning) outputs if the
//! [`fail_on_error`](Emitter::fail_on_error) feature is not enabled and the requested variable is defaulted through error or
//! the [`idempotent`](Emitter::idempotent) flag.
//! - Will emit [`cargo:rerun-if-changed=build.rs`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed)
//! to rerun instruction emission if the `build.rs` file changed.
//! - Will emit [`cargo:rerun-if-env-changed=VERGEN_IDEMPOTENT`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed)
//! to rerun instruction emission if the `VERGEN_IDEMPOTENT` environment variable has changed.
//! - Will emit [`cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed)
//! to rerun instruction emission if the `SOURCE_DATE_EPOCH` environment variable has changed.
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
//! vergen = { version = "9.0.0-beta.0", features = ["build", "cargo", "rustc", "si"] }
//! # or
//! vergen = { version = "9.0.0-beta.0", features = ["build"] }
//! # if you wish to disable certain features
//! ```
//!
//! 3. Create a `build.rs` file that uses `vergen` to emit cargo instructions.  Configuration
//! starts with [`Emitter`].  Eventually you will call [`emit`](Emitter::emit) to output the
//! cargo instructions. See the [`emit`](Emitter::emit) documentation for more robust examples.
//!
//! #### Generate all output
//!
//! ```
//! # use anyhow::Result;
//! # use vergen::Emitter;
#![cfg_attr(feature = "build", doc = r##"# use vergen::BuildBuilder;"##)]
#![cfg_attr(feature = "cargo", doc = r##"# use vergen::CargoBuilder;"##)]
#![cfg_attr(feature = "rustc", doc = r##"# use vergen::RustcBuilder;"##)]
#![cfg_attr(feature = "si", doc = r##"# use vergen::SysinfoBuilder;"##)]
#![cfg_attr(feature = "cargo", doc = r##"# use test_util::with_cargo_vars;"##)]
//!
//! pub fn main() -> Result<()> {
#![cfg_attr(feature = "cargo", doc = r##"# let result = with_cargo_vars(|| {"##)]
//!     // NOTE: This will output everything, and requires all features enabled.
//!     // NOTE: See the specific builder documentation for configuration options.
#![cfg_attr(
    feature = "build",
    doc = r##"    let build = BuildBuilder::default().all_build().build();"##
)]
#![cfg_attr(
    feature = "cargo",
    doc = r##"    let cargo = CargoBuilder::default().all_cargo().build();"##
)]
#![cfg_attr(
    feature = "rustc",
    doc = r##"    let rustc = RustcBuilder::default().all_rustc().build();"##
)]
#![cfg_attr(
    feature = "si",
    doc = r##"    let si = SysinfoBuilder::default().all_sysinfo().build();"##
)]
//!
//!     Emitter::default()
#![cfg_attr(feature = "build", doc = r##"        .add_instructions(&build)?"##)]
#![cfg_attr(feature = "cargo", doc = r##"        .add_instructions(&cargo)?"##)]
#![cfg_attr(feature = "rustc", doc = r##"        .add_instructions(&rustc)?"##)]
#![cfg_attr(feature = "si", doc = r##"        .add_instructions(&si)?"##)]
//!         .emit()?;
#![cfg_attr(
    feature = "cargo",
    doc = r##"
#   Ok(())
# });
# assert!(result.is_ok());"##
)]
//!     Ok(())
//! }
//! ```
//!
//! #### Generate specific output
//!
//! ```
//! # use anyhow::Result;
//! # use vergen::Emitter;
#![cfg_attr(feature = "build", doc = r##"# use vergen::BuildBuilder;"##)]
#![cfg_attr(feature = "cargo", doc = r##"# use vergen::CargoBuilder;"##)]
#![cfg_attr(feature = "rustc", doc = r##"# use vergen::RustcBuilder;"##)]
#![cfg_attr(feature = "si", doc = r##"# use vergen::SysinfoBuilder;"##)]
#![cfg_attr(feature = "cargo", doc = r##"# use test_util::with_cargo_vars;"##)]
//!
//! pub fn main() -> Result<()> {
#![cfg_attr(feature = "cargo", doc = r##"# let result = with_cargo_vars(|| {"##)]
#![cfg_attr(
    feature = "build",
    doc = r##"    // NOTE: This will output only a build timestamp.
    // NOTE: This set requires the build feature.
    // NOTE: See the BuildBuilder documentation for configuration options. 
    let build = BuildBuilder::default().build_timestamp().build();"##
)]
#![cfg_attr(
    feature = "cargo",
    doc = r##"    let cargo = CargoBuilder::default().opt_level().build();"##
)]
#![cfg_attr(
    feature = "rustc",
    doc = r##"    let rustc = RustcBuilder::default().semver().build();"##
)]
#![cfg_attr(
    feature = "si",
    doc = r##"    let si = SysinfoBuilder::default().cpu_core_count().build();"##
)]
//!
//!     Emitter::default()
#![cfg_attr(feature = "build", doc = r##"        .add_instructions(&build)?"##)]
#![cfg_attr(feature = "cargo", doc = r##"        .add_instructions(&cargo)?"##)]
#![cfg_attr(feature = "rustc", doc = r##"        .add_instructions(&rustc)?"##)]
#![cfg_attr(feature = "si", doc = r##"        .add_instructions(&si)?"##)]
//!         .emit()?;
#![cfg_attr(
    feature = "cargo",
    doc = r##"
#   Ok(())
# });
    assert!(result.is_ok());"##
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
//! `vergen` has four main feature toggles allowing you to customize your output. No features are enabled by default.  
//! You **must** specifically enable the features you wish to use.
//!
//! | Feature | Enables |
//! | ------- | ------- |
//! |  build  | `VERGEN_BUILD_*` instructions |
//! |  cargo  | `VERGEN_CARGO_*` instructions |
//! |  rustc  | `VERGEN_RUSTC_*` instructions |
//! |   si    | `VERGEN_SYSINFO_*` instructions |
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
//! and no instructions will be emitted.
//! - The instructions you have configured **will** be emitted.  If there are errors or idempotentcy
//! has been configured, some of those instructions may be defaulted.
//! - Allow overriding configurtion set in `build.rs` through environment variables.  This will allow package
//! maintainers to force sane defaults when packaging rust binaries for distribution.
//!
//! #### Minimize the compile time impact
//! - I've removed some extraneous libraries.   Any libraries added in the future will be checked against
//! the current standard compile times to ensure the impact is not too great.
//! - `vergen` should compile and test from a source tarball.
//!
//! #### Support deterministic output
//! Compilations run from the same source oftentimes need to generate identical binaries.  `vergen` now supports
//! this determinism in a few ways.
//! - An [`idempotent`](Emitter::idempotent) configuration option has been added.  When this is enabled in a
//! build script, each build via cargo against the same source code should generate identical binaries. Instructions
//! that output information that may change between builds (i.e. timestamps, sysinfo) will be defaulted.
//! - Recognize common environment variables that support deterministic builds (i.e. [`SOURCE_DATE_EPOCH`](https://reproducible-builds.org/docs/source-date-epoch/))
//! - Allow `build.rs` configuration overrides though environment variables to allow users building a binary, but
//! not controlling the source to generate deterministic binaries.
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
    all(msrv, feature = "unstable", nightly),
    feature(
        lint_reasons,
        multiple_supertrait_upcastable,
        must_not_suspend,
        non_exhaustive_omitted_patterns_lint,
        strict_provenance,
        type_privacy_lints,
        rustdoc_missing_doc_code_examples,
    )
)]
#![cfg_attr(
    msrv,
    deny(
        absolute_paths_not_starting_with_crate,
        anonymous_parameters,
        array_into_iter,
        asm_sub_register,
        bad_asm_style,
        bare_trait_objects,
        // box_pointers,
        break_with_label_and_loop,
        clashing_extern_declarations,
        coherence_leak_check,
        confusable_idents,
        const_evaluatable_unchecked,
        const_item_mutation,
        dead_code,
        deprecated,
        deprecated_in_future,
        deprecated_where_clause_location,
        deref_into_dyn_supertrait,
        deref_nullptr,
        drop_bounds,
        duplicate_macro_attributes,
        dyn_drop,
        elided_lifetimes_in_paths,
        ellipsis_inclusive_range_patterns,
        explicit_outlives_requirements,
        exported_private_dependencies,
        forbidden_lint_groups,
        for_loops_over_fallibles,
        function_item_references,
        illegal_floating_point_literal_pattern,
        improper_ctypes,
        improper_ctypes_definitions,
        incomplete_features,
        indirect_structural_match,
        inline_no_sanitize,
        invalid_doc_attributes,
        invalid_value,
        irrefutable_let_patterns,
        keyword_idents,
        large_assignments,
        late_bound_lifetime_arguments,
        legacy_derive_helpers,
        let_underscore_drop,
        macro_use_extern_crate,
        meta_variable_misuse,
        missing_abi,
        missing_copy_implementations,
        missing_debug_implementations,
        missing_docs,
        mixed_script_confusables,
        named_arguments_used_positionally,
        no_mangle_generic_items,
        non_ascii_idents,
        non_camel_case_types,
        non_fmt_panics,
        non_shorthand_field_patterns,
        non_snake_case,
        nontrivial_structural_match,
        non_upper_case_globals,
        noop_method_call,
        opaque_hidden_inferred_bound,
        overlapping_range_endpoints,
        path_statements,
        pointer_structural_match,
        redundant_semicolons,
        renamed_and_removed_lints,
        repr_transparent_external_private_fields,
        rust_2021_incompatible_closure_captures,
        rust_2021_incompatible_or_patterns,
        rust_2021_prefixes_incompatible_syntax,
        rust_2021_prelude_collisions,
        semicolon_in_expressions_from_macros,
        single_use_lifetimes,
        special_module_name,
        stable_features,
        suspicious_auto_trait_impls,
        temporary_cstring_as_ptr,
        trivial_bounds,
        trivial_casts,
        trivial_numeric_casts,
        type_alias_bounds,
        tyvar_behind_raw_pointer,
        uncommon_codepoints,
        unconditional_recursion,
        unexpected_cfgs,
        ungated_async_fn_track_caller,
        uninhabited_static,
        unknown_lints,
        unnameable_test_items,
        unreachable_code,
        unreachable_patterns,
        unreachable_pub,
        unsafe_code,
        unsafe_op_in_unsafe_fn,
        unstable_features,
        unstable_name_collisions,
        unstable_syntax_pre_expansion,
        unsupported_calling_conventions,
        unused_allocation,
        unused_assignments,
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
        variant_size_differences,
        where_clauses_object_safety,
        while_true,
    )
)]
#![cfg_attr(msrv, allow(single_use_lifetimes))]
// If nightly or beta and unstable, allow `unstable_features`
#![cfg_attr(
    all(msrv, feature = "unstable", any(nightly, beta)),
    allow(unstable_features)
)]
// The unstable lints
#![cfg_attr(
    all(msrv, feature = "unstable", nightly),
    deny(
        ffi_unwind_calls,
        fuzzy_provenance_casts,
        lossy_provenance_casts,
        multiple_supertrait_upcastable,
        must_not_suspend,
        non_exhaustive_omitted_patterns,
        private_bounds,
        private_interfaces,
        unfulfilled_lint_expectations,
        unnameable_types,
    )
)]
// If nightly and not unstable, deny `unstable_features`
#![cfg_attr(all(msrv, not(feature = "unstable"), nightly), deny(unstable_features))]
// nightly only lints
#![cfg_attr(
    all(msrv, nightly),
    deny(ambiguous_glob_imports, invalid_reference_casting)
)]
// nightly or beta only lints
#![cfg_attr(
    all(msrv, any(beta, nightly)),
    deny(
        ambiguous_glob_reexports,
        byte_slice_in_packed_struct_with_derive,
        dropping_copy_types,
        dropping_references,
        forgetting_copy_types,
        forgetting_references,
        hidden_glob_reexports,
        invalid_from_utf8,
        invalid_macro_export_arguments,
        invalid_nan_comparisons,
        map_unit_fn,
        suspicious_double_ref_op,
        undefined_naked_function_abi,
        unused_associated_type_bounds,
    )
)]
// beta only lints
// #![cfg_attr( all(msrv, beta), deny())]
// beta or stable only lints
#![cfg_attr(all(msrv, any(beta, stable)), deny(unused_tuple_struct_fields))]
// stable only lints
#![cfg_attr(
    all(msrv, stable),
    deny(bindings_with_variant_name, implied_bounds_entailment)
)]
// clippy lints
#![cfg_attr(msrv, deny(clippy::all, clippy::pedantic))]
#![cfg_attr(all(msrv, lints_fix), allow(clippy::struct_field_names))]
// rustdoc lints
#![cfg_attr(
    msrv,
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
    all(msrv, feature = "unstable", nightly),
    deny(rustdoc::missing_doc_code_examples)
)]
#![cfg_attr(all(doc, nightly), feature(doc_auto_cfg))]
#![cfg_attr(all(docsrs, nightly), feature(doc_cfg))]
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

mod emitter;
mod feature;

// This is here to appease the `unused_crate_dependencies` lint
#[cfg(test)]
use {lazy_static as _, regex as _, serial_test as _, temp_env as _, test_util as _};

pub use crate::emitter::Emitter;
#[cfg(feature = "cargo")]
pub use cargo_metadata::DependencyKind;
#[cfg(feature = "build")]
#[doc(hidden)]
pub use feature::build::Build;
#[cfg(feature = "build")]
pub use feature::build::Builder as BuildBuilder;
#[cfg(feature = "cargo")]
pub use feature::cargo::Builder as CargoBuilder;
#[cfg(feature = "cargo")]
#[doc(hidden)]
pub use feature::cargo::Cargo;
#[cfg(feature = "rustc")]
pub use feature::rustc::Builder as RustcBuilder;
#[cfg(feature = "rustc")]
#[doc(hidden)]
pub use feature::rustc::Rustc;
#[cfg(feature = "si")]
pub use feature::si::Builder as SysinfoBuilder;
#[cfg(feature = "si")]
#[doc(hidden)]
pub use feature::si::Sysinfo;
#[cfg(feature = "si")]
pub use sysinfo::CpuRefreshKind;
#[cfg(feature = "si")]
pub use sysinfo::MemoryRefreshKind;
#[cfg(feature = "si")]
pub use sysinfo::ProcessRefreshKind;
#[cfg(feature = "si")]
pub use sysinfo::RefreshKind;
