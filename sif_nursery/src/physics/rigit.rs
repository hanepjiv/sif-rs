// -*- mode:rust; coding:utf-8-unix; -*-

//! rigit.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/04/09
//  @date 2018/05/12

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use sif_math::{Matrix3x3, Number, Quaternion, Vector3};
// ----------------------------------------------------------------------------
use super::Result;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Rigit
#[derive(Debug, Clone)]
pub struct Rigit<V>
where
    V: Number,
{
    /// inertia
    inertia: Matrix3x3<V>,
    /// inertia_inverse
    inertia_inverse: Matrix3x3<V>,
    /// angle
    angle: Quaternion<V>,
    /// moment
    moment: Vector3<V>,
    /// velocity
    velocity: Vector3<V>,
    /// force
    pub force: Vector3<V>,
    /// mass
    mass: V,
}
// ============================================================================
impl<V> Default for Rigit<V>
where
    V: Number,
{
    // ========================================================================
    fn default() -> Self {
        Rigit {
            inertia: Matrix3x3::default(),
            inertia_inverse: Matrix3x3::default(),
            angle: Quaternion::default(),
            moment: Vector3::default(),
            velocity: Vector3::default(),
            force: Vector3::default(),
            mass: V::one(),
        }
    }
}
// ============================================================================
impl<V> Rigit<V>
where
    V: Number,
{
    // ========================================================================
    /// new
    pub fn new(mass: V, inertia: Matrix3x3<V>) -> Result<Self> {
        Ok(Rigit {
            inertia,
            inertia_inverse: inertia.new_inverse()?,
            angle: Quaternion::default(),
            moment: Vector3::default(),
            velocity: Vector3::default(),
            force: Vector3::default(),
            mass,
        })
    }
}
