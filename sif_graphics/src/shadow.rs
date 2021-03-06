// -*- mode:rust; coding:utf-8-unix; -*-

//! shadow.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/03/08
//  @date 2018/08/27

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use gl::types::*;
// ----------------------------------------------------------------------------
use sif_manager::ManagedValue;
use sif_math::Float;
use sif_renderer::{Bind, Program, Texture};
// ----------------------------------------------------------------------------
use super::{
    post::{DepthMap, DepthMapParam},
    Error, Object, Result,
};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Shadow
#[derive(Debug)]
pub struct Shadow<VF>
where
    VF: Float,
{
    /// param
    param: DepthMapParam<VF>,
    /// map
    map: DepthMap,
}
// ============================================================================
impl<VF> AsRef<DepthMapParam<VF>> for Shadow<VF>
where
    VF: Float,
{
    fn as_ref(&self) -> &DepthMapParam<VF> {
        &self.param
    }
}
// ----------------------------------------------------------------------------
impl<VF> AsMut<DepthMapParam<VF>> for Shadow<VF>
where
    VF: Float,
{
    fn as_mut(&mut self) -> &mut DepthMapParam<VF> {
        &mut self.param
    }
}
// ============================================================================
impl<VF> AsRef<DepthMap> for Shadow<VF>
where
    VF: Float,
{
    fn as_ref(&self) -> &DepthMap {
        &self.map
    }
}
// ============================================================================
impl<VF> Shadow<VF>
where
    VF: Float,
{
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
    pub fn size(&self) -> &[i32; 2] {
        self.map.size()
    }
    // ========================================================================
    /// emit
    pub fn emit(
        &self,
        depth_map_program: &Program,
        managed_obj: &ManagedValue<Object<VF>>,
    ) -> Result<&Self>
    where
        GLfloat: From<VF>,
    {
        let _ = self.map.emit(depth_map_program, &self.param, managed_obj)?;
        Ok(self)
    }
}
// ============================================================================
impl<VF> Bind for Shadow<VF>
where
    VF: Float,
{
    // ========================================================================
    type BindError = Error;
    // ========================================================================
    fn id(&self) -> GLuint {
        panic!("::Shadow: No id");
    }
    // ========================================================================
    fn bind(&self) -> Result<()> {
        self.map.bind()
    }
    // ------------------------------------------------------------------------
    fn unbind(&self) -> Result<()> {
        self.map.unbind()
    }
}
