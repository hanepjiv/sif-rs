// -*- mode:rust; coding:utf-8-unix; -*-

//! material.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/18
//  @date 2018/06/16

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::{cell::RefCell, result::Result as StdResult};
// ----------------------------------------------------------------------------
use gl::types::*;
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_manager::{ManagedValue, Manager};
use sif_renderer::{Program, Texture as RendererTexture};
// ----------------------------------------------------------------------------
use super::{ColorIntensity, Error, Result, Texture};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
pub(crate) const MATERIAL_MAX_TEXTURE: usize = 4;
pub(crate) const MATERIAL_TEXTURE_FLAGS: [&str; MATERIAL_MAX_TEXTURE] = [
    "u_Material.is_map_diffuse",
    "u_Material.is_map_specular",
    "u_Material.is_map_normal",
    "u_Material.is_map_emissive",
];
pub(crate) const MATERIAL_TEXTURE_NAMES: [&str; MATERIAL_MAX_TEXTURE] = [
    "u_Texture_Diffuse",
    "u_Texture_Speculer",
    "u_Texture_Normal",
    "u_Texture_Emissive",
];
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// Flags
bitflags! { pub struct Flags: u32 {
    const ANISOTROPIC   = 0b0000_0000_0000_0000_0000_0000_0000_0001u32;
    const BUMP          = 0b0000_0000_0000_0000_0000_0000_0000_0010u32;
    const PARALLAX      = 0b0000_0000_0000_0000_0000_0000_0000_0100u32;

    const DO_NOT_USE    = 0b1000_0000_0000_0000_0000_0000_0000_0000u32;
} }
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Parallax
#[derive(Debug, Clone, Copy)]
pub struct Parallax {
    /// height
    pub height: GLfloat,
    /// shadow_exponent
    pub shadow_exponent: GLfloat,
    /// loop
    pub loop_: GLint,
    /// shadow_loop
    pub shadow_loop: GLint,
}
// ============================================================================
impl Default for Parallax {
    fn default() -> Self {
        Parallax {
            height: 0.025,
            shadow_exponent: 2.0,
            loop_: 32,
            shadow_loop: 8,
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// type Textures
type Textures =
    Option<StdResult<Vec<Option<ManagedValue<Texture>>>, Vec<Option<Uuid>>>>;
// ============================================================================
/// struct Material
#[derive(Debug, Clone)]
pub struct Material {
    /// uuid
    uuid: Uuid,
    /// name
    name: String,
    // ------------------------------------------------------------------------
    /// textures
    pub textures: Textures,
    /// parallax
    pub parallax: Parallax,
    /// diffuse
    pub diffuse: ColorIntensity,
    /// specular
    pub specular: ColorIntensity,
    /// emissive
    pub emissive: ColorIntensity,
    /// shininess
    pub shininess: GLfloat,
    /// alpha [0.0 - 1.0]
    pub alpha: GLfloat,
    /// flags
    pub flags: Flags,
}
// ============================================================================
impl AsRef<Uuid> for Material {
    fn as_ref(&self) -> &Uuid {
        &self.uuid
    }
}
// ============================================================================
impl AsRef<String> for Material {
    fn as_ref(&self) -> &String {
        &self.name
    }
}
// ============================================================================
impl Material {
    // ========================================================================
    /// new
    pub fn new(uuid: Uuid, name: impl Into<String>) -> Self {
        Material {
            uuid,
            name: name.into(),
            // ----------------------------------------------------------------
            textures: None,
            parallax: Parallax::default(),
            diffuse: ColorIntensity::default(),
            specular: ColorIntensity::default(),
            emissive: ColorIntensity::default(),
            shininess: 4.0,
            alpha: 1.0,
            // ----------------------------------------------------------------
            flags: Flags::BUMP | Flags::PARALLAX,
        }
    }
    // ========================================================================
    /// prepare
    pub fn prepare(&mut self, textures: &Manager<Texture>) -> Result<()> {
        let mut v = Vec::new();
        if let Some(Err(ref uuids)) = self.textures {
            for i in uuids {
                if let Some(ref uuid) = *i {
                    if let Some(texture) = textures.get(uuid) {
                        v.push(Some(texture.clone()));
                    } else {
                        return Err(Error::ManagedNotFound(*uuid));
                    }
                } else {
                    v.push(None);
                }
            }
        }
        if !v.is_empty() {
            self.textures = Some(Ok(v));
        }
        Ok(())
    }
    // ========================================================================
    /// duplicate
    pub fn duplicate(&self, uuid: Uuid, name: impl Into<String>) -> Self {
        Material {
            uuid,
            name: name.into(),
            ..self.clone()
        }
    }
    // ========================================================================
    /// set_material
    pub fn set_material(&self, prog: &Program) -> Result<&Self> {
        if let Some(Ok(ref textures)) = self.textures {
            for i in 0..MATERIAL_TEXTURE_NAMES.len() {
                if let Some(ref managed) = textures[i] {
                    let tex = managed.as_ref().borrow();
                    let siftex: &RefCell<RendererTexture> = tex.as_ref();
                    Program::set_uniform1i(
                        sif_renderer_program_location!(
                            prog,
                            MATERIAL_TEXTURE_FLAGS[i]
                        ),
                        1,
                    )?;
                    Program::set_texture(
                        sif_renderer_program_location!(
                            prog,
                            MATERIAL_TEXTURE_NAMES[i]
                        ),
                        i as GLint,
                        &*siftex.borrow(),
                    )?;
                } else {
                    Program::set_uniform1i(
                        sif_renderer_program_location!(
                            prog,
                            MATERIAL_TEXTURE_FLAGS[i]
                        ),
                        0,
                    )?;
                }
            }
        } else {
            for flag in MATERIAL_TEXTURE_FLAGS
                .iter()
                .take(MATERIAL_TEXTURE_NAMES.len())
            {
                Program::set_uniform1i(
                    sif_renderer_program_location!(prog, *flag),
                    0,
                )?;
            }
        }
        Program::set_uniform1f(
            sif_renderer_program_location!(prog, "u_Material.parallax.height"),
            self.parallax.height,
        )?;
        Program::set_uniform1i(
            sif_renderer_program_location!(prog, "u_Material.parallax.loop"),
            self.parallax.loop_,
        )?;
        Program::set_uniform1i(
            sif_renderer_program_location!(
                prog,
                "u_Material.parallax.shadow_loop"
            ),
            self.parallax.shadow_loop,
        )?;
        Program::set_uniform1f(
            sif_renderer_program_location!(
                prog,
                "u_Material.parallax.shadow_exponent"
            ),
            self.parallax.shadow_exponent,
        )?;
        Program::set_uniform3fv(
            sif_renderer_program_location!(prog, "u_Material.diffuse"),
            1,
            (self.diffuse.color * self.diffuse.intensity).as_ptr(),
        )?;
        Program::set_uniform3fv(
            sif_renderer_program_location!(prog, "u_Material.specular"),
            1,
            (self.specular.color * self.specular.intensity).as_ptr(),
        )?;
        Program::set_uniform1f(
            sif_renderer_program_location!(prog, "u_Material.shininess"),
            self.shininess,
        )?;
        Program::set_uniform3fv(
            sif_renderer_program_location!(prog, "u_Material.emissive"),
            1,
            (self.emissive.color * self.emissive.intensity).as_ptr(),
        )?;
        Program::set_uniform1f(
            sif_renderer_program_location!(prog, "u_Material.alpha"),
            self.alpha,
        )?;
        Program::set_uniform1i(
            sif_renderer_program_location!(prog, "u_Material.is_anisotropic"),
            if self.flags.contains(Flags::ANISOTROPIC) {
                1
            } else {
                0
            },
        )?;
        Program::set_uniform1i(
            sif_renderer_program_location!(prog, "u_Material.is_bump"),
            if self.flags.contains(Flags::BUMP) {
                1
            } else {
                0
            },
        )?;
        Program::set_uniform1i(
            sif_renderer_program_location!(prog, "u_Material.is_parallax"),
            if self.flags.contains(Flags::PARALLAX) {
                1
            } else {
                0
            },
        )?;
        Ok(self)
    }
    // ------------------------------------------------------------------------
    /// set_material_silhouette
    pub fn set_material_silhouette(&self, prog: &Program) -> Result<&Self> {
        if let Some(Ok(ref textures)) = self.textures {
            if let Some(ref managed) = textures[0] {
                let tex = managed.as_ref().borrow();
                let siftex: &RefCell<RendererTexture> = tex.as_ref();
                Program::set_uniform1i(
                    sif_renderer_program_location!(
                        prog,
                        MATERIAL_TEXTURE_FLAGS[0]
                    ),
                    1,
                )?;
                Program::set_texture(
                    sif_renderer_program_location!(
                        prog,
                        MATERIAL_TEXTURE_NAMES[0]
                    ),
                    0,
                    &*siftex.borrow(),
                )?;
            } else {
                Program::set_uniform1i(
                    sif_renderer_program_location!(
                        prog,
                        MATERIAL_TEXTURE_FLAGS[0]
                    ),
                    0,
                )?;
            }
        } else {
            Program::set_uniform1i(
                sif_renderer_program_location!(
                    prog,
                    MATERIAL_TEXTURE_FLAGS[0]
                ),
                0,
            )?;
        }
        Program::set_uniform1f(
            sif_renderer_program_location!(prog, "u_Material.alpha"),
            self.alpha,
        )?;
        Ok(self)
    }
}
