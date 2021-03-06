// -*- mode:rust; coding:utf-8-unix; -*-

//! light.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/24
//  @date 2020/03/19

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use bitflags::bitflags;
use gl::types::{GLfloat, GLuint};
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_manager::ManagedValue;
use sif_math::{Float, Vector3};
use sif_renderer::{Bind, Program, Texture};
// ----------------------------------------------------------------------------
use super::{post::DepthMapParam, Error, Object, Result, Shadow};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
#[allow(missing_docs)]
bitflags! {
    #[allow(missing_docs)]
    pub struct Flags: u32 {
    #[allow(missing_docs)]
    const ENABLE                = 0b0000_0000_0000_0000_0000_0000_0000_0001u32;
    #[allow(missing_docs)]
    const POINT                 = 0b0000_0000_0000_0000_0000_0000_0000_0010u32;
    #[allow(missing_docs)]
    const SPOT                  = 0b0000_0000_0000_0000_0000_0000_0000_0100u32;
    #[allow(missing_docs)]
    const SHADOW                = 0b0000_0000_0000_0000_0000_0000_0000_1000u32;

    #[allow(missing_docs)]
    const DO_NOT_USE            = 0b1000_0000_0000_0000_0000_0000_0000_0000u32;
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Light
#[derive(Debug)]
pub struct Light<VF>
where
    VF: Float,
{
    /// uuid
    uuid: Uuid,
    /// name
    name: String,
    /// color
    pub color: Vector3<VF>,
    /// kcklkq
    pub kcklkq: Vector3<VF>,
    /// intensity
    pub intensity: VF,
    /// exponent
    pub exponent: VF,
    /// cutoff
    pub cutoff: VF,
    /// shadow
    shadow: Option<Shadow<VF>>,
    /// flags
    pub flags: Flags,
}
// ============================================================================
impl<VF> AsRef<Uuid> for Light<VF>
where
    VF: Float,
{
    fn as_ref(&self) -> &Uuid {
        &self.uuid
    }
}
// ============================================================================
impl<VF> AsRef<String> for Light<VF>
where
    VF: Float,
{
    fn as_ref(&self) -> &String {
        &self.name
    }
}
// ============================================================================
impl<VF> Light<VF>
where
    VF: Float,
{
    // ========================================================================
    /// fn new
    pub(crate) fn new(
        uuid: Uuid,
        name: String,
        color: Vector3<VF>,
        kcklkq: Vector3<VF>,
        intensity: VF,
        exponent: VF,
        cutoff: VF,
        shadow: Option<Shadow<VF>>,
        flags: Flags,
    ) -> Result<Self> {
        Ok(Light {
            uuid,
            name,
            color,
            kcklkq,
            intensity,
            exponent,
            cutoff,
            shadow,
            flags,
        })
    }
    // ========================================================================
    /// on
    pub fn on(&mut self) {
        self.flags.insert(Flags::ENABLE);
    }
    // ------------------------------------------------------------------------
    /// off
    pub fn off(&mut self) {
        self.flags.remove(Flags::ENABLE);
    }
    // ------------------------------------------------------------------------
    /// toggle
    pub fn toggle(&mut self) {
        self.flags.toggle(Flags::ENABLE);
    }
    // ========================================================================
    /// sun
    pub fn sun(&mut self) {
        self.flags.remove(Flags::POINT | Flags::SPOT);
    }
    // ------------------------------------------------------------------------
    /// point
    pub fn point(&mut self) {
        self.flags.insert(Flags::POINT);
        self.flags.remove(Flags::SPOT);
    }
    // ------------------------------------------------------------------------
    /// spot
    pub fn spot(&mut self) {
        self.flags.insert(Flags::POINT | Flags::SPOT);
    }
    // ========================================================================
    /// as_shadow_param
    pub fn as_shadow_param(&self) -> &DepthMapParam<VF> {
        if let Some(ref shadow) = self.shadow {
            shadow.as_ref()
        } else {
            panic!("Light: no shadow");
        }
    }
    // ------------------------------------------------------------------------
    /// as_shadow_param_mut
    pub fn as_shadow_param_mut(&mut self) -> &mut DepthMapParam<VF> {
        if let Some(ref mut shadow) = self.shadow {
            shadow.as_mut()
        } else {
            panic!("Light: no shadow");
        }
    }
    // ========================================================================
    /// as_shadow_color
    pub fn as_shadow_color(&self) -> &Texture {
        if let Some(ref shadow) = self.shadow {
            shadow.as_color()
        } else {
            panic!("Light: no shadow");
        }
    }
    // ========================================================================
    /// as_shadow_size
    pub fn as_shadow_size(&self) -> &[i32; 2] {
        if let Some(ref shadow) = self.shadow {
            shadow.size()
        } else {
            panic!("Light: no shadow");
        }
    }
    // ========================================================================
    /// shadow_emit
    pub fn shadow_emit(
        &self,
        depth_map_program: &Program,
        managed_obj: &ManagedValue<Object<VF>>,
    ) -> Result<&Self>
    where
        GLfloat: From<VF>,
    {
        if let Some(ref shadow) = self.shadow {
            let _ = shadow.emit(depth_map_program, managed_obj)?;
        } else {
            panic!("Light: no shadow");
        }
        Ok(self)
    }
}
// ============================================================================
impl<VF> Bind for Light<VF>
where
    VF: Float,
{
    // ========================================================================
    type BindError = Error;
    // ========================================================================
    fn id(&self) -> GLuint {
        panic!("Light: No id");
    }
    // ========================================================================
    fn bind(&self) -> Result<()> {
        if let Some(ref shadow) = self.shadow {
            shadow.bind()
        } else {
            Err(Error::Light("bind: invalid shadow".to_string()))
        }
    }
    // ------------------------------------------------------------------------
    fn unbind(&self) -> Result<()> {
        if let Some(ref shadow) = self.shadow {
            shadow.unbind()
        } else {
            Err(Error::Light("unbind: invalid shadow".to_string()))
        }
    }
}
