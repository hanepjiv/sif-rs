// -*- mode:rust; coding:utf-8-unix; -*-

//! light.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/24
//  @date 2018/05/17

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use gl::types::*;
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_manager::ManagedValue;
use sif_math::Vector3;
use sif_renderer::{Bind, Program, Texture};
// ----------------------------------------------------------------------------
use super::{Error, Object, Result, Shadow, post::DepthMapParam};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// Flags
bitflags! { pub struct Flags: u32 {
    const ENABLE            = 0b0000_0000_0000_0000_0000_0000_0000_0001u32;
    const POINT             = 0b0000_0000_0000_0000_0000_0000_0000_0010u32;
    const SPOT              = 0b0000_0000_0000_0000_0000_0000_0000_0100u32;
    const SHADOW            = 0b0000_0000_0000_0000_0000_0000_0000_1000u32;

    const DO_NOT_USE        = 0b1000_0000_0000_0000_0000_0000_0000_0000u32;
} }
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct LightSrc
#[derive(Debug, Clone)]
pub struct LightSrc {
    /// uuid
    pub uuid: Uuid,
    /// name
    pub name: String,
    /// color
    pub color: Vector3<GLfloat>,
    /// kcklkq
    pub kcklkq: Vector3<GLfloat>,
    /// intensity
    pub intensity: GLfloat,
    /// exponent
    pub exponent: GLfloat,
    /// cutoff
    pub cutoff: GLfloat,
    /// flags
    pub flags: Flags,
}
// ============================================================================
impl AsRef<Uuid> for LightSrc {
    fn as_ref(&self) -> &Uuid {
        &self.uuid
    }
}
// ============================================================================
impl AsRef<String> for LightSrc {
    fn as_ref(&self) -> &String {
        &self.name
    }
}
// ============================================================================
impl LightSrc {
    // ========================================================================
    /// new
    pub fn new(uuid: Uuid, name: impl Into<String>) -> Self {
        LightSrc {
            uuid,
            name: name.into(),
            color: Vector3::new(1.0, 1.0, 1.0),
            kcklkq: Vector3::new(1.0, 0.01, 0.0001),
            intensity: 0.8,
            exponent: 2.0,
            cutoff: 0.9,
            flags: Flags::ENABLE,
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Light
#[derive(Debug)]
pub struct Light {
    /// uuid
    uuid: Uuid,
    /// name
    name: String,
    /// color
    pub color: Vector3<GLfloat>,
    /// kcklkq
    pub kcklkq: Vector3<GLfloat>,
    /// intensity
    pub intensity: GLfloat,
    /// exponent
    pub exponent: GLfloat,
    /// cutoff
    pub cutoff: GLfloat,
    /// shadow
    shadow: Option<Shadow>,
    /// flags
    pub flags: Flags,
}
// ============================================================================
impl AsRef<Uuid> for Light {
    fn as_ref(&self) -> &Uuid {
        &self.uuid
    }
}
// ============================================================================
impl AsRef<String> for Light {
    fn as_ref(&self) -> &String {
        &self.name
    }
}
// ============================================================================
impl Light {
    // ========================================================================
    /// from_src
    pub fn from_src(src: LightSrc, texture_size: GLint) -> Result<Self> {
        Ok(Light {
            uuid: src.uuid,
            name: src.name,
            color: src.color,
            kcklkq: src.kcklkq,
            intensity: src.intensity,
            exponent: src.exponent,
            cutoff: src.cutoff,
            shadow: if src.flags.contains(Flags::SHADOW) {
                Some(Shadow::new(texture_size, texture_size)?)
            } else {
                None
            },
            flags: src.flags,
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
    pub fn as_shadow_param(&self) -> &DepthMapParam {
        if let Some(ref shadow) = self.shadow {
            shadow.as_ref()
        } else {
            panic!("::Light: no shadow");
        }
    }
    // ------------------------------------------------------------------------
    /// as_shadow_param_mut
    pub fn as_shadow_param_mut(&mut self) -> &mut DepthMapParam {
        if let Some(ref mut shadow) = self.shadow {
            shadow.as_mut()
        } else {
            panic!("::Light: no shadow");
        }
    }
    // ========================================================================
    /// as_shadow_color
    pub fn as_shadow_color(&self) -> &Texture {
        if let Some(ref shadow) = self.shadow {
            shadow.as_color()
        } else {
            panic!("::Light: no shadow");
        }
    }
    // ========================================================================
    /// as_shadow_size
    pub fn as_shadow_size(&self) -> &[GLint; 2] {
        if let Some(ref shadow) = self.shadow {
            shadow.size()
        } else {
            panic!("::Light: no shadow");
        }
    }
    // ========================================================================
    /// shadow_emit
    pub fn shadow_emit(
        &self,
        depth_map_program: &Program,
        managed_obj: &ManagedValue<Object>,
    ) -> Result<&Self> {
        if let Some(ref shadow) = self.shadow {
            let _ = shadow.emit(depth_map_program, managed_obj)?;
        } else {
            panic!("::Light: no shadow");
        }
        Ok(self)
    }
}
// ============================================================================
impl Bind for Light {
    // ========================================================================
    type BindError = Error;
    // ========================================================================
    fn id(&self) -> GLuint {
        panic!("::Light: No id");
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
            Err(Error::Light(
                "unbind: invalid shadow".to_string(),
            ))
        }
    }
}
