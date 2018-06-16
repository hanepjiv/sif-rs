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
use lua::{ffi::lua_Integer, State as LuaState};
// ----------------------------------------------------------------------------
use super::{
    super::{super::Image, LBF},
    Error, Result,
};
// ----------------------------------------------------------------------------
use self::lua_state_ex::LuaStateEx;
// mod  =======================================================================
mod lua_state_ex;
mod lua_type;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
static CURRENT: lua_Integer = 4;
static AGE: lua_Integer = 0;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// get_current
fn get_current(state: &mut LuaState, idx: i32) -> Result<lua_Integer> {
    let current = state.idxtbl(idx, &"current")?;
    if current < (CURRENT - AGE) || CURRENT < current {
        Err(Error::Current(CURRENT, AGE, current))
    } else {
        Ok(current)
    }
}
// ============================================================================
/// from_str
pub fn from_str(path: impl AsRef<Path>, src: impl AsRef<str>) -> Result<LBF> {
    info!(
        "sif_graphics::lbf::loader::lua::from_str({:?}, ...)",
        path.as_ref()
    );

    let path_base = path.as_ref().parent().ok_or_else(|| {
        Error::OptNone("lbf::LBF::from_str: path.as_ref.parent".to_string())
    })?;

    let mut state = LuaState::new();

    let e = state.do_string(src.as_ref());
    if e.is_err() {
        Err(Error::Loader(format!("{:?}", e)))
    } else {
        let _ = get_current(&mut state, -1)?;
        Ok(LBF {
            images: state
                .idxtbl::<Vec<Image>>(-1, &"images")
                .unwrap_or_default()
                .into_iter()
                .map(|mut x| {
                    if let Image::File(ref mut im) = x {
                        let _ = im.set_path_base(path_base);
                    }
                    x
                })
                .collect(),
            textures: state.idxtbl(-1, &"textures").unwrap_or_default(),
            materials: state.idxtbl(-1, &"materials").unwrap_or_default(),
            meshes: state.idxtbl(-1, &"meshes").unwrap_or_default(),
            armatures: state.idxtbl(-1, &"armatures").unwrap_or_default(),
            models: state.idxtbl(-1, &"models").unwrap_or_default(),
            lights: state.idxtbl(-1, &"lights").unwrap_or_default(),
            cameras: state.idxtbl(-1, &"cameras").unwrap_or_default(),
            objects: state.idxtbl(-1, &"objects").unwrap_or_default(),
        })
    }
}
