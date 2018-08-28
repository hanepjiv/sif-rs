// -*- mode:rust; coding:utf-8-unix; -*-

//! segment.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/03/17
//  @date 2018/08/27

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use sif_math::{Float, Vector3};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Segment
#[derive(Debug, Clone)]
pub struct Segment<V>
where
    V: Float,
{
    /// start
    pub start: Vector3<V>,
    /// end
    pub end: Vector3<V>,
}
// ============================================================================
impl<V> Default for Segment<V>
where
    V: Float,
{
    // ========================================================================
    fn default() -> Self {
        Segment {
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
impl<V> Segment<V>
where
    V: Float,
{
    // ========================================================================
    /// new
    pub fn new(start: Vector3<V>, end: Vector3<V>) -> Self {
        Segment::<V> { start, end }
    }
}
