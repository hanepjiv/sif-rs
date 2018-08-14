// -*- mode:rust; coding:utf-8-unix; -*-

//! model.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/08/01
//  @date 2018/08/11

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::marker::PhantomData;
// ----------------------------------------------------------------------------
use gl::types::*;
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_manager::Manager;
// ----------------------------------------------------------------------------
use super::{
    Armature, Error, GraphicsMaterial, GraphicsMesh, GraphicsModel,
    GraphicsResult, GraphicsScene, IntoGraphics, Result,
};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Model
#[derive(Debug, Clone)]
pub struct Model<'a, 'b> {
    /// uuid
    uuid: Uuid,
    /// name
    name: String,
    /// meshes
    meshes: Vec<Uuid>,
    /// materials
    materials: Vec<Uuid>,
    /// armature
    armature: Option<Uuid>,
    /// phantom
    phantom: PhantomData<&'a ()>,
    /// phantom
    phantom1: PhantomData<&'b ()>,
}
// ============================================================================
impl<'a, 'b> AsRef<Uuid> for Model<'a, 'b> {
    fn as_ref(&self) -> &Uuid {
        &self.uuid
    }
}
// ============================================================================
impl<'a, 'b> AsRef<String> for Model<'a, 'b> {
    fn as_ref(&self) -> &String {
        &self.name
    }
}
impl<'a, 'b> Model<'a, 'b> {
    // ========================================================================
    /// fn new
    pub(crate) fn new(
        uuid: Uuid,
        name: impl Into<String>,
        meshes: Vec<Uuid>,
        materials: Vec<Uuid>,
        armature: Option<Uuid>,
    ) -> Result<Self> {
        Ok(Self {
            uuid,
            name: name.into(),
            meshes,
            materials,
            armature,
            phantom: PhantomData::default(),
            phantom1: PhantomData::default(),
        })
    }
}
// ============================================================================
impl<'a, 'b> IntoGraphics for Model<'a, 'b> {
    type Target = GraphicsModel;
    type Param = (
        &'a GraphicsScene,
        &'b Manager<GraphicsMesh>,
        &'b Manager<GraphicsMaterial>,
        &'b Manager<Armature<GLfloat>>,
    );
    // ========================================================================
    fn into_graphics(
        self,
        (scene, meshes, materials, armatures): Self::Param,
    ) -> GraphicsResult<(Self::Target, Self::Param)> {
        let mut new_meshes = Vec::new();
        for uuid in self.meshes {
            if let Some(mesh) = {
                if let Some(x) = meshes.get(&uuid) {
                    Some(x)
                } else {
                    AsRef::<Manager<GraphicsMesh>>::as_ref(scene).get(&uuid)
                }
            } {
                new_meshes.push(mesh.clone());
            } else {
                return Err(Error::Model(
                    format!(
                        "lbf::Model: into_graphics: mesh not found {}",
                        uuid
                    ).to_string(),
                ).into());
            }
        }

        let mut new_materials = Vec::new();
        for uuid in self.materials {
            if let Some(material) = {
                if let Some(x) = materials.get(&uuid) {
                    Some(x)
                } else {
                    AsRef::<Manager<GraphicsMaterial>>::as_ref(scene)
                        .get(&uuid)
                }
            } {
                new_materials.push(material.clone());
            } else {
                return Err(Error::Model(
                    format!(
                        "lbf::Model: into_graphics: material not found {}",
                        uuid
                    ).to_string(),
                ).into());
            }
        }

        let new_armature = if let Some(uuid) = self.armature {
            if let Some(armature) = {
                if let Some(x) = armatures.get(&uuid) {
                    Some(x)
                } else {
                    AsRef::<Manager<Armature<GLfloat>>>::as_ref(scene)
                        .get(&uuid)
                }
            } {
                Some(armature.clone())
            } else {
                return Err(Error::Model(
                    format!(
                        "lbf::Model: into_graphics: armature not found {}",
                        uuid
                    ).to_string(),
                ).into());
            }
        } else {
            None
        };

        Ok((
            GraphicsModel::build(
                self.uuid,
                self.name,
                new_meshes,
                new_materials,
                new_armature,
            ),
            (scene, meshes, materials, armatures),
        ))
    }
}
