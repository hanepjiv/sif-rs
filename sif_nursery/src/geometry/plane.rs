// -*- mode:rust; coding:utf-8-unix; -*-

//! plane.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/12
//  @date 2018/05/12

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use sif_math::{Number, Vector3};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Plane
#[derive(Debug, Clone)]
pub struct Plane<V>
where
    V: Number,
{
    /// normal
    normal: Vector3<V>,
    /// distance
    distance: V,
}
// ============================================================================
impl<V> Default for Plane<V>
where
    V: Number,
{
    // ========================================================================
    fn default() -> Self {
        Plane {
            normal: Vector3::<V>::from_no_clean([
                V::zero(),
                V::zero(),
                V::one(),
            ]),
            distance: V::zero(),
        }
    }
}
// ============================================================================
impl<V> Plane<V>
where
    V: Number,
{
    // ========================================================================
    /// new
    pub fn new(normal: &mut Vector3<V>, distance: V) -> Self {
        Plane {
            normal: *normal.clone().normalize(),
            distance,
        }
    }
    // ========================================================================
    /// as_normal
    pub fn as_normal(&self) -> &Vector3<V> {
        &self.normal
    }
    // ------------------------------------------------------------------------
    /// set_normal
    pub fn set_normal(&mut self, src: &mut Vector3<V>) -> &mut Self {
        self.normal = *src.clone().normalize();
        self
    }
    // ========================================================================
    /// as_distance
    pub fn as_distance(&self) -> &V {
        &self.distance
    }
    // ------------------------------------------------------------------------
    /// set_distance
    pub fn set_distance(&mut self, src: V) -> &mut Self {
        self.distance = src;
        self
    }
    // ========================================================================
    /// symmetry
    pub fn symmetry(&self, px: V, py: V, pz: V) -> Vector3<V> {
        let mut r = Vector3::from_no_clean([px, py, pz]);
        r -= self.normal * self.distance;
        let f = r.dot(&self.normal);
        r -= self.normal * V::from(2).unwrap() * f;
        r
    }
}
