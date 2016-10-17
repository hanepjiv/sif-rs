// -*- mode:rust; coding:utf-8-unix; -*-

//! capsule.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/12
//  @date 2016/10/10

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use super::super::math::{ Number, Vector3, };
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Capsulee
#[derive( Debug, Clone, )]
pub struct Capsulee<V>
    where V:    Number,         {
    /// center
    pub center: Vector3<V>,
    /// radius
    pub radius: V,
}
// ============================================================================
impl <V> Default for Capsulee<V>
    where V:    Number,         {
    // ========================================================================
    fn default() -> Self { Capsulee {
        center: Vector3::<V>::default(),
        radius: V::zero(),
    } }
}
// ============================================================================
impl <V> Capsulee<V>
    where V:    Number,         {
    // ========================================================================
    /// new
    pub fn new(center: Vector3<V>, radius: V) -> Self { Capsulee::<V> {
        center: center,
        radius: radius,
    } }
}
