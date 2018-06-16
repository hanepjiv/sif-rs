// -*- mode:rust; coding:utf-8-unix; -*-

//! mod.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/06/13
//  @date 2018/06/16

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::path::Path;
// ----------------------------------------------------------------------------
use gl::types::*;
use rlua::{Integer, Lua, Table};
// ----------------------------------------------------------------------------
use sif_three::Armature;
// ----------------------------------------------------------------------------
use super::{
    super::{
        super::{Camera, Image, Material, Model, Texture},
        LBFLight, LBFMesh, LBFObject, LBF,
    },
    Error, Result,
};
// ----------------------------------------------------------------------------
use self::lua_type::LuaType;
// mod  =======================================================================
mod lua_type;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
static CURRENT: Integer = 4;
static AGE: Integer = 0;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// get_current
fn get_current(tbl: &Table) -> Result<Integer> {
    let current = tbl.get::<_, Integer>("current")?;
    if current < (CURRENT - AGE) || CURRENT < current {
        Err(Error::Current(CURRENT, AGE, current))
    } else {
        Ok(current)
    }
}
// ============================================================================
/// from_str
pub(crate) fn from_str(
    path: impl AsRef<Path>,
    src: impl AsRef<str>,
) -> Result<LBF> {
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

    let lua = Lua::new();

    let tbl = lua.eval::<Table>(src.as_ref(), path.as_ref().to_str())?;

    let _ = get_current(&tbl)?;

    let images = Vec::<Image>::from_lua(tbl.get("images")?)
        .unwrap_or_default()
        .into_iter()
        .map(|mut x| {
            if let Image::File(ref mut im) = x {
                let _ = im.set_path_base(path_base);
            }
            x
        })
        .collect();

    let textures =
        Vec::<Texture>::from_lua(tbl.get("textures")?).unwrap_or_default();

    let materials =
        Vec::<Material>::from_lua(tbl.get("materials")?).unwrap_or_default();

    let meshes =
        Vec::<LBFMesh>::from_lua(tbl.get("meshes")?).unwrap_or_default();

    let armatures = Vec::<Armature<GLfloat>>::from_lua(tbl.get("armatures")?)
        .unwrap_or_default();

    let models =
        Vec::<Model>::from_lua(tbl.get("models")?).unwrap_or_default();

    let lights =
        Vec::<LBFLight>::from_lua(tbl.get("lights")?).unwrap_or_default();

    let cameras =
        Vec::<Camera>::from_lua(tbl.get("cameras")?).unwrap_or_default();

    let objects =
        Vec::<LBFObject>::from_lua(tbl.get("objects")?).unwrap_or_default();

    Ok(LBF {
        images,
        textures,
        materials,
        meshes,
        armatures,
        models,
        lights,
        cameras,
        objects,
    })
}
