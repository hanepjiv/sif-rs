// -*- mode:rust; coding:utf-8-unix; -*-

//! cuboid.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/19
//  @date 2016/10/10

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use super::super::math::{ Number, Vector3, };
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Cuboid
#[derive( Debug, Clone, )]
pub struct Cuboid<V>
    where V:    Number,         {
    /// center
    pub center: Vector3<V>,
}
// ============================================================================
impl <V> Default for Cuboid<V>
    where V:    Number,         {
    // ========================================================================
    fn default() -> Self { Cuboid {
        center: Vector3::<V>::default(),
    } }
}
// ============================================================================
impl <V> Cuboid<V>
    where V:    Number,         {
    // ========================================================================
    /// new
    pub fn new() -> Self { Cuboid {
        center: Vector3::<V>::default(),
    } }
    // ========================================================================
    /// is_cube
    pub fn is_cube(&self) -> bool {
        false  // TODO(hanepjiv): UNDERCONSTRUCT
    }
}
