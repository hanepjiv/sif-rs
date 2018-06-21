// -*- mode:rust; coding:utf-8-unix; -*-

//! pose.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/02/25
//  @date 2018/06/18

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use sif_math::{Matrix4x4, Number};
// ----------------------------------------------------------------------------
use super::super::trarotsca::TraRotSca;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// Flags
bitflags! {
    #[allow(missing_docs)]
    pub struct Flags: u32 {
    #[allow(missing_docs)]
    const DIRTY     = 0b0000_0000_0000_0000_0000_0000_0000_0001u32;
    #[allow(missing_docs)]
    const UPDATED           = 0b0000_0000_0000_0000_0000_0000_0000_0010u32;
    }
}
// ============================================================================
impl Default for Flags {
    fn default() -> Self {
        Flags::DIRTY
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Pose
#[derive(Debug, Clone)]
pub struct Pose<V>
where
    V: Number,
{
    /// matrix
    pub matrix: Vec<Matrix4x4<V>>,
    /// local
    pub local: Vec<TraRotSca<V>>,
    /// flags
    pub flags: Vec<Flags>,
}
// ============================================================================
impl<V> ::std::ops::Index<usize> for Pose<V>
where
    V: Number,
{
    type Output = TraRotSca<V>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.local[index]
    }
}
// ----------------------------------------------------------------------------
impl<V> ::std::ops::IndexMut<usize> for Pose<V>
where
    V: Number,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.flags[index].insert(Flags::DIRTY);
        &mut self.local[index]
    }
}
// ============================================================================
impl<V> Pose<V>
where
    V: Number,
{
    // ========================================================================
    /// fn new
    pub fn new(len: usize) -> Self {
        Pose {
            matrix: vec![Matrix4x4::<V>::default(); len],
            local: vec![TraRotSca::<V>::default(); len],
            flags: vec![Flags::default(); len],
        }
    }
    // ========================================================================
    /// fn len
    pub fn len(&self) -> usize {
        self.matrix.len()
    }
    // ========================================================================
    /// fn is_empty
    pub fn is_empty(&self) -> bool {
        self.matrix.is_empty()
    }
    // ========================================================================
    /// fn as_ptr
    pub fn as_ptr(&self) -> *const V {
        self.matrix.as_ptr() as *const V
    }
}
