// -*- mode:rust; coding:utf-8-unix; -*-

//! quaternion.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/19
//  @date 2018/08/27

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use std::ops::{Index, IndexMut};
// ----------------------------------------------------------------------------
use super::{Float, Vector4};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Quaternion
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Quaternion<V: Float>(Vector4<V>);
// ============================================================================
impl<V> Default for Quaternion<V>
where
    V: Float,
{
    fn default() -> Self {
        Quaternion(Vector4::from([V::zero(), V::zero(), V::zero(), V::one()]))
    }
}
// ============================================================================
impl<V> From<[V; 4]> for Quaternion<V>
where
    V: Float,
{
    fn from(inner: [V; 4]) -> Self {
        Quaternion::from(Vector4::from(inner))
    }
}
// ============================================================================
impl<V> From<Vector4<V>> for Quaternion<V>
where
    V: Float,
{
    fn from(inner: Vector4<V>) -> Self {
        *Quaternion(inner).cleanup()
    }
}
// ============================================================================
impl<V> Index<usize> for Quaternion<V>
where
    V: Float,
{
    type Output = V;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
// ============================================================================
impl<V> IndexMut<usize> for Quaternion<V>
where
    V: Float,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
// ============================================================================
impl<V> Quaternion<V>
where
    V: Float,
{
    // ========================================================================
    /// as_ptr
    pub fn as_ptr(&self) -> *const V {
        self.0.as_ptr()
    }
    // ------------------------------------------------------------------------
    /// as_mut_ptr
    pub fn as_mut_ptr(&mut self) -> *mut V {
        self.0.as_mut_ptr()
    }
    // ========================================================================
    /// cleanup
    pub fn cleanup(&mut self) -> &mut Self {
        let _ = self.0.cleanup();
        self
    }
}
