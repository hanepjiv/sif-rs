// -*- mode:rust; coding:utf-8-unix; -*-

//! object.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/06/13
//  @date 2018/08/01

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::marker::PhantomData;
// ----------------------------------------------------------------------------
use gl::types::*;
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_manager::{ManagedValue, Manager};
use sif_three::{Graph, Node, NodeHolder, Pose, TraRotSca};
// ----------------------------------------------------------------------------
use super::{
    Error, GraphicsCamera, GraphicsLight, GraphicsModel, GraphicsObject,
    GraphicsResult, GraphicsScene, IntoGraphics, ObjectData,
};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Object
#[derive(Debug, Clone)]
pub struct Object<'a> {
    /// uuid
    pub uuid: Uuid,
    /// name
    pub name: String,
    /// parent
    pub parent: Option<Uuid>,
    /// data_type
    pub data_type: String,
    /// data_uuid
    pub data_uuid: Option<Uuid>,
    /// trarotsca
    pub trarotsca: TraRotSca<GLfloat>,
    /// phantom
    phantom: PhantomData<&'a ()>,
}
// ============================================================================
impl<'a> Object<'a> {
    // ========================================================================
    /// new
    pub fn new(
        uuid: Uuid,
        name: impl Into<String>,
        parent: Option<Uuid>,
        data_type: impl Into<String>,
        data_uuid: Option<Uuid>,
        trarotsca: TraRotSca<GLfloat>,
    ) -> Self {
        Object {
            uuid,
            name: name.into(),
            parent,
            data_type: data_type.into(),
            data_uuid,
            trarotsca,
            phantom: PhantomData::default(),
        }
    }
}
// ============================================================================
impl<'a> AsRef<Uuid> for Object<'a> {
    fn as_ref(&self) -> &Uuid {
        &self.uuid
    }
}
// ============================================================================
impl<'a> AsRef<String> for Object<'a> {
    fn as_ref(&self) -> &String {
        &self.name
    }
}
// ============================================================================
impl<'a> IntoGraphics for Object<'a> {
    type Target = GraphicsObject;
    type Param = (
        &'a mut Graph<GLfloat>,
        &'a Manager<GraphicsModel>,
        &'a Manager<GraphicsLight>,
        &'a Manager<GraphicsCamera>,
    );
    // ========================================================================
    fn into_graphics(
        self,
        scene: &GraphicsScene,
        (graph, models, lights, cameras): Self::Param,
    ) -> GraphicsResult<Self::Target> {
        if let Some(mut obj) = match self.data_type.as_str() {
            "EMPTY" | "ARMATURE" => Some(GraphicsObject::new(
                *AsRef::<Uuid>::as_ref(&self),
                AsRef::<String>::as_ref(&self).as_str(),
                ObjectData::Empty,
            )),
            "MODEL" => {
                let uuid = &self.data_uuid.unwrap();
                {
                    if let Some(x) = models.get(uuid) {
                        Some(x)
                    } else {
                        AsRef::<Manager<GraphicsModel>>::as_ref(scene)
                            .get(uuid)
                    }
                }.map(|m| {
                    let armature_len = (*m.as_ref().borrow()).armature_len();
                    let pose = if 0 < armature_len {
                        Some(Pose::<GLfloat>::new(armature_len))
                    } else {
                        None
                    };
                    GraphicsObject::new(
                        *AsRef::<Uuid>::as_ref(&self),
                        AsRef::<String>::as_ref(&self).as_str(),
                        ObjectData::Model(m.clone(), pose),
                    )
                })
            }
            "LIGHT" => {
                let uuid = &self.data_uuid.unwrap();
                {
                    if let Some(x) = lights.get(uuid) {
                        Some(x)
                    } else {
                        AsRef::<Manager<GraphicsLight>>::as_ref(scene)
                            .get(uuid)
                    }
                }.map(|m| {
                    GraphicsObject::new(
                        *AsRef::<Uuid>::as_ref(&self),
                        AsRef::<String>::as_ref(&self).as_str(),
                        ObjectData::Light(m.clone()),
                    )
                })
            }
            "CAMERA" => {
                let uuid = &self.data_uuid.unwrap();
                {
                    if let Some(x) = cameras.get(uuid) {
                        Some(x)
                    } else {
                        AsRef::<Manager<GraphicsCamera>>::as_ref(scene)
                            .get(uuid)
                    }
                }.map(|m| {
                    GraphicsObject::new(
                        *AsRef::<Uuid>::as_ref(&self),
                        AsRef::<String>::as_ref(&self).as_str(),
                        ObjectData::Camera(m.clone()),
                    )
                })
            }
            _ => None,
        } {
            let parent: Option<ManagedValue<Node<GLfloat>>> = if let Some(p) =
                self.parent
            {
                Some(graph.get(&p).ok_or_else(|| {
                    Error::OptNone(
                        "lbf::Object: into_graphics: graph.get".to_string(),
                    )
                })?)
            } else {
                None
            };
            let _ =
                graph.emplace(AsRef::<Uuid>::as_ref(&self).clone(), parent)?;
            let node = graph.get(self.as_ref()).ok_or_else(|| {
                Error::OptNone(
                    "lbf::Object: into_graphics: graph.insert".to_string(),
                )
            })?;
            {
                let mut m = node.as_ref().borrow_mut();
                let trs = AsMut::<TraRotSca<GLfloat>>::as_mut(&mut *m);
                trs.translation = self.trarotsca.translation;
                trs.rotation = self.trarotsca.rotation;
                trs.scaling = self.trarotsca.scaling;
            }
            obj.set_node(Some(node));
            Ok(obj)
        } else {
            Err(Error::Object(
                format!(
                    "lbf::Object: into_graphics: {}({})",
                    self.name, self.data_type
                ).to_string(),
            ).into())
        }
    }
}
