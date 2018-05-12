// -*- mode:rust; coding:utf-8-unix; -*-

//! polygon.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/12/01
//  @date 2018/05/12

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use gl::types::*;
// ----------------------------------------------------------------------------
use super::{Error, Result, super::submesh};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// Flags
bitflags! {
    #[allow(missing_docs)]
    pub struct Flags: u32 {
    #[allow(missing_docs)]
    const SMOOTH                = 0b0000_0000_0000_0000_0000_0000_0000_0001u32;
    #[allow(missing_docs)]
    const DOUBLE_SIDED  = 0b0000_0000_0000_0000_0000_0000_0000_0010u32;
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Polygon
#[derive(Debug, Clone)]
pub struct Polygon {
    /// indices
    pub indices: Vec<(usize, usize)>,
    /// material_index
    pub material_index: Option<usize>,
    /// flags
    pub flags: Flags,
}
// ============================================================================
impl Polygon {
    // ========================================================================
    /// new
    pub fn new(
        flags: Flags,
        material_index: Option<usize>,
        indices: impl Into<Vec<GLuint>>,
    ) -> Result<Self> {
        let mut ids = Vec::<(usize, usize)>::default();
        let vs = indices.into();
        let mut i = vs.iter();
        let mut opt = i.next();
        while let Some(v) = opt {
            let idx_p = *v as usize;
            let idx_c = if let Some(x) = i.next() {
                *x as usize
            } else {
                return Err(Error::Polygon(format!(
                    "{}({}): Polygon::new",
                    file!(),
                    line!()
                )));
            };
            ids.push((idx_p, idx_c));
            opt = i.next();
        }
        Ok(Polygon {
            indices: ids,
            material_index,
            flags,
        })
    }
    // ========================================================================
    /// submesh_flags
    pub fn submesh_flags(&self) -> submesh::Flags {
        let mut ret = submesh::Flags::default();
        if self.flags.contains(Flags::SMOOTH) {
            ret.insert(submesh::Flags::SMOOTH);
        } else {
            ret.remove(submesh::Flags::SMOOTH);
        };
        if self.flags.contains(Flags::DOUBLE_SIDED) {
            ret.remove(submesh::Flags::CULLING);
            ret.remove(submesh::Flags::BACKFACE);
        } else {
            ret.insert(submesh::Flags::CULLING);
            ret.remove(submesh::Flags::BACKFACE);
        };
        ret
    }
}
