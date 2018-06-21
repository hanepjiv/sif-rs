// -*- mode:rust; coding:utf-8-unix; -*-

//! mesh.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/02
//  @date 2018/06/18

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use gl::types::*;
use uuid::Uuid;
// ----------------------------------------------------------------------------
use super::{
    super::{Element, Offsets, ELEMENT_SIZE},
    polygon::Polygon,
    Error, Result,
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
