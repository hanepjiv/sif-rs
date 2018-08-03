// -*- mode:rust; coding:utf-8-unix; -*-

//! cleanup.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/22
//  @date 2018/08/03

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Cleanup
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cleanup<V>
where
    V: ::num::Float,
{
    /// max
    max: V,
}
// ============================================================================
impl<V> Default for Cleanup<V>
where
    V: ::num::Float,
{
    fn default() -> Self {
        Cleanup {
            max: ::num::Float::neg_infinity(),
        }
    }
}
// ============================================================================
impl<V> Cleanup<V>
where
    V: ::num::Float,
{
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
