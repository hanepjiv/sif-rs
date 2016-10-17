// -*- mode:rust; coding:utf-8-unix; -*-

//! cube.rs

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
/// struct Cube
#[derive( Debug, Clone, )]
pub struct Cube<V>
    where V:    Number,         {
    /// center
    pub center: Vector3<V>,
}
// ============================================================================
impl <V> Default for Cube<V>
    where V:    Number,         {
    // ========================================================================
    fn default() -> Self { Cube {
        center: Vector3::<V>::default(),
    } }
}
// ============================================================================
impl <V> Cube<V>
    where V:    Number,         {
    // ========================================================================
    /// new
    pub fn new() -> Self { Cube {
        center: Vector3::<V>::default(),
    } }
}
