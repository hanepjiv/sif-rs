// -*- mode:rust; coding:utf-8-unix; -*-

//! into_graphics.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/07/31
//  @date 2018/08/05

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use super::Result;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// trait IntoGraphics
pub trait IntoGraphics: ::std::fmt::Debug {
    /// type Target
    type Target;
    /// type Param
    type Param;
    // ========================================================================
    /// fn into_graphics
    fn into_graphics(
        self,
        param: Self::Param,
    ) -> Result<(Self::Target, Self::Param)>;
}
