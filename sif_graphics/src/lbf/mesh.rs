// -*- mode:rust; coding:utf-8-unix; -*-

//! mesh.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/02
//  @date 2018/08/05

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::collections::BTreeMap;
// ----------------------------------------------------------------------------
use gl::types::*;
use num::Float;
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_math::{Vector2, Vector3};
// ----------------------------------------------------------------------------
use super::{
    super::{submesh, Element, Offsets, SubMesh, ELEMENT_SIZE},
    polygon::Polygon,
    Error, GraphicsMesh, GraphicsResult, IntoGraphics, Result,
};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// Flags
bitflags! {
    #[allow(missing_docs)]
    pub struct Flags: u32 {
    #[allow(missing_docs)]
    const DIRTY         = 0b0000_0000_0000_0000_0000_0000_0001_0000u32;
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Mesh
#[derive(Debug, Clone)]
pub struct Mesh {
    /// uuid
    pub uuid: Uuid,
    /// name
    pub name: String,
    /// position
    pub position: Option<Vec<GLfloat>>,
    /// normal
    pub normal: Option<Vec<GLfloat>>,
    /// coord
    pub coord: Option<Vec<GLfloat>>,
    /// bone
    pub bone: Option<Vec<GLfloat>>,
    /// weight
    pub weight: Option<Vec<GLfloat>>,
    /// polygones
    pub polygones: Option<Vec<Polygon>>,
    /// flags
    pub flags: Flags,
}
// ============================================================================
impl Mesh {
    // ========================================================================
    /// new
    pub fn new(
        uuid: Uuid,
        name: impl Into<String>,
        position: impl Into<Vec<GLfloat>>,
        normal: impl Into<Vec<GLfloat>>,
        coord: impl Into<Vec<GLfloat>>,
        bone: impl Into<Vec<GLfloat>>,
        weight: impl Into<Vec<GLfloat>>,
        polygons: impl Into<Vec<Polygon>>,
    ) -> Result<Self> {
        let pos = position.into();
        let nor = normal.into();

        if pos.len() != nor.len() {
            return Err(Error::Mesh(format!(
                "{}({}): Mesh::new: invalid element length",
                file!(),
                line!()
            )));
        }

        fn vec_opt<T>(vec: Vec<T>) -> Option<Vec<T>> {
            if vec.is_empty() {
                None
            } else {
                Some(vec)
            }
        }

        fn into_opt<T>(vs: impl Into<Vec<T>>) -> Option<Vec<T>> {
            vec_opt(vs.into())
        }

        Ok(Mesh {
            uuid,
            name: name.into(),
            position: vec_opt(pos),
            normal: vec_opt(nor),
            coord: into_opt(coord),
            bone: into_opt(bone),
            weight: into_opt(weight),
            polygones: into_opt(polygons),
            flags: Flags::DIRTY,
        })
    }
    // ========================================================================
    /// offsets_stride
    pub fn offsets_stride(&self) -> (Offsets, usize) {
        let mut offsets = Offsets::default();
        let mut stride = 0usize;

        let mut elem_is_some = [false; ELEMENT_SIZE];
        elem_is_some[Element::POSITION.bits() as usize] =
            self.position.is_some();
        elem_is_some[Element::NORMAL.bits() as usize] = self.normal.is_some();
        elem_is_some[Element::COORD.bits() as usize] = self.coord.is_some();
        elem_is_some[Element::BONE.bits() as usize] = self.bone.is_some();
        elem_is_some[Element::WEIGHT.bits() as usize] = self.weight.is_some();
        elem_is_some[Element::TANGENT.bits() as usize] = false;

        for i in Element::BEGIN_.bits()..Element::END_.bits() {
            if elem_is_some[i as usize] {
                let e = unwrap!(Element::from_bits(i));
                offsets[e] = stride as isize;
                stride += unwrap!(e.length());
            }
        }

        (offsets, stride)
    }
    // ========================================================================
    /// elem
    pub fn elem(&self, e: Element, idx: usize) -> Result<Vec<GLfloat>> {
        let mut vtx = Vec::<GLfloat>::default();
        self.expand_elem(&mut vtx, e, idx)?;
        Ok(vtx)
    }
    // ========================================================================
    /// expand_elem
    pub fn expand_elem(
        &self,
        vtx: &mut Vec<GLfloat>,
        e: Element,
        idx: usize,
    ) -> Result<()> {
        if let Some(ref collect) = *match e {
            Element::POSITION => &self.position,
            Element::NORMAL => &self.normal,
            Element::COORD => &self.coord,
            Element::BONE => &self.bone,
            Element::WEIGHT => &self.weight,
            _ => &None,
        } {
            let l = unwrap!(e.length());
            for i in 0..l {
                vtx.push(collect[l * idx + i]);
            }
            Ok(())
        } else {
            Err(Error::Elem(format!(
                "{}({}): Mesh::expand_elem: unsupported",
                file!(),
                line!()
            )))
        }
    }
    // ========================================================================
    /// check_mode
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
impl IntoGraphics for Mesh {
    type Target = GraphicsMesh;
    type Param = ();
    // ========================================================================
    fn into_graphics(
        self,
        _: Self::Param,
    ) -> GraphicsResult<(Self::Target, Self::Param)> {
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

        let (mut offsets, mut stride) = self.offsets_stride();
        let mut tmps = BTreeMap::<TmpKey, Tmp>::default();
        let mut tmp_vertices = Vec::<Vec<GLfloat>>::default();

        if let Some(ref polygons) = self.polygones {
            for polygon in polygons {
                let tmp_key = TmpKey {
                    mode: Self::check_mode(polygon.indices.len())?,
                    material: polygon.material_index,
                    flags: polygon.submesh_flags(),
                };
                let tmp =
                    tmps.entry(tmp_key.clone()).or_insert_with(Tmp::default);

                let mut p = Vector3::<GLfloat>::from(
                    &self.elem(Element::POSITION, polygon.indices[1].0)?[0..3],
                );
                let mut q = Vector3::<GLfloat>::from(
                    &self.elem(Element::POSITION, polygon.indices[2].0)?[0..3],
                );
                {
                    let e = Vector3::<GLfloat>::from(
                        &self.elem(Element::POSITION, polygon.indices[0].0)?
                            [0..3],
                    );
                    p -= e;
                    q -= e;
                }
                if p.length2() < GLfloat::epsilon()
                    || q.length2() < GLfloat::epsilon()
                {
                    return Err(Error::Mesh(format!(
                        "{}({}): lbf::Mesh: degenerate face",
                        file!(),
                        line!()
                    )).into());
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
                        &self.elem(Element::COORD, polygon.indices[1].1)?
                            [0..2],
                    );
                    let mut c2 = Vector2::<GLfloat>::from(
                        &self.elem(Element::COORD, polygon.indices[2].1)?
                            [0..2],
                    );
                    {
                        let c0 = Vector2::<GLfloat>::from(
                            &self
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
                                &self.elem(Element::NORMAL, i.0)?[0..3],
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
                    self.expand_elem(&mut vtx, Element::POSITION, idx.0)?;
                    vtx.push(nor[0]);
                    vtx.push(nor[1]);
                    vtx.push(nor[2]);
                    self.expand_elem(&mut vtx, Element::COORD, idx.1)?;
                    if offsets.check(Element::BONE) {
                        self.expand_elem(&mut vtx, Element::BONE, idx.0)?;
                    }
                    if offsets.check(Element::WEIGHT) {
                        self.expand_elem(&mut vtx, Element::WEIGHT, idx.0)?;
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
                    'same_loop: for (i, v) in tmp_vertices.iter().enumerate() {
                        for j in 0..v.len() {
                            if (v[j] - vtx[j]).abs() > GLfloat::epsilon() {
                                continue 'same_loop;
                            }
                        }
                        l = i;
                        break;
                    }
                    if l == tmp_vertices.len() {
                        tmp_vertices.push(vtx);
                        tmp.indices.push(tmp_vertices.len() as GLuint - 1);
                    } else {
                        tmp.indices.push(l as GLuint);
                    }
                }
            }
        }

        if offsets.check(Element::COORD) {
            // tangent
            offsets[Element::TANGENT] = stride as isize;
            stride += Element::TANGENT.length().ok_or_else(|| {
                Error::OptNone(
                    "lbf::Mesh: into_graphics: TANGENT.len()".to_string(),
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

        Ok((
            GraphicsMesh::new(
                self.uuid,
                self.name,
                offsets,
                stride,
                &vertices[..],
                ::gl::STATIC_DRAW,
                submeshes,
            ),
            (),
        ))
    }
}
