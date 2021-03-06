// -*- mode:rust; coding:utf-8-unix; -*-

//! trarotsca.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/04/10
//  @date 2018/08/27

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use sif_math::{Float, Matrix4x4, Quaternion, Vector3, Vector4};
// ----------------------------------------------------------------------------
use super::{Error, Result};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum TraRotScaType
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum TraRotScaType {
    /// Translate
    Translate,
    /// Rotate
    Rotate,
    /// Scale
    Scale,
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct TraRotSca
#[derive(Debug, Clone)]
pub struct TraRotSca<V>
where
    V: Float,
{
    // ------------------------------------------------------------------------
    /// rotate
    pub rotate: Quaternion<V>,
    /// translate
    pub translate: Vector3<V>,
    /// scale
    pub scale: Vector3<V>,
}
// ============================================================================
impl<V> Default for TraRotSca<V>
where
    V: Float,
{
    // ========================================================================
    fn default() -> Self {
        TraRotSca {
            translate: Vector3::<V>::new(V::zero(), V::zero(), V::zero()),
            rotate: Quaternion::<V>::default(),
            scale: Vector3::<V>::new(V::one(), V::one(), V::one()),
        }
    }
}
// ============================================================================
impl<V> TraRotSca<V>
where
    V: Float,
{
    // ========================================================================
    /// new
    pub fn new(
        translate: Vector3<V>,
        rotate: Quaternion<V>,
        scale: Vector3<V>,
    ) -> Self {
        TraRotSca {
            rotate,
            translate,
            scale,
        }
    }
    // ========================================================================
    /// matrix
    pub fn matrix(&self) -> Matrix4x4<V> {
        new_mat4_trarotsca(
            self.translate[0],
            self.translate[1],
            self.translate[2],
            self.rotate[0],
            self.rotate[1],
            self.rotate[2],
            self.rotate[3],
            self.scale[0],
            self.scale[1],
            self.scale[2],
        )
    }
    // ------------------------------------------------------------------------
    /// inverse_matrix
    pub fn inverse_matrix(&self) -> Matrix4x4<V> {
        new_mat4_inverse_trarotsca(
            self.translate[0],
            self.translate[1],
            self.translate[2],
            self.rotate[0],
            self.rotate[1],
            self.rotate[2],
            self.rotate[3],
            self.scale[0],
            self.scale[1],
            self.scale[2],
        )
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// new_mat4_tra
pub fn new_mat4_tra<V>(tx: V, ty: V, tz: V) -> Matrix4x4<V>
where
    V: Float,
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
    V: Float,
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
    V: Float,
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
    V: Float,
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
    V: Float,
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
    x: V,
    y: V,
    z: V,
    theta: V,
) -> Result<Quaternion<V>>
where
    V: Float,
{
    // let pi = ::num::cast::cast::<_, V>(::std::f64::consts::PI).unwrap();
    let l = (x * x) + (y * y) + (z * z);
    if l < V::epsilon() {
        Err(Error::InvalidArgument(String::from(
            "::three::trarotsca::::new_quaternion_rot",
        )))
    } else {
        let (mut sin, cos) =
            ::num::Float::sin_cos(theta / V::from(2).unwrap());
        sin /= ::num::Float::sqrt(l);
        Ok(Quaternion::from([x * sin, y * sin, z * sin, cos]))
    }
}
