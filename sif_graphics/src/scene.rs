// -*- mode:rust; coding:utf-8-unix; -*-

//! scene.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/02/27
//  @date 2018/08/27

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::marker::PhantomData;
// ----------------------------------------------------------------------------
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_manager::{ManagedValue, Manager};
use sif_math::{Float, Integer};
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
pub struct Scene<VF, VI>
where
    VF: Float,
    VI: Integer,
{
    /// graph
    graph: Graph<VF>,
    /// images
    images: Manager<Image>,
    /// textures
    textures: Manager<Texture>,
    /// materials
    materials: Manager<Material>,
    /// meshes
    meshes: Manager<Mesh>,
    /// armatures
    armatures: Manager<Armature<VF>>,
    /// models
    models: Manager<Model>,
    /// lights
    lights: Manager<Light<VF>>,
    /// cameras
    cameras: Manager<Camera<VF>>,
    /// animations
    animations: Manager<Animation<VF>>,
    /// objects
    objects: Manager<Object<VF>>,
    /// animation_drivers
    animation_drivers: Manager<AnimationDriver<VF>>,
    /// phantom1
    phantom1: PhantomData<fn() -> VI>,
}
// ============================================================================
impl<VF, VI> Scene<VF, VI>
where
    VF: Float,
    VI: Integer,
{
    // ========================================================================
    /// fn new
    pub fn new(uuid: Uuid) -> Result<Self> {
        Ok(Scene {
            graph: Graph::<VF>::new(uuid)?,
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
            phantom1: PhantomData::default(),
        })
    }
    // ------------------------------------------------------------------------
    /// fn build
    pub fn build(
        graph: Graph<VF>,
        images: Manager<Image>,
        textures: Manager<Texture>,
        materials: Manager<Material>,
        meshes: Manager<Mesh>,
        armatures: Manager<Armature<VF>>,
        models: Manager<Model>,
        lights: Manager<Light<VF>>,
        cameras: Manager<Camera<VF>>,
        animations: Manager<Animation<VF>>,
        objects: Manager<Object<VF>>,
        animation_drivers: Manager<AnimationDriver<VF>>,
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
            phantom1: PhantomData::default(),
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
    pub fn insert_armature(&mut self, v: Armature<VF>) -> Result<Uuid> {
        Ok(self.armatures.insert(v)?)
    }
    // ------------------------------------------------------------------------
    /// fn insert_model
    pub fn insert_model(&mut self, v: Model) -> Result<Uuid> {
        Ok(self.models.insert(v)?)
    }
    // ------------------------------------------------------------------------
    /// fn insert_light
    pub fn insert_light(&mut self, v: Light<VF>) -> Result<Uuid> {
        Ok(self.lights.insert(v)?)
    }
    // ------------------------------------------------------------------------
    /// fn insert_camera
    pub fn insert_camera(&mut self, v: Camera<VF>) -> Result<Uuid> {
        Ok(self.cameras.insert(v)?)
    }
    // ------------------------------------------------------------------------
    /// fn insert_object
    pub fn insert_object(&mut self, v: Object<VF>) -> Result<Uuid> {
        Ok(self.objects.insert(v)?)
    }
    // ========================================================================
    /// fn append
    pub fn append(&mut self, src: &mut Self) -> Result<&mut Self> {
        self.graph.append(&mut src.graph);
        self.images.append(&mut src.images);
        self.textures.append(&mut src.textures);
        self.materials.append(&mut src.materials);
        self.meshes.append(&mut src.meshes);
        self.armatures.append(&mut src.armatures);
        self.models.append(&mut src.models);
        self.lights.append(&mut src.lights);
        self.cameras.append(&mut src.cameras);
        self.animations.append(&mut src.animations);
        self.objects.append(&mut src.objects);
        self.animation_drivers.append(&mut src.animation_drivers);
        Ok(self)
    }
    // ------------------------------------------------------------------------
    /// fn append_graphics
    pub fn append_graphics<'a>(
        &'a mut self,
        src: impl IntoGraphics<
            Target = Scene<VF, VI>,
            Param = (&'a mut Scene<VF, VI>, VI),
        >,
        texture_size: VI,
    ) -> Result<&mut Self> {
        let (mut src, (slf, _)) = src.into_graphics((self, texture_size))?;
        slf.append(&mut src)
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
    pub fn update(&mut self) -> Result<&mut Self>
    where
        Model: AsRef<Option<ManagedValue<Armature<VF>>>>,
    {
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
impl<VF, VI> AsRef<Graph<VF>> for Scene<VF, VI>
where
    VF: Float,
    VI: Integer,
{
    fn as_ref(&self) -> &Graph<VF> {
        &self.graph
    }
}
// ----------------------------------------------------------------------------
impl<VF, VI> AsMut<Graph<VF>> for Scene<VF, VI>
where
    VF: Float,
    VI: Integer,
{
    fn as_mut(&mut self) -> &mut Graph<VF> {
        &mut self.graph
    }
}
// ============================================================================
impl<VF, VI> AsRef<Manager<Image>> for Scene<VF, VI>
where
    VF: Float,
    VI: Integer,
{
    fn as_ref(&self) -> &Manager<Image> {
        &self.images
    }
}
// ----------------------------------------------------------------------------
impl<VF, VI> AsMut<Manager<Image>> for Scene<VF, VI>
where
    VF: Float,
    VI: Integer,
{
    fn as_mut(&mut self) -> &mut Manager<Image> {
        &mut self.images
    }
}
// ============================================================================
impl<VF, VI> AsRef<Manager<Texture>> for Scene<VF, VI>
where
    VF: Float,
    VI: Integer,
{
    fn as_ref(&self) -> &Manager<Texture> {
        &self.textures
    }
}
// ----------------------------------------------------------------------------
impl<VF, VI> AsMut<Manager<Texture>> for Scene<VF, VI>
where
    VF: Float,
    VI: Integer,
{
    fn as_mut(&mut self) -> &mut Manager<Texture> {
        &mut self.textures
    }
}
// ============================================================================
impl<VF, VI> AsRef<Manager<Material>> for Scene<VF, VI>
where
    VF: Float,
    VI: Integer,
{
    fn as_ref(&self) -> &Manager<Material> {
        &self.materials
    }
}
// ----------------------------------------------------------------------------
impl<VF, VI> AsMut<Manager<Material>> for Scene<VF, VI>
where
    VF: Float,
    VI: Integer,
{
    fn as_mut(&mut self) -> &mut Manager<Material> {
        &mut self.materials
    }
}
// ============================================================================
impl<VF, VI> AsRef<Manager<Mesh>> for Scene<VF, VI>
where
    VF: Float,
    VI: Integer,
{
    fn as_ref(&self) -> &Manager<Mesh> {
        &self.meshes
    }
}
// ----------------------------------------------------------------------------
impl<VF, VI> AsMut<Manager<Mesh>> for Scene<VF, VI>
where
    VF: Float,
    VI: Integer,
{
    fn as_mut(&mut self) -> &mut Manager<Mesh> {
        &mut self.meshes
    }
}
// ============================================================================
impl<VF, VI> AsRef<Manager<Armature<VF>>> for Scene<VF, VI>
where
    VF: Float,
    VI: Integer,
{
    fn as_ref(&self) -> &Manager<Armature<VF>> {
        &self.armatures
    }
}
// ----------------------------------------------------------------------------
impl<VF, VI> AsMut<Manager<Armature<VF>>> for Scene<VF, VI>
where
    VF: Float,
    VI: Integer,
{
    fn as_mut(&mut self) -> &mut Manager<Armature<VF>> {
        &mut self.armatures
    }
}
// ============================================================================
impl<VF, VI> AsRef<Manager<Model>> for Scene<VF, VI>
where
    VF: Float,
    VI: Integer,
{
    fn as_ref(&self) -> &Manager<Model> {
        &self.models
    }
}
// ----------------------------------------------------------------------------
impl<VF, VI> AsMut<Manager<Model>> for Scene<VF, VI>
where
    VF: Float,
    VI: Integer,
{
    fn as_mut(&mut self) -> &mut Manager<Model> {
        &mut self.models
    }
}
// ============================================================================
impl<VF, VI> AsRef<Manager<Light<VF>>> for Scene<VF, VI>
where
    VF: Float,
    VI: Integer,
{
    fn as_ref(&self) -> &Manager<Light<VF>> {
        &self.lights
    }
}
// ----------------------------------------------------------------------------
impl<VF, VI> AsMut<Manager<Light<VF>>> for Scene<VF, VI>
where
    VF: Float,
    VI: Integer,
{
    fn as_mut(&mut self) -> &mut Manager<Light<VF>> {
        &mut self.lights
    }
}
// ============================================================================
impl<VF, VI> AsRef<Manager<Camera<VF>>> for Scene<VF, VI>
where
    VF: Float,
    VI: Integer,
{
    fn as_ref(&self) -> &Manager<Camera<VF>> {
        &self.cameras
    }
}
// ----------------------------------------------------------------------------
impl<VF, VI> AsMut<Manager<Camera<VF>>> for Scene<VF, VI>
where
    VF: Float,
    VI: Integer,
{
    fn as_mut(&mut self) -> &mut Manager<Camera<VF>> {
        &mut self.cameras
    }
}
// ============================================================================
impl<VF, VI> AsRef<Manager<Animation<VF>>> for Scene<VF, VI>
where
    VF: Float,
    VI: Integer,
{
    fn as_ref(&self) -> &Manager<Animation<VF>> {
        &self.animations
    }
}
// ----------------------------------------------------------------------------
impl<VF, VI> AsMut<Manager<Animation<VF>>> for Scene<VF, VI>
where
    VF: Float,
    VI: Integer,
{
    fn as_mut(&mut self) -> &mut Manager<Animation<VF>> {
        &mut self.animations
    }
}
// ============================================================================
impl<VF, VI> AsRef<Manager<Object<VF>>> for Scene<VF, VI>
where
    VF: Float,
    VI: Integer,
{
    fn as_ref(&self) -> &Manager<Object<VF>> {
        &self.objects
    }
}
// ----------------------------------------------------------------------------
impl<VF, VI> AsMut<Manager<Object<VF>>> for Scene<VF, VI>
where
    VF: Float,
    VI: Integer,
{
    fn as_mut(&mut self) -> &mut Manager<Object<VF>> {
        &mut self.objects
    }
}
// ============================================================================
impl<VF, VI> AsRef<Manager<AnimationDriver<VF>>> for Scene<VF, VI>
where
    VF: Float,
    VI: Integer,
{
    fn as_ref(&self) -> &Manager<AnimationDriver<VF>> {
        &self.animation_drivers
    }
}
// ----------------------------------------------------------------------------
impl<VF, VI> AsMut<Manager<AnimationDriver<VF>>> for Scene<VF, VI>
where
    VF: Float,
    VI: Integer,
{
    fn as_mut(&mut self) -> &mut Manager<AnimationDriver<VF>> {
        &mut self.animation_drivers
    }
}
