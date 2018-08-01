// -*- mode:rust; coding:utf-8-unix; -*-

//! keyframe.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/07/30
//  @date 2018/07/31

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use gl::types::*;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Keyframe
#[derive(Debug, Clone, Copy)]
pub struct Keyframe {
    key: isize,
    value: GLfloat,
    interpolation: GLfloat,
    ctrl_l_key: GLfloat,
    ctrl_l_value: GLfloat,
    ctrl_r_key: GLfloat,
    ctrl_r_value: GLfloat,
}
// ============================================================================
impl Keyframe {
    // ========================================================================
    /// fn new
    pub(crate) fn new(
        key: isize,
        value: GLfloat,
        interpolation: GLfloat,
        ctrl_l_key: GLfloat,
        ctrl_l_value: GLfloat,
        ctrl_r_key: GLfloat,
        ctrl_r_value: GLfloat,
    ) -> Self {
        Keyframe {
            key,
            value,
            interpolation,
            ctrl_l_key,
            ctrl_l_value,
            ctrl_r_key,
            ctrl_r_value,
        }
    }
}
