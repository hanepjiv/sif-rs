// -*- mode:rust; coding:utf-8-unix; -*-

//! offsets.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/12/12
//  @date 2020/03/19

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use super::element::{Element, ELEMENT_SIZE};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Offset
#[derive(Debug, Clone, Copy)]
pub struct Offsets([isize; ELEMENT_SIZE]);
// ============================================================================
impl Offsets {
    // ========================================================================
    /// check
    pub fn check(&self, e: Element) -> bool {
        -1 != self[e]
    }
}
// ============================================================================
impl Default for Offsets {
    fn default() -> Self {
        Offsets([-1isize; ELEMENT_SIZE])
    }
}
// ============================================================================
impl ::std::ops::Index<Element> for Offsets {
    type Output = isize;
    fn index(&self, index: Element) -> &Self::Output {
        &self.0[index.bits() as usize]
    }
}
// ----------------------------------------------------------------------------
impl ::std::ops::IndexMut<Element> for Offsets {
    fn index_mut(&mut self, index: Element) -> &mut Self::Output {
        &mut self.0[index.bits() as usize]
    }
}
