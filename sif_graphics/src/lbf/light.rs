// -*- mode:rust; coding:utf-8-unix; -*-

//! light.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/06/13
//  @date 2018/06/13

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use gl::types::*;
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_math::Vector3;
// ----------------------------------------------------------------------------
use super::super::LightFlags;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Light
#[derive(Debug, Clone)]
pub struct Light {
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
    pub flags: LightFlags,
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
    /// new
    pub fn new(uuid: Uuid, name: impl Into<String>) -> Self {
        Light {
            uuid,
            name: name.into(),
            color: Vector3::new(1.0, 1.0, 1.0),
            kcklkq: Vector3::new(1.0, 0.01, 0.0001),
            intensity: 0.8,
            exponent: 2.0,
            cutoff: 0.9,
            flags: LightFlags::ENABLE,
        }
    }
}
