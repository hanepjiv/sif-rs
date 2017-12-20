// -*- mode:rust; coding:utf-8-unix; -*-

//! cleanup.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/22
//  @date 2017/04/12

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Cleanup
#[derive(Debug)]
pub struct Cleanup<V>
where
    V: ::num::Float,
{
    /// max
    max: V,
}
// ============================================================================
impl<V> Cleanup<V>
where
    V: ::num::Float,
{
    // ========================================================================
    /// new
    pub fn new() -> Self {
        Cleanup {
            max: ::num::Float::neg_infinity(),
        }
    }
    // ========================================================================
    /// collect
    pub fn collect(&mut self, n: V) {
        if self.max < V::abs(n) {
            self.max = V::abs(n);
        }
    }
    // ========================================================================
    /// check
    pub fn check(&self, n: V) -> V {
        if (V::abs(n) / self.max) < V::epsilon() {
            V::zero()
        } else {
            n
        }
    }
}
