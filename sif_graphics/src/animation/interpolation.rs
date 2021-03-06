// -*- mode:rust; coding:utf-8-unix; -*-

//! interpolation.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/07/31
//  @date 2018/08/01

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Interpolation
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Interpolation {
    /// Constant
    Constant,
    /// Linear
    Linear,
    /// Bezier
    Bezier,
}
