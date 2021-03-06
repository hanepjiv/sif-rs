// -*- mode:rust; coding:utf-8-unix; -*-

//! capsule.rs

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
/// struct Capsule
#[derive(Debug, Clone)]
pub struct Capsule<V>
where
    V: Float,
{
    /// start
    pub start: Vector3<V>,
    /// end
    pub end: Vector3<V>,
    /// radius
    pub radius: V,
}
// ============================================================================
impl<V> Default for Capsule<V>
where
    V: Float,
{
    // ========================================================================
    fn default() -> Self {
        Capsule {
            start: Vector3::<V>::from_no_clean([
                V::zero(),
                V::zero(),
                V::zero(),
            ]),
            end: Vector3::<V>::from_no_clean([V::zero(), V::zero(), V::one()]),
            radius: V::one(),
        }
    }
}
// ============================================================================
impl<V> Capsule<V>
where
    V: Float,
{
    // ========================================================================
    /// new
    pub fn new(start: Vector3<V>, end: Vector3<V>, radius: V) -> Self {
        Capsule::<V> { start, end, radius }
    }
}
