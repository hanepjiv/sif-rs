// -*- mode:rust; coding:utf-8-unix; -*-

//! bone.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/19
//  @date 2018/08/11

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use sif_math::{Matrix4x4, Number, Vector3};
// ----------------------------------------------------------------------------
use super::super::new_mat4_tra;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Bone
#[derive(Debug, Default, Clone)]
pub struct Bone<V>
where
    V: Number,
{
    /// name
    name: String,
    /// offset
    offset: Vector3<V>,
    /// parent
    pub parent: Option<usize>,
}
// ============================================================================
impl<V> Bone<V>
where
    V: Number,
{
    // ========================================================================
    /// new
    pub fn new(
        name: impl Into<String>,
        offset: Vector3<V>,
        parent: Option<usize>,
    ) -> Self {
        Bone {
            name: name.into(),
            offset,
            parent,
        }
    }
    // ========================================================================
    /// offset_matrix
    pub fn offset_matrix(&self) -> Matrix4x4<V> {
        new_mat4_tra::<V>(self.offset[0], self.offset[1], self.offset[2])
    }
    // ------------------------------------------------------------------------
    /// inverse_offset_matrix
    pub fn inverse_offset_matrix(&self) -> Matrix4x4<V> {
        new_mat4_tra::<V>(-self.offset[0], -self.offset[1], -self.offset[2])
    }
}
// ============================================================================
impl<V> AsRef<str> for Bone<V>
where
    V: Number,
{
    fn as_ref(&self) -> &str {
        self.name.as_str()
    }
}
