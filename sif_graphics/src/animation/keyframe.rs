// -*- mode:rust; coding:utf-8-unix; -*-

//! keyframe.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/07/30
//  @date 2018/08/05

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use sif_math::Number;
// ----------------------------------------------------------------------------
use super::Interpolation;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Keyframe
#[derive(Debug, Clone, Copy)]
pub struct Keyframe<V: Number> {
    /// key
    key: isize,
    /// value
    value: V,
    /// interpolation
    interpolation: Interpolation,
    /// ctrl
    ctrl: ((V, V), (V, V)),
}
// ============================================================================
impl<V: Number> Keyframe<V> {
    // ========================================================================
    /// fn new
    pub(crate) fn new(
        key: isize,
        value: V,
        interpolation: Interpolation,
        ctrl_l_key: V,
        ctrl_l_value: V,
        ctrl_r_key: V,
        ctrl_r_value: V,
    ) -> Self {
        Keyframe {
            key,
            value,
            interpolation,
            ctrl: ((ctrl_l_key, ctrl_l_value), (ctrl_r_key, ctrl_r_value)),
        }
    }
    // ========================================================================
    /// fn get_key
    pub fn get_key(&self) -> isize {
        self.key
    }
    // ------------------------------------------------------------------------
    /// fn get_value
    pub fn get_value(&self) -> V {
        self.value
    }
    // ------------------------------------------------------------------------
    /// fn get_interpolation
    pub fn get_interpolation(&self) -> Interpolation {
        self.interpolation
    }
    // ------------------------------------------------------------------------
    /// fn as_ctrl
    pub fn as_ctrl(&self) -> &((V, V), (V, V)) {
        &self.ctrl
    }
}
