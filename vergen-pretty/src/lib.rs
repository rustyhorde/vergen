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
#![cfg_attr(
    feature = "header",
    doc = r"If you enable the header feature, you can also use the [`header()`] function
with the associated [`Config`] as a convenience wrapper around [`Pretty`].

# Example
```
# use anyhow::Result;
# use vergen_pretty::{ConfigBuilder, header, vergen_pretty_env};"
)]
#![cfg_attr(feature = "color", doc = r"# use vergen_pretty::Style;")]
#![cfg_attr(
    feature = "header",
    doc = r"
#
# pub fn main() -> Result<()> {
let mut buf = vec![];
let config = ConfigBuilder::default()"
)]
#![cfg_attr(
    all(feature = "color", feature = "header"),
    doc = r"    .style(Style::new().green())"
)]
#![cfg_attr(
    feature = "header",
    doc = r#"
    .prefix("HEADER_PREFIX")
    .env(vergen_pretty_env!())
    .suffix("HEADER_SUFFIX")
    .build()?;
assert!(header(&config, Some(&mut buf)).is_ok());
assert!(!buf.is_empty());
#     Ok(())
# }
```
"#
)]
//!
//! ## Features
//! `vergen-pretty` has two feature toggles allowing you to customize your output. No features are enabled by default.
//! You **must** specifically enable the features you wish to use.
//!
//! | Feature | Enables |
//! | ------- | ------- |
//! |  color  | Colorize output, allow configuration of coloring via [`console`] |
//! |  header | Generate pretty printed header output based on the given [`Config`] |
//! |  trace  | Enable support for [`tracing`](https://docs.rs/tracing/latest/tracing/) output |
//!

// rustc lints
#![cfg_attr(
    all(feature = "unstable", nightly),
    feature(
        multiple_supertrait_upcastable,
        must_not_suspend,
        non_exhaustive_omitted_patterns_lint,
        rustdoc_missing_doc_code_examples,
        strict_provenance_lints,
        supertrait_item_shadowing,
        unqualified_local_imports,
    )
)]
#![cfg_attr(nightly, allow(single_use_lifetimes, unexpected_cfgs))]
#![cfg_attr(
    nightly,
    deny(
        abi_unsupported_vector_types,
        absolute_paths_not_starting_with_crate,
        ambiguous_glob_imports,
        ambiguous_glob_reexports,
        ambiguous_negative_literals,
        ambiguous_wide_pointer_comparisons,
        anonymous_parameters,
        array_into_iter,
        asm_sub_register,
        async_fn_in_trait,
        bad_asm_style,
        bare_trait_objects,
        boxed_slice_into_iter,
        break_with_label_and_loop,
        clashing_extern_declarations,
        closure_returning_async_block,
        coherence_leak_check,
        confusable_idents,
        const_evaluatable_unchecked,
        const_item_mutation,
        dangling_pointers_from_temporaries,
        dead_code,
        dependency_on_unit_never_type_fallback,
        deprecated,
        deprecated_in_future,
        deprecated_safe_2024,
        deprecated_where_clause_location,
        deref_into_dyn_supertrait,
        deref_nullptr,
        double_negations,
        drop_bounds,
        dropping_copy_types,
        dropping_references,
        duplicate_macro_attributes,
        dyn_drop,
        // Add this back with 2024 edition
        // edition_2024_expr_fragment_specifier,
        elided_lifetimes_in_paths,
        elided_named_lifetimes,
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
        if_let_rescope,
        impl_trait_overcaptures,
        impl_trait_redundant_captures,
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
        missing_unsafe_on_extern,
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
        out_of_scope_macro_calls,
        overlapping_range_endpoints,
        path_statements,
        private_bounds,
        private_interfaces,
        ptr_to_integer_transmute_in_consts,
        redundant_imports,
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
        rust_2024_guarded_string_incompatible_syntax,
        rust_2024_incompatible_pat,
        rust_2024_prelude_collisions,
        self_constructor_from_outer_item,
        semicolon_in_expressions_from_macros,
        single_use_lifetimes,
        special_module_name,
        stable_features,
        static_mut_refs,
        suspicious_double_ref_op,
        tail_expr_drop_order,
        trivial_bounds,
        trivial_casts,
        trivial_numeric_casts,
        type_alias_bounds,
        tyvar_behind_raw_pointer,
        uncommon_codepoints,
        unconditional_recursion,
        uncovered_param_in_projection,
        undefined_naked_function_abi,
        unfulfilled_lint_expectations,
        ungated_async_fn_track_caller,
        uninhabited_static,
        unit_bindings,
        unknown_lints,
        unknown_or_malformed_diagnostic_attributes,
        unnameable_test_items,
        unnameable_types,
        unpredictable_function_pointer_comparisons,
        unreachable_code,
        unreachable_patterns,
        unreachable_pub,
        unsafe_attr_outside_unsafe,
        unsafe_code,
        unsafe_op_in_unsafe_fn,
        unstable_name_collisions,
        unstable_syntax_pre_expansion,
        unsupported_fn_ptr_calling_conventions,
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
        uses_power_alignment,
        variant_size_differences,
        wasm_c_abi,
        while_true,
    )
)]
// If nightly and unstable, allow `incomplete_features` and `unstable_features`
#![cfg_attr(
    all(feature = "unstable", nightly),
    allow(incomplete_features, unstable_features)
)]
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
        supertrait_item_shadowing_definition,
        supertrait_item_shadowing_usage,
        unqualified_local_imports,
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

#[cfg(feature = "header")]
mod header;
mod pretty;
mod utils;

#[cfg(feature = "header")]
pub use self::header::header;
#[cfg(feature = "header")]
pub use self::header::Config;
#[cfg(feature = "header")]
pub use self::header::ConfigBuilder;
#[cfg(feature = "header")]
pub use self::header::Env;
pub use self::pretty::prefix::Prefix;
pub use self::pretty::prefix::PrefixBuilder;
pub use self::pretty::suffix::Suffix;
pub use self::pretty::suffix::SuffixBuilder;
pub use self::pretty::Pretty;
pub use self::pretty::PrettyBuilder;
pub use self::pretty::PrettyBuilderError;
#[cfg(feature = "color")]
#[doc(inline)]
pub use console::Style;
#[cfg(feature = "trace")]
#[doc(inline)]
pub use tracing::Level;

#[cfg(all(feature = "header", not(feature = "color")))]
use rand as _;
#[cfg(test)]
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
        let _old = map.insert(
            "VERGEN_CARGO_DEPENDENCIES",
            option_env!("VERGEN_CARGO_DEPENDENCIES"),
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
        let _old = map.insert("VERGEN_GIT_DIRTY", option_env!("VERGEN_GIT_DIRTY"));
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
    ( $( $x:expr ),* ) => {{
        {
            let mut map = $crate::vergen_pretty_env!();
            $(
                let _old = map.insert($x, option_env!($x));
            )*
            map
        }
    }};
}
