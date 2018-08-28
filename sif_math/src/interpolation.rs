// -*- mode:rust; coding:utf-8-unix; -*-

//! interpolation.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/08/05
//  @date 2018/08/27

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use super::Float;
// ============================================================================
/// bezier
pub fn bezier<V: Float>(v0: V, v1: V, v2: V, v3: V, t: V) -> V {
    let it = V::from(1).unwrap() - t;
    ((it * it * ((it * v0) + (V::from(3).unwrap() * t * v1)))
        + (t * t * ((V::from(3).unwrap() * it * v2) + (t * v3))))
}
