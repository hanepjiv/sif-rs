// -*- mode:rust; coding:utf-8-unix; -*-

//! curve.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/07/30
//  @date 2018/07/31

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::result::Result as StdResult;
// ----------------------------------------------------------------------------
use super::Interpolation;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum CurveType
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CurveType {
    /// LocationX
    LOCATIONX,
    /// LocationY
    LOCATIONY,
    /// LOCationZ
    LOCATIONZ,
    /// RotationQuaternionW
    RotationQuaternionW,
    /// RotationQuaternionX
    RotationQuaternionX,
    /// RotationQUATERNIONY
    RotationQuaternionY,
    /// RotationQuaternionZ
    RotationQuaternionZ,
    /// ScaleX
    ScaleX,
    /// ScaleY
    ScaleY,
    /// ScaleZ
    ScaleZ,
    /// BoneLocationX
    BoneLocationX,
    /// BoneLocationY
    BoneLocationY,
    /// BoneLocationZ
    BoneLocationZ,
    /// BoneRotationQuaternionW
    BoneRotationQuaternionW,
    /// BoneRotationQuaternionX
    BoneRotationQuaternionX,
    /// BoneRotationQuaternionY
    BoneRotationQuaternionY,
    /// BoneRotationQuaternionZ
    BoneRotationQuaternionZ,
    /// BoneScaleX
    BoneScaleX,
    /// BoneScaleY
    BoneScaleY,
    /// BoneScaleZ
    BoneScaleZ,
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// type CurveTarget
type CurveTarget = Option<StdResult<usize, String>>;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Curve
#[derive(Debug, Clone)]
pub struct Curve {
    /// type_
    type_: CurveType,
    /// target
    target: CurveTarget,
    /// extrapolation
    extrapolation: Interpolation,
}
// ============================================================================
impl Curve {
    // ========================================================================
    /// fn new
    pub(crate) fn new(
        type_: CurveType,
        target: CurveTarget,
        extrapolation: Interpolation,
    ) -> Self {
        Curve {
            type_,
            target,
            extrapolation,
        }
    }
}
