// -*- mode:rust; coding:utf-8-unix; -*-

//! object.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/02/23
//  @date 2018/08/27

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::cell::RefCell;
// ----------------------------------------------------------------------------
use gl::types::GLfloat;
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_manager::ManagedValue;
use sif_math::{Float, Vector3, Vector4};
use sif_renderer::Program;
use sif_three::{Armature, AsNodeHolder, NodeHolder, NodeHolderField, Pose};
// ----------------------------------------------------------------------------
use super::{
    Error, Result, {Camera, Light, Model},
};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum ObjectData
#[allow(variant_size_differences)]
#[derive(Debug, Clone)]
pub enum ObjectData<VF>
where
    VF: Float,
{
    /// Empty
    Empty,
    /// Model
    Model(ManagedValue<Model>, Option<Pose<VF>>),
    /// Light
    Light(ManagedValue<Light<VF>>),
    /// Camera
    Camera(ManagedValue<Camera<VF>>),
}
// ============================================================================
impl<VF> AsRef<RefCell<Model>> for ObjectData<VF>
where
    VF: Float,
{
    fn as_ref(&self) -> &RefCell<Model> {
        match *self {
            ObjectData::Model(ref managed, _) => managed.as_ref(),
            _ => panic!("AsRef<RefCell<Model>> for ObjectData"),
        }
    }
}
// ============================================================================
impl<VF> AsRef<Option<Pose<VF>>> for ObjectData<VF>
where
    VF: Float,
{
    fn as_ref(&self) -> &Option<Pose<VF>> {
        match *self {
            ObjectData::Model(_, ref opt) => opt,
            _ => panic!("AsRef<Option<Pose<VF>>> for ObjectData"),
        }
    }
}
// ----------------------------------------------------------------------------
impl<VF> AsMut<Option<Pose<VF>>> for ObjectData<VF>
where
    VF: Float,
{
    fn as_mut(&mut self) -> &mut Option<Pose<VF>> {
        match *self {
            ObjectData::Model(_, ref mut opt) => opt,
            _ => panic!("AsMut<Option<Pose<VF>>> for ObjectData"),
        }
    }
}
// ============================================================================
impl<VF> AsRef<RefCell<Light<VF>>> for ObjectData<VF>
where
    VF: Float,
{
    fn as_ref(&self) -> &RefCell<Light<VF>> {
        match *self {
            ObjectData::Light(ref managed) => managed.as_ref(),
            _ => panic!("AsRef<RefCell<Light<VF>>> for ObjectData"),
        }
    }
}
// ============================================================================
impl<VF> AsRef<RefCell<Camera<VF>>> for ObjectData<VF>
where
    VF: Float,
{
    fn as_ref(&self) -> &RefCell<Camera<VF>> {
        match *self {
            ObjectData::Camera(ref managed) => managed.as_ref(),
            _ => panic!("AsRef<RefCell<Camera>> for ObjectData"),
        }
    }
}
// ============================================================================
impl<VF> ObjectData<VF>
where
    VF: Float,
{
    // ========================================================================
    /// fn is_model
    pub fn is_model(&self) -> bool {
        match *self {
            ObjectData::Model(_, _) => true,
            _ => false,
        }
    }
    // ========================================================================
    /// fn is_light
    pub fn is_light(&self) -> bool {
        match *self {
            ObjectData::Light(_) => true,
            _ => false,
        }
    }
    // ========================================================================
    /// fn is_camera
    pub fn is_camera(&self) -> bool {
        match *self {
            ObjectData::Camera(_) => true,
            _ => false,
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Object
#[derive(Debug, Clone)]
pub struct Object<VF>
where
    VF: Float,
{
    /// uuid
    uuid: Uuid,
    /// name
    name: String,
    /// data
    object_data: ObjectData<VF>,
    /// node_holder_field
    node_holder_field: NodeHolderField<VF>,
}
// ============================================================================
impl<VF> AsRef<Uuid> for Object<VF>
where
    VF: Float,
{
    fn as_ref(&self) -> &Uuid {
        &self.uuid
    }
}
// ============================================================================
impl<VF> AsRef<String> for Object<VF>
where
    VF: Float,
{
    fn as_ref(&self) -> &String {
        &self.name
    }
}
// ============================================================================
impl<VF> AsRef<ObjectData<VF>> for Object<VF>
where
    VF: Float,
{
    fn as_ref(&self) -> &ObjectData<VF> {
        &self.object_data
    }
}
// ----------------------------------------------------------------------------
impl<VF> AsMut<ObjectData<VF>> for Object<VF>
where
    VF: Float,
{
    fn as_mut(&mut self) -> &mut ObjectData<VF> {
        &mut self.object_data
    }
}
// ============================================================================
impl<VF> AsNodeHolder for Object<VF>
where
    VF: Float,
{
    // ========================================================================
    type Float = VF;
    // ========================================================================
    fn as_node_holder(&self) -> &NodeHolderField<Self::Float> {
        &self.node_holder_field
    }
    // ------------------------------------------------------------------------
    fn as_node_holder_mut(&mut self) -> &mut NodeHolderField<Self::Float> {
        &mut self.node_holder_field
    }
}
// ============================================================================
impl<VF> AsRef<RefCell<Light<VF>>> for Object<VF>
where
    VF: Float,
{
    fn as_ref(&self) -> &RefCell<Light<VF>> {
        self.object_data.as_ref()
    }
}
// ----------------------------------------------------------------------------
impl<VF> AsRef<RefCell<Camera<VF>>> for Object<VF>
where
    VF: Float,
{
    fn as_ref(&self) -> &RefCell<Camera<VF>> {
        self.object_data.as_ref()
    }
}
// ----------------------------------------------------------------------------
impl<VF> AsRef<RefCell<Model>> for Object<VF>
where
    VF: Float,
{
    fn as_ref(&self) -> &RefCell<Model> {
        self.object_data.as_ref()
    }
}
// ----------------------------------------------------------------------------
impl<VF> AsRef<Option<Pose<VF>>> for Object<VF>
where
    VF: Float,
{
    fn as_ref(&self) -> &Option<Pose<VF>> {
        self.object_data.as_ref()
    }
}
// ----------------------------------------------------------------------------
impl<VF> AsMut<Option<Pose<VF>>> for Object<VF>
where
    VF: Float,
{
    fn as_mut(&mut self) -> &mut Option<Pose<VF>> {
        self.object_data.as_mut()
    }
}
// ============================================================================
impl<VF> Object<VF>
where
    VF: Float,
{
    // ========================================================================
    /// fn new
    pub fn new(
        uuid: Uuid,
        name: impl Into<String>,
        object_data: ObjectData<VF>,
    ) -> Self {
        Object {
            uuid,
            name: name.into(),
            object_data,
            node_holder_field: NodeHolderField::<VF>::default(),
        }
    }
    // ========================================================================
    /// fn is_light
    pub fn is_light(&self) -> bool {
        self.object_data.is_light()
    }
    // ========================================================================
    /// fn is_camera
    pub fn is_camera(&self) -> bool {
        self.object_data.is_camera()
    }
    // ========================================================================
    /// fn is_model
    pub fn is_model(&self) -> bool {
        self.object_data.is_model()
    }
    // ========================================================================
    /// fn default_left
    pub fn default_left(&self) -> Vector3<VF> {
        match self.object_data {
            ObjectData::Light(_) | ObjectData::Camera(_) => {
                Vector3::from_no_clean([-VF::one(), VF::zero(), VF::zero()])
            }
            _ => Vector3::from_no_clean([VF::one(), VF::zero(), VF::zero()]),
        }
    }
    // ------------------------------------------------------------------------
    /// fn default_up
    pub fn default_up(&self) -> Vector3<VF> {
        match self.object_data {
            _ => Vector3::from_no_clean([VF::zero(), VF::one(), VF::zero()]),
        }
    }
    // ------------------------------------------------------------------------
    /// fn default_front
    pub fn default_front(&self) -> Vector3<VF> {
        match self.object_data {
            ObjectData::Light(_) | ObjectData::Camera(_) => {
                Vector3::from_no_clean([VF::zero(), VF::zero(), -VF::one()])
            }
            _ => Vector3::from_no_clean([VF::zero(), VF::zero(), VF::one()]),
        }
    }
    // ========================================================================
    /// fn position
    pub fn position(&self) -> Result<Vector3<VF>> {
        let n = self.as_node()?.borrow();
        let m = n.as_matrix();
        Ok(Vector3::<VF>::new(m[3][0], m[3][1], m[3][2]))
    }
    // ------------------------------------------------------------------------
    /// fn front
    pub fn front(&self) -> Result<Vector3<VF>> {
        let n = self.as_node()?.borrow();
        let m = n.as_matrix();
        let v4 = *m * Vector4::from_vector3(&self.default_front(), VF::zero());
        let mut v3 = Vector3::<VF>::new(v4[0], v4[1], v4[2]);
        Ok(*v3.normalize())
    }
    // ========================================================================
    /// fn update
    pub fn update(&mut self) -> Result<&mut Object<VF>>
    where
        Model: AsRef<Option<ManagedValue<Armature<VF>>>>,
    {
        if let ObjectData::Model(ref managed_model, Some(ref mut pose)) =
            self.object_data
        {
            let model = managed_model.as_ref().borrow();
            if let Some(managed_armature) =
                AsRef::<Option<ManagedValue<Armature<VF>>>>::as_ref(&*model)
            {
                let armature = managed_armature.as_ref().borrow();
                let _ = armature.update(pose)?;
            }
        }
        Ok(self)
    }
    // ========================================================================
    /// fn emit_pose
    fn emit_pose(pose: &Pose<VF>, prog: &Program) -> Result<()> {
        let l = pose.len();
        if 0 < l {
            Program::set_uniform1i(
                sif_renderer_program_location!(prog, "u_Skinning"),
                1,
            )?;
            Program::set_uniform_matrix4fv(
                sif_renderer_program_location!(prog, "u_Bones[0]"),
                l as i32,
                ::gl::FALSE,
                pose.matrix.as_ptr() as *const GLfloat,
            )?;
        } else {
            Program::set_uniform1i(
                sif_renderer_program_location!(prog, "u_Skinning"),
                0,
            )?;
        }
        Ok(())
    }
    // ========================================================================
    /// fn draw_impl
    fn draw_impl(
        &mut self,
        prog: &Program,
        func: impl FnOnce(&mut Model, &Program) -> Result<()>,
    ) -> Result<()> {
        match self.object_data {
            ObjectData::Model(ref managed, Some(ref pose)) => {
                Object::emit_pose(&pose, prog)?;
                let mut model = managed.as_ref().borrow_mut();
                func(&mut *model, prog)
            }
            ObjectData::Model(ref managed, None) => {
                Program::set_uniform1i(
                    sif_renderer_program_location!(prog, "u_Skinning"),
                    0,
                )?;
                let mut model = managed.as_ref().borrow_mut();
                func(&mut *model, prog)
            }
            _ => Err(Error::InvalidArg(
                "Object::draw_impl: invalid object_data".to_string(),
            )),
        }
    }
    // ------------------------------------------------------------------------
    /// fn draw
    pub fn draw(&mut self, prog: &Program) -> Result<()> {
        self.draw_impl(prog, Model::draw)
    }
    // ------------------------------------------------------------------------
    /// fn draw_silhouette
    pub fn draw_silhouette(&mut self, prog: &Program) -> Result<()> {
        self.draw_impl(prog, Model::draw_silhouette)
    }
}
