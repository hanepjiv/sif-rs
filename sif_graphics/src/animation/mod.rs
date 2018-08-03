// -*- mode:rust; coding:utf-8-unix; -*-

//! mod.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/07/30
//  @date 2018/08/09

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_math::Number;
// ----------------------------------------------------------------------------
use super::{Error, Object, ObjectData, Result};
// ----------------------------------------------------------------------------
pub use self::{
    curve::{Curve, CurveType},
    driver::{Driver, Flags as DriverFrags},
    interpolation::Interpolation,
    keyframe::Keyframe,
};
// mod  =======================================================================
mod curve;
mod driver;
mod interpolation;
mod keyframe;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Animation
#[derive(Debug, Clone)]
pub struct Animation<V: Number> {
    /// uuid
    uuid: Uuid,
    /// name
    name: String,
    /// fps (frame / sec)
    fps: V,
    /// end (frame)
    end: isize,
    /// curves
    curves: Vec<Curve<V>>,
}
// ============================================================================
impl<V: Number> AsRef<Uuid> for Animation<V> {
    fn as_ref(&self) -> &Uuid {
        &self.uuid
    }
}
// ============================================================================
impl<V: Number> AsRef<String> for Animation<V> {
    fn as_ref(&self) -> &String {
        &self.name
    }
}
// ============================================================================
impl<V: Number> AsRef<Vec<Curve<V>>> for Animation<V> {
    fn as_ref(&self) -> &Vec<Curve<V>> {
        &self.curves
    }
}
// ============================================================================
impl<V: Number> Animation<V> {
    // ========================================================================
    /// fn new
    pub fn new(
        uuid: Uuid,
        name: impl Into<String>,
        fps: V,
        curves: &[Curve<V>],
    ) -> Self {
        let end = curves.iter().fold(0isize, |end, x| {
            let xend = x.end();
            if end < xend {
                xend
            } else {
                end
            }
        });
        Animation {
            uuid,
            name: name.into(),
            fps,
            end,
            curves: curves.into(),
        }
    }
    // ========================================================================
    /// fn fps
    pub fn fps(&self) -> V {
        self.fps
    }
    // ========================================================================
    /// fn duration (millisec)
    pub fn duration(&self) -> isize {
        (V::from(1000.0).unwrap() / self.fps * V::from(self.end).unwrap())
            .to_isize()
            .unwrap()
    }
}
