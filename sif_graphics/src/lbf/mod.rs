// -*- mode:rust; coding:utf-8-unix; -*-

//! mod.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/02
//  @date 2018/08/05

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
    Animation, AnimationDriver as GraphicsAnimationDriver, Camera, Curve,
    CurveType, Image, Interpolation, IntoGraphics, Keyframe,
    Light as GraphicsLight, Material as GraphicsMaterial, MaterialFlags,
    Mesh as GraphicsMesh, Model as GraphicsModel, Object as GraphicsObject,
    ObjectData, Result as GraphicsResult, Scene as GraphicsScene,
    Texture as GraphicsTexture,
};
// ============================================================================
pub use self::error::{Error, Result};
// ----------------------------------------------------------------------------
use self::{
    animation_driver::Driver as LBFAnimationDriver, light::Light as LBFLight,
    material::Material as LBFMaterial, mesh::Mesh as LBFMesh,
    model::Model as LBFModel, object::Object as LBFObject,
    polygon::Flags as LBFPolygonFlags, polygon::Polygon as LBFPolygon,
    texture::Texture as LBFTexture,
};
// mod  =======================================================================
mod animation_driver;
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
pub struct LBFScene<'a, 'b> {
    /// images
    images: Vec<Image>,
    /// textures
    textures: Vec<LBFTexture<'a, 'b>>,
    /// materials
    materials: Vec<LBFMaterial<'a, 'b>>,
    /// meshes
    meshes: Vec<LBFMesh>,
    /// armatures
    armatures: Vec<Armature<GLfloat>>,
    /// models
    models: Vec<LBFModel<'a, 'b>>,
    /// lights
    lights: Vec<LBFLight>,
    /// cameras
    cameras: Vec<Camera>,
    /// animations
    animations: Vec<Animation<GLfloat>>,
    /// objects
    objects: Vec<LBFObject<'a, 'b>>,
    /// animation_drivers
    animation_drivers: Vec<LBFAnimationDriver<'a, 'b>>,
}
// ============================================================================
impl<'a, 'b> LBFScene<'a, 'b> {
    // ========================================================================
    /// load
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let mut src = String::new();
        let _ = File::open(path.as_ref())?.read_to_string(&mut src)?;
        self::loader::from_str(path, &src)
    }
}
// ============================================================================
impl<'a, 'b> IntoGraphics for LBFScene<'a, 'b> {
    type Target = GraphicsScene;
    type Param = (&'a mut GraphicsScene, GLint);
    // ========================================================================
    fn into_graphics(
        mut self,
        (scene, texture_size): Self::Param,
    ) -> GraphicsResult<(Self::Target, Self::Param)> {
        let mut graph = Graph::<GLfloat>::new(Uuid::new_v4())?;

        let mut images = Manager::default();
        while let Some(v) = self.images.pop() {
            info!(
                "Image: \"{}\", {}, {:?}",
                AsRef::<String>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v).as_bytes()
            );
            let _ = images.insert(v)?;
        }

        let mut textures = Manager::default();
        while let Some(v) = self.textures.pop() {
            info!(
                "Texture: \"{}\", {}, {:?}",
                AsRef::<String>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v).as_bytes()
            );
            let (texture, _) = v.into_graphics((scene, &images))?;
            let _ = textures.insert(texture)?;
        }

        let mut materials = Manager::default();
        while let Some(v) = self.materials.pop() {
            info!(
                "Material: \"{}\", {}, {:?}",
                AsRef::<String>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v).as_bytes()
            );
            let (material, _) = v.into_graphics((scene, &textures))?;
            let _ = materials.insert(material)?;
        }

        let mut meshes = Manager::default();
        while let Some(v) = self.meshes.pop() {
            info!(
                "Mesh: \"{}\", {}, {:?}",
                AsRef::<String>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v).as_bytes()
            );
            let (mesh, _) = v.into_graphics(())?;
            let _ = meshes.insert(mesh)?;
        }

        let mut armatures = Manager::default();
        while let Some(v) = self.armatures.pop() {
            info!(
                "Armature<GLfloat>: \"{}\", {}, {:?}",
                AsRef::<String>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v).as_bytes()
            );
            let _ = armatures.insert(v)?;
        }

        let mut models = Manager::default();
        while let Some(v) = self.models.pop() {
            info!(
                "Model: \"{}\", {}, {:?}",
                AsRef::<String>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v).as_bytes()
            );
            let (model, _) =
                v.into_graphics((scene, &meshes, &materials, &armatures))?;
            let _ = models.insert(model)?;
        }

        let mut lights = Manager::default();
        while let Some(v) = self.lights.pop() {
            info!(
                "Light: \"{}\", {}, {:?}",
                AsRef::<String>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v).as_bytes()
            );
            let (light, _) = v.into_graphics(texture_size)?;
            let _ = lights.insert(light)?;
        }

        let mut cameras = Manager::default();
        while let Some(v) = self.cameras.pop() {
            info!(
                "Camera: \"{}\", {}, {:?}",
                AsRef::<String>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v).as_bytes()
            );
            let _ = cameras.insert(v)?;
        }

        let mut animations = Manager::default();
        while let Some(v) = self.animations.pop() {
            info!(
                "Animation: \"{}\", {}, {:?}",
                AsRef::<String>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v).as_bytes()
            );
            let _ = animations.insert(v)?;
        }

        let mut objects = Manager::default();
        self.objects.reverse(); // reverse for pop (= get last).
        while let Some(v) = self.objects.pop() {
            info!(
                "Object: \"{}\", {}, {:?}",
                AsRef::<String>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v).as_bytes()
            );
            let (object, _) = v.into_graphics((
                scene, &mut graph, &models, &lights, &cameras,
            ))?;
            let _ = objects.insert(object)?;
        }

        let mut animation_drivers = Manager::default();
        while let Some(v) = self.animation_drivers.pop() {
            info!(
                "AnimationDriver: \"{}\", {}, {:?}",
                AsRef::<String>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v),
                AsRef::<Uuid>::as_ref(&v).as_bytes()
            );
            let (animation_driver, _) =
                v.into_graphics((scene, &animations, &objects))?;
            let _ = animation_drivers.insert(animation_driver)?;
        }

        Ok((
            GraphicsScene::build(
                graph,
                images,
                textures,
                materials,
                meshes,
                armatures,
                models,
                lights,
                cameras,
                animations,
                objects,
                animation_drivers,
            )?,
            (scene, texture_size),
        ))
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
