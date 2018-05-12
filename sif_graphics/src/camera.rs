// -*- mode:rust; coding:utf-8-unix; -*-

//! camera.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/19
//  @date 2018/05/12

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use gl::types::*;
use num::{Float, One, Zero};
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_math::{Matrix4x4, Vector4};
// ----------------------------------------------------------------------------
use super::{Error, Result};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum CameraType
#[derive(Debug, Clone, Copy)]
pub enum CameraType {
    /// Frustum
    Frustum(GLfloat, GLfloat),
    /// Ortho
    Ortho(GLfloat, GLfloat),
}
// ============================================================================
impl CameraType {
    // ========================================================================
    /// set_focus
    pub fn set_focus(&mut self, f: GLfloat) -> Result<()> {
        match *self {
            CameraType::Frustum(ref mut focus, _) => {
                *focus = f;
                Ok(())
            }
            CameraType::Ortho(_, _) => Err(Error::InvalidEnum),
        }
    }
    // ------------------------------------------------------------------------
    /// set_aspect
    pub fn set_aspect(&mut self, a: GLfloat) -> Result<()> {
        match *self {
            CameraType::Frustum(_, ref mut aspect) => {
                *aspect = a;
                Ok(())
            }
            CameraType::Ortho(_, _) => Err(Error::InvalidEnum),
        }
    }
    // ========================================================================
    /// set_width
    pub fn set_width(&mut self, w: GLfloat) -> Result<()> {
        match *self {
            CameraType::Frustum(_, _) => Err(Error::InvalidEnum),
            CameraType::Ortho(ref mut width, _) => {
                *width = w;
                Ok(())
            }
        }
    }
    // ------------------------------------------------------------------------
    /// set_height
    pub fn set_height(&mut self, h: GLfloat) -> Result<()> {
        match *self {
            CameraType::Frustum(_, _) => Err(Error::InvalidEnum),
            CameraType::Ortho(_, ref mut height) => {
                *height = h;
                Ok(())
            }
        }
    }
    // ------------------------------------------------------------------------
    /// set_width_height
    pub fn set_width_height(
        &mut self,
        (w, h): (GLfloat, GLfloat),
    ) -> Result<()> {
        match *self {
            CameraType::Frustum(_, _) => Err(Error::InvalidEnum),
            CameraType::Ortho(ref mut width, ref mut height) => {
                *width = w;
                *height = h;
                Ok(())
            }
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Camera
#[derive(Debug, Clone)]
pub struct Camera {
    /// uuid
    uuid: Uuid,
    /// name
    name: String,
    /// near
    near: GLfloat,
    /// far
    far: GLfloat,
    /// camera_type
    camera_type: CameraType,
}
// ============================================================================
impl AsRef<Uuid> for Camera {
    fn as_ref(&self) -> &Uuid {
        &self.uuid
    }
}
// ============================================================================
impl AsRef<String> for Camera {
    fn as_ref(&self) -> &String {
        &self.name
    }
}
// ============================================================================
impl Camera {
    // ========================================================================
    /// frustum
    pub fn frustum(
        near: GLfloat,
        far: GLfloat,
        focus: GLfloat,
        aspect: GLfloat,
    ) -> Matrix4x4<GLfloat> {
        let c = GLfloat::one() / (near - far);
        Matrix4x4::from([
            Vector4::from_no_clean([
                focus,
                GLfloat::zero(),
                GLfloat::zero(),
                GLfloat::zero(),
            ]),
            Vector4::from_no_clean([
                GLfloat::zero(),
                focus / aspect,
                GLfloat::zero(),
                GLfloat::zero(),
            ]),
            Vector4::from_no_clean([
                GLfloat::zero(),
                GLfloat::zero(),
                (near + far) * c,
                -GLfloat::one(),
            ]),
            Vector4::from_no_clean([
                GLfloat::zero(),
                GLfloat::zero(),
                2.0 * near * far * c,
                GLfloat::zero(),
            ]),
        ])
    }
    // ------------------------------------------------------------------------
    /// inverse_frustum
    pub fn inverse_frustum(
        near: GLfloat,
        far: GLfloat,
        focus: GLfloat,
        aspect: GLfloat,
    ) -> Matrix4x4<GLfloat> {
        let c = GLfloat::one() / (2.0 * far * near);
        Matrix4x4::from([
            Vector4::from_no_clean([
                GLfloat::one() / focus,
                GLfloat::zero(),
                GLfloat::zero(),
                GLfloat::zero(),
            ]),
            Vector4::from_no_clean([
                GLfloat::zero(),
                aspect / focus,
                GLfloat::zero(),
                GLfloat::zero(),
            ]),
            Vector4::from_no_clean([
                GLfloat::zero(),
                GLfloat::zero(),
                GLfloat::zero(),
                -GLfloat::one(),
            ]),
            Vector4::from_no_clean([
                GLfloat::zero(),
                GLfloat::zero(),
                (near - far) * c,
                (near + far) * c,
            ]),
        ])
    }
    // ========================================================================
    /// ortho
    pub fn ortho(
        near: GLfloat,
        far: GLfloat,
        width: GLfloat,
        height: GLfloat,
    ) -> Matrix4x4<GLfloat> {
        let c = GLfloat::one() / (near - far);
        Matrix4x4::from([
            Vector4::from_no_clean([
                2.0 / width,
                GLfloat::zero(),
                GLfloat::zero(),
                GLfloat::zero(),
            ]),
            Vector4::from_no_clean([
                GLfloat::zero(),
                2.0 / height,
                GLfloat::zero(),
                GLfloat::zero(),
            ]),
            Vector4::from_no_clean([
                GLfloat::zero(),
                GLfloat::zero(),
                2.0 * c,
                GLfloat::zero(),
            ]),
            Vector4::from_no_clean([
                GLfloat::zero(),
                GLfloat::zero(),
                (near + far) * c,
                GLfloat::one(),
            ]),
        ])
    }
    // ========================================================================
    /// new_frustum
    pub fn new_frustum(
        uuid: Uuid,
        name: impl Into<String>,
        near: GLfloat,
        far: GLfloat,
        focus: GLfloat,
        aspect: GLfloat,
    ) -> Self {
        Camera {
            uuid,
            name: name.into(),
            near,
            far,
            camera_type: CameraType::Frustum(focus, aspect),
        }
    }
    // ========================================================================
    /// new_ortho
    pub fn new_ortho(
        uuid: Uuid,
        name: impl Into<String>,
        near: GLfloat,
        far: GLfloat,
        width: GLfloat,
        height: GLfloat,
    ) -> Self {
        Camera {
            uuid,
            name: name.into(),
            near,
            far,
            camera_type: CameraType::Ortho(width, height),
        }
    }
    // ========================================================================
    /// is_frustum
    pub fn is_frustum(&self) -> bool {
        match self.camera_type {
            CameraType::Frustum(_, _) => true,
            CameraType::Ortho(_, _) => false,
        }
    }
    // ------------------------------------------------------------------------
    /// is_ortho
    pub fn is_ortho(&self) -> bool {
        match self.camera_type {
            CameraType::Frustum(_, _) => false,
            CameraType::Ortho(_, _) => true,
        }
    }
    // ========================================================================
    /// set_focus
    pub fn set_focus(&mut self, f: GLfloat) -> Result<()> {
        self.camera_type.set_focus(f)
    }
    // ------------------------------------------------------------------------
    /// set_aspect
    pub fn set_aspect(&mut self, a: GLfloat) -> Result<()> {
        self.camera_type.set_aspect(a)
    }
    // ========================================================================
    /// set_width
    pub fn set_width(&mut self, w: GLfloat) -> Result<()> {
        self.camera_type.set_width(w)
    }
    // ------------------------------------------------------------------------
    /// set_height
    pub fn set_height(&mut self, h: GLfloat) -> Result<()> {
        self.camera_type.set_height(h)
    }
    // ------------------------------------------------------------------------
    /// set_width_height
    pub fn set_width_height(&mut self, wh: (GLfloat, GLfloat)) -> Result<()> {
        self.camera_type.set_width_height(wh)
    }
    // ========================================================================
    /// projection_matrix
    pub fn projection_matrix(&self) -> Matrix4x4<GLfloat> {
        match self.camera_type {
            CameraType::Frustum(focus, aspect) => {
                Camera::frustum(self.near, self.far, focus, aspect)
            }
            CameraType::Ortho(width, height) => {
                Camera::ortho(self.near, self.far, width, height)
            }
        }
    }
    // ========================================================================
    /// focus2alpha
    pub fn focus2alpha(focus: GLfloat) -> GLfloat {
        2.0 as GLfloat * Float::atan(1.0 as GLfloat / focus)
    }
    // ------------------------------------------------------------------------
    /// alpha2focus
    pub fn alpha2focus(alpha: GLfloat) -> GLfloat {
        1.0 as GLfloat / Float::tan(alpha / 2.0 as GLfloat)
    }
}
