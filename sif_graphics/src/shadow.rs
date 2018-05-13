// -*- mode:rust; coding:utf-8-unix; -*-

//! shadow.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/03/08
//  @date 2018/05/12

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use gl::types::*;
// ----------------------------------------------------------------------------
use sif_manager::ManagedValue;
use sif_renderer::{Bind, Program, Texture};
// ----------------------------------------------------------------------------
use super::{
    post::{DepthMap, DepthMapParam}, Object, Result,
};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Shadow
#[derive(Debug)]
pub struct Shadow {
    /// param
    param: DepthMapParam,
    /// map
    map: DepthMap,
}
// ============================================================================
impl AsRef<DepthMapParam> for Shadow {
    fn as_ref(&self) -> &DepthMapParam {
        &self.param
    }
}
// ----------------------------------------------------------------------------
impl AsMut<DepthMapParam> for Shadow {
    fn as_mut(&mut self) -> &mut DepthMapParam {
        &mut self.param
    }
}
// ============================================================================
impl AsRef<DepthMap> for Shadow {
    fn as_ref(&self) -> &DepthMap {
        &self.map
    }
}
// ============================================================================
impl Shadow {
    // ========================================================================
    /// new
    pub fn new(width: GLsizei, height: GLsizei) -> Result<Self> {
        Ok(Shadow {
            param: DepthMapParam::default(),
            map: DepthMap::new(width, height)?,
        })
    }
    // ========================================================================
    /// as_color
    pub fn as_color(&self) -> &Texture {
        self.map.as_color()
    }
    // ========================================================================
    /// size
    pub fn size(&self) -> &[GLint; 2] {
        self.map.size()
    }
    // ========================================================================
    /// emit
    pub fn emit(
        &self,
        depth_map_program: &Program,
        managed_obj: &ManagedValue<Object>,
    ) -> Result<&Self> {
        let _ = self.map.emit(depth_map_program, &self.param, managed_obj)?;
        Ok(self)
    }
}
// ============================================================================
impl Bind for Shadow {
    // ========================================================================
    fn id(&self) -> GLuint {
        panic!("::Shadow: No id.");
    }
    // ========================================================================
    fn bind(&self) {
        self.map.bind();
    }
    // ------------------------------------------------------------------------
    fn unbind(&self) {
        self.map.unbind();
    }
}
