// -*- mode:rust; coding:utf-8-unix; -*-

//! mesh.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/18
//  @date 2018/06/14

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use std::{collections::BTreeMap, mem::size_of};
// ----------------------------------------------------------------------------
use gl::types::*;
use num::Float;
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_manager::ManagedValue;
use sif_math::{Vector2, Vector3};
use sif_renderer::{Buffer, Program};
// ----------------------------------------------------------------------------
use super::Material;
// ============================================================================
use super::{lbf, submesh, Element, Error, Offsets, Result, SubMesh};
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
    // check_mode
    fn check_mode(len: usize) -> Result<GLenum> {
        match len {
            3 => Ok(::gl::TRIANGLES),
            _ => Err(Error::Mesh(format!(
                "{}({}): Mesh::check_mode: unsupported",
                file!(),
                line!()
            ))),
        }
    }
    // ========================================================================
    /// from_lbf
    pub fn from_lbf(lbf_mesh: &lbf::LBFMesh) -> Result<Mesh> {
        // --------------------------------------------------------------------
        /// struct TmpKey
        #[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
        struct TmpKey {
            /// mode
            pub(crate) mode: GLenum,
            /// material
            pub(crate) material: Option<usize>,
            /// flags
            pub(crate) flags: submesh::Flags,
        }
        // --------------------------------------------------------------------
        /// struct Tmp
        #[derive(Debug, Default, Clone)]
        struct Tmp {
            /// idxs
            pub(crate) idxs: Vec<(usize, usize, usize)>,
            /// nors
            pub(crate) nors: Vec<Vector3<GLfloat>>,
            /// tans
            pub(crate) tans: Vec<Vector3<GLfloat>>,
            /// bins
            pub(crate) bins: Vec<Vector3<GLfloat>>,
            /// indices
            pub(crate) indices: Vec<GLuint>,
        }

        let (mut offsets, mut stride) = lbf_mesh.offsets_stride();
        let mut tmps = BTreeMap::<TmpKey, Tmp>::default();
        let mut tmp_vertices = Vec::<Vec<GLfloat>>::default();

        if let Some(ref polygons) = lbf_mesh.polygones {
            for polygon in polygons {
                let tmp_key = TmpKey {
                    mode: Self::check_mode(polygon.indices.len())?,
                    material: polygon.material_index,
                    flags: polygon.submesh_flags(),
                };
                let tmp = tmps.entry(tmp_key.clone())
                    .or_insert_with(Tmp::default);

                let mut p = Vector3::<GLfloat>::from(
                    &lbf_mesh.elem(Element::POSITION, polygon.indices[1].0)?
                        [0..3],
                );
                let mut q = Vector3::<GLfloat>::from(
                    &lbf_mesh.elem(Element::POSITION, polygon.indices[2].0)?
                        [0..3],
                );
                {
                    let e = Vector3::<GLfloat>::from(
                        &lbf_mesh
                            .elem(Element::POSITION, polygon.indices[0].0)?
                            [0..3],
                    );
                    p -= e;
                    q -= e;
                }
                if p.length2() < GLfloat::epsilon()
                    || q.length2() < GLfloat::epsilon()
                {
                    return Err(Error::Mesh(format!(
                        "{}({}): Mesh::from_lbf: degenerate face",
                        file!(),
                        line!()
                    )));
                }

                let p_nor = *p.cross(&q).normalize();
                let (tan, bin) = if !offsets.check(Element::COORD) {
                    (
                        Vector3::<GLfloat>::default(),
                        Vector3::<GLfloat>::default(),
                    )
                } else {
                    // tangent
                    let mut c1 = Vector2::<GLfloat>::from(
                        &lbf_mesh.elem(Element::COORD, polygon.indices[1].1)?
                            [0..2],
                    );
                    let mut c2 = Vector2::<GLfloat>::from(
                        &lbf_mesh.elem(Element::COORD, polygon.indices[2].1)?
                            [0..2],
                    );
                    {
                        let c0 = Vector2::<GLfloat>::from(
                            &lbf_mesh
                                .elem(Element::COORD, polygon.indices[0].1)?
                                [0..2],
                        );
                        c1 -= c0;
                        c2 -= c0;
                    }
                    {
                        let det = c1[0] * c2[1] - c2[0] * c1[1];
                        c1[0] /= det;
                        c1[1] /= -det;
                        c2[0] /= -det;
                        c2[1] /= det;
                    }
                    (
                        *(p * c2[1] + q * c1[1]).normalize(), // tan
                        *(p * c2[0] + q * c1[0]).normalize(), // bin
                    )
                };

                for i in &polygon.indices {
                    let idx = (
                        i.0,
                        i.1,
                        if tmp_key.flags.contains(submesh::Flags::SMOOTH) {
                            tmp.nors.push(Vector3::<GLfloat>::from(
                                &lbf_mesh.elem(Element::NORMAL, i.0)?[0..3],
                            ));
                            tmp.nors.len() - 1
                        } else {
                            tmp.nors.push(p_nor);
                            tmp.nors.len() - 1
                        },
                    );
                    tmp.tans.push(tan);
                    tmp.bins.push(bin);
                    tmp.idxs.push(idx);
                }
            }
            for tmp in tmps.values_mut() {
                for idx in &tmp.idxs {
                    let nor = *tmp.nors[idx.2].normalize();

                    let mut vtx = Vec::<GLfloat>::default();
                    lbf_mesh.expand_elem(&mut vtx, Element::POSITION, idx.0)?;
                    vtx.push(nor[0]);
                    vtx.push(nor[1]);
                    vtx.push(nor[2]);
                    lbf_mesh.expand_elem(&mut vtx, Element::COORD, idx.1)?;
                    if offsets.check(Element::BONE) {
                        lbf_mesh.expand_elem(&mut vtx, Element::BONE, idx.0)?;
                    }
                    if offsets.check(Element::WEIGHT) {
                        lbf_mesh.expand_elem(
                            &mut vtx,
                            Element::WEIGHT,
                            idx.0,
                        )?;
                    }
                    if offsets.check(Element::COORD) {
                        // tangent
                        let mut tan = *tmp.tans[idx.2].normalize();
                        let mut bin = *tmp.bins[idx.2].normalize();
                        {
                            // Gram-Schmidt orthogonalization
                            tan -= nor * nor.dot(&tan);
                            let _ = tan.normalize();
                            bin -= nor * nor.dot(&bin) + tan * tan.dot(&bin);
                            let _ = bin.normalize();
                        }
                        vtx.push(tan[0]);
                        vtx.push(tan[1]);
                        vtx.push(tan[2]);
                        vtx.push(if nor.cross(&tan).dot(&bin) > 0.0 {
                            1.0
                        } else {
                            -1.0
                        });
                    }

                    let mut l = tmp_vertices.len();
                    'same_check: for (i, v) in tmp_vertices.iter().enumerate()
                    {
                        for j in 0..v.len() {
                            if (v[j] - vtx[j]).abs() > GLfloat::epsilon() {
                                continue 'same_check;
                            }
                        }
                        l = i;
                        break;
                    }
                    if l == tmp_vertices.len() {
                        tmp_vertices.push(vtx);
                        tmp.indices
                            .push(tmp_vertices.len() as GLuint - 1);
                    } else {
                        tmp.indices.push(l as GLuint);
                    }
                }
            }
        }

        if offsets.check(Element::COORD) {
            // tangent
            offsets[Element::TANGENT] = stride as isize;
            stride += Element::TANGENT.len().ok_or_else(|| {
                Error::OptNone(
                    "graphics: mesh: from_lbf: TANGENT.len()".to_string(),
                )
            })?;
        }

        let mut vertices = Vec::<GLfloat>::default();
        for v in tmp_vertices {
            for x in v {
                vertices.push(x);
            }
        }

        let mut submeshes = Vec::<SubMesh>::default();
        for (key, tmp) in &tmps {
            submeshes.push(SubMesh::new(
                &tmp.indices[..],
                ::gl::STATIC_DRAW,
                key.mode,
                key.material,
                key.flags,
            ));
        }

        Ok(Mesh::new(
            lbf_mesh.uuid,
            lbf_mesh.name.clone(),
            offsets,
            stride,
            &vertices[..],
            ::gl::STATIC_DRAW,
            submeshes,
        ))
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
            e.len().ok_or_else(|| {
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
        materials: Option<&Vec<ManagedValue<Material>>>,
    ) -> Result<()> {
        let _ = self.check_draw()?;
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
        materials: Option<&Vec<ManagedValue<Material>>>,
    ) -> Result<()> {
        let _ = self.check_draw()?;
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
