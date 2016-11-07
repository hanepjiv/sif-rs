// -*- mode:rust; coding:utf-8-unix; -*-

//! capsule.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/12
//  @date 2016/11/07

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use super::super::math::{ Number, Vector3, };
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Capsule
#[derive( Debug, Clone, )]
pub struct Capsule<V>
    where V:    Number,         {
    /// center
    pub center: Vector3<V>,
    /// radius
    pub radius: V,
}
// ============================================================================
impl <V> Default for Capsule<V>
    where V:    Number,         {
    // ========================================================================
    fn default() -> Self { Capsule {
        center: Vector3::<V>::default(),
        radius: V::zero(),
    } }
}
// ============================================================================
impl <V> Capsule<V>
    where V:    Number,         {
    // ========================================================================
    /// new
    pub fn new(center: Vector3<V>, radius: V) -> Self { Capsule::<V> {
        center: center,
        radius: radius,
    } }
}
