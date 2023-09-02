// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `vergen-pretty` - A pretty printer for vergen environment variables
//!
//! Because `cargo` doesn't pass compile time environment variables to dependencies,
//! the [`vergen_pretty_env`] macro embeds a map of all the possible `vergen` environment variables with
//! [`option_env!`](std::option_env!).  Values not set in by your `build.rs` are skipped
//! when pretty-printing the output.
//!
//! # Example
//! ```
//! # use anyhow::Result;
//! # use std::{collections::BTreeMap, io::Write};
//! # use vergen_pretty::{vergen_pretty_env, PrettyBuilder};
//! # fn has_value(
//! #     tuple: (&&'static str, &Option<&'static str>),
//! # ) -> Option<(&'static str, &'static str)> {
//! #     let (key, value) = tuple;
//! #     if value.is_some() {
//! #         Some((*key, value.unwrap_or_default()))
//! #     } else {
//! #         None
//! #     }
//! # }
//! # fn is_empty(map: &BTreeMap<&'static str, Option<&'static str>>) -> bool {
//! #     map.iter().filter_map(has_value).count() == 0
//! # }
//! # fn main() -> Result<()> {
//! let mut stdout = vec![];
//! # let map = vergen_pretty_env!();
//! # let empty = is_empty(&map);
//! PrettyBuilder::default()
//!     .env(vergen_pretty_env!())
//!     .build()?
//!     .display(&mut stdout)?;
//! # if empty {
//! #    assert!(stdout.is_empty());
//! # } else {
//! assert!(!stdout.is_empty());
//! # }
//! #     Ok(())
//! # }
//! ```
//!
//! See the [`Pretty`] documentation for more examples
//!
//! ## Features
//! `vergen-pretty` has two feature toggles allowing you to customize your output. No features are enabled by default.  
//! You **must** specifically enable the features you wish to use.
//!
//! | Feature | Enables |
//! | ------- | ------- |
//! |  color  | Colorize output, allow configuration of coloring via [`console`] |
//! |  trace  | Enable support for [`tracing`](https://docs.rs/tracing/latest/tracing/) output |
//!

#![cfg_attr(docsrs, feature(doc_cfg))]
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
        unused_tuple_struct_fields,
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
    deny(
        ambiguous_glob_imports,
        invalid_reference_casting,
        unknown_diagnostic_attributes
    )
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
// #![cfg_attr(all(msrv, any(beta, stable)), deny())]
// stable only lints
#![cfg_attr(
    all(msrv, stable),
    deny(bindings_with_variant_name, implied_bounds_entailment)
)]
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
#![cfg_attr(all(doc, nightly), feature(doc_auto_cfg))]
#![cfg_attr(all(docsrs, nightly), feature(doc_cfg))]

#[cfg(feature = "header")]
mod header;
mod pretty;
mod utils;

#[cfg(feature = "color")]
#[doc(inline)]
pub use console::Style;
#[cfg(feature = "header")]
pub use header::header;
#[cfg(feature = "header")]
pub use header::Config;
#[cfg(feature = "header")]
pub use header::ConfigBuilder;
#[cfg(feature = "header")]
pub use header::Env;
pub use pretty::prefix::Prefix;
pub use pretty::prefix::PrefixBuilder;
pub use pretty::suffix::Suffix;
pub use pretty::suffix::SuffixBuilder;
pub use pretty::Pretty;
pub use pretty::PrettyBuilder;
pub use pretty::PrettyBuilderError;
#[cfg(feature = "trace")]
#[doc(inline)]
pub use tracing::Level;

#[cfg(all(test, not(feature = "header")))]
use lazy_static as _;
#[cfg(all(feature = "header", not(feature = "color")))]
use rand as _;
#[cfg(all(test, not(feature = "header")))]
use regex as _;
#[cfg(all(test, not(feature = "serde")))]
use serde_json as _;
#[cfg(all(test, not(feature = "trace")))]
use tracing_subscriber as _;

