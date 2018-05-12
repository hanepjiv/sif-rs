// -*- mode:rust; coding:utf-8-unix; -*-

//! color.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/05/12
//  @date 2018/05/12

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use gl::types::*;
// ----------------------------------------------------------------------------
use sif_math::Vector3;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct ColorIntensity
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ColorIntensity {
    /// color
    pub color: Vector3<GLfloat>,
    /// intensity [0.0 - 1.0]
    pub intensity: GLfloat,
}
// ============================================================================
impl Default for ColorIntensity {
    fn default() -> Self {
        ColorIntensity {
            color: Vector3::<GLfloat>::new(1.0, 1.0, 1.0),
            intensity: 1.0,
        }
    }
}
// ============================================================================
impl ColorIntensity {
    // ========================================================================
    /// new
    pub fn new(
        r: GLfloat,
        g: GLfloat,
        b: GLfloat,
        intensity: GLfloat,
    ) -> Self {
        ColorIntensity {
            color: Vector3::<GLfloat>::new(r, g, b),
            intensity,
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct ColorExponent
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct ColorExponent {
    /// color
    pub color: Vector3<GLfloat>,
    /// exponent
    pub exponent: GLfloat,
}
// ============================================================================
impl ColorExponent {
    // ========================================================================
    /// new
    pub fn new(r: GLfloat, g: GLfloat, b: GLfloat, exponent: GLfloat) -> Self {
        ColorExponent {
            color: Vector3::<GLfloat>::new(r, g, b),
            exponent,
        }
    }
}
