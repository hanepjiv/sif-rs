// -*- mode:rust; coding:utf-8-unix; -*-

//! model.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/19
//  @date 2018/08/11

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use gl::types::*;
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_manager::ManagedValue;
use sif_renderer::Program;
use sif_three::Armature;
// ----------------------------------------------------------------------------
use super::{Material, Mesh, Result};
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
    pub meshes: Vec<ManagedValue<Mesh>>,
    /// materials
    pub materials: Vec<ManagedValue<Material>>,
    /// armature
    pub armature: Option<ManagedValue<Armature<GLfloat>>>,
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
        &self.meshes
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Vec<ManagedValue<Mesh>>> for Model {
    fn as_mut(&mut self) -> &mut Vec<ManagedValue<Mesh>> {
        &mut self.meshes
    }
}
// ============================================================================
impl AsRef<Vec<ManagedValue<Material>>> for Model {
    fn as_ref(&self) -> &Vec<ManagedValue<Material>> {
        &self.materials
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Vec<ManagedValue<Material>>> for Model {
    fn as_mut(&mut self) -> &mut Vec<ManagedValue<Material>> {
        &mut self.materials
    }
}
// ============================================================================
impl AsRef<Option<ManagedValue<Armature<GLfloat>>>> for Model {
    fn as_ref(&self) -> &Option<ManagedValue<Armature<GLfloat>>> {
        &self.armature
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
            meshes: Vec::default(),
            materials: Vec::default(),
            armature: None,
        }
    }
    // ========================================================================
    /// build
    pub fn build(
        uuid: Uuid,
        name: impl Into<String>,
        meshes: Vec<ManagedValue<Mesh>>,
        materials: Vec<ManagedValue<Material>>,
        armature: Option<ManagedValue<Armature<GLfloat>>>,
    ) -> Self {
        Model {
            uuid,
            name: name.into(),
            meshes,
            materials,
            armature,
        }
    }
    // ========================================================================
    /// armature_len
    pub fn armature_len(&self) -> usize {
        if let Some(ref armature) = self.armature {
            armature.as_ref().borrow().len()
        } else {
            0
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
            &[ManagedValue<Material>],
        ) -> Result<()>,
    ) -> Result<()> {
        for managed in &self.meshes {
            let mut mesh = managed.as_ref().borrow_mut();
            func(
                &mut *mesh,
                prog,
                AsRef::<Vec<ManagedValue<Material>>>::as_ref(self),
            )?;
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
