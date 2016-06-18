/* -*- mode:rust; coding:utf-8-unix; -*- */

//! quaternion.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/19
//  @date 2016/06/18

/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
use super::{ MathError, Number, Vector4, };
/* ========================================================================== */
use ::num::{ Float, };
/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
/// struct Quaternion
#[derive( Debug, Clone, Copy, PartialEq, Eq, )]
pub struct Quaternion< V: Number >(Vector4< V >);
/* ========================================================================== */
impl <V> Default for Quaternion<V>
    where V: Number {
    fn default() -> Self {
        Quaternion(Vector4::from([V::zero(), V::zero(), V::zero(), V::one()]))
    }
}
/* ========================================================================== */
impl <V> From< [V; 4] > for Quaternion<V>
    where V: Number {
    fn from(inner: [V; 4]) -> Self {
        Quaternion::from(Vector4::from(inner))
    }
}
/* ========================================================================== */
impl <V> From< Vector4< V > > for Quaternion<V>
    where V: Number {
    fn from(inner: Vector4< V >) -> Self {
        let mut q = Quaternion(inner);
        q.cleanup();
        q
    }
}
/* ========================================================================== */
impl <V> ::std::ops::Index< usize, > for Quaternion<V>
    where V: Number {
    type Output         = V;
    fn index(&self, index: usize) -> &Self::Output {
        let &Quaternion(ref inner) = self;
        &inner[index]
    }
}
/* ========================================================================== */
impl <V> ::std::ops::IndexMut< usize, > for Quaternion<V>
    where V: Number {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let &mut Quaternion(ref mut inner) = self;
        &mut inner[index]
    }
}
/* ========================================================================== */
impl < V, > Quaternion< V, >
    where V: Number, {
    /* ====================================================================== */
    /// as_ptr
    pub fn as_ptr(&self) -> *const V {
        let &Quaternion(ref inner) = self;
        inner.as_ptr()
    }
    /* ====================================================================== */
    /// as_mut_ptr
    pub fn as_mut_ptr(&mut self) -> *mut V {
        let &mut Quaternion(ref mut inner) = self;
        inner.as_mut_ptr()
    }
    /* ====================================================================== */
    /// cleanup
    pub fn cleanup(&mut self) {
        let &mut Quaternion(ref mut inner) = self;
        inner.cleanup();
    }
    /* ====================================================================== */
    /// rot
    pub fn rot(theata: V, x: V, y: V, z: V)
               -> Result< Quaternion< V, >, MathError > {
        let n = Float::sqrt((x * x) + (y * y) + (z * z));
        if n < V::epsilon() {
            return Err(MathError::InvaridArguments(
                String::from("math::Quaternion_::rot")));
        }
        let m = Float::sin(theata / V::from(2).unwrap()) / n;
        Ok(Quaternion::from([
            x*m, y*m, z*m, Float::cos(theata / V::from(2).unwrap())
        ]))
    }
}
