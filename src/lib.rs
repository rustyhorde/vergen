// Copyright (c) 2016, 2018, 2021 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! # vergen - Generate Cargo Build Instructions
//! `vergen`, when used in conjunction with cargo [build scripts], will generate `cargo:` instructions.
//!
//! * The [cargo:rustc-env] instructions add environment variables that can be used with the [env!](std::env!) macro in your code.
//! * The [cargo:rerun-if-changed] instructions tell `cargo` to re-run the build script if the file at the given path has changed.
//!
//! ## Features
//! `vergen` has three features toggles allowing you to customize your output.
//!
//! | Feature | Enables |
//! | ------- | ------- |
//! |  build  | `VERGEN_BUILD_*` instructions |
//! |   git   | `VERGEN_GIT_*` instructions, the `cargo:rerun-if-changed` instructions, and the [`REBUILD_ON_HEAD_CHANGE`] flag  |
//! |  rustc  | `VERGEN_RUSTC_*` instructions |
//!
//! **NOTE** - All three features are enabled by default.
//!
//! ## Sample Output
//! If all three features are enabled, and all flags are toggled on, the build script will generate instructions for cargo similar to below
//!
//! ```text, no_run
//! cargo:rustc-env=VERGEN_BUILD_DATE=2021-02-12
//! cargo:rustc-env=VERGEN_BUILD_TIMESTAMP=2021-02-12T01:54:15.134750+00:00
//! cargo:rustc-env=VERGEN_GIT_BRANCH=feature/git2
//! cargo:rustc-env=VERGEN_GIT_COMMIT_DATE=2021-02-11T20:05:53-05:00
//! cargo:rustc-env=VERGEN_GIT_SEMVER=v3.2.0-86-g95fc0f5
//! cargo:rustc-env=VERGEN_GIT_SEMVER_LIGHTWEIGHT=blah-33-g95fc0f5
//! cargo:rustc-env=VERGEN_GIT_SHA=95fc0f5d066710f16e0c23ce3239d6e040abca0d
//! cargo:rustc-env=VERGEN_GIT_SHA_SHORT=95fc0f5
//! cargo:rustc-env=VERGEN_RUSTC_CHANNEL=nightly
//! cargo:rustc-env=VERGEN_RUSTC_COMMIT_DATE=2021-02-10
//! cargo:rustc-env=VERGEN_RUSTC_COMMIT_HASH=07194ffcd25b0871ce560b9f702e52db27ac9f77
//! cargo:rustc-env=VERGEN_RUSTC_HOST_TRIPLE=x86_64-apple-darwin
//! cargo:rustc-env=VERGEN_RUSTC_LLVM_VERSION=11.0
//! cargo:rustc-env=VERGEN_RUSTC_SEMVER=1.52.0-nightly
//! cargo:rerun-if-changed=/Users/yoda/projects/rust-lang/vergen/.git/HEAD
//! cargo:rerun-if-changed=/Users/yoda/projects/rust-lang/vergen/.git/refs/heads/feature/git2
//! ```
//!
//! ## Example Usage
//! ### Cargo.toml
//! ```toml
//! [package]
//! #..
//! build = "build.rs"
//!
//! [dependencies]
//! #..
//!
//! [build-dependencies]
//! vergen = "4"
//! # or
//! vergen = { version = "4", default-features = false, features = ["build", "rustc"] }
//! # if you wish to disable certain features
//! ```
//!
//! ### build.rs
//! **NOTE** - Individual instruction generation can be toggled on or off via [`ConstantsFlags`](crate::constants::ConstantsFlags)
//! ```
//! # use vergen::{ConstantsFlags, gen};
//! #
//! # fn main() {
//! // Setup the flags, toggling off the 'SEMVER_FROM_CARGO_PKG' flag
//! let mut flags = ConstantsFlags::all();
//! flags.toggle(ConstantsFlags::SEMVER_FROM_CARGO_PKG);
//!
//! // Generate the 'cargo:' instruction output
//! gen(flags).expect("Unable to generate the cargo keys!");
//! # }
//! ```
//!
//! ### Use in code
//! ```
//! println!("Build Timestamp: {}", env!("VERGEN_BUILD_TIMESTAMP"));
//! println!("git semver: {}", env!("VERGEN_GIT_SEMVER"));
//! ```
//!
//! ### Note on `VERGEN_SEMVER` and `VERGEN_SEMVER_LIGHTWEIGHT`
//! `VERGEN_SEMVER` and `VERGEN_SEMVER_LIGHTWEIGHT` can be generated via two methods
//! 1. [`git2::Repository::describe`]
//! 2. [`CARGO_PKG_VERSION`]
//!
//! By default, if the `git` feature is enabled semver generation will use the first method.
//! If the `git` feature is disabled or method one errors, generation falls back to the second method.
//! Note that the `git describe` method is only useful if you have tags on your repository.
//! I recommend [`SemVer`] tags, but this will work with any tag format.
//! If your repository has no tags, this method will always fall back to [`CARGO_PKG_VERSION`].
//! Also worth noting, `VERGEN_SEMVER` and `VERGEN_SEMVER_LIGHTWEIGHT` will only differ if you use [lightweight] tags in your repository.
//!
//! If you wish to force method two even if the `git` feature is enabled you may toggle off [`SEMVER`] and toggle on [`SEMVER_FROM_CARGO_PKG`].
//!
//! ### Note on `REBUILD_ON_HEAD_CHANGE`
//! `vergen` can also be configured to instruct `cargo` to re-run the build script when either `<gitpath>/HEAD` or the file that `<gitpath>/HEAD` points at changes.
//!
//! This can behavior can be toggled on or off with the [`REBUILD_ON_HEAD_CHANGE`] flag.
//!
//! [`SEMVER`]: crate::constants::ConstantsFlags::SEMVER
//! [`SEMVER_FROM_CARGO_PKG`]: crate::constants::ConstantsFlags::SEMVER_FROM_CARGO_PKG
//! [`REBUILD_ON_HEAD_CHANGE`]: crate::constants::ConstantsFlags::REBUILD_ON_HEAD_CHANGE
//! [git describe]: git2::Repository::describe
//! [`CARGO_PKG_VERSION`]: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates
//! [build scripts]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
//! [cargo:rustc-env]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-env
//! [cargo:rerun-if-changed]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed
//! [lightweight]: https://git-scm.com/book/en/v2/Git-Basics-Tagging
//! [SemVer]: https://semver.org/
//!
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

#[cfg(all(test, not(feature = "rustc")))]
use rustversion as _;
#[cfg(all(test, not(feature = "cargo")))]
use serial_test as _;

#[cfg(all(
    test,
    any(
        feature = "build",
        feature = "cargo",
        feature = "git",
        feature = "rustc"
    )
))]
pub(crate) mod test {
    use crate::config::VergenKey;
    use std::{collections::BTreeMap, convert::identity};

    pub(crate) fn get_map_value(
        key: VergenKey,
        cfg_map: &BTreeMap<VergenKey, Option<String>>,
    ) -> String {
        cfg_map
            .get(&key)
            .unwrap_or_else(|| &None)
            .clone()
            .map_or_else(String::default, identity)
    }
}