/// Used to initialize `env` in [`PrettyBuilder`](self::PrettyBuilder)
///
/// Because `cargo` doesn't pass compile time environment variables to dependencies,
/// this macro embeds a map of all the possible `vergen` environment variables with
/// [`option_env!`](std::option_env!).  Non-set values are skipped when pretty-printing the output.
///
/// # Example
/// ```
/// # use anyhow::Result;
/// # use std::{collections::BTreeMap, io::Write};
/// # use vergen_pretty::{vergen_pretty_env, PrettyBuilder};
/// #
/// # fn main() -> Result<()> {
/// let mut stdout = vec![];
/// PrettyBuilder::default()
///     .env(vergen_pretty_env!())
///     .build()?
///     .display(&mut stdout)?;
/// #     Ok(())
/// # }
#[macro_export]
macro_rules! vergen_pretty_env {
    () => {{
        use std::collections::BTreeMap;
        let mut map = BTreeMap::new();
        let _old = map.insert("VERGEN_BUILD_DATE", option_env!("VERGEN_BUILD_DATE"));
        let _old = map.insert(
            "VERGEN_BUILD_TIMESTAMP",
            option_env!("VERGEN_BUILD_TIMESTAMP"),
        );
        let _old = map.insert("VERGEN_CARGO_DEBUG", option_env!("VERGEN_CARGO_DEBUG"));
        let _old = map.insert(
            "VERGEN_CARGO_FEATURES",
            option_env!("VERGEN_CARGO_FEATURES"),
        );
        let _old = map.insert(
            "VERGEN_CARGO_OPT_LEVEL",
            option_env!("VERGEN_CARGO_OPT_LEVEL"),
        );
        let _old = map.insert(
            "VERGEN_CARGO_TARGET_TRIPLE",
            option_env!("VERGEN_CARGO_TARGET_TRIPLE"),
        );
        let _old = map.insert("VERGEN_GIT_BRANCH", option_env!("VERGEN_GIT_BRANCH"));
        let _old = map.insert(
            "VERGEN_GIT_COMMIT_AUTHOR_EMAIL",
            option_env!("VERGEN_GIT_COMMIT_AUTHOR_EMAIL"),
        );
        let _old = map.insert(
            "VERGEN_GIT_COMMIT_AUTHOR_NAME",
            option_env!("VERGEN_GIT_COMMIT_AUTHOR_NAME"),
        );
        let _old = map.insert(
            "VERGEN_GIT_COMMIT_COUNT",
            option_env!("VERGEN_GIT_COMMIT_COUNT"),
        );
        let _old = map.insert(
            "VERGEN_GIT_COMMIT_DATE",
            option_env!("VERGEN_GIT_COMMIT_DATE"),
        );
        let _old = map.insert(
            "VERGEN_GIT_COMMIT_MESSAGE",
            option_env!("VERGEN_GIT_COMMIT_MESSAGE"),
        );
        let _old = map.insert(
            "VERGEN_GIT_COMMIT_TIMESTAMP",
            option_env!("VERGEN_GIT_COMMIT_TIMESTAMP"),
        );
        let _old = map.insert("VERGEN_GIT_DESCRIBE", option_env!("VERGEN_GIT_DESCRIBE"));
        let _old = map.insert("VERGEN_GIT_SHA", option_env!("VERGEN_GIT_SHA"));
        let _old = map.insert("VERGEN_RUSTC_CHANNEL", option_env!("VERGEN_RUSTC_CHANNEL"));
        let _old = map.insert(
            "VERGEN_RUSTC_COMMIT_DATE",
            option_env!("VERGEN_RUSTC_COMMIT_DATE"),
        );
        let _old = map.insert(
            "VERGEN_RUSTC_COMMIT_HASH",
            option_env!("VERGEN_RUSTC_COMMIT_HASH"),
        );
        let _old = map.insert(
            "VERGEN_RUSTC_HOST_TRIPLE",
            option_env!("VERGEN_RUSTC_HOST_TRIPLE"),
        );
        let _old = map.insert(
            "VERGEN_RUSTC_LLVM_VERSION",
            option_env!("VERGEN_RUSTC_LLVM_VERSION"),
        );
        let _old = map.insert("VERGEN_RUSTC_SEMVER", option_env!("VERGEN_RUSTC_SEMVER"));
        let _old = map.insert("VERGEN_SYSINFO_NAME", option_env!("VERGEN_SYSINFO_NAME"));
        let _old = map.insert(
            "VERGEN_SYSINFO_OS_VERSION",
            option_env!("VERGEN_SYSINFO_OS_VERSION"),
        );
        let _old = map.insert("VERGEN_SYSINFO_USER", option_env!("VERGEN_SYSINFO_USER"));
        let _old = map.insert(
            "VERGEN_SYSINFO_TOTAL_MEMORY",
            option_env!("VERGEN_SYSINFO_TOTAL_MEMORY"),
        );
        let _old = map.insert(
            "VERGEN_SYSINFO_CPU_VENDOR",
            option_env!("VERGEN_SYSINFO_CPU_VENDOR"),
        );
        let _old = map.insert(
            "VERGEN_SYSINFO_CPU_CORE_COUNT",
            option_env!("VERGEN_SYSINFO_CPU_CORE_COUNT"),
        );
        let _old = map.insert(
            "VERGEN_SYSINFO_CPU_NAME",
            option_env!("VERGEN_SYSINFO_CPU_NAME"),
        );
        let _old = map.insert(
            "VERGEN_SYSINFO_CPU_BRAND",
            option_env!("VERGEN_SYSINFO_CPU_BRAND"),
        );
        let _old = map.insert(
            "VERGEN_SYSINFO_CPU_FREQUENCY",
            option_env!("VERGEN_SYSINFO_CPU_FREQUENCY"),
        );
        map
    }};
}
