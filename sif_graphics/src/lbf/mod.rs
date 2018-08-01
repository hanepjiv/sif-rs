// -*- mode:rust; coding:utf-8-unix; -*-

//! mod.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/02
//  @date 2018/08/01

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::{fs::File, io::Read, path::Path};
// ----------------------------------------------------------------------------
use gl::types::*;
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_manager::Manager;
use sif_three::{Armature, Graph};
// ----------------------------------------------------------------------------
use super::{
    Camera as GraphicsCamera, Image, IntoGraphics, Light as GraphicsLight,
    Material as GraphicsMaterial, MaterialFlags, Mesh as GraphicsMesh,
    Model as GraphicsModel, Object as GraphicsObject, ObjectData,
    Result as GraphicsResult, Scene as GraphicsScene,
    Texture as GraphicsTexture,
};
// ============================================================================
pub use self::error::{Error, Result};
// ----------------------------------------------------------------------------
use self::{
    light::Light as LBFLight, material::Material as LBFMaterial,
    mesh::Mesh as LBFMesh, model::Model as LBFModel,
    object::Object as LBFObject, polygon::Flags as LBFPolygonFlags,
    polygon::Polygon as LBFPolygon, texture::Texture as LBFTexture,
};
// mod  =======================================================================
pub mod error;
mod light;
mod loader;
mod material;
mod mesh;
mod model;
mod object;
mod polygon;
mod texture;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct LBFScene
#[derive(Debug, Default, Clone)]
pub struct LBFScene<'a> {
    /// images
    images: Vec<Image>,
    /// textures
    textures: Vec<LBFTexture<'a>>,
    /// materials
    materials: Vec<LBFMaterial<'a>>,
    /// meshes
    meshes: Vec<LBFMesh>,
    /// armatures
    armatures: Vec<Armature<GLfloat>>,
    /// models
    models: Vec<LBFModel<'a>>,
    /// lights
    lights: Vec<LBFLight>,
    /// cameras
    cameras: Vec<GraphicsCamera>,
    /// objects
    objects: Vec<LBFObject<'a>>,
}
// ============================================================================
impl<'a> AsRef<Vec<Image>> for LBFScene<'a> {
    fn as_ref(&self) -> &Vec<Image> {
        &self.images
    }
}
// ----------------------------------------------------------------------------
impl<'a> AsMut<Vec<Image>> for LBFScene<'a> {
    fn as_mut(&mut self) -> &mut Vec<Image> {
        &mut self.images
    }
}
// ============================================================================
impl<'a> AsRef<Vec<LBFTexture<'a>>> for LBFScene<'a> {
    fn as_ref(&self) -> &Vec<LBFTexture<'a>> {
        &self.textures
    }
}
// ----------------------------------------------------------------------------
impl<'a> AsMut<Vec<LBFTexture<'a>>> for LBFScene<'a> {
    fn as_mut(&mut self) -> &mut Vec<LBFTexture<'a>> {
        &mut self.textures
    }
}
// ============================================================================
impl<'a> AsRef<Vec<LBFMaterial<'a>>> for LBFScene<'a> {
    fn as_ref(&self) -> &Vec<LBFMaterial<'a>> {
        &self.materials
    }
}
// ----------------------------------------------------------------------------
impl<'a> AsMut<Vec<LBFMaterial<'a>>> for LBFScene<'a> {
    fn as_mut(&mut self) -> &mut Vec<LBFMaterial<'a>> {
        &mut self.materials
    }
}
// ============================================================================
impl<'a> AsRef<Vec<LBFMesh>> for LBFScene<'a> {
    fn as_ref(&self) -> &Vec<LBFMesh> {
        &self.meshes
    }
}
// ----------------------------------------------------------------------------
impl<'a> AsMut<Vec<LBFMesh>> for LBFScene<'a> {
    fn as_mut(&mut self) -> &mut Vec<LBFMesh> {
        &mut self.meshes
    }
}
// ============================================================================
impl<'a> AsRef<Vec<Armature<GLfloat>>> for LBFScene<'a> {
    fn as_ref(&self) -> &Vec<Armature<GLfloat>> {
        &self.armatures
    }
}
// ----------------------------------------------------------------------------
impl<'a> AsMut<Vec<Armature<GLfloat>>> for LBFScene<'a> {
    fn as_mut(&mut self) -> &mut Vec<Armature<GLfloat>> {
        &mut self.armatures
    }
}
// ============================================================================
impl<'a> AsRef<Vec<LBFModel<'a>>> for LBFScene<'a> {
    fn as_ref(&self) -> &Vec<LBFModel<'a>> {
        &self.models
    }
}
// ----------------------------------------------------------------------------
impl<'a> AsMut<Vec<LBFModel<'a>>> for LBFScene<'a> {
    fn as_mut(&mut self) -> &mut Vec<LBFModel<'a>> {
        &mut self.models
    }
}
// ============================================================================
impl<'a> AsRef<Vec<LBFLight>> for LBFScene<'a> {
    fn as_ref(&self) -> &Vec<LBFLight> {
        &self.lights
    }
}
// ----------------------------------------------------------------------------
impl<'a> AsMut<Vec<LBFLight>> for LBFScene<'a> {
    fn as_mut(&mut self) -> &mut Vec<LBFLight> {
        &mut self.lights
    }
}
// ============================================================================
impl<'a> AsRef<Vec<GraphicsCamera>> for LBFScene<'a> {
    fn as_ref(&self) -> &Vec<GraphicsCamera> {
        &self.cameras
    }
}
// ----------------------------------------------------------------------------
impl<'a> AsMut<Vec<GraphicsCamera>> for LBFScene<'a> {
    fn as_mut(&mut self) -> &mut Vec<GraphicsCamera> {
        &mut self.cameras
    }
}
// ============================================================================
impl<'a> AsRef<Vec<LBFObject<'a>>> for LBFScene<'a> {
    fn as_ref(&self) -> &Vec<LBFObject<'a>> {
        &self.objects
    }
}
// ----------------------------------------------------------------------------
impl<'a> AsMut<Vec<LBFObject<'a>>> for LBFScene<'a> {
    fn as_mut(&mut self) -> &mut Vec<LBFObject<'a>> {
        &mut self.objects
    }
}
// ============================================================================
impl<'a> LBFScene<'a> {
    // ========================================================================
    /// load
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let mut src = String::new();
        let _ = File::open(path.as_ref())?.read_to_string(&mut src)?;
        self::loader::from_str(path, &src)
    }
}
// ============================================================================
impl<'a> IntoGraphics for LBFScene<'a> {
    type Target = GraphicsScene;
    type Param = GLint;
    // ========================================================================
    fn into_graphics(
        mut self,
        scene: &GraphicsScene,
        texture_size: Self::Param,
    ) -> GraphicsResult<Self::Target> {
        let mut graph = Graph::<GLfloat>::new(Uuid::new_v4())?;
        let mut images = Manager::default();
        let mut textures = Manager::default();
        let mut materials = Manager::default();
        let mut meshes = Manager::default();
        let mut armatures = Manager::default();
        let mut models = Manager::default();
        let mut lights = Manager::default();
        let mut cameras = Manager::default();
        let mut objects = Manager::default();

        while let Some(v) = AsMut::<Vec<Image>>::as_mut(&mut self).pop() {
            info!(
                "Image: \"{}\", {}, {:?}",
                AsRef::<String>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v).as_bytes()
            );
            let _ = images.insert(v)?;
        }

