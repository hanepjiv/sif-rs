// -*- mode:rust; coding:utf-8-unix; -*-

//! object.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/02/23
//  @date 2018/06/17

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::cell::RefCell;
// ----------------------------------------------------------------------------
use gl::types::*;
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_manager::{ManagedValue, Manager};
use sif_math::{Vector3, Vector4};
use sif_renderer::Program;
use sif_three::{
    Armature, AsNodeHolder, Graph, Node, NodeHolder, NodeHolderField, Pose,
    TraRotSca,
};
// ----------------------------------------------------------------------------
use super::{
    lbf, Error, Result, {Camera, Light, Model},
};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum ObjectData
#[allow(variant_size_differences)]
#[derive(Debug, Clone)]
pub enum ObjectData {
    /// Armature
    Armature(ManagedValue<Armature<GLfloat>>),
    /// Model
    Model(ManagedValue<Model>, Option<Pose<GLfloat>>),
    /// Light
    Light(ManagedValue<Light>),
    /// Camera
    Camera(ManagedValue<Camera>),
}
// ============================================================================
impl AsRef<RefCell<Armature<GLfloat>>> for ObjectData {
    fn as_ref(&self) -> &RefCell<Armature<GLfloat>> {
        match *self {
            ObjectData::Armature(ref managed) => managed.as_ref(),
            _ => panic!("AsRef<RefCell<Armature<GLfloat>>> for ObjectData"),
        }
    }
}
// ============================================================================
impl AsRef<RefCell<Model>> for ObjectData {
    fn as_ref(&self) -> &RefCell<Model> {
        match *self {
            ObjectData::Model(ref managed, _) => managed.as_ref(),
            _ => panic!("AsRef<RefCell<Model>> for ObjectData"),
        }
    }
}
// ============================================================================
impl AsRef<Option<Pose<GLfloat>>> for ObjectData {
    fn as_ref(&self) -> &Option<Pose<GLfloat>> {
        match *self {
            ObjectData::Model(_, ref opt) => opt,
            _ => panic!("AsRef<Option<Pose<GLfloat>>> for ObjectData"),
        }
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Option<Pose<GLfloat>>> for ObjectData {
    fn as_mut(&mut self) -> &mut Option<Pose<GLfloat>> {
        match *self {
            ObjectData::Model(_, ref mut opt) => opt,
            _ => panic!("AsMut<Option<Pose<GLfloat>>> for ObjectData"),
        }
    }
}
// ============================================================================
impl AsRef<RefCell<Light>> for ObjectData {
    fn as_ref(&self) -> &RefCell<Light> {
        match *self {
            ObjectData::Light(ref managed) => managed.as_ref(),
            _ => panic!("AsRef<RefCell<Light>> for ObjectData"),
        }
    }
}
// ============================================================================
impl AsRef<RefCell<Camera>> for ObjectData {
    fn as_ref(&self) -> &RefCell<Camera> {
        match *self {
            ObjectData::Camera(ref managed) => managed.as_ref(),
            _ => panic!("AsRef<RefCell<Camera>> for ObjectData"),
        }
    }
}
// ============================================================================
impl ObjectData {
    // ========================================================================
    /// fn is_armature
    pub fn is_armature(&self) -> bool {
        match *self {
            ObjectData::Armature(_) => true,
            _ => false,
        }
    }
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
pub struct Object {
    /// uuid
    uuid: Uuid,
    /// name
    name: String,
    /// data
    object_data: ObjectData,
    /// node_holder_field
    node_holder_field: NodeHolderField<GLfloat>,
}
// ============================================================================
impl AsRef<Uuid> for Object {
    fn as_ref(&self) -> &Uuid {
        &self.uuid
    }
}
// ============================================================================
impl AsRef<String> for Object {
    fn as_ref(&self) -> &String {
        &self.name
    }
}
// ============================================================================
impl AsRef<ObjectData> for Object {
    fn as_ref(&self) -> &ObjectData {
        &self.object_data
    }
}
// ============================================================================
impl AsNodeHolder for Object {
    // ========================================================================
    type Number = GLfloat;
    // ========================================================================
    fn as_node_holder(&self) -> &NodeHolderField<Self::Number> {
        &self.node_holder_field
    }
    // ------------------------------------------------------------------------
    fn as_node_holder_mut(&mut self) -> &mut NodeHolderField<Self::Number> {
        &mut self.node_holder_field
    }
}
// ============================================================================
impl AsRef<RefCell<Light>> for Object {
    fn as_ref(&self) -> &RefCell<Light> {
        self.object_data.as_ref()
    }
}
// ----------------------------------------------------------------------------
impl AsRef<RefCell<Camera>> for Object {
    fn as_ref(&self) -> &RefCell<Camera> {
        self.object_data.as_ref()
    }
}
// ----------------------------------------------------------------------------
impl AsRef<RefCell<Armature<GLfloat>>> for Object {
    fn as_ref(&self) -> &RefCell<Armature<GLfloat>> {
        self.object_data.as_ref()
    }
}
// ----------------------------------------------------------------------------
impl AsRef<RefCell<Model>> for Object {
    fn as_ref(&self) -> &RefCell<Model> {
        self.object_data.as_ref()
    }
}
// ----------------------------------------------------------------------------
impl AsRef<Option<Pose<GLfloat>>> for Object {
    fn as_ref(&self) -> &Option<Pose<GLfloat>> {
        self.object_data.as_ref()
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Option<Pose<GLfloat>>> for Object {
    fn as_mut(&mut self) -> &mut Option<Pose<GLfloat>> {
        self.object_data.as_mut()
    }
}
// ============================================================================
impl Object {
    // ========================================================================
    /// fn new
    pub fn new(
        uuid: Uuid,
        name: impl Into<String>,
        object_data: ObjectData,
    ) -> Self {
        Object {
            uuid,
            name: name.into(),
            object_data,
            node_holder_field: NodeHolderField::<GLfloat>::default(),
        }
    }
    // ========================================================================
    /// fn from_lbf
    pub fn from_lbf(
        src: &lbf::LBFObject,
        graph: &mut Graph<GLfloat>,
        armatures: &Manager<Armature<GLfloat>>,
        models: &Manager<Model>,
        lights: &Manager<Light>,
        cameras: &Manager<Camera>,
    ) -> Result<Self> {
        if let Some(mut obj) = match src.data_type.as_str() {
            "ARMATURE" => armatures.get(&src.data_uuid).map(|m| {
                Object::new(
                    *AsRef::<Uuid>::as_ref(&src),
                    AsRef::<String>::as_ref(&src).as_str(),
                    ObjectData::Armature(m.clone()),
                )
            }),
            "MODEL" => models.get(&src.data_uuid).map(|m| {
                let armature_len = (*m.as_ref().borrow()).armature_len();
                let pose = if 0 < armature_len {
                    Some(Pose::<GLfloat>::new(armature_len))
                } else {
                    None
                };
                Object::new(
                    *AsRef::<Uuid>::as_ref(&src),
                    AsRef::<String>::as_ref(&src).as_str(),
                    ObjectData::Model(m.clone(), pose),
                )
            }),
            "LIGHT" => lights.get(&src.data_uuid).map(|m| {
                Object::new(
                    *AsRef::<Uuid>::as_ref(&src),
                    AsRef::<String>::as_ref(&src).as_str(),
                    ObjectData::Light(m.clone()),
                )
            }),
            "CAMERA" => cameras.get(&src.data_uuid).map(|m| {
                Object::new(
                    *AsRef::<Uuid>::as_ref(&src),
                    AsRef::<String>::as_ref(&src).as_str(),
                    ObjectData::Camera(m.clone()),
                )
            }),
            _ => None,
        } {
            let parent: Option<ManagedValue<Node<GLfloat>>> =
                if let Some(p) = src.parent {
                    Some(graph.get(&p).ok_or_else(|| {
                        Error::OptNone(
                            "graphics: scene: from_lbf: graph.get".to_string(),
                        )
                    })?)
                } else {
                    None
                };
            let _ = graph.insert(AsRef::<Uuid>::as_ref(&src).clone(), parent)?;
            let node = graph.get(src.as_ref()).ok_or_else(|| {
                Error::OptNone(
                    "graphics: scene: from_lbf: graph.insert".to_string(),
                )
            })?;
            {
                let mut m = node.as_ref().borrow_mut();
                let trs = AsMut::<TraRotSca<GLfloat>>::as_mut(&mut *m);
                trs.translation = src.trarotsca.translation;
                trs.rotation = src.trarotsca.rotation;
                trs.scaling = src.trarotsca.scaling;
            }
            obj.set_node(Some(node));
            Ok(obj)
        } else {
            Err(Error::ManagedNotFound(src.data_uuid))
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
    /// fn is_armature
    pub fn is_armature(&self) -> bool {
        self.object_data.is_armature()
    }
    // ========================================================================
    /// fn is_model
    pub fn is_model(&self) -> bool {
        self.object_data.is_model()
    }
    // ========================================================================
    /// fn default_left
    pub fn default_left(&self) -> Vector3<GLfloat> {
        match self.object_data {
            ObjectData::Light(_) | ObjectData::Camera(_) => {
                Vector3::from_no_clean([-1.0, 0.0, 0.0])
            }
            _ => Vector3::from_no_clean([1.0, 0.0, 0.0]),
        }
    }
    // ------------------------------------------------------------------------
    /// fn default_up
    pub fn default_up(&self) -> Vector3<GLfloat> {
        match self.object_data {
            _ => Vector3::from_no_clean([0.0, 1.0, 0.0]),
        }
    }
    // ------------------------------------------------------------------------
    /// fn default_front
    pub fn default_front(&self) -> Vector3<GLfloat> {
        match self.object_data {
            ObjectData::Light(_) | ObjectData::Camera(_) => {
                Vector3::from_no_clean([0.0, 0.0, -1.0])
            }
            _ => Vector3::from_no_clean([0.0, 0.0, 1.0]),
        }
    }
    // ========================================================================
    /// fn position
    pub fn position(&self) -> Result<Vector3<GLfloat>> {
        let n = self.as_node()?.borrow();
        let m = n.as_matrix();
        Ok(Vector3::<GLfloat>::new(m[3][0], m[3][1], m[3][2]))
    }
    // ------------------------------------------------------------------------
    /// fn front
    pub fn front(&self) -> Result<Vector3<GLfloat>> {
        let n = self.as_node()?.borrow();
        let m = n.as_matrix();
        let v4 = *m * Vector4::from_vector3(&self.default_front(), 0.0);
        let mut v3 = Vector3::<GLfloat>::new(v4[0], v4[1], v4[2]);
        Ok(*v3.normalize())
    }
    // ========================================================================
    /// fn update
    pub fn update(&mut self) -> Result<&mut Object> {
        if let ObjectData::Model(ref managed_model, Some(ref mut pose)) =
            self.object_data
        {
            let model = managed_model.as_ref().borrow();
            let managed_armature =
                AsRef::<ManagedValue<Armature<GLfloat>>>::as_ref(&*model);
            let armature = managed_armature.as_ref().borrow();
            let _ = armature.update(pose)?;
        }
        Ok(self)
    }
    // ========================================================================
    /// fn emit_pose
    fn emit_pose(pose: &Pose<GLfloat>, prog: &Program) -> Result<()> {
        let l = pose.len();
        if 0 < l {
            Program::set_uniform1i(
                sif_renderer_program_location!(prog, "u_Skinning"),
                1,
            )?;
            Program::set_uniform_matrix4fv(
                sif_renderer_program_location!(prog, "u_Bones[0]"),
                l as GLint,
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
