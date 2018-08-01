// -*- mode:rust; coding:utf-8-unix; -*-

//! material.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/18
//  @date 2018/08/01

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::cell::RefCell;
// ----------------------------------------------------------------------------
use gl::types::*;
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_manager::ManagedValue;
use sif_renderer::{Program, Texture as RendererTexture};
// ----------------------------------------------------------------------------
use super::{ColorIntensity, Result, Texture};
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
#[allow(missing_docs)]
bitflags! {
    #[allow(missing_docs)]
    pub struct Flags: u32 {
        #[allow(missing_docs)]
        const ANISOTROPIC   = 0b0000_0000_0000_0000_0000_0000_0000_0001u32;
        #[allow(missing_docs)]
        const BUMP          = 0b0000_0000_0000_0000_0000_0000_0000_0010u32;
        #[allow(missing_docs)]
        const PARALLAX              = 0b0000_0000_0000_0000_0000_0000_0000_0100u32;

        #[allow(missing_docs)]
        const DO_NOT_USE    = 0b1000_0000_0000_0000_0000_0000_0000_0000u32;
    }
}
// ============================================================================
impl Default for Flags {
    fn default() -> Self {
        Flags::BUMP | Flags::PARALLAX
    }
}
// ============================================================================
impl Flags {
    /// fn new
    pub fn new() -> Self {
        Self::default()
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Parallax
#[derive(Debug, Clone, Copy)]
pub struct Parallax {
    /// height
    pub height: GLfloat,
    /// shadow_exponent
    pub shadow_exponent: GLfloat,
    /// loop_
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
// ============================================================================
impl Parallax {
    // ========================================================================
    /// fn new
    pub(crate) fn new(
        height: GLfloat,
        shadow_exponent: GLfloat,
        loop_: GLint,
        shadow_loop: GLint,
    ) -> Self {
        Parallax {
            height,
            shadow_exponent,
            loop_,
            shadow_loop,
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Material
#[derive(Debug, Clone)]
pub struct Material {
    /// uuid
    uuid: Uuid,
    /// name
    name: String,
    /// textures
    pub textures: Vec<Option<ManagedValue<Texture>>>,
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
            textures: vec![None; 4],
            parallax: Parallax::default(),
            diffuse: ColorIntensity::default(),
            specular: ColorIntensity::default(),
            emissive: ColorIntensity::default(),
            shininess: 4.0,
            alpha: 1.0,
            flags: Flags::default(),
        }
    }
    // ------------------------------------------------------------------------
    /// build
    pub fn build(
        uuid: Uuid,
        name: impl Into<String>,
        textures: Vec<Option<ManagedValue<Texture>>>,
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
        }
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
    /// emit
    pub fn emit(&self, prog: &Program) -> Result<&Self> {
        {
            let loc_is_map = (0..MATERIAL_MAX_TEXTURE)
                .map(|i| {
                    sif_renderer_program_location!(
                        prog,
                        MATERIAL_TEXTURE_FLAGS[i]
                    )
                }).collect::<Vec<_>>();
            for (i, texture) in self.textures.iter().enumerate() {
                if let Some(ref managed) = texture {
                    let tex = managed.as_ref().borrow();
                    let siftex: &RefCell<RendererTexture> = tex.as_ref();
                    Program::set_texture(
                        sif_renderer_program_location!(
                            prog,
                            MATERIAL_TEXTURE_NAMES[i]
                        ),
                        i as GLint,
                        &*siftex.borrow(),
                    )?;
                    Program::set_uniform1i(loc_is_map[i], 1)?;
                } else {
                    Program::set_uniform1i(loc_is_map[i], 0)?;
                }
            }
            for i in self.textures.len()..MATERIAL_MAX_TEXTURE {
                Program::set_uniform1i(loc_is_map[i], 0)?;
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
    /// emit_silhouette
    pub fn emit_silhouette(&self, prog: &Program) -> Result<&Self> {
        let loc_is_map_diffuse =
            sif_renderer_program_location!(prog, MATERIAL_TEXTURE_FLAGS[0]);
        if !self.textures.is_empty() {
            if let Some(ref managed) = self.textures[0] {
                let tex = managed.as_ref().borrow();
                let siftex: &RefCell<RendererTexture> = tex.as_ref();
                Program::set_texture(
                    sif_renderer_program_location!(
                        prog,
                        MATERIAL_TEXTURE_NAMES[0]
                    ),
                    0,
                    &*siftex.borrow(),
                )?;
                Program::set_uniform1i(loc_is_map_diffuse, 1)?;
            } else {
                Program::set_uniform1i(loc_is_map_diffuse, 0)?;
            }
        } else {
            Program::set_uniform1i(loc_is_map_diffuse, 0)?;
        }
        Program::set_uniform1f(
            sif_renderer_program_location!(prog, "u_Material.alpha"),
            self.alpha,
        )?;
        Ok(self)
    }
}
