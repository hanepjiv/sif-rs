// -*- mode:rust; coding:utf-8-unix; -*-

//! pose.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/02/25
//  @date 2018/08/27

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::{
    iter::{Enumerate, Iterator},
    slice::{Iter, IterMut},
};
// ----------------------------------------------------------------------------
use sif_math::{Float, Matrix4x4};
// ----------------------------------------------------------------------------
use super::super::trarotsca::TraRotSca;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// Flags
bitflags! {
    #[allow(missing_docs)]
    pub struct Flags: u32 {
    #[allow(missing_docs)]
    const DIRTY                 = 0b0000_0000_0000_0000_0000_0000_0000_0001u32;
    #[allow(missing_docs)]
    const UPDATED               = 0b0000_0000_0000_0000_0000_0000_0000_0010u32;
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
    V: Float,
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
    V: Float,
{
    type Output = TraRotSca<V>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.local[index]
    }
}
// ----------------------------------------------------------------------------
impl<V> ::std::ops::IndexMut<usize> for Pose<V>
where
    V: Float,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.flags[index].insert(Flags::DIRTY);
        &mut self.local[index]
    }
}
// ============================================================================
impl<V> Pose<V>
where
    V: Float,
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
    // ========================================================================
    /// iter_local
    pub fn iter_local(&self) -> Iter<TraRotSca<V>> {
        self.local.iter()
    }
    // ------------------------------------------------------------------------
    /// iter_local_mut
    pub fn iter_local_mut(&mut self) -> PoseLocalIterMut<V> {
        PoseLocalIterMut::new(
            &mut self.flags,
            self.local.iter_mut().enumerate(),
        )
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct PoseLocalIterMut
#[derive(Debug)]
pub struct PoseLocalIterMut<'a, V>
where
    V: 'a + Float,
{
    /// flags
    flags: &'a mut Vec<Flags>,
    /// iter
    iter: Enumerate<IterMut<'a, TraRotSca<V>>>,
}
// ============================================================================
impl<'a, V> PoseLocalIterMut<'a, V>
where
    V: Float,
{
    // ========================================================================
    /// fn new
    pub(crate) fn new(
        flags: &'a mut Vec<Flags>,
        iter: Enumerate<IterMut<'a, TraRotSca<V>>>,
    ) -> Self {
        Self { flags, iter }
    }
}
// ============================================================================
impl<'a, V> Iterator for PoseLocalIterMut<'a, V>
where
    V: Float,
{
    type Item = &'a mut TraRotSca<V>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((i, x)) = self.iter.next() {
            self.flags[i].insert(Flags::DIRTY);
            Some(x)
        } else {
            None
        }
    }
}
