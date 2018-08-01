// -*- mode:rust; coding:utf-8-unix; -*-

//! material.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/08/01
//  @date 2018/08/01

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
    super::{ColorIntensity, Parallax},
    Error, GraphicsMaterial, GraphicsResult, GraphicsScene, GraphicsTexture,
    IntoGraphics, MaterialFlags as Flags,
};
// ////////////////////////////////////////////////////////////////////////////
/// struct Material
#[derive(Debug, Clone)]
pub(crate) struct Material<'a> {
    /// uuid
    uuid: Uuid,
    /// name
    name: String,
    /// textures
    textures: Vec<Option<Uuid>>,
    /// parallax
    parallax: Parallax,
    /// diffuse
    diffuse: ColorIntensity,
    /// specular
    specular: ColorIntensity,
    /// emissive
    emissive: ColorIntensity,
    /// shininess
    shininess: GLfloat,
    /// alpha [0.0 - 1.0]
    alpha: GLfloat,
    /// flags
    flags: Flags,
    /// phantom
    phantom: PhantomData<&'a ()>,
}
// ============================================================================
impl<'a> AsRef<Uuid> for Material<'a> {
    fn as_ref(&self) -> &Uuid {
        &self.uuid
    }
}
// ============================================================================
impl<'a> AsRef<String> for Material<'a> {
    fn as_ref(&self) -> &String {
        &self.name
    }
}
// ============================================================================
impl<'a> Material<'a> {
    /// fn new
    pub(crate) fn new(
        uuid: Uuid,
        name: impl Into<String>,
        textures: Vec<Option<Uuid>>,
        parallax: Parallax,
        diffuse: ColorIntensity,
        specular: ColorIntensity,
        emissive: ColorIntensity,
        shininess: GLfloat,
        alpha: GLfloat,
        flags: Flags,
    ) -> Self {
        Material {
            uuid,
            name: name.into(),
            textures,
            parallax,
            diffuse,
            specular,
            emissive,
            shininess,
            alpha,
            flags,
            phantom: PhantomData::default(),
        }
    }
}
// ============================================================================
impl<'a> IntoGraphics for Material<'a> {
    type Target = GraphicsMaterial;
    type Param = &'a Manager<GraphicsTexture>;
    // ========================================================================
    fn into_graphics(
        self,
        scene: &GraphicsScene,
        textures: Self::Param,
    ) -> GraphicsResult<Self::Target> {
        let mut new_textures = Vec::new();
        for i in self.textures {
            if let Some(ref uuid) = i {
                if let Some(texture) = {
                    if let Some(x) = textures.get(uuid) {
                        Some(x)
                    } else {
                        AsRef::<Manager<GraphicsTexture>>::as_ref(scene)
                            .get(uuid)
                    }
                } {
                    new_textures.push(Some(texture.clone()));
                } else {
                    return Err(Error::Material(
                        format!(
                            "lbf::Material: texture not found {:?}",
                            *uuid,
                        ).to_string(),
                    ).into());
                }
            } else {
                new_textures.push(None);
            }
        }
        Ok(GraphicsMaterial::build(
            self.uuid,
            self.name,
            new_textures,
            self.parallax,
            self.diffuse,
            self.specular,
            self.emissive,
            self.shininess,
            self.alpha,
            self.flags,
        ))
    }
}
