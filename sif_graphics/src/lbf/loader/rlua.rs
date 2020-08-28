// -*- mode:rust; coding:utf-8-unix; -*-

//! mod.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/06/13
//  @date 2020/03/19

// ////////////////////////////////////////////////////////////////////////////
// mod  =======================================================================
mod lua_type;
// use  =======================================================================
use std::path::Path;
// ----------------------------------------------------------------------------
use gl::types::*;
use log::info;
use rlua::{Integer, Lua, Table};
// ----------------------------------------------------------------------------
use sif_three::Armature;
// ----------------------------------------------------------------------------
use super::{
    super::{
        Animation, Camera, Image, LBFAnimationDriver, LBFLight, LBFMaterial,
        LBFMesh, LBFModel, LBFObject, LBFScene, LBFTexture,
    },
    Error, Result,
};
// ----------------------------------------------------------------------------
use self::lua_type::LuaType;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
const CURRENT: Integer = 4;
const AGE: Integer = 0;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// get_current
fn get_current(tbl: &Table<'_>) -> Result<Integer> {
    let current = tbl.get::<_, Integer>("current")?;
    if current < (CURRENT - AGE) || CURRENT < current {
        Err(Error::Current(CURRENT, AGE, current))
    } else {
        Ok(current)
    }
}
// ============================================================================
/// from_str
pub(crate) fn from_str<'a, 'b>(
    path: impl AsRef<Path>,
    src: impl AsRef<str>,
) -> Result<LBFScene<'a, 'b>> {
    info!(
        "sif_graphics::lbf::loader::rlua::from_str({:?}, ...)",
        path.as_ref()
    );

    let path_base = path.as_ref().parent().ok_or_else(|| {
        Error::OptNone(format!(
            "lbf::LBF::from_str: path.as_ref.parent: \"{:?}\"",
            path.as_ref()
        ))
    })?;

    Lua::new().context(|ctx| {
        let tbl = ctx.load(src.as_ref()).eval()?;

        let _ = get_current(&tbl)?;

        let images = Vec::<Image>::from_lua(tbl.get("images")?)?
            .into_iter()
            .map(|mut x| {
                if let Image::File(ref mut im) = x {
                    let _ = im.set_path_base(path_base);
                }
                x
            })
            .collect();
        let textures =
            Vec::<LBFTexture<'_, '_>>::from_lua(tbl.get("textures")?)?;
        let materials =
            Vec::<LBFMaterial<'_, '_>>::from_lua(tbl.get("materials")?)?;
        let meshes = Vec::<LBFMesh>::from_lua(tbl.get("meshes")?)?;
        let armatures =
            Vec::<Armature<GLfloat>>::from_lua(tbl.get("armatures")?)?;
        let models = Vec::<LBFModel<'_, '_>>::from_lua(tbl.get("models")?)?;
        let lights = Vec::<LBFLight>::from_lua(tbl.get("lights")?)?;
        let cameras = Vec::<Camera<GLfloat>>::from_lua(tbl.get("cameras")?)?;
        let animations =
            Vec::<Animation<GLfloat>>::from_lua(tbl.get("animations")?)?;
        let objects = Vec::<LBFObject<'_, '_>>::from_lua(tbl.get("objects")?)?;
        let animation_drivers =
            Vec::<LBFAnimationDriver<'_, '_, GLfloat, GLint>>::from_lua(
                tbl.get("animation_drivers")?,
            )?;

        Ok(LBFScene {
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
        })
    })
}
