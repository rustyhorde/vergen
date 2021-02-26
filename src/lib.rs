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
//! ## Uses
//! I personally use `vergen` for two use cases.
//!
//! The first is generating verbose output describing a command line application.
//!
//! ```text
//! ~/p/r/app λ app -vv
//! app 0.1.0
//!
//! Build Timestamp:     2021-02-23T20:14:46.558472672+00:00
//! Build Version:       0.1.0-9-g46f83e1
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
//! The second is information endpoints in web apis
//!
//! ```json
//! ~/p/r/app λ curl https://some.app.com/info | jq
//! {
//!   "build_timestamp": "2021-02-19T21:32:22.932833758+00:00",
//!   "git_semver": "0.0.0-7-gc96c096",
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
//! ## Features
//! `vergen` has four feature toggles allowing you to customize your output.
//!
//! | Feature | Enables |
//! | ------- | ------- |
//! |  build  | `VERGEN_BUILD_*` instructions |
//! |  cargo  | `VERGEN_CARGO_*` instructions |
//! |   git   | `VERGEN_GIT_*` instructions and the `cargo:rerun-if-changed` instructions  |
//! |  rustc  | `VERGEN_RUSTC_*` instructions |
//!
//! **NOTE** - All four features are enabled by default.
//!
//! ## Sample Output
//! If all features are enabled and the default [`Config`] is used the build script will generate instructions for cargo similar to below.
//!
//! Please see [`Config`](crate::Config) for more details on instruction generation.
//!
//! ```text, no_run
//! cargo:rustc-env=VERGEN_BUILD_TIMESTAMP=2021-02-25T23:28:39.493201+00:00
//! cargo:rustc-env=VERGEN_BUILD_SEMVER=4.1.0
//! cargo:rustc-env=VERGEN_GIT_BRANCH=feature/fun
//! cargo:rustc-env=VERGEN_GIT_COMMIT_TIMESTAMP=2021-02-24T20:55:21+00:00
//! cargo:rustc-env=VERGEN_GIT_SEMVER=4.1.0-2-gf49246c
//! cargo:rustc-env=VERGEN_GIT_SHA=f49246ce334567bff9f950bfd0f3078184a2738a
//! cargo:rustc-env=VERGEN_RUSTC_CHANNEL=nightly
//! cargo:rustc-env=VERGEN_RUSTC_COMMIT_DATE=2021-02-24
//! cargo:rustc-env=VERGEN_RUSTC_COMMIT_HASH=a8486b64b0c87dabd045453b6c81500015d122d6
//! cargo:rustc-env=VERGEN_RUSTC_HOST_TRIPLE=x86_64-apple-darwin
//! cargo:rustc-env=VERGEN_RUSTC_LLVM_VERSION=11.0
//! cargo:rustc-env=VERGEN_RUSTC_SEMVER=1.52.0-nightly
//! cargo:rustc-env=VERGEN_CARGO_TARGET_TRIPLE=x86_64-unknown-linux-gnu
//! cargo:rustc-env=VERGEN_CARGO_PROFILE=debug
//! cargo:rustc-env=VERGEN_CARGO_FEATURES=git,build
//! cargo:rerun-if-changed=/Users/yoda/projects/rust-lang/vergen/.git/HEAD
//! cargo:rerun-if-changed=/Users/yoda/projects/rust-lang/vergen/.git/refs/heads/feature/fun
//! ```
//!
//! ## Usage
//!
//! 1. Ensure you have build scripts enabled via the `build` configuration in your `Cargo.toml`
//! 1. Add `vergen` as a build dependency, optionally disabling default features in your `Cargo.toml`
//! 1. Create a `build.rs` file that uses `vergen` to generate `cargo:` instructions.
//! 1. Use the `env!` macro in your code
//!
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
//! **NOTE** - Individual instruction generation can be toggled on or off via [`Config`](crate::Config)
//! ```
//! use vergen::{Config, Error, vergen};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!   // Generate the default 'cargo:' instruction output
//!   vergen(Config::default()).map_err(Error::into)
//! }
//! ```
//!
//! ### Use in code
//! ```
//! println!("Build Timestamp: {}", env!("VERGEN_BUILD_TIMESTAMP"));
//! println!("git semver: {}", env!("VERGEN_GIT_SEMVER"));
//! ```
//!
//! [build scripts]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
//! [cargo:rustc-env]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-env
//! [cargo:rerun-if-changed]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed
//!
//! ## Deprecation Warning
//! The [`gen`](gen()) function and [`ConstantsFlags`](crate::ConstantsFlags) have been deprecated.  They will be removed when version 5 is released.
//!
//! Please switch to using the [`vergen`](vergen()) function with [`Config`](crate::Config) instead.
//!
//! This change was made because the [`ConstantsFlags`](crate::ConstantsFlags) were growing bloated.
//!
#![deny(
    absolute_paths_not_starting_with_crate,
    anonymous_parameters,
    array_into_iter,
    asm_sub_register,
    bare_trait_objects,
    bindings_with_variant_name,
    box_pointers,
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
    // deprecated_in_future,
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
    overlapping_range_endpoints,
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
    unstable_features,
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
#![allow(clippy::multiple_crate_versions)]

mod config;
mod constants;
mod error;
mod feature;
mod gen;

pub use crate::config::Instructions as Config;
#[deprecated(since = "4.2.0", note = "Please use `Config` instead")]
pub use crate::constants::ConstantsFlags;
pub use crate::error::Error;
#[cfg(feature = "build")]
pub use crate::feature::Build;
#[cfg(feature = "cargo")]
pub use crate::feature::Cargo;
#[cfg(feature = "git")]
pub use crate::feature::Git;
#[cfg(feature = "rustc")]
pub use crate::feature::Rustc;
#[cfg(feature = "git")]
pub use crate::feature::SemverKind;
#[cfg(feature = "git")]
pub use crate::feature::ShaKind;
#[cfg(any(feature = "git", feature = "build"))]
pub use crate::feature::TimeZone;
#[cfg(any(feature = "git", feature = "build"))]
pub use crate::feature::TimestampKind;
#[allow(deprecated)]
pub use crate::gen::gen;
pub use crate::gen::vergen;

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

#[cfg(test)]
pub(crate) mod testutils {
    use std::env;

    pub(crate) fn setup() {
        env::set_var("TARGET", "x86_64-unknown-linux-gnu");
        env::set_var("PROFILE", "debug");
        env::set_var("CARGO_FEATURE_GIT", "git");
        env::set_var("CARGO_FEATURE_BUILD", "build");
    }

    pub(crate) fn teardown() {
        env::remove_var("TARGET");
        env::remove_var("PROFILE");
        env::remove_var("CARGO_FEATURE_GIT");
        env::remove_var("CARGO_FEATURE_BUILD");
    }
}
