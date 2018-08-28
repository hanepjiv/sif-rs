// -*- mode:rust; coding:utf-8-unix; -*-

//! curve.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/07/30
//  @date 2018/08/27

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use sif_math::Float;
// ----------------------------------------------------------------------------
use super::{Interpolation, Keyframe};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct CurveData
#[derive(Debug, Clone)]
pub struct CurveData {}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum CurveType
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CurveType {
    /// Translate(data_idx)
    Translate(usize),
    /// RotateQuaternion(data_idx)
    RotateQuaternion(usize),
    /// Scale(data_idx)
    Scale(usize),
    /// BoneTranslate(name, data_idx)
    BoneTranslate(String, usize),
    /// BoneRotateQuaternion(name, data_idx)
    BoneRotateQuaternion(String, usize),
    /// BoneScale(name, data_idx)
    BoneScale(String, usize),
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Curve
#[derive(Debug, Clone)]
pub struct Curve<V: Float> {
    /// type_
    type_: CurveType,
    /// keyframes
    keyframes: Vec<Keyframe<V>>,
}
// ============================================================================
impl<V: Float> AsRef<CurveType> for Curve<V> {
    fn as_ref(&self) -> &CurveType {
        &self.type_
    }
}
// ============================================================================
impl<V: Float> AsRef<Vec<Keyframe<V>>> for Curve<V> {
    fn as_ref(&self) -> &Vec<Keyframe<V>> {
        &self.keyframes
    }
}
// ============================================================================
impl<V: Float> Curve<V> {
    // ========================================================================
    /// fn new
    pub fn new(type_: CurveType, keyframe: &[Keyframe<V>]) -> Self {
        let mut keyframes: Vec<Keyframe<V>> = keyframe.into();
        keyframes.sort_unstable_by_key(|x| x.get_key());
        keyframes.dedup_by_key(|x| x.get_key());
        keyframes.shrink_to_fit();
        Curve { type_, keyframes }
    }
    // ========================================================================
    /// fn end
    pub fn end(&self) -> isize {
        self.keyframes
            .last()
            .map(|x| x.get_key())
            .unwrap_or_default()
    }
    // ========================================================================
    /// fn value
    pub fn value(&self, key: V) -> V {
        match self
            .keyframes
            .binary_search_by_key(&{ key.to_isize().unwrap() }, |x| {
                x.get_key()
            }) {
            Ok(i) => self.keyframes[i].get_value(),
            Err(i) => {
                if i == 0 {
                    self.keyframes.first().unwrap().get_value()
                } else if i != self.keyframes.len() {
                    Curve::val(
                        &self.keyframes[i - 1],
                        &self.keyframes[i - 1],
                        &self.keyframes[i],
                        key,
                    )
                } else {
                    self.keyframes.last().unwrap().get_value()
                }
            }
        }
    }
    // ------------------------------------------------------------------------
    fn val(
        constant: &Keyframe<V>,
        left: &Keyframe<V>,
        right: &Keyframe<V>,
        key: V,
    ) -> V {
        match constant.get_interpolation() {
            Interpolation::Constant => constant.get_value(),
            Interpolation::Linear => Curve::linear(left, right, key),
            Interpolation::Bezier => Curve::bezier(left, right, key),
        }
    }
    // ------------------------------------------------------------------------
    fn linear(left: &Keyframe<V>, right: &Keyframe<V>, key: V) -> V {
        let l_key = left.get_key();
        let l_val = left.get_value();
        let mut r_val = right.get_value();
        r_val -= l_val;
        r_val /= V::from(right.get_key() - l_key).unwrap();
        r_val *= key - V::from(l_key).unwrap();
        r_val += l_val;
        r_val
    }
    // ------------------------------------------------------------------------
    fn bezier(left: &Keyframe<V>, right: &Keyframe<V>, key: V) -> V {
        let l_key = V::from(left.get_key()).unwrap();
        let r_key = V::from(right.get_key()).unwrap();
        let l_ctrl = left.as_ctrl();
        let r_ctrl = right.as_ctrl();
        let mut t = key;
        t -= l_key;
        t /= r_key - l_key;
        ::sif_math::bezier(
            left.get_value(),
            (l_ctrl.1).1,
            (r_ctrl.0).1,
            right.get_value(),
            t,
        )
    }
}
