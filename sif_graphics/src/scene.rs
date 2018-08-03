// -*- mode:rust; coding:utf-8-unix; -*-

//! scene.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/02/27
//  @date 2018/08/05

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use gl::types::*;
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_manager::Manager;
use sif_three::{Armature, Graph};
// ----------------------------------------------------------------------------
use super::{
    Animation, AnimationDriver, Camera, Image, IntoGraphics, Light, Material,
    Mesh, Model, Object, Result, Texture,
};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Scene
#[derive(Debug)]
pub struct Scene {
    /// graph
    graph: Graph<GLfloat>,
    /// images
    images: Manager<Image>,
    /// textures
    textures: Manager<Texture>,
    /// materials
    materials: Manager<Material>,
    /// meshes
    meshes: Manager<Mesh>,
    /// armatures
    armatures: Manager<Armature<GLfloat>>,
    /// models
    models: Manager<Model>,
    /// lights
    lights: Manager<Light>,
    /// cameras
    cameras: Manager<Camera>,
    /// animations
    animations: Manager<Animation<GLfloat>>,
    /// objects
    objects: Manager<Object>,
    /// animation_drivers
    animation_drivers: Manager<AnimationDriver>,
}
// ============================================================================
impl Scene {
    // ========================================================================
    /// fn new
    pub fn new(uuid: Uuid) -> Result<Self> {
        Ok(Scene {
            graph: Graph::<GLfloat>::new(uuid)?,
            images: Manager::default(),
            textures: Manager::default(),
            materials: Manager::default(),
            meshes: Manager::default(),
            armatures: Manager::default(),
            models: Manager::default(),
            lights: Manager::default(),
            cameras: Manager::default(),
            animations: Manager::default(),
            objects: Manager::default(),
            animation_drivers: Manager::default(),
        })
    }
    // ------------------------------------------------------------------------
    /// fn build
    pub fn build(
        graph: Graph<GLfloat>,
        images: Manager<Image>,
        textures: Manager<Texture>,
        materials: Manager<Material>,
        meshes: Manager<Mesh>,
        armatures: Manager<Armature<GLfloat>>,
        models: Manager<Model>,
        lights: Manager<Light>,
        cameras: Manager<Camera>,
        animations: Manager<Animation<GLfloat>>,
        objects: Manager<Object>,
        animation_drivers: Manager<AnimationDriver>,
    ) -> Result<Self> {
        Ok(Scene {
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
        })
    }
    // ========================================================================
    /// fn insert_image
    pub fn insert_image(&mut self, v: Image) -> Result<Uuid> {
        Ok(self.images.insert(v)?)
    }
    // ------------------------------------------------------------------------
    /// fn insert_texture
    pub fn insert_texture(&mut self, v: Texture) -> Result<Uuid> {
        Ok(self.textures.insert(v)?)
    }
    // ------------------------------------------------------------------------
    /// fn insert_material
    pub fn insert_material(&mut self, v: Material) -> Result<Uuid> {
        Ok(self.materials.insert(v)?)
    }
    // ------------------------------------------------------------------------
    /// fn insert_mesh
    pub fn insert_mesh(&mut self, v: Mesh) -> Result<Uuid> {
        Ok(self.meshes.insert(v)?)
    }
    // ------------------------------------------------------------------------
    /// fn insert_armature
    pub fn insert_armature(&mut self, v: Armature<GLfloat>) -> Result<Uuid> {
        Ok(self.armatures.insert(v)?)
    }
    // ------------------------------------------------------------------------
    /// fn insert_model
    pub fn insert_model(&mut self, v: Model) -> Result<Uuid> {
        Ok(self.models.insert(v)?)
    }
    // ------------------------------------------------------------------------
    /// fn insert_light
    pub fn insert_light(&mut self, v: Light) -> Result<Uuid> {
        Ok(self.lights.insert(v)?)
    }
    // ------------------------------------------------------------------------
    /// fn insert_camera
    pub fn insert_camera(&mut self, v: Camera) -> Result<Uuid> {
        Ok(self.cameras.insert(v)?)
    }
    // ------------------------------------------------------------------------
    /// fn insert_object
    pub fn insert_object(&mut self, v: Object) -> Result<Uuid> {
        Ok(self.objects.insert(v)?)
    }
    // ========================================================================
    /// fn append
    pub fn append(&mut self, src: &Self) -> Result<&mut Self> {
        src.graph
            .root()
            .as_ref()
            .borrow_mut()
            .set_parent(Some(self.graph.root().downgrade()));
        for (_k, v) in src.graph.iter() {
            let _ = self.graph.insert(v.clone())?;
        }
        for (_k, v) in src.images.iter() {
            let _ = self.images.insert_managed(v.clone())?;
        }
        for (_k, v) in src.textures.iter() {
            let _ = self.textures.insert_managed(v.clone())?;
        }
        for (_k, v) in src.materials.iter() {
            let _ = self.materials.insert_managed(v.clone())?;
        }
        for (_k, v) in src.meshes.iter() {
            let _ = self.meshes.insert_managed(v.clone())?;
        }
        for (_k, v) in src.armatures.iter() {
            let _ = self.armatures.insert_managed(v.clone())?;
        }
        for (_k, v) in src.models.iter() {
            let _ = self.models.insert_managed(v.clone())?;
        }
        for (_k, v) in src.lights.iter() {
            let _ = self.lights.insert_managed(v.clone())?;
        }
        for (_k, v) in src.cameras.iter() {
            let _ = self.cameras.insert_managed(v.clone())?;
        }
        for (_k, v) in src.animations.iter() {
            let _ = self.animations.insert_managed(v.clone())?;
        }
        for (_k, v) in src.objects.iter() {
            let _ = self.objects.insert_managed(v.clone())?;
        }
        for (_k, v) in src.animation_drivers.iter() {
            let _ = self.animation_drivers.insert_managed(v.clone())?;
        }
        Ok(self)
    }
    // ------------------------------------------------------------------------
    /// fn append_into_graphics
    pub fn append_into_graphics<'a>(
        &'a mut self,
        src: impl IntoGraphics<Target = Scene, Param = (&'a mut Scene, GLint)>,
        texture_size: GLint,
    ) -> Result<&mut Self> {
        let (src, (slf, _)) = src.into_graphics((self, texture_size))?;
        slf.append(&src)
    }
    // ========================================================================
    /// fn elapsed
    pub fn elapsed(&mut self, millisec: isize) -> Result<&mut Self> {
        for (_, ref v) in self.animation_drivers.iter() {
            let _ = v.as_ref().borrow_mut().elapsed(millisec)?;
        }
        Ok(self)
    }
    // ------------------------------------------------------------------------
    /// fn update
    pub fn update(&mut self) -> Result<&mut Self> {
        for (_, ref v) in self.animation_drivers.iter() {
            let _ = v.as_ref().borrow_mut().update()?;
        }
        for (_, ref v) in self.objects.iter() {
            let _ = v.as_ref().borrow_mut().update()?;
        }
        self.graph.update();
        Ok(self)
    }
}
// ============================================================================
impl AsRef<Graph<GLfloat>> for Scene {
    fn as_ref(&self) -> &Graph<GLfloat> {
        &self.graph
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Graph<GLfloat>> for Scene {
    fn as_mut(&mut self) -> &mut Graph<GLfloat> {
        &mut self.graph
    }
}
// ============================================================================
impl AsRef<Manager<Image>> for Scene {
    fn as_ref(&self) -> &Manager<Image> {
        &self.images
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Manager<Image>> for Scene {
    fn as_mut(&mut self) -> &mut Manager<Image> {
        &mut self.images
    }
}
// ============================================================================
impl AsRef<Manager<Texture>> for Scene {
    fn as_ref(&self) -> &Manager<Texture> {
        &self.textures
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Manager<Texture>> for Scene {
    fn as_mut(&mut self) -> &mut Manager<Texture> {
        &mut self.textures
    }
}
// ============================================================================
impl AsRef<Manager<Material>> for Scene {
    fn as_ref(&self) -> &Manager<Material> {
        &self.materials
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Manager<Material>> for Scene {
    fn as_mut(&mut self) -> &mut Manager<Material> {
        &mut self.materials
    }
}
// ============================================================================
impl AsRef<Manager<Mesh>> for Scene {
    fn as_ref(&self) -> &Manager<Mesh> {
        &self.meshes
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Manager<Mesh>> for Scene {
    fn as_mut(&mut self) -> &mut Manager<Mesh> {
        &mut self.meshes
    }
}
// ============================================================================
impl AsRef<Manager<Armature<GLfloat>>> for Scene {
    fn as_ref(&self) -> &Manager<Armature<GLfloat>> {
        &self.armatures
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Manager<Armature<GLfloat>>> for Scene {
    fn as_mut(&mut self) -> &mut Manager<Armature<GLfloat>> {
        &mut self.armatures
    }
}
// ============================================================================
impl AsRef<Manager<Model>> for Scene {
    fn as_ref(&self) -> &Manager<Model> {
        &self.models
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Manager<Model>> for Scene {
    fn as_mut(&mut self) -> &mut Manager<Model> {
        &mut self.models
    }
}
// ============================================================================
impl AsRef<Manager<Light>> for Scene {
    fn as_ref(&self) -> &Manager<Light> {
        &self.lights
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Manager<Light>> for Scene {
    fn as_mut(&mut self) -> &mut Manager<Light> {
        &mut self.lights
    }
}
// ============================================================================
impl AsRef<Manager<Camera>> for Scene {
    fn as_ref(&self) -> &Manager<Camera> {
        &self.cameras
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Manager<Camera>> for Scene {
    fn as_mut(&mut self) -> &mut Manager<Camera> {
        &mut self.cameras
    }
}
// ============================================================================
impl AsRef<Manager<Animation<GLfloat>>> for Scene {
    fn as_ref(&self) -> &Manager<Animation<GLfloat>> {
        &self.animations
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Manager<Animation<GLfloat>>> for Scene {
    fn as_mut(&mut self) -> &mut Manager<Animation<GLfloat>> {
        &mut self.animations
    }
}
// ============================================================================
impl AsRef<Manager<Object>> for Scene {
    fn as_ref(&self) -> &Manager<Object> {
        &self.objects
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Manager<Object>> for Scene {
    fn as_mut(&mut self) -> &mut Manager<Object> {
        &mut self.objects
    }
}
// ============================================================================
impl AsRef<Manager<AnimationDriver>> for Scene {
    fn as_ref(&self) -> &Manager<AnimationDriver> {
        &self.animation_drivers
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Manager<AnimationDriver>> for Scene {
    fn as_mut(&mut self) -> &mut Manager<AnimationDriver> {
        &mut self.animation_drivers
    }
}
