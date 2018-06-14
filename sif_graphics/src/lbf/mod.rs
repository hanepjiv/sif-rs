// -*- mode:rust; coding:utf-8-unix; -*-

//! mod.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/02
//  @date 2018/06/14

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::{collections::BTreeMap, fs::File, io::Read, path::Path};
// ----------------------------------------------------------------------------
use gl::types::*;
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_three::Armature;
// ----------------------------------------------------------------------------
use super::{Camera, Image, Material, Model, Texture};
// ============================================================================
pub use self::{error::{Error, Result},
               light::Light as LBFLight,
               mesh::Mesh as LBFMesh,
               object::Object as LBFObject,
               polygon::Flags as LBFPolygonFlags,
               polygon::Polygon as LBFPolygon};
// mod  =======================================================================
pub mod error;
mod light;
mod loader;
mod mesh;
mod object;
mod polygon;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct LBF
#[derive(Debug, Default, Clone)]
pub struct LBF {
    /// images
    images: BTreeMap<Uuid, Image>,
    /// textures
    textures: BTreeMap<Uuid, Texture>,
    /// materials
    materials: BTreeMap<Uuid, Material>,
    /// meshes
    meshes: BTreeMap<Uuid, LBFMesh>,
    /// armatures
    armatures: BTreeMap<Uuid, Armature<GLfloat>>,
    /// models
    models: BTreeMap<Uuid, Model>,
    /// lights
    lights: BTreeMap<Uuid, LBFLight>,
    /// cameras
    cameras: BTreeMap<Uuid, Camera>,
    /// objects
    objects: Vec<LBFObject>,
}
// ============================================================================
impl AsRef<BTreeMap<Uuid, LBFLight>> for LBF {
    fn as_ref(&self) -> &BTreeMap<Uuid, LBFLight> {
        &self.lights
    }
}
// ----------------------------------------------------------------------------
impl AsMut<BTreeMap<Uuid, LBFLight>> for LBF {
    fn as_mut(&mut self) -> &mut BTreeMap<Uuid, LBFLight> {
        &mut self.lights
    }
}
// ============================================================================
impl AsRef<BTreeMap<Uuid, Camera>> for LBF {
    fn as_ref(&self) -> &BTreeMap<Uuid, Camera> {
        &self.cameras
    }
}
// ----------------------------------------------------------------------------
impl AsMut<BTreeMap<Uuid, Camera>> for LBF {
    fn as_mut(&mut self) -> &mut BTreeMap<Uuid, Camera> {
        &mut self.cameras
    }
}
// ============================================================================
impl AsRef<BTreeMap<Uuid, Image>> for LBF {
    fn as_ref(&self) -> &BTreeMap<Uuid, Image> {
        &self.images
    }
}
// ----------------------------------------------------------------------------
impl AsMut<BTreeMap<Uuid, Image>> for LBF {
    fn as_mut(&mut self) -> &mut BTreeMap<Uuid, Image> {
        &mut self.images
    }
}
// ============================================================================
impl AsRef<BTreeMap<Uuid, Texture>> for LBF {
    fn as_ref(&self) -> &BTreeMap<Uuid, Texture> {
        &self.textures
    }
}
// ----------------------------------------------------------------------------
impl AsMut<BTreeMap<Uuid, Texture>> for LBF {
    fn as_mut(&mut self) -> &mut BTreeMap<Uuid, Texture> {
        &mut self.textures
    }
}
// ============================================================================
impl AsRef<BTreeMap<Uuid, Material>> for LBF {
    fn as_ref(&self) -> &BTreeMap<Uuid, Material> {
        &self.materials
    }
}
// ----------------------------------------------------------------------------
impl AsMut<BTreeMap<Uuid, Material>> for LBF {
    fn as_mut(&mut self) -> &mut BTreeMap<Uuid, Material> {
        &mut self.materials
    }
}
// ============================================================================
impl AsRef<BTreeMap<Uuid, LBFMesh>> for LBF {
    fn as_ref(&self) -> &BTreeMap<Uuid, LBFMesh> {
        &self.meshes
    }
}
// ----------------------------------------------------------------------------
impl AsMut<BTreeMap<Uuid, LBFMesh>> for LBF {
    fn as_mut(&mut self) -> &mut BTreeMap<Uuid, LBFMesh> {
        &mut self.meshes
    }
}
// ============================================================================
impl AsRef<BTreeMap<Uuid, Armature<GLfloat>>> for LBF {
    fn as_ref(&self) -> &BTreeMap<Uuid, Armature<GLfloat>> {
        &self.armatures
    }
}
// ----------------------------------------------------------------------------
impl AsMut<BTreeMap<Uuid, Armature<GLfloat>>> for LBF {
    fn as_mut(&mut self) -> &mut BTreeMap<Uuid, Armature<GLfloat>> {
        &mut self.armatures
    }
}
// ============================================================================
impl AsRef<BTreeMap<Uuid, Model>> for LBF {
    fn as_ref(&self) -> &BTreeMap<Uuid, Model> {
        &self.models
    }
}
// ----------------------------------------------------------------------------
impl AsMut<BTreeMap<Uuid, Model>> for LBF {
    fn as_mut(&mut self) -> &mut BTreeMap<Uuid, Model> {
        &mut self.models
    }
}
// ============================================================================
impl AsRef<Vec<LBFObject>> for LBF {
    fn as_ref(&self) -> &Vec<LBFObject> {
        &self.objects
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Vec<LBFObject>> for LBF {
    fn as_mut(&mut self) -> &mut Vec<LBFObject> {
        &mut self.objects
    }
}
// ============================================================================
impl LBF {
    // ========================================================================
    /// load
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let mut src = String::new();
        let _ = File::open(path.as_ref())?.read_to_string(&mut src)?;
        self::loader::from_str(path, &src)
    }
}
// ============================================================================
pub(crate) fn texture_wrap_match(s: String) -> Result<GLenum> {
    match s.as_str() {
        "CLAMP_TO_EDGE" => Ok(::gl::CLAMP_TO_EDGE),
        "REPEAT" => Ok(::gl::REPEAT),
        "MIRRORED_REPEAT" => Ok(::gl::MIRRORED_REPEAT),
        _ => Err(Error::Type(format!(
            "texture_wrap_match: invalid wrap {}",
            s
        ))),
    }
}
// ----------------------------------------------------------------------------
pub(crate) fn texture_filter_match(s: String) -> Result<GLenum> {
    match s.as_str() {
        "NEAREST" => Ok(::gl::NEAREST),
        "LINEAR" => Ok(::gl::LINEAR),
        "NEAREST_MIPMAP_NEAREST" => Ok(::gl::NEAREST_MIPMAP_NEAREST),
        "NEAREST_MIPMAP_LINEAR" => Ok(::gl::NEAREST_MIPMAP_LINEAR),
        "LINEAR_MIPMAP_NEAREST" => Ok(::gl::LINEAR_MIPMAP_NEAREST),
        "LINEAR_MIPMAP_LINEAR" => Ok(::gl::LINEAR_MIPMAP_LINEAR),
        _ => Err(Error::Type(format!(
            "texture_filter_match: invalid filter {}",
            s
        ))),
    }
}
