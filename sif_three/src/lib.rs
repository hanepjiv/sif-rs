// -*- mode:rust; coding:utf-8-unix; -*-

//! lib.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/05/12
//  @date 2019/07/09

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
// rustc 1.36.0 (a53f9df32 2019-07-03)
#![deny(
    absolute_paths_not_starting_with_crate,
    anonymous_parameters,
    bare_trait_objects,
    box_pointers,
    deprecated_in_future,
    elided_lifetimes_in_paths,
    ellipsis_inclusive_range_patterns,
    keyword_idents,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    missing_doc_code_examples,
    private_doc_tests,
    question_mark_macro_sep,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_labels,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences,
    deprecated,
    exported_private_dependencies,
    illegal_floating_point_literal_pattern,
    ill_formed_attribute_input,
    improper_ctypes,
    intra_doc_link_resolution_failure,
    irrefutable_let_patterns,
    late_bound_lifetime_arguments,
    mutable_borrow_reservation_conflict,
    nested_impl_trait,
    non_camel_case_types,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    no_mangle_generic_items,
    path_statements,
    patterns_in_fns_without_body,
    plugin_as_library,
    private_in_public,
    proc_macro_derive_resolution_fallback,
    safe_packed_borrows,
    stable_features,
    trivial_bounds,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    unconditional_recursion,
    unions_with_drop_fields,
    unknown_lints,
    unnameable_test_items,
    unreachable_code,
    unreachable_patterns,
    unstable_name_collisions,
    unused_allocation,
    unused_assignments,
    unused_attributes,
    unused_comparisons,
    unused_doc_comments,
    unused_features,
    unused_imports,
    unused_macros,
    unused_must_use,
    unused_mut,
    unused_parens,
    unused_unsafe,
    unused_variables,
    where_clauses_object_safety,
    while_true,
    ambiguous_associated_items,
    const_err,
    duplicate_macro_exports,
    exceeding_bitshifts,
    invalid_type_param_default,
    legacy_constructor_visibility,
    legacy_directory_ownership,
    macro_expanded_macro_exports_accessed_by_absolute_paths,
    missing_fragment_specifier,
    mutable_transmutes,
    no_mangle_const_items,
    order_dependent_trait_objects,
    overflowing_literals,
    parenthesized_params_in_types_and_modules,
    pub_use_of_private_extern_crate,
    safe_extern_statics,
    unknown_crate_types
)]
#![warn(renamed_and_removed_lints)]
#![allow(
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    explicit_outlives_requirements,
    unreachable_pub,
    macro_use_extern_crate,
    dead_code
)]
// extern  ====================================================================
#[macro_use]
extern crate bitflags;
// ----------------------------------------------------------------------------
extern crate num;
extern crate uuid;
// ----------------------------------------------------------------------------
extern crate sif_manager;
extern crate sif_math;
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
// mod  =======================================================================
mod armature;
mod error;
mod graph;
mod trarotsca;
