// -*- mode:rust; coding:utf-8-unix; -*-

//! lib.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/05/12
//  @date 2018/08/11

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
// rustc 1.28.0 (9634041f0 2018-07-30)
#![deny(
    absolute_paths_not_starting_with_crate,
    anonymous_parameters,
    bare_trait_objects,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_labels,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences,
    deprecated,
    duplicate_associated_type_bindings,
    illegal_floating_point_literal_pattern,
    improper_ctypes,
    intra_doc_link_resolution_failure,
    late_bound_lifetime_arguments,
    non_camel_case_types,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    plugin_as_library,
    private_in_public,
    private_no_mangle_fns,
    private_no_mangle_statics,
    safe_packed_borrows,
    stable_features,
    trivial_bounds,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    unconditional_recursion,
    unions_with_drop_fields,
    unknown_lints,
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
    while_true,
    const_err,
    duplicate_macro_exports,
    exceeding_bitshifts,
    incoherent_fundamental_impls,
    invalid_type_param_default,
    legacy_constructor_visibility,
    legacy_directory_ownership,
    missing_fragment_specifier,
    mutable_transmutes,
    no_mangle_const_items,
    parenthesized_params_in_types_and_modules,
    pub_use_of_private_extern_crate,
    safe_extern_statics,
    unknown_crate_types
)]
#![warn(dead_code, renamed_and_removed_lints, unreachable_pub)]
#![allow(
    box_pointers,
    elided_lifetimes_in_paths,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code
)]
// extern  ====================================================================
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate log;
// ----------------------------------------------------------------------------
extern crate gl;
extern crate num;
extern crate rand;
#[cfg(feature = "lbf-rlua")]
extern crate rlua;
extern crate sdl2;
extern crate uuid;
// ----------------------------------------------------------------------------
#[macro_use]
extern crate sif_error;
#[macro_use]
extern crate sif_renderer;
// ----------------------------------------------------------------------------
extern crate sif_manager;
extern crate sif_math;
extern crate sif_three;
// use  =======================================================================
pub use self::{
    animation::{
        Animation, Curve, CurveType, Driver as AnimationDriver, Interpolation,
        Keyframe,
    },
    camera::{Camera, CameraType},
    color::{ColorExponent, ColorIntensity},
    element::{Element, ELEMENT_SIZE},
    error::{Error, Result},
    image::{Image, ImageFile, ImageProcedual, ImageProcedualMethod},
    into_graphics::IntoGraphics,
    light::{Flags as LightFlags, Light},
    material::{Flags as MaterialFlags, Material, Parallax},
    mesh::Mesh,
    model::Model,
    object::{Object, ObjectData},
    offsets::Offsets,
    pipeline::{
        Flags as PipelineFlags, Pipeline, PipelineParam, PIPELINE_MAX_BONE,
        PIPELINE_MAX_LIGHT,
    },
    post::{
        Blur, DepthMap, DepthMapParam, Effect, EffectArgs, Pass, Screen,
        SquareBuffer,
    },
    scene::Scene,
    shadow::Shadow,
    submesh::SubMesh,
    text::{Font, FontReserve, Layer as TextLayer, Metal as TextMetal},
    texture::Texture,
};
// mod  =======================================================================
mod animation;
mod camera;
mod color;
mod element;
mod error;
mod image;
mod into_graphics;
pub mod lbf;
mod light;
mod material;
mod mesh;
mod model;
mod object;
mod offsets;
mod pipeline;
mod post;
mod scene;
mod shadow;
mod submesh;
mod text;
mod texture;
