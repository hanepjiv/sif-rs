/* -*- mode:rust; coding:utf-8-unix; -*- */

//! lib.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/06/18
//  @date 2016/06/18

/* ////////////////////////////////////////////////////////////////////////// */
/* attribute  =============================================================== */
#![deny(missing_docs, dead_code, unused_imports, unused_variables)]
/* use  ===================================================================== */
/* extern  ================================================================== */
extern crate gl;
extern crate image;
extern crate libc;
#[macro_use] extern crate log;
extern crate num;
extern crate uuid;
/* mod  ===================================================================== */
#[macro_use] mod unwrap;
pub mod math;
pub mod geometry;
pub mod renderer;
