// -*- mode:rust; coding:utf-8-unix; -*-

//! mod.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/07/30
//  @date 2018/07/31

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use gl::types::*;
use uuid::Uuid;
// ----------------------------------------------------------------------------
pub use self::{
    curve::Curve, interpolation::Interpolation, keyframe::Keyframe,
};
// mod  =======================================================================
mod curve;
mod interpolation;
mod keyframe;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Animation
#[derive(Debug, Clone)]
pub struct Animation {
    /// uuid
    uuid: Uuid,
    /// name
    name: String,
    /// fps
    fps: GLfloat,
    /// curves
    curves: Vec<Curve>,
}
// ============================================================================
impl Animation {
    // ========================================================================
    /// fn new
    pub(crate) fn new(uuid: Uuid, name: impl Into<String>) -> Self {
        Animation {
            uuid,
            name: name.into(),
            fps: 60.0,
            curves: Vec::default(),
        }
    }
}
