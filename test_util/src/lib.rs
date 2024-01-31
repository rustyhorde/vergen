// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `test_util` - Test utilities for the `vergen` libraries
//!
//! ## `vergen` -  cargo feature testing utilities
//!
//! There are a couple of functions that are wrappers around the [`temp_env::with_vars`]
//! function that serves to replicate some cargo environment variables.  These are
//! mainly useful for testing the cargo feature in `vergen`.
//!
//! # Example
//! ```
//! # use anyhow::Result;
//! # use std::env::var;
//! # use test_util::with_cargo_vars;
//! #
//! let result = with_cargo_vars(|| {
//!     assert_eq!(var("OPT_LEVEL")?, "1");
//!     Ok(())
//! });
//! assert!(result.is_ok());
//! ```
//!
//! ```
//! # use anyhow::Result;
//! # use std::env::var;
//! # use test_util::with_cargo_vars_ext;
//! #
//! let result = with_cargo_vars_ext(&[("MY_VAR", Some("12"))], || {
//!     assert_eq!(var("MY_VAR")?, "12");
//!     assert_eq!(var("OPT_LEVEL")?, "1");
//!     Ok(())
//! });
//! assert!(result.is_ok());
//! ```
//!
#![cfg_attr(
    feature = "repo",
    doc = r##"## `vergen` - Test git repositories (`repo` feature)

If you enable the `repo` feature of `test_util` you can also use
the [`TestRepos`] struct to creat temporary git repositories useful for `vergen-gi*` testing

# Example
 ```
 # use anyhow::Result;
 # use std::path::PathBuf;
 # use test_util::TestRepos;
 # pub fn main() -> Result<()> {
 let mut path = PathBuf::default();
 {
     let repo = TestRepos::new(false, false, false)?;
     path = repo.path();
     assert!(gix::discover(&path).is_ok());
     assert!(path.exists());
 }
 // When dropped, the repositories will be removed.
 assert!(!path.exists());
 #     Ok(())
 # }
 ```
"##
)]
// rustc lints
#![cfg_attr(
    all(feature = "unstable", nightly),
    feature(
        diagnostic_namespace,
        lint_reasons,
        multiple_supertrait_upcastable,
        must_not_suspend,
        non_exhaustive_omitted_patterns_lint,
        rustdoc_missing_doc_code_examples,
        strict_provenance,
        type_privacy_lints,
    )
)]
#![cfg_attr(nightly, allow(box_pointers))]
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
        const_evaluatable_unchecked,
        const_item_mutation,
        const_patterns_without_partial_eq,
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
        illegal_floating_point_literal_pattern,
        improper_ctypes,
        improper_ctypes_definitions,
        incomplete_features,
        indirect_structural_match,
        inline_no_sanitize,
        internal_features,
        invalid_doc_attributes,
        invalid_from_utf8,
        invalid_macro_export_arguments,
        invalid_nan_comparisons,
        invalid_value,
        irrefutable_let_patterns,
        keyword_idents,
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
        private_bounds,
        private_interfaces,
        redundant_semicolons,
        refining_impl_trait,
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
        static_mut_ref,
        suspicious_auto_trait_impls,
        suspicious_double_ref_op,
        temporary_cstring_as_ptr,
        trivial_bounds,
        trivial_casts,
        trivial_numeric_casts,
        type_alias_bounds,
        tyvar_behind_raw_pointer,
        uncommon_codepoints,
        unconditional_recursion,
        undefined_naked_function_abi,
        unexpected_cfgs,
        ungated_async_fn_track_caller,
        uninhabited_static,
        unit_bindings,
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
        where_clauses_object_safety,
        while_true,
        writes_through_immutable_pointer,
    )
)]
// If nightly and unstable, allow `unstable_features`
#![cfg_attr(all(feature = "unstable", nightly), allow(unstable_features))]
// If nightly and not unstable, deny `unstable_features`
#![cfg_attr(all(not(feature = "unstable"), nightly), deny(unstable_features))]
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
        unknown_or_malformed_diagnostic_attributes,
        unnameable_types,
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

#[cfg(all(test, not(feature = "repo")))]
use {anyhow as _, serial_test as _};

#[cfg(feature = "repo")]
mod repo;
mod utils;

#[cfg(feature = "repo")]
pub use repo::TestRepos;
pub use utils::with_cargo_vars;
pub use utils::with_cargo_vars_ext;
