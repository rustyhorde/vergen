// Copyright (c) 2016, 2018, 2021 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! # Generate Build Time Information
//! `vergen`, when used in conjunction with cargo [build scripts], will
//! generate environment variables to use with the `env!` macro.  Below
//! is a list of the supported variables.
//!
//! Key                         | Sample Value
//! ----------------------------|----------------------------------------
//! `VERGEN_BUILD_TIMESTAMP`    |2018-08-09T15:15:57.282334589+00:000
//! `VERGEN_BUILD_DATE`         |2018-08-09
//! `VERGEN_SHA`                |75b390dc6c05a6a4aa2791cc7b3934591803bc22
//! `VERGEN_SHA_SHORT`          |75b390d
//! `VERGEN_COMMIT_DATE`        |2018-08-08
//! `VERGEN_TARGET_TRIPLE`      |x86_64-unknown-linux-gnu
//! `VERGEN_SEMVER`             |v0.1.0
//! `VERGEN_SEMVER_LIGHTWEIGHT` |v0.1.0
//! `VERGEN_BRANCH`             |master
//! `VERGEN_RUSTC_SEMVER`       |1.4.3
//! `VERGEN_RUSTC_CHANNEL`      |nightly
//! `VERGEN_HOST_TRIPLE`        |x86_64-unknown-linux-gnu
//!
//! The variable generation can be toggled on or off at an individual level
//! via [`ConstantsFlags`](crate::constants::ConstantsFlags)
//!
//! ### Note on SEMVER
//! `VERGEN_SEMVER` can be generated via `git describe` or by
//! `env::var("CARGO_PKG_VERSION")`.
//!
//! By default, `SEMVER` uses `git describe` if possible, and falls back to `CARGO_PKG_VERSION`.
//!
//! If you wish to force `CARGO_PKG_VERSION`, toggle off `SEMVER` and toggle
//! on `SEMVER_FROM_CARGO_PKG`.
//!
//! `VERGEN_SEMVER` will also include a dirty tag if the build happend in a directory with
//! changes, i.e. `75b390d-dirty`.  This behavior can be toggled off via `TAG_DIRTY`.
//!
//! # Re-build On Changed HEAD
//! `vergen` can also be configured to re-run `build.rs` when either `.git/HEAD` or
//! the file that `.git/HEAD` points at changes.
//!
//! This can behavior can be toggled on or of with the [`REBUILD_ON_HEAD_CHANGE`] flag.
//!
//! [`REBUILD_ON_HEAD_CHANGE`]: crate::constants::ConstantsFlags::REBUILD_ON_HEAD_CHANGE
//! [build scripts]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
//!
//! ## 'cargo:' Key Build Script Output
//! ```toml
//! [package]
//! #..
//! build = "build.rs"
//!
//! [dependencies]
//! #..
//!
//! [build-dependencies]
//! vergen = "3"
//! ```
//!
//! ### Example 'build.rs'
//! ```
//! # use vergen::{ConstantsFlags, gen};
//! #
//! # fn main() {
//!     // Setup the flags, toggling off the 'SEMVER_FROM_CARGO_PKG' flag
//!     let mut flags = ConstantsFlags::all();
//!     flags.toggle(ConstantsFlags::SEMVER_FROM_CARGO_PKG);
//!
//!     // Generate the 'cargo:' key output
//!     gen(flags).expect("Unable to generate the cargo keys!");
//! # }
//! ```
//! ### Example 'build.rs' with SEMVER from `CARGO_PKG_VERSION`
//! ```
//! # use vergen::{ConstantsFlags, gen};
//! #
//! # fn other() {
//!     // Setup the flags, toggling off the 'SEMVER' flag to use `CARGO_PKG_VERSION`
//!     let mut flags = ConstantsFlags::all();
//!     flags.toggle(ConstantsFlags::SEMVER);
//!
//!     // Generate the 'cargo:' key output
//!     gen(flags).expect("Unable to generate the cargo keys!");
//! # }
//! ```
//!
//! ### Use the constants in your code
//! ```
//! # fn my_fn() {
//!     println!("Build Timestamp: {}", env!("VERGEN_BUILD_TIMESTAMP"));
//! # }
//! ```
#![deny(
    absolute_paths_not_starting_with_crate,
    anonymous_parameters,
    array_into_iter,
    asm_sub_register,
    bare_trait_objects,
    bindings_with_variant_name,
    // box_pointers,
    broken_intra_doc_links,
    cenum_impl_drop_cast,
    clashing_extern_declarations,
    clippy::all,
    clippy::pedantic,
    clippy::cargo,
    coherence_leak_check,
    confusable_idents,
    const_evaluatable_unchecked,
    const_item_mutation,
    dead_code,
    deprecated,
    deprecated_in_future,
    drop_bounds,
    elided_lifetimes_in_paths,
    ellipsis_inclusive_range_patterns,
    explicit_outlives_requirements,
    exported_private_dependencies,
    illegal_floating_point_literal_pattern,
    improper_ctypes,
    improper_ctypes_definitions,
    incomplete_features,
    indirect_structural_match,
    inline_no_sanitize,
    invalid_codeblock_attributes,
    invalid_html_tags,
    invalid_value,
    irrefutable_let_patterns,
    keyword_idents,
    late_bound_lifetime_arguments,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_copy_implementations,
    missing_crate_level_docs,
    missing_debug_implementations,
    // missing_doc_code_examples,
    missing_docs,
    mixed_script_confusables,
    mutable_borrow_reservation_conflict,
    no_mangle_generic_items,
    non_ascii_idents,
    non_camel_case_types,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    nontrivial_structural_match,
    // overlapping_range_endpoints
    path_statements,
    pointer_structural_match,
    // private_doc_tests,
    private_in_public,
    proc_macro_derive_resolution_fallback,
    redundant_semicolons,
    renamed_and_removed_lints,
    safe_packed_borrows,
    single_use_lifetimes,
    stable_features,
    trivial_bounds,
    trivial_casts,
    trivial_numeric_casts,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    unaligned_references,
    uncommon_codepoints,
    unconditional_recursion,
    unknown_lints,
    unnameable_test_items,
    unreachable_code,
    unreachable_patterns,
    unreachable_pub,
    unsafe_code,
    // unsafe_op_in_unsafe_fn,
    // unstable_features,
    unstable_name_collisions,
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
)]
#![allow(clippy::clippy::multiple_crate_versions)]

mod config;
mod constants;
mod error;
mod feature;
mod gen;

pub use crate::constants::ConstantsFlags;
pub use crate::error::Error;
pub use crate::gen::gen;
