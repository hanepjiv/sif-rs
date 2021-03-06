// -*- mode:rust; coding:utf-8-unix; -*-

//! lib.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/05/12
//  @date 2020/08/28

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
// rustc 1.46.0 (04488afe3 2020-08-24)
#![allow(unused_crate_dependencies)]
#![deny(
    absolute_paths_not_starting_with_crate,
    anonymous_parameters,
    box_pointers,
    clashing_extern_declarations,
    deprecated_in_future,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    indirect_structural_match,
    keyword_idents,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_copy_implementations,
    missing_crate_level_docs,
    missing_debug_implementations,
    missing_docs,
    missing_doc_code_examples,
    non_ascii_idents,
    private_doc_tests,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unaligned_references,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences,
    array_into_iter,
    asm_sub_register,
    bare_trait_objects,
    bindings_with_variant_name,
    cenum_impl_drop_cast,
    coherence_leak_check,
    confusable_idents,
    deprecated,
    ellipsis_inclusive_range_patterns,
    exported_private_dependencies,
    illegal_floating_point_literal_pattern,
    improper_ctypes,
    improper_ctypes_definitions,
    incomplete_features,
    inline_no_sanitize,
    intra_doc_link_resolution_failure,
    invalid_codeblock_attributes,
    invalid_value,
    irrefutable_let_patterns,
    late_bound_lifetime_arguments,
    mixed_script_confusables,
    mutable_borrow_reservation_conflict,
    non_camel_case_types,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    no_mangle_generic_items,
    overlapping_patterns,
    path_statements,
    private_in_public,
    proc_macro_derive_resolution_fallback,
    redundant_semicolons,
    safe_packed_borrows,
    stable_features,
    trivial_bounds,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    uncommon_codepoints,
    unconditional_recursion,
    unknown_lints,
    unnameable_test_items,
    unreachable_code,
    unreachable_patterns,
    unstable_name_collisions,
    unused_allocation,
    unused_assignments,
    unused_attributes,
    unused_braces,
    unused_comparisons,
    unused_doc_comments,
    unused_features,
    unused_imports,
    unused_labels,
    unused_macros,
    unused_must_use,
    unused_mut,
    unused_parens,
    unused_unsafe,
    unused_variables,
    where_clauses_object_safety,
    while_true,
    ambiguous_associated_items,
    arithmetic_overflow,
    conflicting_repr_hints,
    const_err,
    ill_formed_attribute_input,
    incomplete_include,
    invalid_type_param_default,
    macro_expanded_macro_exports_accessed_by_absolute_paths,
    missing_fragment_specifier,
    mutable_transmutes,
    no_mangle_const_items,
    order_dependent_trait_objects,
    overflowing_literals,
    patterns_in_fns_without_body,
    pub_use_of_private_extern_crate,
    soft_unstable,
    unconditional_panic,
    unknown_crate_types
)]
#![warn(unsafe_code, dead_code, renamed_and_removed_lints, unreachable_pub)]
// mod  =======================================================================
mod armature;
mod error;
mod graph;
mod trarotsca;
// use  =======================================================================
pub use self::armature::{Armature, Bone, Pose, PoseFlags};
pub use self::error::{Error, Result};
pub use self::graph::{
    AsNodeHolder, Graph, Node, NodeHolder, NodeHolderField,
};
pub use self::trarotsca::{
    new_mat4_inverse_trarotsca, new_mat4_rot, new_mat4_sca, new_mat4_tra,
    new_mat4_trarotsca, new_quaternion_rot, TraRotSca, TraRotScaType,
};
