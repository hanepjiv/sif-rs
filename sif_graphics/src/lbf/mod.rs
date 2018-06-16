// -*- mode:rust; coding:utf-8-unix; -*-

//! mod.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/02
//  @date 2018/06/16

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::{fs::File, io::Read, path::Path};
// ----------------------------------------------------------------------------
use gl::types::*;
// ----------------------------------------------------------------------------
use sif_three::Armature;
// ----------------------------------------------------------------------------
use super::{Camera, Image, Material, Model, Texture};
// ============================================================================
pub use self::{
    error::{Error, Result},
    light::Light as LBFLight,
    mesh::Mesh as LBFMesh,
    object::Object as LBFObject,
    polygon::Flags as LBFPolygonFlags,
    polygon::Polygon as LBFPolygon,
};
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
    images: Vec<Image>,
    /// textures
    textures: Vec<Texture>,
    /// materials
    materials: Vec<Material>,
    /// meshes
    meshes: Vec<LBFMesh>,
    /// armatures
    armatures: Vec<Armature<GLfloat>>,
    /// models
    models: Vec<Model>,
    /// lights
    lights: Vec<LBFLight>,
    /// cameras
    cameras: Vec<Camera>,
    /// objects
    objects: Vec<LBFObject>,
}
// ============================================================================
impl AsRef<Vec<Image>> for LBF {
    fn as_ref(&self) -> &Vec<Image> {
        &self.images
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Vec<Image>> for LBF {
    fn as_mut(&mut self) -> &mut Vec<Image> {
        &mut self.images
    }
}
// ============================================================================
impl AsRef<Vec<Texture>> for LBF {
    fn as_ref(&self) -> &Vec<Texture> {
        &self.textures
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Vec<Texture>> for LBF {
    fn as_mut(&mut self) -> &mut Vec<Texture> {
        &mut self.textures
    }
}
// ============================================================================
impl AsRef<Vec<Material>> for LBF {
    fn as_ref(&self) -> &Vec<Material> {
        &self.materials
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Vec<Material>> for LBF {
    fn as_mut(&mut self) -> &mut Vec<Material> {
        &mut self.materials
    }
}
// ============================================================================
impl AsRef<Vec<LBFMesh>> for LBF {
    fn as_ref(&self) -> &Vec<LBFMesh> {
        &self.meshes
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Vec<LBFMesh>> for LBF {
    fn as_mut(&mut self) -> &mut Vec<LBFMesh> {
        &mut self.meshes
    }
}
// ============================================================================
impl AsRef<Vec<Armature<GLfloat>>> for LBF {
    fn as_ref(&self) -> &Vec<Armature<GLfloat>> {
        &self.armatures
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Vec<Armature<GLfloat>>> for LBF {
    fn as_mut(&mut self) -> &mut Vec<Armature<GLfloat>> {
        &mut self.armatures
    }
}
// ============================================================================
impl AsRef<Vec<Model>> for LBF {
    fn as_ref(&self) -> &Vec<Model> {
        &self.models
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Vec<Model>> for LBF {
    fn as_mut(&mut self) -> &mut Vec<Model> {
        &mut self.models
    }
}
// ============================================================================
impl AsRef<Vec<LBFLight>> for LBF {
    fn as_ref(&self) -> &Vec<LBFLight> {
        &self.lights
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Vec<LBFLight>> for LBF {
    fn as_mut(&mut self) -> &mut Vec<LBFLight> {
        &mut self.lights
    }
}
// ============================================================================
impl AsRef<Vec<Camera>> for LBF {
    fn as_ref(&self) -> &Vec<Camera> {
        &self.cameras
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Vec<Camera>> for LBF {
    fn as_mut(&mut self) -> &mut Vec<Camera> {
        &mut self.cameras
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
pub(crate) fn texture_wrap_match(s: &str) -> Result<GLenum> {
    match s {
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
pub(crate) fn texture_filter_match(s: &str) -> Result<GLenum> {
    match s {
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
