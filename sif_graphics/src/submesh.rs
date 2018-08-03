// -*- mode:rust; coding:utf-8-unix; -*-

//! submesh.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/28
//  @date 2018/08/02

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::mem::size_of;
// ----------------------------------------------------------------------------
use gl::types::*;
// ----------------------------------------------------------------------------
use sif_manager::ManagedValue;
use sif_renderer::{gl_result, Buffer, Program};
// ----------------------------------------------------------------------------
use super::{Error, Material, Result};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// Flags
bitflags! { pub struct Flags: u32 {
    const SMOOTH        = 0b0000_0000_0000_0000_0000_0000_0000_0001u32;
    const CULLING       = 0b0000_0000_0000_0000_0000_0000_0000_0010u32;
    const BACKFACE      = 0b0000_0000_0000_0000_0000_0000_0000_0100u32;
    // ------------------------------------------------------------------------
    const DIRTY         = 0b0000_0000_0000_0000_0000_0000_0001_0000u32;
    // ------------------------------------------------------------------------
    const FLAT_DOUBLE   = 0b0000_0000_0000_0000_0000_0000_0000_0000u32;
    const FLAT_FLONT    = Self::CULLING.bits;
    const FLAT_BACK     = Self::CULLING.bits | Self::BACKFACE.bits;
    const SMOOTH_DOUBLE = Self::SMOOTH.bits;
    const SMOOTH_FRONT  = Self::SMOOTH.bits  | Self::CULLING.bits;
    const SMOOTH_BACK   =
    Self::SMOOTH .bits | Self::CULLING.bits | Self::BACKFACE.bits;
} }
// ============================================================================
impl Default for Flags {
    fn default() -> Self {
        Flags::DIRTY | Flags::SMOOTH_FRONT
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct SubMesh
#[derive(Debug)]
pub struct SubMesh {
    /// buffer
    buffer: Option<Buffer>,
    /// indices
    indices: Option<Vec<GLuint>>,
    /// len
    len: GLint,
    /// usage
    usage: GLenum,
    /// mode
    mode: GLenum,
    /// material_index
    material_index: Option<usize>,
    /// flags
    flags: Flags,
}
// ============================================================================
impl AsRef<Flags> for SubMesh {
    fn as_ref(&self) -> &Flags {
        &self.flags
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Flags> for SubMesh {
    fn as_mut(&mut self) -> &mut Flags {
        &mut self.flags
    }
}
// ============================================================================
impl AsRef<[GLuint]> for SubMesh {
    fn as_ref(&self) -> &[GLuint] {
        unwrap!(self.indices.as_ref())
    }
}
// ----------------------------------------------------------------------------
impl AsMut<[GLuint]> for SubMesh {
    fn as_mut(&mut self) -> &mut [GLuint] {
        self.flags.insert(Flags::DIRTY);
        unwrap!(self.indices.as_mut())
    }
}
// ============================================================================
impl SubMesh {
    // ========================================================================
    /// new
    pub fn new(
        indices: impl Into<Vec<GLuint>>,
        usage: GLenum,
        mode: GLenum,
        material_index: Option<usize>,
        flags: Flags,
    ) -> Self {
        let indices = indices.into();
        let len = indices.len() as GLint;
        SubMesh {
            buffer: None,
            indices: Some(indices),
            len,
            usage,
            mode,
            material_index,
            flags: Flags::DIRTY | flags,
        }
    }
    // ========================================================================
    /// get_mode
    pub fn get_mode(&self) -> GLenum {
        self.mode
    }
    // ------------------------------------------------------------------------
    /// set_mode
    pub fn set_mode(&mut self, mode: GLenum) {
        self.mode = mode
    }
    // ========================================================================
    /// check_draw
    fn check_draw(&mut self) -> Result<&mut Self> {
        if self.buffer.is_none() {
            self.buffer = Some(Buffer::new_indices(
                self.indices.as_ref().ok_or_else(|| {
                    Error::OptNone(
                        "graphics: submesh: check_draw: self.indices"
                            .to_string(),
                    )
                })?,
                self.usage,
            )?);
            if ::gl::DYNAMIC_DRAW != self.usage {
                self.indices = None;
            }
        }
        if self.flags.contains(Flags::DIRTY) {
            if self.indices.is_some() {
                let is = AsRef::<[GLuint]>::as_ref(self);
                let buffer = self.buffer.as_ref().ok_or_else(|| {
                    Error::OptNone(
                        "graphics: submesh: check_draw: self.buffer"
                            .to_string(),
                    )
                })?;
                unsafe {
                    let _ = buffer.sub_data(
                        0,
                        is.len() * size_of::<GLuint>(),
                        is.as_ptr(),
                    )?;
                }
            }
            self.flags.remove(Flags::DIRTY);
        }
        Ok(self)
    }
    // ------------------------------------------------------------------------
    /// draw_impl
    fn draw_impl(&mut self) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::FrontFace(::gl::CCW);
                ::gl::Enable(::gl::CULL_FACE);
                let buffer = self.buffer.as_ref().ok_or_else(|| {
                    Error::OptNone(
                        "graphics: submesh: draw_impl: self.buffer"
                            .to_string(),
                    )
                })?;
                let is_alpha = false; // TODO(hanepjiv): alpha
                if is_alpha {
                    ::gl::Enable(::gl::BLEND);
                    ::gl::BlendFunc(
                        ::gl::SRC_ALPHA,
                        ::gl::ONE_MINUS_SRC_ALPHA,
                    );
                    ::gl::DepthMask(::gl::FALSE);

                    ::gl::CullFace(::gl::FRONT);

                    let _ = buffer.draw_elements(self.mode, self.len)?;

                    ::gl::CullFace(::gl::BACK);

                    let _ = buffer.draw_elements(self.mode, self.len)?;
                } else {
                    ::gl::Disable(::gl::BLEND);
                    ::gl::DepthMask(::gl::TRUE);
                    ::gl::CullFace(::gl::BACK);

                    if self.flags.contains(Flags::CULLING) {
                        ::gl::Enable(::gl::CULL_FACE);
                        if self.flags.contains(Flags::BACKFACE) {
                            ::gl::CullFace(::gl::FRONT);
                        } else {
                            ::gl::CullFace(::gl::BACK);
                        }
                    } else {
                        ::gl::Disable(::gl::CULL_FACE);
                    }

                    let _ = buffer.draw_elements(self.mode, self.len)?;
                }
                Ok(())
            }
        })?;
        Ok(())
    }
    // ------------------------------------------------------------------------
    /// draw
    pub fn draw(
        &mut self,
        prog: &Program,
        materials: &[ManagedValue<Material>],
    ) -> Result<()> {
        let _ = self.check_draw()?;
        if let Some(material_index) = self.material_index {
            let material = materials[material_index].as_ref().borrow();
            let _ = material.emit(prog)?;
        }
        self.draw_impl()
    }
    // ------------------------------------------------------------------------
    /// draw_silhouette
    pub fn draw_silhouette(
        &mut self,
        prog: &Program,
        materials: &[ManagedValue<Material>],
    ) -> Result<()> {
        let _ = self.check_draw()?;
        if let Some(material_index) = self.material_index {
            let material = materials[material_index].as_ref().borrow();
            let _ = material.emit_silhouette(prog)?;
        }
        self.draw_impl()
    }
}
