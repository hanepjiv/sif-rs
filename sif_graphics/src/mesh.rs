// -*- mode:rust; coding:utf-8-unix; -*-

//! mesh.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/18
//  @date 2018/08/02

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use std::mem::size_of;
// ----------------------------------------------------------------------------
use gl::types::*;
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_manager::ManagedValue;
use sif_renderer::{Buffer, Program};
// ----------------------------------------------------------------------------
use super::Material;
// ============================================================================
use super::{submesh, Element, Error, Offsets, Result, SubMesh};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// Flags
bitflags! { pub struct Flags: u32 {
    const DIRTY                 = 0b0000_0000_0000_0000_0000_0000_0001_0000u32;
} }
// ============================================================================
impl Default for Flags {
    fn default() -> Self {
        Flags::DIRTY
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Mesh
#[derive(Debug)]
pub struct Mesh {
    /// uuid
    uuid: Uuid,
    /// name
    name: String,
    /// buffer
    buffer: Option<Buffer>,
    /// vertices
    vertices: Option<Vec<GLfloat>>,
    /// usage
    usage: GLenum,
    /// submeshes
    submeshes: Vec<SubMesh>,
    /// offsets
    offsets: Offsets,
    /// stride
    stride: usize,
    /// flags
    flags: Flags,
}
// ============================================================================
impl AsRef<Uuid> for Mesh {
    fn as_ref(&self) -> &Uuid {
        &self.uuid
    }
}
// ============================================================================
impl AsRef<String> for Mesh {
    fn as_ref(&self) -> &String {
        &self.name
    }
}
// ============================================================================
impl AsRef<Flags> for Mesh {
    fn as_ref(&self) -> &Flags {
        &self.flags
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Flags> for Mesh {
    fn as_mut(&mut self) -> &mut Flags {
        &mut self.flags
    }
}
// ============================================================================
impl AsRef<[GLfloat]> for Mesh {
    fn as_ref(&self) -> &[GLfloat] {
        unwrap!(self.vertices.as_ref())
    }
}
// ----------------------------------------------------------------------------
impl AsMut<[GLfloat]> for Mesh {
    fn as_mut(&mut self) -> &mut [GLfloat] {
        self.flags.insert(Flags::DIRTY);
        unwrap!(self.vertices.as_mut())
    }
}
// ============================================================================
impl AsRef<[SubMesh]> for Mesh {
    fn as_ref(&self) -> &[SubMesh] {
        self.submeshes.as_ref()
    }
}
// ----------------------------------------------------------------------------
impl AsMut<[SubMesh]> for Mesh {
    fn as_mut(&mut self) -> &mut [SubMesh] {
        self.submeshes.as_mut()
    }
}
// ============================================================================
impl Mesh {
    // ========================================================================
    /// new
    pub fn new(
        uuid: Uuid,
        name: impl Into<String>,
        offsets: Offsets,
        stride: usize,
        vertices: impl Into<Vec<GLfloat>>,
        usage: GLenum,
        submeshes: Vec<SubMesh>,
    ) -> Self {
        match usage {
            ::gl::STREAM_DRAW | ::gl::STATIC_DRAW | ::gl::DYNAMIC_DRAW => {}
            _ => {
                error!("Mesh::new invalid usage: {}", usage);
            }
        }
        Mesh {
            uuid,
            name: name.into(),
            buffer: None,
            vertices: Some(vertices.into()),
            usage,
            submeshes,
            offsets,
            stride,
            flags: Flags::DIRTY,
        }
    }
    // ========================================================================
    /// new_square
    pub fn new_square(
        uuid: Uuid,
        name: impl Into<String>,
        material_index: Option<usize>,
        mut flags: submesh::Flags,
    ) -> Self {
        flags.remove(submesh::Flags::SMOOTH); // Ignore SMOOTH flag.
        let mut offsets = Offsets::default();
        {
            offsets[Element::POSITION] = 0isize;
            offsets[Element::NORMAL] = 3isize;
            offsets[Element::COORD] = 6isize;
            offsets[Element::TANGENT] = 8isize;
        }
        Mesh::new(
            uuid,
            name,
            offsets,
            12,
            &([
                -1.0 as GLfloat,
                -1.0,
                0.0, // position
                0.0,
                0.0,
                1.0, // normal
                0.0,
                0.0, // coord
                1.0,
                0.0,
                0.0,
                1.0, // tangent
                1.0,
                -1.0,
                0.0,
                0.0,
                0.0,
                1.0,
                1.0,
                0.0,
                1.0,
                0.0,
                0.0,
                1.0,
                1.0,
                1.0,
                0.0,
                0.0,
                0.0,
                1.0,
                1.0,
                1.0,
                1.0,
                0.0,
                0.0,
                1.0,
                -1.0,
                1.0,
                0.0,
                0.0,
                0.0,
                1.0,
                0.0,
                1.0,
                1.0,
                0.0,
                0.0,
                1.0,
            ][..]),
            ::gl::STATIC_DRAW,
            vec![SubMesh::new(
                &[0, 1, 2, 0, 2, 3][..],
                ::gl::STATIC_DRAW,
                ::gl::TRIANGLES,
                material_index,
                flags,
            )],
        )
    }
    // ========================================================================
    /// set_attribute
    fn set_attribute(&self, p: &Program, e: Element) -> Result<&Self> {
        Program::set_attribute(
            sif_renderer_program_location!(
                p,
                e.location_name().ok_or_else(|| Error::OptNone(
                    "graphics: mesh: set_attribute: e.location_name()"
                        .to_string(),
                ))?
            ),
            self.buffer.as_ref().ok_or_else(|| {
                Error::OptNone(
                    "graphics: mesh: set_attribute: self.buffer".to_string(),
                )
            })?,
            e.length().ok_or_else(|| {
                Error::OptNone(
                    "graphics: mesh: set_attribute: e.len()".to_string(),
                )
            })?,
            ::gl::FLOAT,
            ::gl::FALSE,
            size_of::<GLfloat>() * self.stride,
            size_of::<GLfloat>() * self.offsets[e] as usize,
        )?;
        Ok(self)
    }
    // ========================================================================
    /// check_draw
    fn check_draw(&mut self) -> Result<()> {
        if self.buffer.is_none() {
            self.buffer = Some(Buffer::new_vertices(
                self.vertices.as_ref().ok_or_else(|| {
                    Error::OptNone(
                        "graphics: mesh: check_draw: self.indices".to_string(),
                    )
                })?,
                self.usage,
            )?);
            self.flags.remove(Flags::DIRTY);
            if ::gl::DYNAMIC_DRAW != self.usage {
                self.vertices = None;
            }
        }
        if self.flags.contains(Flags::DIRTY) {
            if let Some(ref vs) = self.vertices {
                let buffer = self.buffer.as_ref().ok_or_else(|| {
                    Error::OptNone(
                        "graphics: mesh: check_draw: self.buffer".to_string(),
                    )
                })?;
                unsafe {
                    let _ = buffer.sub_data(
                        0,
                        vs.len() * size_of::<GLfloat>(),
                        vs.as_ptr(),
                    )?;
                }
            }
            self.flags.remove(Flags::DIRTY);
        }
        Ok(())
    }
    // ------------------------------------------------------------------------
    /// draw
    pub fn draw(
        &mut self,
        prog: &Program,
        materials: &[ManagedValue<Material>],
    ) -> Result<()> {
        self.check_draw()?;
        if self.offsets.check(Element::POSITION) {
            let _ = self.set_attribute(prog, Element::POSITION)?;
        }
        if self.offsets.check(Element::NORMAL) {
            let _ = self.set_attribute(prog, Element::NORMAL)?;
        }
        if self.offsets.check(Element::COORD) {
            let _ = self.set_attribute(prog, Element::COORD)?;
        }
        if self.offsets.check(Element::BONE) {
            let _ = self.set_attribute(prog, Element::BONE)?;
        }
        if self.offsets.check(Element::WEIGHT) {
            let _ = self.set_attribute(prog, Element::WEIGHT)?;
        }
        if self.offsets.check(Element::TANGENT) {
            let _ = self.set_attribute(prog, Element::TANGENT)?;
        }
        for s in &mut self.submeshes {
            s.draw(prog, materials)?;
        }
        Ok(())
    }
    // ------------------------------------------------------------------------
    /// draw_silhouette
    pub fn draw_silhouette(
        &mut self,
        prog: &Program,
        materials: &[ManagedValue<Material>],
    ) -> Result<()> {
        self.check_draw()?;
        if self.offsets.check(Element::POSITION) {
            let _ = self.set_attribute(prog, Element::POSITION)?;
        }
        if self.offsets.check(Element::COORD) {
            let _ = self.set_attribute(prog, Element::COORD)?;
        }
        if self.offsets.check(Element::BONE) {
            let _ = self.set_attribute(prog, Element::BONE)?;
        }
        if self.offsets.check(Element::WEIGHT) {
            let _ = self.set_attribute(prog, Element::WEIGHT)?;
        }
        for s in &mut self.submeshes {
            s.draw_silhouette(prog, materials)?;
        }
        Ok(())
    }
}
