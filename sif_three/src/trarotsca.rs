// -*- mode:rust; coding:utf-8-unix; -*-

//! trarotsca.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/04/10
//  @date 2018/06/21

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use num::Float;
// ----------------------------------------------------------------------------
use sif_math::{Matrix4x4, Number, Quaternion, Vector3, Vector4};
// ----------------------------------------------------------------------------
use super::{Error, Result};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct TraRotSca
#[derive(Debug, Clone)]
pub struct TraRotSca<V>
where
    V: Number,
{
    // ------------------------------------------------------------------------
    /// rotation
    pub rotation: Quaternion<V>,
    /// translation
    pub translation: Vector3<V>,
    /// scaling
    pub scaling: Vector3<V>,
}
// ============================================================================
impl<V> Default for TraRotSca<V>
where
    V: Number,
{
    // ========================================================================
    fn default() -> Self {
        TraRotSca {
            translation: Vector3::<V>::new(V::zero(), V::zero(), V::zero()),
            rotation: Quaternion::<V>::default(),
            scaling: Vector3::<V>::new(V::one(), V::one(), V::one()),
        }
    }
}
// ============================================================================
impl<V> TraRotSca<V>
where
    V: Number,
{
    // ========================================================================
    /// new
    pub fn new(
        translation: Vector3<V>,
        rotation: Quaternion<V>,
        scaling: Vector3<V>,
    ) -> Self {
        TraRotSca {
            rotation,
            translation,
            scaling,
        }
    }
    // ========================================================================
    /// matrix
    pub fn matrix(&self) -> Matrix4x4<V> {
        new_mat4_trarotsca(
            self.translation[0],
            self.translation[1],
            self.translation[2],
            self.rotation[0],
            self.rotation[1],
            self.rotation[2],
            self.rotation[3],
            self.scaling[0],
            self.scaling[1],
            self.scaling[2],
        )
    }
    // ------------------------------------------------------------------------
    /// inverse_matrix
    pub fn inverse_matrix(&self) -> Matrix4x4<V> {
        new_mat4_inverse_trarotsca(
            self.translation[0],
            self.translation[1],
            self.translation[2],
            self.rotation[0],
            self.rotation[1],
            self.rotation[2],
            self.rotation[3],
            self.scaling[0],
            self.scaling[1],
            self.scaling[2],
        )
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// new_mat4_tra
pub fn new_mat4_tra<V>(tx: V, ty: V, tz: V) -> Matrix4x4<V>
where
    V: Number,
{
    Matrix4x4::from([
        Vector4::from_no_clean([V::one(), V::zero(), V::zero(), V::zero()]),
        Vector4::from_no_clean([V::zero(), V::one(), V::zero(), V::zero()]),
        Vector4::from_no_clean([V::zero(), V::zero(), V::one(), V::zero()]),
        Vector4::from_no_clean([tx, ty, tz, V::one()]),
    ])
}
// ----------------------------------------------------------------------------
/// new_mat4_rot
pub fn new_mat4_rot<V>(rx: V, ry: V, rz: V, rw: V) -> Matrix4x4<V>
where
    V: Number,
{
    Matrix4x4::from([
        Vector4::from_no_clean([
            (V::one() - V::from(2).unwrap() * (ry * ry + rz * rz)),
            (V::from(2).unwrap() * (rw * rz + rx * ry)),
            (V::from(2).unwrap() * (rx * rz - rw * ry)),
            V::zero(),
        ]),
        Vector4::from_no_clean([
            (V::from(2).unwrap() * (rx * ry - rw * rz)),
            (V::one() - V::from(2).unwrap() * (rx * rx + rz * rz)),
            (V::from(2).unwrap() * (ry * rz + rw * rx)),
            V::zero(),
        ]),
        Vector4::from_no_clean([
            (V::from(2).unwrap() * (rx * rz + rw * ry)),
            (V::from(2).unwrap() * (ry * rz - rw * rx)),
            (V::one() - V::from(2).unwrap() * (rx * rx + ry * ry)),
            V::zero(),
        ]),
        Vector4::from_no_clean([V::zero(), V::zero(), V::zero(), V::one()]),
    ])
}
// ----------------------------------------------------------------------------
/// new_mat4_sca
pub fn new_mat4_sca<V>(sx: V, sy: V, sz: V) -> Matrix4x4<V>
where
    V: Number,
{
    Matrix4x4::from([
        Vector4::from_no_clean([sx, V::zero(), V::zero(), V::zero()]),
        Vector4::from_no_clean([V::zero(), sy, V::zero(), V::zero()]),
        Vector4::from_no_clean([V::zero(), V::zero(), sz, V::zero()]),
        Vector4::from_no_clean([V::zero(), V::zero(), V::zero(), V::one()]),
    ])
}
// ----------------------------------------------------------------------------
/// new_mat4_trarotsca
pub fn new_mat4_trarotsca<V>(
    tx: V,
    ty: V,
    tz: V,
    rx: V,
    ry: V,
    rz: V,
    rw: V,
    sx: V,
    sy: V,
    sz: V,
) -> Matrix4x4<V>
where
    V: Number,
{
    Matrix4x4::from([
        Vector4::from_no_clean([
            (V::one() - V::from(2).unwrap() * (ry * ry + rz * rz)) * sx,
            (V::from(2).unwrap() * (rw * rz + rx * ry)) * sx,
            (V::from(2).unwrap() * (rx * rz - rw * ry)) * sx,
            V::zero(),
        ]),
        Vector4::from_no_clean([
            (V::from(2).unwrap() * (rx * ry - rw * rz)) * sy,
            (V::one() - V::from(2).unwrap() * (rx * rx + rz * rz)) * sy,
            (V::from(2).unwrap() * (ry * rz + rw * rx)) * sy,
            V::zero(),
        ]),
        Vector4::from_no_clean([
            (V::from(2).unwrap() * (rx * rz + rw * ry)) * sz,
            (V::from(2).unwrap() * (ry * rz - rw * rx)) * sz,
            (V::one() - V::from(2).unwrap() * (rx * rx + ry * ry)) * sz,
            V::zero(),
        ]),
        Vector4::from_no_clean([tx, ty, tz, V::one()]),
    ])
}
// ----------------------------------------------------------------------------
/// new_mat4_inverse_trarotsca
pub fn new_mat4_inverse_trarotsca<V>(
    tx: V,
    ty: V,
    tz: V,
    rx: V,
    ry: V,
    rz: V,
    rw: V,
    sx: V,
    sy: V,
    sz: V,
) -> Matrix4x4<V>
where
    V: Number,
{
    let t0 = Vector4::from_no_clean([
        (V::one() - V::from(2).unwrap() * (ry * ry + rz * rz)) / sx,
        (V::from(2).unwrap() * (rx * ry - rw * rz)) / sy,
        (V::from(2).unwrap() * (rw * ry + rx * rz)) / sz,
        V::zero(),
    ]);
    let t1 = Vector4::from_no_clean([
        (V::from(2).unwrap() * (rx * ry + rw * rz)) / sx,
        (V::one() - V::from(2).unwrap() * (rx * rx + rz * rz)) / sy,
        (V::from(2).unwrap() * (ry * rz - rw * rx)) / sz,
        V::zero(),
    ]);
    let t2 = Vector4::from_no_clean([
        (V::from(2).unwrap() * (rx * rz - rw * ry)) / sx,
        (V::from(2).unwrap() * (rw * rx + ry * rz)) / sy,
        (V::one() - V::from(2).unwrap() * (rx * rx + ry * ry)) / sz,
        V::zero(),
    ]);
    Matrix4x4::from([
        t0,
        t1,
        t2,
        Vector4::from_no_clean([
            t0[0] * -tx + t1[0] * -ty + t2[0] * -tz,
            t0[1] * -tx + t1[1] * -ty + t2[1] * -tz,
            t0[2] * -tx + t1[2] * -ty + t2[2] * -tz,
            V::one(),
        ]),
    ])
}
// ============================================================================
/// new_quaternion_rot
pub fn new_quaternion_rot<V>(
    theta: V,
    x: V,
    y: V,
    z: V,
) -> Result<Quaternion<V>>
where
    V: Number,
{
    // let pi = ::num::cast::cast::<_, V>(::std::f64::consts::PI).unwrap();
    let l = (x * x) + (y * y) + (z * z);
    if l < V::epsilon() {
        Err(Error::InvalidArgument(String::from(
            "::three::trarotsca::::new_quaternion_rot",
        )))
    } else {
        let (mut sin, cos) = Float::sin_cos(theta / V::from(2).unwrap());
        sin /= Float::sqrt(l);
        Ok(Quaternion::from([x * sin, y * sin, z * sin, cos]))
    }
}
