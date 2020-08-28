// -*- mode:rust; coding:utf-8-unix; -*-

//! camera.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/19
//  @date 2019/07/14

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_math::{Float, Matrix4x4, Vector4};
// ----------------------------------------------------------------------------
use super::{Error, Result};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum CameraType
#[derive(Debug, Clone, Copy)]
pub enum CameraType<V>
where
    V: Float,
{
    /// Frustum
    Frustum(V, V),
    /// Ortho
    Ortho(V, V),
}
// ============================================================================
impl<V> CameraType<V>
where
    V: Float,
{
    // ========================================================================
    /// set_focus
    pub fn set_focus(&mut self, f: V) -> Result<()> {
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
    pub fn set_aspect(&mut self, a: V) -> Result<()> {
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
    pub fn set_width(&mut self, w: V) -> Result<()> {
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
    pub fn set_height(&mut self, h: V) -> Result<()> {
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
    pub fn set_width_height(&mut self, (w, h): (V, V)) -> Result<()> {
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
pub struct Camera<V>
where
    V: Float,
{
    /// uuid
    uuid: Uuid,
    /// name
    name: String,
    /// near
    near: V,
    /// far
    far: V,
    /// camera_type
    camera_type: CameraType<V>,
}
// ============================================================================
impl<V> AsRef<Uuid> for Camera<V>
where
    V: Float,
{
    fn as_ref(&self) -> &Uuid {
        &self.uuid
    }
}
// ============================================================================
impl<V> AsRef<String> for Camera<V>
where
    V: Float,
{
    fn as_ref(&self) -> &String {
        &self.name
    }
}
// ============================================================================
impl<V> Camera<V>
where
    V: Float,
{
    // ========================================================================
    /// frustum
    pub fn frustum(near: V, far: V, focus: V, aspect: V) -> Matrix4x4<V> {
        let c = V::one() / (near - far);
        Matrix4x4::from([
            Vector4::from_no_clean([focus, V::zero(), V::zero(), V::zero()]),
            Vector4::from_no_clean([
                V::zero(),
                focus / aspect,
                V::zero(),
                V::zero(),
            ]),
            Vector4::from_no_clean([
                V::zero(),
                V::zero(),
                (near + far) * c,
                -V::one(),
            ]),
            Vector4::from_no_clean([
                V::zero(),
                V::zero(),
                V::from(2).unwrap() * near * far * c,
                V::zero(),
            ]),
        ])
    }
    // ------------------------------------------------------------------------
    /// inverse_frustum
    pub fn inverse_frustum(
        near: V,
        far: V,
        focus: V,
        aspect: V,
    ) -> Matrix4x4<V> {
        let c = V::one() / (V::from(2).unwrap() * far * near);
        Matrix4x4::from([
            Vector4::from_no_clean([
                V::one() / focus,
                V::zero(),
                V::zero(),
                V::zero(),
            ]),
            Vector4::from_no_clean([
                V::zero(),
                aspect / focus,
                V::zero(),
                V::zero(),
            ]),
            Vector4::from_no_clean([
                V::zero(),
                V::zero(),
                V::zero(),
                -V::one(),
            ]),
            Vector4::from_no_clean([
                V::zero(),
                V::zero(),
                (near - far) * c,
                (near + far) * c,
            ]),
        ])
    }
    // ========================================================================
    /// ortho
    pub fn ortho(near: V, far: V, width: V, height: V) -> Matrix4x4<V> {
        let c = V::one() / (near - far);
        Matrix4x4::from([
            Vector4::from_no_clean([
                V::from(2).unwrap() / width,
                V::zero(),
                V::zero(),
                V::zero(),
            ]),
            Vector4::from_no_clean([
                V::zero(),
                V::from(2).unwrap() / height,
                V::zero(),
                V::zero(),
            ]),
            Vector4::from_no_clean([
                V::zero(),
                V::zero(),
                V::from(2).unwrap() * c,
                V::zero(),
            ]),
            Vector4::from_no_clean([
                V::zero(),
                V::zero(),
                (near + far) * c,
                V::one(),
            ]),
        ])
    }
    // ========================================================================
    /// new_frustum
    pub fn new_frustum(
        uuid: Uuid,
        name: impl Into<String>,
        near: V,
        far: V,
        focus: V,
        aspect: V,
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
        near: V,
        far: V,
        width: V,
        height: V,
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
    pub fn set_focus(&mut self, f: V) -> Result<()> {
        self.camera_type.set_focus(f)
    }
    // ------------------------------------------------------------------------
    /// set_aspect
    pub fn set_aspect(&mut self, a: V) -> Result<()> {
        self.camera_type.set_aspect(a)
    }
    // ========================================================================
    /// set_width
    pub fn set_width(&mut self, w: V) -> Result<()> {
        self.camera_type.set_width(w)
    }
    // ------------------------------------------------------------------------
    /// set_height
    pub fn set_height(&mut self, h: V) -> Result<()> {
        self.camera_type.set_height(h)
    }
    // ------------------------------------------------------------------------
    /// set_width_height
    pub fn set_width_height(&mut self, wh: (V, V)) -> Result<()> {
        self.camera_type.set_width_height(wh)
    }
    // ========================================================================
    /// projection_matrix
    pub fn projection_matrix(&self) -> Matrix4x4<V> {
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
    pub fn focus2alpha(focus: V) -> V {
        V::from(2).unwrap() * ::num::Float::atan(V::one() / focus)
    }
    // ------------------------------------------------------------------------
    /// alpha2focus
    pub fn alpha2focus(alpha: V) -> V {
        V::one() / ::num::Float::tan(alpha / V::from(2).unwrap())
    }
}
