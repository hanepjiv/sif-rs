// -*- mode:rust; coding:utf-8-unix; -*-

//! lib.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/05/12
//  @date 2018/05/26

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
// rustc 1.26.0 (a77568041 2018-05-07)
#![deny(
    anonymous_parameters, missing_copy_implementations,
    missing_debug_implementations, missing_docs, unstable_features,
    unused_extern_crates, unused_import_braces, unused_qualifications,
    unused_results, variant_size_differences, const_err, deprecated,
    illegal_floating_point_literal_pattern, improper_ctypes,
    incoherent_fundamental_impls, late_bound_lifetime_arguments,
    non_camel_case_types, non_shorthand_field_patterns, non_snake_case,
    non_upper_case_globals, no_mangle_generic_items, overflowing_literals,
    path_statements, patterns_in_fns_without_body, plugin_as_library,
    private_in_public, private_no_mangle_fns, private_no_mangle_statics,
    safe_packed_borrows, stable_features, type_alias_bounds,
    tyvar_behind_raw_pointer, unconditional_recursion, unions_with_drop_fields,
    unknown_lints, unreachable_code, unreachable_patterns,
    unstable_name_collision, unused_allocation, unused_assignments,
    unused_attributes, unused_comparisons, unused_doc_comment, unused_features,
    unused_imports, unused_macros, unused_must_use, unused_mut, unused_parens,
    unused_unsafe, unused_variables, while_true, exceeding_bitshifts,
    invalid_type_param_default, legacy_constructor_visibility,
    legacy_directory_ownership, legacy_imports, missing_fragment_specifier,
    mutable_transmutes, no_mangle_const_items,
    parenthesized_params_in_types_and_modules, pub_use_of_private_extern_crate,
    safe_extern_statics, unknown_crate_types
)]
#![warn(
    bare_trait_object, dead_code, renamed_and_removed_lints, unreachable_pub
)]
#![allow(
    box_pointers, elided_lifetime_in_path, unsafe_code, trivial_casts,
    single_use_lifetime, trivial_numeric_casts
)]
// extern  ====================================================================
extern crate num;
// ----------------------------------------------------------------------------
extern crate sif_error;
// use  =======================================================================
pub use sif_error::*;
// ----------------------------------------------------------------------------
pub use self::cleanup::Cleanup;
pub use self::matrix::{
    Matrix2x2, Matrix2x3, Matrix3x2, Matrix3x3, Matrix3x4, Matrix4x3,
    Matrix4x4,
};
pub use self::number::Number;
pub use self::quaternion::Quaternion;
pub use self::vector::{Vector2, Vector3, Vector4};
// mod  =======================================================================
mod cleanup;
mod matrix;
mod number;
mod quaternion;
mod vector;