        while let Some(v) = AsMut::<Vec<LBFTexture>>::as_mut(&mut self).pop() {
            info!(
                "Texture: \"{}\", {}, {:?}",
                AsRef::<String>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v).as_bytes()
            );
            let _ = textures.insert(v.into_graphics(scene, &images)?)?;
        }

        while let Some(v) = AsMut::<Vec<LBFMaterial>>::as_mut(&mut self).pop()
        {
            info!(
                "Material: \"{}\", {}, {:?}",
                AsRef::<String>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v).as_bytes()
            );
            let _ = materials.insert(v.into_graphics(scene, &textures)?)?;
        }

        while let Some(v) = AsMut::<Vec<LBFMesh>>::as_mut(&mut self).pop() {
            info!(
                "Mesh: \"{}\", {}, {:?}",
                AsRef::<String>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v).as_bytes()
            );
            let _ = meshes.insert(v.into_graphics(scene, ())?)?;
        }

        while let Some(v) =
            AsMut::<Vec<Armature<GLfloat>>>::as_mut(&mut self).pop()
        {
            info!(
                "Armature<GLfloat>: \"{}\", {}, {:?}",
                AsRef::<String>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v).as_bytes()
            );
            let _ = armatures.insert(v)?;
        }

        while let Some(v) = AsMut::<Vec<LBFModel>>::as_mut(&mut self).pop() {
            info!(
                "Model: \"{}\", {}, {:?}",
                AsRef::<String>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v).as_bytes()
            );
            let _ = models.insert(
                v.into_graphics(scene, (&meshes, &materials, &armatures))?,
            )?;
        }

        while let Some(v) = AsMut::<Vec<LBFLight>>::as_mut(&mut self).pop() {
            info!(
                "Light: \"{}\", {}, {:?}",
                AsRef::<String>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v).as_bytes()
            );
            let _ = lights.insert(v.into_graphics(scene, texture_size)?)?;
        }

        while let Some(v) =
            AsMut::<Vec<GraphicsCamera>>::as_mut(&mut self).pop()
        {
            info!(
                "Camera: \"{}\", {}, {:?}",
                AsRef::<String>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v).as_bytes()
            );
            let _ = cameras.insert(v)?;
        }

        let objs = AsMut::<Vec<LBFObject<'a>>>::as_mut(&mut self);
        objs.reverse(); // reverse for pop (= get last).
        while let Some(v) = objs.pop() {
            info!(
                "Object: \"{}\", {}, {:?}",
                AsRef::<String>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v).as_bytes()
            );
            let _ = objects.insert(v.into_graphics(
                scene,
                (&mut graph, &models, &lights, &cameras),
            )?)?;
        }

        GraphicsScene::build(
            graph, images, textures, materials, meshes, armatures, models,
            lights, cameras, objects,
        )
    }
}
// ////////////////////////////////////////////////////////////////////////////
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
