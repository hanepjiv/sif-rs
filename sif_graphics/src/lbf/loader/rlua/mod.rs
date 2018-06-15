// -*- mode:rust; coding:utf-8-unix; -*-

//! mod.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/06/13
//  @date 2018/06/14

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::{collections::BTreeMap, path::Path};
// ----------------------------------------------------------------------------
use gl::types::*;
use rlua::{Integer, Lua, Table};
use uuid::Uuid;
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
static CURRENT: Integer = 3;
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
    info!("sif_graphics::lbf::LBF::from_str({:?}, ...)", path.as_ref());

    let path_base = path.as_ref().parent().ok_or_else(|| {
        Error::OptNone(format!(
            "lbf::LBF::from_str: path.as_ref.parent: \"{:?}\"",
            path.as_ref()
        ))
    })?;

    let lua = Lua::new();

    let tbl = lua.eval::<Table>(src.as_ref(), path.as_ref().to_str())?;

    let _ = get_current(&tbl)?;

    let mut images = BTreeMap::<Uuid, Image>::from_lua(tbl.get("images")?)
        .unwrap_or_default();
    for v in &mut images.values_mut() {
        if let Image::File(ref mut im) = *v {
            let _ = im.set_path_base(path_base);
        }
    }

    let textures = BTreeMap::<Uuid, Texture>::from_lua(tbl.get("textures")?)
        .unwrap_or_default();

    let materials = BTreeMap::<Uuid, Material>::from_lua(
        tbl.get("materials")?,
    ).unwrap_or_default();

    let meshes = BTreeMap::<Uuid, LBFMesh>::from_lua(tbl.get("meshes")?)
        .unwrap_or_default();

    let armatures = BTreeMap::<Uuid, Armature<GLfloat>>::from_lua(
        tbl.get("armatures")?,
    ).unwrap_or_default();

    let models = BTreeMap::<Uuid, Model>::from_lua(tbl.get("models")?)
        .unwrap_or_default();

    let lights = BTreeMap::<Uuid, LBFLight>::from_lua(tbl.get("lights")?)
        .unwrap_or_default();

    let cameras = BTreeMap::<Uuid, Camera>::from_lua(tbl.get("cameras")?)
        .unwrap_or_default();

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
