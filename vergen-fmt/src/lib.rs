// Copyright (c) 2022 vergen developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `vergen-fmt` - A pretty printer for vergen environment variables
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

mod fmt;

pub use fmt::display::display;
#[cfg(feature = "trace")]
pub use fmt::trace::trace;
