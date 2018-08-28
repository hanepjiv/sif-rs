// -*- mode:rust; coding:utf-8-unix; -*-

//! color.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/05/12
//  @date 2018/08/27

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use num::traits::identities::one;
use sif_math::{Float, Vector3};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct ColorIntensity
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ColorIntensity<V: Float> {
    /// color
    pub color: Vector3<V>,
    /// intensity [0.0 - 1.0]
    pub intensity: V,
}
// ============================================================================
impl<V: Float> Default for ColorIntensity<V> {
    fn default() -> Self {
        ColorIntensity {
            color: Vector3::new(one(), one(), one()),
            intensity: one(),
        }
    }
}
// ============================================================================
impl<V: Float> ColorIntensity<V> {
    // ========================================================================
    /// new
    pub fn new(r: V, g: V, b: V, intensity: V) -> Self {
        ColorIntensity {
            color: Vector3::<V>::new(r, g, b),
            intensity,
        }
    }
    // ------------------------------------------------------------------------
    /// from_vec
    pub fn from_vec(color: Vector3<V>, intensity: V) -> Self {
        ColorIntensity { color, intensity }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct ColorExponent
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct ColorExponent<V: Float> {
    /// color
    pub color: Vector3<V>,
    /// exponent
    pub exponent: V,
}
// ============================================================================
impl<V: Float> ColorExponent<V> {
    // ========================================================================
    /// new
    pub fn new(r: V, g: V, b: V, exponent: V) -> Self {
        ColorExponent {
            color: Vector3::<V>::new(r, g, b),
            exponent,
        }
    }
}
