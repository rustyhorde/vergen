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
//! - Will emit [`cargo:rerun-if-changed=.git/HEAD`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed)
//! if the git feature is enabled.  This is done to ensure any git instructions are regenerated when commits are made.
//! - Will emit [`cargo:rerun-if-changed=.git/<path_to_ref>`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed)
//! if the git feature is enabled.  This is done to ensure any git instructions are regenerated when commits are made.
//! - Can emit [`cargo:warning`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargo-warning) outputs if the
//! [`fail_on_error`](EmitBuilder::fail_on_error) feature is not enabled and the requested variable is defaulted through error or
//! the [`idempotent`](EmitBuilder::idempotent) flag.
//! - Will emit [`cargo:rerun-if-changed=build.rs`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed)
//! to rerun instruction emission if the `build.rs` file changed.
//! - Will emit [`cargo:rurun-if-env-changed=VERGEN_IDEMPOTENT`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed)
//! to rerun instruction emission if the `VERGEN_IDEMPOTENT` environment variable has changed.
//! - Will emit [`cargo:rurun-if-env-changed=SOURCE_DATE_EPOCH`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed)
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
//! 1. Add `vergen` as a build dependency in `Cargo.toml`, specifying the features you wish to enable.
//!
//! ```toml
//! [dependencies]
//! #..
//!
//! [build-dependencies]
//! # All features enabled
//! vergen = { version = "8.0.0-beta.0", features = ["build", "cargo", "git", "gitcl", "rustc", "si"] }
//! # or
//! vergen = { version = "8.0.0-beta.0", features = ["build", "git", "gitcl"] }
//! # if you wish to disable certain features
//! ```
//!
//! 1. Create a `build.rs` file that uses `vergen` to emit cargo instructions.  Configuration
//! starts with [`EmitBuilder`].  Eventually you will call [`emit`](EmitBuilder::emit) to output the
//! cargo instructions. See the [`emit`](EmitBuilder::emit) documentation for more robust examples.
//!
//! ```
//! use std::error::Error;
//! use vergen::EmitBuilder;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     // Emit the instructions
//!     EmitBuilder::builder().emit()?;
//!     Ok(())
//! }
//! ```
//!
//! 1. Use the [`env!`](std::env!) macro in your code to read the environment variables.
//!
//! ```compile_fail
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
//! |   gitcl  | `VERGEN_GIT_` instructions emitted via the `git` binary at the command line |
//! |   git2   | `VERGEN_GIT_` instructions emitted via git `git2` library |
//! |   gix    | `VERGEN_GIT_` instructions emitted via the `gitoxide` library |
//!
//! A common configuration would be as follows:
//! ```toml
//! [build-dependencies]
//! vergen = { version = "8.0.0-beta.0", features = [ "build", "git", "gitcl" ]}
//! # ...
//! ```
//!
//! ## Environment Variables
//! `vergen` currently recognizes the following environment variables
//!
//! | Variable | Functionality |
//! | -------- | ------------- |
//! | `VERGEN_IDEMPOTENT` | If this environment variable is set `vergen` will use the idempotent output feature regardless of the configuration set in `build.rs`.  This exists mainly to allow package maintainers to force idempotent output to generate deterministic binary output. |
//! | `SOURCE_DATE_EPOCH` | If this environment variable is set `vergen` will use the value (unix time since epoch) as the basis for a time based instructions.  This can help emit deterministic instructions. |
//!
//! ## Goals
//! I initially wrote `vergen` (**ver**sion **gen**erator, so original) so I could embed a some git information in my
//! personal projects.  Now, usage has grown to the point that `vergen` need to fit better in the rust ecosystem.
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
//! - `git2` and `gitoxide` are large features.  These are opt-in now.  I've also added back support for
//! generating git instructions via the `git` binary.
//! - I've removed some extraneous libraries.   Any libraries added in the future will be checked against
//! the current standard compile times to ensure the impact is not too great.
//! - `vergen` should compile and test from a source tarball.
//!
//! #### Support deterministic output
//! Compilations run from the same source oftentimes need to generate identical binaries.  `vergen` now supports
//! this determinism in a few ways.
//! - An [`idempotent`](EmitBuilder::idempotent) configuration option has been added.  When this is enabled in a
//! build script, each build via cargo against the same source code should generate identical binaries. Instructions
//! that output information that may change between builds (i.e. timestamps, sysinfo) will be defaulted.
//! - Recognize common environment variables that support deterministic builds (i.e. [`SOURCE_DATE_EPOCH`](https://reproducible-builds.org/docs/source-date-epoch/))
//! - Allow `build.rs` configuration overrides though enviornment variables to allow users building a binary, but
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
#![cfg_attr(docsrs, feature(doc_cfg))]
// rustc lints
#![cfg_attr(
    all(msrv, feature = "unstable", nightly),
    feature(
        c_unwind,
        lint_reasons,
        must_not_suspend,
        non_exhaustive_omitted_patterns_lint,
        strict_provenance,
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
        bindings_with_variant_name,
        box_pointers,
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
        private_in_public,
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
        uninhabited_static,
        unknown_lints,
        unnameable_test_items,
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
        unused_tuple_struct_fields,
        unused_unsafe,
        unused_variables,
        variant_size_differences,
        where_clauses_object_safety,
        while_true,
    )
)]
// If nightly and unstable, allow `unstable_features`
#![cfg_attr(all(msrv, feature = "unstable", nightly), allow(unstable_features))]
// The unstable lints
#![cfg_attr(
    all(msrv, feature = "unstable", nightly),
    deny(
        ffi_unwind_calls,
        fuzzy_provenance_casts,
        lossy_provenance_casts,
        must_not_suspend,
        non_exhaustive_omitted_patterns,
        unfulfilled_lint_expectations,
    )
)]
// If nightly and not unstable, deny `unstable_features`
#![cfg_attr(all(msrv, not(feature = "unstable"), nightly), deny(unstable_features))]
// nightly only lints
// #![cfg_attr(all(msrv, nightly),deny())]
// nightly or beta only lints
#![cfg_attr(
    all(msrv, any(beta, nightly)),
    deny(implied_bounds_entailment, ungated_async_fn_track_caller,)
)]
// beta only lints
// #![cfg_attr( all(msrv, beta), deny())]
// beta or stable only lints
// #![cfg_attr(all(msrv, any(beta, stable)), deny())]
// stable only lints
// #![cfg_attr(all(msrv, stable), deny())]
// clippy lints
#![cfg_attr(msrv, deny(clippy::all, clippy::pedantic))]
// #![cfg_attr(msrv, allow())]
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

mod constants;
mod emitter;
mod feature;
mod key;
mod utils;

// This is here to appease the `unused_crate_dependencies` lint
#[cfg(test)]
use {git_repository as _, lazy_static as _, regex as _, serial_test as _};

pub use crate::emitter::EmitBuilder;
