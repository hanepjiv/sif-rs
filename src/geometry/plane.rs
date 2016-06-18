/* -*- mode:rust; coding:utf-8-unix; -*- */

//! plane.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/12
//  @date 2016/05/19

/* ////////////////////////////////////////////////////////////////////////// */
/* use  ===================================================================== */
use super::super::math::{ Number, Vector3, };
/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
/// struct Plane
#[derive( Debug, Clone, )]
pub struct Plane<V>
    where V:    Number,         {
    /// normal
    pub normal:         Vector3<V>,
    /// distance
    pub distance:       V,
}
/* ========================================================================== */
impl <V> Default for Plane<V>
    where V:    Number,         {
    /* ====================================================================== */
    fn default() -> Self { Plane {
        normal:         Vector3::<V>::default(),
        distance:       V::zero(),
    } }
}
/* ========================================================================== */
impl <V> Plane<V>
    where V:    Number,         {
    /* ====================================================================== */
    /// new
    pub fn new(normal: Vector3<V>, distance: V) -> Self { Plane {
        normal:         normal,
        distance:       distance,
    } }
}
