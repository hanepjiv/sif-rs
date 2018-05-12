// -*- mode:rust; coding:utf-8-unix; -*-

//! cuboid.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/19
//  @date 2018/05/12

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use sif_math::{Number, Vector3};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Cuboid
#[derive(Debug, Clone)]
pub struct Cuboid<V>
where
    V: Number,
{
    /// center
    pub center: Vector3<V>,
    /// radius
    pub radius: Vector3<V>,
}
// ============================================================================
impl<V> Default for Cuboid<V>
where
    V: Number,
{
    // ========================================================================
    fn default() -> Self {
        Cuboid {
            center: Vector3::<V>::default(),
            radius: Vector3::<V>::from_no_clean([
                V::one(),
                V::one(),
                V::one(),
            ]),
        }
    }
}
// ============================================================================
impl<V> Cuboid<V>
where
    V: Number,
{
    // ========================================================================
    /// new
    pub fn new(center: Vector3<V>, radius: Vector3<V>) -> Self {
        Cuboid { center, radius }
    }
    // ========================================================================
    /// is_cube
    pub fn is_cube(&self) -> Option<V> {
        if V::epsilon() < V::abs(self.radius[0] - self.radius[1])
            || V::epsilon() < V::abs(self.radius[0] - self.radius[2])
        {
            None
        } else {
            Some(self.radius[0])
        }
    }
}
