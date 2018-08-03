// -*- mode:rust; coding:utf-8-unix; -*-

//! mod.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/27
//  @date 2018/08/03

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
pub use super::{Error, Result};
// ----------------------------------------------------------------------------
pub use self::{
    font::{Font, FontReserve},
    layer::Layer,
    metal::Metal,
};
// mod  =======================================================================
mod font;
mod glyph;
mod layer;
mod metal;
