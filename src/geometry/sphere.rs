// -*- mode:rust; coding:utf-8-unix; -*-

//! sphere.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/12
//  @date 2017/03/17

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use super::super::math::{ Number, Vector3, };
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Sphere
#[derive( Debug, Clone, )]
pub struct Sphere<V>
    where V: Number,            {
    /// center
    pub center: Vector3<V>,
    /// radius
    pub radius: V,
}
// ============================================================================
impl <V> Default for Sphere<V>
    where V: Number,            {
    // ========================================================================
    fn default() -> Self        { Sphere {
        center: Vector3::<V>::default(),
        radius: V::one(),
    } }
}
// ============================================================================
impl <V> Sphere<V>
    where V: Number,            {
    // ========================================================================
    /// new
    pub fn new(center: Vector3<V>, radius: V) -> Self { Sphere::<V> {
        center: center,
        radius: radius,
    } }
}
