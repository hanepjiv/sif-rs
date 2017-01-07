// -*- mode:rust; coding:utf-8-unix; -*-

//! lib.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/06/18
//  @date 2016/12/29

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
#![deny(fat_ptr_transmutes, missing_copy_implementations,
        missing_debug_implementations, missing_docs, unstable_features,
        unused_qualifications, unused_results, variant_size_differences)]
#![warn(unused_extern_crates, warnings)]
#![allow(box_pointers, trivial_casts, trivial_numeric_casts, unsafe_code,
         unused_import_braces)]
// use  =======================================================================
// extern  ====================================================================
extern crate                    gl;
extern crate                    image;
#[macro_use] extern crate       log;
extern crate                    num;
extern crate                    uuid;
// mod  =======================================================================
pub mod                         error;
#[macro_use] pub mod            unwrap;
pub mod                         math;
pub mod                         geometry;
pub mod                         renderer;
