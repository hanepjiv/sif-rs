// -*- mode:rust; coding:utf-8-unix; -*-

//! cube.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/12
//  @date 2018/08/27

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use sif_math::{Float, Vector3};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Cube
#[derive(Debug, Clone)]
pub struct Cube<V>
where
    V: Float,
{
    /// center
    pub center: Vector3<V>,
    /// radius
    pub radius: V,
}
// ============================================================================
impl<V> Default for Cube<V>
where
    V: Float,
{
    // ========================================================================
    fn default() -> Self {
        Cube {
            center: Vector3::<V>::default(),
            radius: V::one(),
        }
    }
}
// ============================================================================
impl<V> Cube<V>
where
    V: Float,
{
    // ========================================================================
    /// new
    pub fn new(center: Vector3<V>, radius: V) -> Self {
        Cube { center, radius }
    }
}
