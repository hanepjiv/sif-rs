// -*- mode:rust; coding:utf-8-unix; -*-

//! ray.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/03/17
//  @date 2018/04/16

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use super::super::math::{Number, Vector3};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Ray
#[derive(Debug, Clone)]
pub struct Ray<V>
where
    V: Number,
{
    /// start
    pub start: Vector3<V>,
    /// end
    pub end: Vector3<V>,
}
// ============================================================================
impl<V> Default for Ray<V>
where
    V: Number,
{
    // ========================================================================
    fn default() -> Self {
        Ray {
            start: Vector3::<V>::from_no_clean([
                V::zero(),
                V::zero(),
                V::zero(),
            ]),
            end: Vector3::<V>::from_no_clean([V::zero(), V::zero(), V::one()]),
        }
    }
}
// ============================================================================
impl<V> Ray<V>
where
    V: Number,
{
    // ========================================================================
    /// new
    pub fn new(start: Vector3<V>, end: Vector3<V>) -> Self {
        Ray::<V> { start, end }
    }
}
