// -*- mode:rust; coding:utf-8-unix; -*-

//! mod.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/27
//  @date 2018/05/11

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
pub use super::{Error, Result};
// ----------------------------------------------------------------------------
pub use self::font::{Font, TFontReserve};
pub use self::layer::{Layer, TLayerAppend};
pub use self::metal::Metal;
// mod  =======================================================================
pub mod font;
mod glyph;
pub mod layer;
pub mod metal;
