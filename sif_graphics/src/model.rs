// -*- mode:rust; coding:utf-8-unix; -*-

//! model.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/19
//  @date 2018/05/12

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::result::Result as StdResult;
// ----------------------------------------------------------------------------
use gl::types::*;
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_manager::{ManagedValue, Manager};
use sif_renderer::Program;
use sif_three::Armature;
// ----------------------------------------------------------------------------
use super::{Error, Material, Mesh, Result};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Model
#[derive(Debug, Clone)]
pub struct Model {
    /// uuid
    uuid: Uuid,
    /// name
    name: String,
    /// meshes
    pub meshes: Option<StdResult<Vec<ManagedValue<Mesh>>, Vec<Uuid>>>,
    /// materials
    pub materials: Option<StdResult<Vec<ManagedValue<Material>>, Vec<Uuid>>>,
    /// armature
    pub armature: Option<StdResult<ManagedValue<Armature<GLfloat>>, Uuid>>,
}
// ============================================================================
impl AsRef<Uuid> for Model {
    fn as_ref(&self) -> &Uuid {
        &self.uuid
    }
}
// ============================================================================
impl AsRef<String> for Model {
    fn as_ref(&self) -> &String {
        &self.name
    }
}
// ============================================================================
impl AsRef<Vec<ManagedValue<Mesh>>> for Model {
    fn as_ref(&self) -> &Vec<ManagedValue<Mesh>> {
        if let Some(ref result) = self.meshes {
            if let Ok(ref meshes) = *result {
                meshes
            } else {
                panic!("AsRef<Vec<ManagedValue<Mesh>>> for Model");
            }
        } else {
            panic!("AsRef<Vec<ManagedValue<Mesh>>> for Model");
        }
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Vec<ManagedValue<Mesh>>> for Model {
    fn as_mut(&mut self) -> &mut Vec<ManagedValue<Mesh>> {
        if let Some(ref mut result) = self.meshes {
            if let Ok(ref mut meshes) = *result {
                meshes
            } else {
                panic!("AsMut<Vec<ManagedValue<Mesh>>> for Model");
            }
        } else {
            panic!("AsMut<Vec<ManagedValue<Mesh>>> for Model: {:?}", self);
        }
    }
}
// ============================================================================
impl AsRef<Vec<ManagedValue<Material>>> for Model {
    fn as_ref(&self) -> &Vec<ManagedValue<Material>> {
        if let Some(ref result) = self.materials {
            if let Ok(ref materials) = *result {
                materials
            } else {
                panic!(
                    "AsRef<Vec<ManagedValue<Material>>> for Model: {:?}",
                    self
                );
            }
        } else {
            panic!("AsRef<Vec<ManagedValue<Material>>> for Model");
        }
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Vec<ManagedValue<Material>>> for Model {
    fn as_mut(&mut self) -> &mut Vec<ManagedValue<Material>> {
        if let Some(ref mut result) = self.materials {
            if let Ok(ref mut materials) = *result {
                materials
            } else {
                panic!(format!(
                    "AsMut<Vec<ManagedValue<Material>>> for Model: {:?}",
                    result
                ));
            }
        } else {
            panic!("AsMut<Vec<ManagedValue<Material>>> for Model");
        }
    }
}
// ============================================================================
impl AsRef<ManagedValue<Armature<GLfloat>>> for Model {
    fn as_ref(&self) -> &ManagedValue<Armature<GLfloat>> {
        if let Some(ref result) = self.armature {
            if let Ok(ref armature) = *result {
                armature
            } else {
                panic!(format!(
                    "AsRef<Armature<GLfloat>> for Model: {:?}",
                    result
                ));
            }
        } else {
            panic!("AsRef<Armature<GLfloat>> for Model");
        }
    }
}
// ----------------------------------------------------------------------------
impl AsMut<ManagedValue<Armature<GLfloat>>> for Model {
    fn as_mut(&mut self) -> &mut ManagedValue<Armature<GLfloat>> {
        if let Some(ref mut result) = self.armature {
            if let Ok(ref mut armature) = *result {
                armature
            } else {
                panic!(format!(
                    "AsMut<Armature<GLfloat>> for Model: {:?}",
                    result
                ));
            }
        } else {
            panic!("AsMut<Armature<GLfloat>> for Model");
        }
    }
}
// ============================================================================
impl Model {
    // ========================================================================
    /// new
    pub fn new(uuid: Uuid, name: impl Into<String>) -> Self {
        Model {
            uuid,
            name: name.into(),
            meshes: None,
            materials: None,
            armature: None,
        }
    }
    // ========================================================================
    /// prepare
    pub fn prepare(
        &mut self,
        meshes: &Manager<Mesh>,
        materials: &Manager<Material>,
        armatures: &Manager<Armature<GLfloat>>,
    ) -> Result<()> {
        {
            // meshes
            let mut v = Vec::new();
            if let Some(Err(ref uuids)) = self.meshes {
                for uuid in uuids {
                    if let Some(mesh) = meshes.get(uuid) {
                        v.push(mesh.clone());
                    } else {
                        return Err(Error::ManagedNotFound(*uuid));
                    }
                }
            }
            if v.is_empty() {
                self.meshes = None;
            } else {
                self.meshes = Some(Ok(v));
            }
        }
        {
            // materials
            let mut v = Vec::new();
            if let Some(Err(ref uuids)) = self.materials {
                for uuid in uuids {
                    if let Some(material) = materials.get(uuid) {
                        v.push(material.clone());
                    } else {
                        return Err(Error::ManagedNotFound(*uuid));
                    }
                }
            }
            if v.is_empty() {
                self.materials = None;
            } else {
                self.materials = Some(Ok(v));
            }
        }
        {
            // armature
            let mut v = None;
            if let Some(Err(ref uuid)) = self.armature {
                if let Some(armature) = armatures.get(uuid) {
                    v = Some(Ok((*armature).clone()));
                } else {
                    return Err(Error::ManagedNotFound(*uuid));
                }
            }
            self.armature = v;
        }
        Ok(())
    }
    // ========================================================================
    /// armature_len
    pub fn armature_len(&self) -> usize {
        if let Some(Ok(ref v)) = self.armature {
            let armature = v.as_ref().borrow();
            armature.len()
        } else {
            0
        }
    }
    // ========================================================================
    /// as_materials
    fn as_materials(&self) -> Option<&Vec<ManagedValue<Material>>> {
        if let Some(ref result) = self.materials {
            if let Ok(ref materials) = *result {
                Some(materials)
            } else {
                panic!(format!("Model::as_materials {:?}", result));
            }
        } else {
            None
        }
    }
    // ========================================================================
    /// draw_impl
    fn draw_impl(
        &mut self,
        prog: &Program,
        mut func: impl FnMut(
            &mut Mesh,
            &Program,
            Option<&Vec<ManagedValue<Material>>>,
        ) -> Result<()>,
    ) -> Result<()> {
        if let Some(Ok(ref v)) = self.meshes {
            for managed in v {
                let mut mesh = managed.as_ref().borrow_mut();
                func(&mut *mesh, prog, self.as_materials())?;
            }
        }
        Ok(())
    }
    // ------------------------------------------------------------------------
    /// draw
    pub fn draw(&mut self, prog: &Program) -> Result<()> {
        self.draw_impl(prog, Mesh::draw)
    }
    // ------------------------------------------------------------------------
    /// draw_silhouette
    pub fn draw_silhouette(&mut self, prog: &Program) -> Result<()> {
        self.draw_impl(prog, Mesh::draw_silhouette)
    }
}
