/* -*- mode:rust; coding:utf-8-unix; -*- */

//! matrix.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/19
//  @date 2016/06/18

/* ////////////////////////////////////////////////////////////////////////// */
/* use  ===================================================================== */
use super::{ Number, Cleanup,
             //Vector2,
             //Vector3,
             Vector4, };
/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
/// specify!
macro_rules! specify  {
    ($name:ty; $($t:ty),*)      => (concat_idents!($name, "<", $($t),*, ">"));
    ($name:ty; :: $($t:ty),*)   => (concat_idents!($name, "::<", $($t),*, ">"));
}
/* ========================================================================== */
/// matrix_define!
macro_rules! matrix_define {
    ($name:ident($vector:ident; $i:expr))               => {
        /* ================================================================== */
        /// struct $name
        #[derive( Debug, Default, Clone, Copy, )]
        pub struct $name<V: Number>([specify!($vector; V); $i]);
        /* ================================================================== */
        impl <V> From< [specify!($vector; V); $i] > for $name<V>
            where V: Number {
            fn from(inner: [specify!($vector; V); $i]) -> Self {
                let mut m = $name(inner);
                m.cleanup();
                m
            }
        }
        /* ================================================================== */
        impl <V> ::std::ops::Index< usize, > for $name<V>
            where V: Number {
            type Output = specify!($vector; V);
            fn index(&self, index: usize) -> &Self::Output {
                let &$name(ref inner) = self;
                &inner[index]
            }
        }
        /* ================================================================== */
        impl <V> ::std::ops::IndexMut< usize, > for $name<V>
            where V: Number {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                let &mut $name(ref mut inner) = self;
                &mut inner[index]
            }
        }
        /* ================================================================== */
        matrix_define_impl!($name);
        /* ================================================================== */
        impl <V> $name< V, >
            where V:    Number {
            /* ============================================================== */
            /// from_no_clean
            pub fn from_no_clean(inner: [specify!($vector; V); $i]) -> Self {
                $name(inner)
            }
            /* ============================================================== */
            /// as_ptr
            pub fn as_ptr(&self) -> *const V {
                let &$name(ref inner) = self;
                inner as *const _ as *const V
            }
            /* ============================================================== */
            /// as_mut_ptr
            pub fn as_mut_ptr(&mut self) -> *mut V {
                let &mut $name(ref mut inner) = self;
                inner as *mut _ as *mut V
            }
            /* ============================================================== */
            /// cleanup
            pub fn cleanup(&mut self) {
                let mut c = Cleanup::new();
                for i in 0 .. $i {
                    for j in 0 .. $vector::<V>::size() {
                        c.collect(self[i][j]);
                    }
                }
                for i in 0 .. $i {
                    for j in 0 .. $vector::<V>::size() {
                        self[i][j] = c.check(self[i][j]);
                    }
                }
            }
            /* ============================================================== */
            matrix_define_inner!($name);
        }
    };
}
/* ========================================================================== */
/// matrix_define_inner!
macro_rules! matrix_define_inner {
    (Matrix4x4)                   => {
        /* ================================================================== */
        /// mul
        pub fn mul(lhs: &$name< V, >, rhs: &$name< V, >) -> $name< V, > {
            $name::from([
                $vector::from_no_clean([
                    (lhs[0][0] * rhs[0][0] + lhs[1][0] * rhs[0][1] +
                     lhs[2][0] * rhs[0][2] + lhs[3][0] * rhs[0][3]),
                    (lhs[0][1] * rhs[0][0] + lhs[1][1] * rhs[0][1] +
                     lhs[2][1] * rhs[0][2] + lhs[3][1] * rhs[0][3]),
                    (lhs[0][2] * rhs[0][0] + lhs[1][2] * rhs[0][1] +
                     lhs[2][2] * rhs[0][2] + lhs[3][2] * rhs[0][3]),
                    (lhs[0][3] * rhs[0][0] + lhs[1][3] * rhs[0][1] +
                     lhs[2][3] * rhs[0][2] + lhs[3][3] * rhs[0][3]),
                ]),
                $vector::from_no_clean([
                    (lhs[0][0] * rhs[1][0] + lhs[1][0] * rhs[1][1] +
                     lhs[2][0] * rhs[1][2] + lhs[3][0] * rhs[1][3]),
                    (lhs[0][1] * rhs[1][0] + lhs[1][1] * rhs[1][1] +
                     lhs[2][1] * rhs[1][2] + lhs[3][1] * rhs[1][3]),
                    (lhs[0][2] * rhs[1][0] + lhs[1][2] * rhs[1][1] +
                     lhs[2][2] * rhs[1][2] + lhs[3][2] * rhs[1][3]),
                    (lhs[0][3] * rhs[1][0] + lhs[1][3] * rhs[1][1] +
                     lhs[2][3] * rhs[1][2] + lhs[3][3] * rhs[1][3]),
                ]),
                $vector::from_no_clean([
                    (lhs[0][0] * rhs[2][0] + lhs[1][0] * rhs[2][1] +
                     lhs[2][0] * rhs[2][2] + lhs[3][0] * rhs[2][3]),
                    (lhs[0][1] * rhs[2][0] + lhs[1][1] * rhs[2][1] +
                     lhs[2][1] * rhs[2][2] + lhs[3][1] * rhs[2][3]),
                    (lhs[0][2] * rhs[2][0] + lhs[1][2] * rhs[2][1] +
                     lhs[2][2] * rhs[2][2] + lhs[3][2] * rhs[2][3]),
                    (lhs[0][3] * rhs[2][0] + lhs[1][3] * rhs[2][1] +
                     lhs[2][3] * rhs[2][2] + lhs[3][3] * rhs[2][3]),
                ]),
                $vector::from_no_clean([
                    (lhs[0][0] * rhs[3][0] + lhs[1][0] * rhs[3][1] +
                     lhs[2][0] * rhs[3][2] + lhs[3][0] * rhs[3][3]),
                    (lhs[0][1] * rhs[3][0] + lhs[1][1] * rhs[3][1] +
                     lhs[2][1] * rhs[3][2] + lhs[3][1] * rhs[3][3]),
                    (lhs[0][2] * rhs[3][0] + lhs[1][2] * rhs[3][1] +
                     lhs[2][2] * rhs[3][2] + lhs[3][2] * rhs[3][3]),
                    (lhs[0][3] * rhs[3][0] + lhs[1][3] * rhs[3][1] +
                     lhs[2][3] * rhs[3][2] + lhs[3][3] * rhs[3][3]),
                ])
            ])
        }
        /* ================================================================== */
        /// trarotsca
        pub fn trarotsca(tx: V, ty: V, tz: V,
                         rx: V, ry: V, rz: V, rw: V,
                         sx: V, sy: V, sz: V,) -> $name< V, > {
            $name::from([
                $vector::from_no_clean([
                    (V::one() - V::from(2).unwrap() * (ry*ry + rz*rz)) * sx,
                    (           V::from(2).unwrap() * (rw*rz + rx*ry)) * sx,
                    (           V::from(2).unwrap() * (rx*rz - rw*ry)) * sx,
                    V::zero(),
                ]),
                $vector::from_no_clean([
                    (           V::from(2).unwrap() * (rx*ry - rw*rz)) * sy,
                    (V::one() - V::from(2).unwrap() * (rx*rx + rz*rz)) * sy,
                    (           V::from(2).unwrap() * (ry*rz + rw*rx)) * sy,
                    V::zero(),
                ]),
                $vector::from_no_clean([
                    (           V::from(2).unwrap() * (rx*rz + rw*ry)) * sz,
                    (           V::from(2).unwrap() * (ry*rz - rw*rx)) * sz,
                    (V::one() - V::from(2).unwrap() * (rx*rx + ry*ry)) * sz,
                    V::zero(),
                ]),
                $vector::from_no_clean([ tx, ty, tz, V::one(), ]),
            ])
        }
        /* ================================================================== */
        /// frustum
        pub fn frustum(near: V, far: V, focus: V, aspect: V) -> $name< V, > {
            let c = V::one() / (near - far);
            $name::from([
                $vector::from_no_clean([
                    focus,         V::zero(),  V::zero(),         V::zero()
                ]),
                $vector::from_no_clean([
                    V::zero(), focus / aspect, V::zero(),         V::zero()
                ]),
                $vector::from_no_clean([
                    V::zero(), V::zero(),      (near + far) * c, -V::one()
                ]),
                $vector::from_no_clean([
                    V::zero(), V::zero(),
                    V::from(2).unwrap() * near * far * c,        V::zero()
                ]),
            ])
        }
        /* ================================================================== */
        /// ortho
        pub fn ortho(near: V, far: V, width: V, height: V) -> $name< V, > {
            let c = V::one() / (near - far);
            $name::from([
                $vector::from_no_clean([
                    V::from(2).unwrap()/width, V::zero(),  V::zero(), V::zero()
                ]),
                $vector::from_no_clean([
                    V::zero(), V::from(2).unwrap()/height, V::zero(), V::zero()
                ]),
                $vector::from_no_clean([
                    V::zero(), V::zero(), V::from(2).unwrap() * c,    V::zero()
                ]),
                $vector::from_no_clean([
                    V::zero(), V::zero(),  (near + far) * c,          V::one()
                ]),
            ])
        }
    };
    ($name: ident)              => {
    };
}
/* ========================================================================== */
/// matrix_define_impl!
macro_rules! matrix_define_impl {
    (Matrix4x4)                 => {
        /* ================================================================== */
        impl <V> ::std::ops::Mul<$name<V>> for $name<V>
            where V: Number {
            type Output = Self;
            fn mul(self, rhs: $name<V>) -> Self::Output {
                $name::from([
                    $vector::from_no_clean([
                        (self[0][0] * rhs[0][0] + self[1][0] * rhs[0][1] +
                         self[2][0] * rhs[0][2] + self[3][0] * rhs[0][3]),
                        (self[0][1] * rhs[0][0] + self[1][1] * rhs[0][1] +
                         self[2][1] * rhs[0][2] + self[3][1] * rhs[0][3]),
                        (self[0][2] * rhs[0][0] + self[1][2] * rhs[0][1] +
                         self[2][2] * rhs[0][2] + self[3][2] * rhs[0][3]),
                        (self[0][3] * rhs[0][0] + self[1][3] * rhs[0][1] +
                         self[2][3] * rhs[0][2] + self[3][3] * rhs[0][3]),
                    ]),
                    $vector::from_no_clean([
                        (self[0][0] * rhs[1][0] + self[1][0] * rhs[1][1] +
                         self[2][0] * rhs[1][2] + self[3][0] * rhs[1][3]),
                        (self[0][1] * rhs[1][0] + self[1][1] * rhs[1][1] +
                         self[2][1] * rhs[1][2] + self[3][1] * rhs[1][3]),
                        (self[0][2] * rhs[1][0] + self[1][2] * rhs[1][1] +
                         self[2][2] * rhs[1][2] + self[3][2] * rhs[1][3]),
                        (self[0][3] * rhs[1][0] + self[1][3] * rhs[1][1] +
                         self[2][3] * rhs[1][2] + self[3][3] * rhs[1][3]),
                    ]),
                    $vector::from_no_clean([
                        (self[0][0] * rhs[2][0] + self[1][0] * rhs[2][1] +
                         self[2][0] * rhs[2][2] + self[3][0] * rhs[2][3]),
                        (self[0][1] * rhs[2][0] + self[1][1] * rhs[2][1] +
                         self[2][1] * rhs[2][2] + self[3][1] * rhs[2][3]),
                        (self[0][2] * rhs[2][0] + self[1][2] * rhs[2][1] +
                         self[2][2] * rhs[2][2] + self[3][2] * rhs[2][3]),
                        (self[0][3] * rhs[2][0] + self[1][3] * rhs[2][1] +
                         self[2][3] * rhs[2][2] + self[3][3] * rhs[2][3]),
                    ]),
                    $vector::from_no_clean([
                        (self[0][0] * rhs[3][0] + self[1][0] * rhs[3][1] +
                         self[2][0] * rhs[3][2] + self[3][0] * rhs[3][3]),
                        (self[0][1] * rhs[3][0] + self[1][1] * rhs[3][1] +
                         self[2][1] * rhs[3][2] + self[3][1] * rhs[3][3]),
                        (self[0][2] * rhs[3][0] + self[1][2] * rhs[3][1] +
                         self[2][2] * rhs[3][2] + self[3][2] * rhs[3][3]),
                        (self[0][3] * rhs[3][0] + self[1][3] * rhs[3][1] +
                         self[2][3] * rhs[3][2] + self[3][3] * rhs[3][3]),
                    ]),
                ])
            }
        }
        /* ------------------------------------------------------------------ */
        impl <V> ::std::ops::MulAssign<$name<V>> for $name<V>
            where V: Number {
            fn mul_assign(&mut self, rhs: $name<V>) {
                let tmp = self * rhs;
                self = tmp;
            }
        }
        /* ================================================================== */
        impl <V> ::std::ops::Mul<$vector<V>> for Matrix4x4<V>
            where V: Number {
            type Output = $vector<V>;
            fn mul(self, rhs: $vector<V>) -> Self::Output {
                $vector::from([
                    (self[0][0] * rhs[0] + self[1][0] * rhs[1] +
                     self[2][0] * rhs[2] + self[3][0] * rhs[3]),
                    (self[0][1] * rhs[0] + self[1][1] * rhs[1] +
                     self[2][1] * rhs[2] + self[3][1] * rhs[3]),
                    (self[0][2] * rhs[0] + self[1][2] * rhs[1] +
                     self[2][2] * rhs[2] + self[3][2] * rhs[3]),
                    (self[0][3] * rhs[0] + self[1][3] * rhs[1] +
                     self[2][3] * rhs[2] + self[3][3] * rhs[3]),
                ])
            }
        }
    };
    ($name: ident)              => {
    };
}
/* ========================================================================== */
//matrix_define!(Matrix2x2(Vector2; 2));
//matrix_define!(Matrix2x3(Vector3; 2));
//matrix_define!(Matrix3x2(Vector2; 3));
//matrix_define!(Matrix3x3(Vector3; 3));
//matrix_define!(Matrix3x4(Vector4; 3));
//matrix_define!(Matrix4x3(Vector3; 4));
//matrix_define!(Matrix4x4(Vector4; 4));
/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
/// struct Matrix4x4
#[derive( Debug, Default, Clone, Copy, PartialEq, Eq, )]
pub struct Matrix4x4<V: Number>([Vector4< V, >; 4]);
/* ========================================================================== */
impl <V> Matrix4x4< V, >
    where V:    Number {
    /* ====================================================================== */
    /// from_no_clean
    pub fn from_no_clean(inner: [Vector4<V>; 4]) -> Self { Matrix4x4(inner) }
    /* ====================================================================== */
    /// as_ptr
    pub fn as_ptr(&self) -> *const V {
        let &Matrix4x4(ref inner) = self;
        inner as *const _ as *const V
    }
    /* ====================================================================== */
    /// as_mut_ptr
    pub fn as_mut_ptr(&mut self) -> *mut V {
        let &mut Matrix4x4(ref mut inner) = self;
        inner as *mut _ as *mut V
    }
    /* ====================================================================== */
    /// cleanup
    pub fn cleanup(&mut self) {
        let mut c = Cleanup::new();
        for i in 0 .. 4 { for j in 0 .. Vector4::<V>::size() {
            c.collect(self[i][j]);
        } }
        for i in 0 .. 4 { for j in 0 .. Vector4::<V>::size() {
            self[i][j] = c.check(self[i][j]);
        } }
    }
    /* ====================================================================== */
    /// mul
    pub fn mul(lhs: &Matrix4x4< V, >, rhs: &Matrix4x4< V, >)
               -> Matrix4x4< V, > {
        Matrix4x4::from([
            Vector4::from_no_clean([
                (lhs[0][0] * rhs[0][0] + lhs[1][0] * rhs[0][1] +
                 lhs[2][0] * rhs[0][2] + lhs[3][0] * rhs[0][3]),
                (lhs[0][1] * rhs[0][0] + lhs[1][1] * rhs[0][1] +
                 lhs[2][1] * rhs[0][2] + lhs[3][1] * rhs[0][3]),
                (lhs[0][2] * rhs[0][0] + lhs[1][2] * rhs[0][1] +
                 lhs[2][2] * rhs[0][2] + lhs[3][2] * rhs[0][3]),
                (lhs[0][3] * rhs[0][0] + lhs[1][3] * rhs[0][1] +
                 lhs[2][3] * rhs[0][2] + lhs[3][3] * rhs[0][3]),
            ]),
            Vector4::from_no_clean([
                (lhs[0][0] * rhs[1][0] + lhs[1][0] * rhs[1][1] +
                 lhs[2][0] * rhs[1][2] + lhs[3][0] * rhs[1][3]),
                (lhs[0][1] * rhs[1][0] + lhs[1][1] * rhs[1][1] +
                 lhs[2][1] * rhs[1][2] + lhs[3][1] * rhs[1][3]),
                (lhs[0][2] * rhs[1][0] + lhs[1][2] * rhs[1][1] +
                 lhs[2][2] * rhs[1][2] + lhs[3][2] * rhs[1][3]),
                (lhs[0][3] * rhs[1][0] + lhs[1][3] * rhs[1][1] +
                 lhs[2][3] * rhs[1][2] + lhs[3][3] * rhs[1][3]),
            ]),
            Vector4::from_no_clean([
                (lhs[0][0] * rhs[2][0] + lhs[1][0] * rhs[2][1] +
                 lhs[2][0] * rhs[2][2] + lhs[3][0] * rhs[2][3]),
                (lhs[0][1] * rhs[2][0] + lhs[1][1] * rhs[2][1] +
                 lhs[2][1] * rhs[2][2] + lhs[3][1] * rhs[2][3]),
                (lhs[0][2] * rhs[2][0] + lhs[1][2] * rhs[2][1] +
                 lhs[2][2] * rhs[2][2] + lhs[3][2] * rhs[2][3]),
                (lhs[0][3] * rhs[2][0] + lhs[1][3] * rhs[2][1] +
                 lhs[2][3] * rhs[2][2] + lhs[3][3] * rhs[2][3]),
            ]),
            Vector4::from_no_clean([
                (lhs[0][0] * rhs[3][0] + lhs[1][0] * rhs[3][1] +
                 lhs[2][0] * rhs[3][2] + lhs[3][0] * rhs[3][3]),
                (lhs[0][1] * rhs[3][0] + lhs[1][1] * rhs[3][1] +
                 lhs[2][1] * rhs[3][2] + lhs[3][1] * rhs[3][3]),
                (lhs[0][2] * rhs[3][0] + lhs[1][2] * rhs[3][1] +
                 lhs[2][2] * rhs[3][2] + lhs[3][2] * rhs[3][3]),
                (lhs[0][3] * rhs[3][0] + lhs[1][3] * rhs[3][1] +
                 lhs[2][3] * rhs[3][2] + lhs[3][3] * rhs[3][3]),
            ])
        ])
    }
    /* ====================================================================== */
    /// trarotsca
    pub fn trarotsca(tx: V, ty: V, tz: V,
                     rx: V, ry: V, rz: V, rw: V,
                     sx: V, sy: V, sz: V,) -> Matrix4x4< V, > {
        Matrix4x4::from([
            Vector4::from_no_clean([
                (V::one() - V::from(2).unwrap() * (ry * ry + rz * rz)) * sx,
                (           V::from(2).unwrap() * (rw * rz + rx * ry)) * sx,
                (           V::from(2).unwrap() * (rx * rz - rw * ry)) * sx,
                V::zero(),
            ]),
            Vector4::from_no_clean([
                (           V::from(2).unwrap() * (rx * ry - rw * rz)) * sy,
                (V::one() - V::from(2).unwrap() * (rx * rx + rz * rz)) * sy,
                (           V::from(2).unwrap() * (ry * rz + rw * rx)) * sy,
                V::zero(),
            ]),
            Vector4::from_no_clean([
                (           V::from(2).unwrap() * (rx * rz + rw * ry)) * sz,
                (           V::from(2).unwrap() * (ry * rz - rw * rx)) * sz,
                (V::one() - V::from(2).unwrap() * (rx * rx + ry * ry)) * sz,
                V::zero(),
            ]),
            Vector4::from_no_clean([ tx, ty, tz, V::one(), ]),
        ])
    }
    /* ====================================================================== */
    /// frustum
    pub fn frustum(near: V, far: V, focus: V, aspect: V) -> Matrix4x4< V, > {
        let c = V::one() / (near - far);
        Matrix4x4::from([
            Vector4::from_no_clean([
                focus, V::zero(), V::zero(), V::zero()
            ]),
            Vector4::from_no_clean([
                V::zero(), focus / aspect, V::zero(), V::zero()
            ]),
            Vector4::from_no_clean([
                V::zero(), V::zero(), (near + far) * c, -V::one()
            ]),
            Vector4::from_no_clean([
                V::zero(), V::zero(), V::from(2).unwrap()*near*far*c, V::zero()
            ]),
        ])
    }
    /* ====================================================================== */
    /// ortho
    pub fn ortho(near: V, far: V, width: V, height: V) -> Matrix4x4< V, > {
        let c = V::one() / (near - far);
        Matrix4x4::from([
            Vector4::from_no_clean([
                V::from(2).unwrap() / width, V::zero(), V::zero(),  V::zero()
            ]),
            Vector4::from_no_clean([
                V::zero(), V::from(2).unwrap() / height, V::zero(), V::zero()
            ]),
            Vector4::from_no_clean([
                V::zero(), V::zero(), V::from(2).unwrap() * c,      V::zero()
            ]),
            Vector4::from_no_clean([
                V::zero(), V::zero(),  (near + far) * c,            V::one()
            ]),
        ])
    }
}
/* ========================================================================== */
impl <V> From< [Vector4< V, >; 4] > for Matrix4x4<V>
    where V: Number {
    fn from(inner: [Vector4< V, >; 4]) -> Self {
        let mut m = Matrix4x4(inner);
        m.cleanup();
        m
    }
}
/* ========================================================================== */
impl <V> ::std::ops::Index< usize, > for Matrix4x4<V>
    where V: Number {
    type Output         = Vector4< V, >;
    fn index(&self, index: usize) -> &Self::Output {
        let &Matrix4x4(ref inner) = self;
        &inner[index]
    }
}
/* ========================================================================== */
impl <V> ::std::ops::IndexMut< usize, > for Matrix4x4<V>
    where V: Number {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let &mut Matrix4x4(ref mut inner) = self;
        &mut inner[index]
    }
}
/* ========================================================================== */
impl <V> ::std::ops::Mul<Matrix4x4<V>> for Matrix4x4<V>
    where V: Number {
    type Output = Self;
    fn mul(self, rhs: Matrix4x4<V>) -> Self::Output {
        Matrix4x4::from([
            Vector4::from_no_clean([
                (self[0][0] * rhs[0][0] + self[1][0] * rhs[0][1] +
                 self[2][0] * rhs[0][2] + self[3][0] * rhs[0][3]),
                (self[0][1] * rhs[0][0] + self[1][1] * rhs[0][1] +
                 self[2][1] * rhs[0][2] + self[3][1] * rhs[0][3]),
                (self[0][2] * rhs[0][0] + self[1][2] * rhs[0][1] +
                 self[2][2] * rhs[0][2] + self[3][2] * rhs[0][3]),
                (self[0][3] * rhs[0][0] + self[1][3] * rhs[0][1] +
                 self[2][3] * rhs[0][2] + self[3][3] * rhs[0][3]),
            ]),
            Vector4::from_no_clean([
                (self[0][0] * rhs[1][0] + self[1][0] * rhs[1][1] +
                 self[2][0] * rhs[1][2] + self[3][0] * rhs[1][3]),
                (self[0][1] * rhs[1][0] + self[1][1] * rhs[1][1] +
                 self[2][1] * rhs[1][2] + self[3][1] * rhs[1][3]),
                (self[0][2] * rhs[1][0] + self[1][2] * rhs[1][1] +
                 self[2][2] * rhs[1][2] + self[3][2] * rhs[1][3]),
                (self[0][3] * rhs[1][0] + self[1][3] * rhs[1][1] +
                 self[2][3] * rhs[1][2] + self[3][3] * rhs[1][3]),
            ]),
            Vector4::from_no_clean([
                (self[0][0] * rhs[2][0] + self[1][0] * rhs[2][1] +
                 self[2][0] * rhs[2][2] + self[3][0] * rhs[2][3]),
                (self[0][1] * rhs[2][0] + self[1][1] * rhs[2][1] +
                 self[2][1] * rhs[2][2] + self[3][1] * rhs[2][3]),
                (self[0][2] * rhs[2][0] + self[1][2] * rhs[2][1] +
                 self[2][2] * rhs[2][2] + self[3][2] * rhs[2][3]),
                (self[0][3] * rhs[2][0] + self[1][3] * rhs[2][1] +
                 self[2][3] * rhs[2][2] + self[3][3] * rhs[2][3]),
            ]),
            Vector4::from_no_clean([
                (self[0][0] * rhs[3][0] + self[1][0] * rhs[3][1] +
                 self[2][0] * rhs[3][2] + self[3][0] * rhs[3][3]),
                (self[0][1] * rhs[3][0] + self[1][1] * rhs[3][1] +
                 self[2][1] * rhs[3][2] + self[3][1] * rhs[3][3]),
                (self[0][2] * rhs[3][0] + self[1][2] * rhs[3][1] +
                 self[2][2] * rhs[3][2] + self[3][2] * rhs[3][3]),
                (self[0][3] * rhs[3][0] + self[1][3] * rhs[3][1] +
                 self[2][3] * rhs[3][2] + self[3][3] * rhs[3][3]),
            ]),
        ])
    }
}
/* -------------------------------------------------------------------------- */
impl <V> ::std::ops::MulAssign<Matrix4x4<V>> for Matrix4x4<V>
    where V: Number {
    fn mul_assign(&mut self, rhs: Matrix4x4<V>) {
        *self = ::std::ops::Mul::<Matrix4x4<V>>::mul(*self, rhs);
    }
}
/* ========================================================================== */
impl <V> ::std::ops::Mul<Vector4<V>> for Matrix4x4<V>
    where V: Number {
    type Output = Vector4<V>;
    fn mul(self, rhs: Vector4<V>) -> Self::Output {
        Vector4::from([
            (self[0][0] * rhs[0] + self[1][0] * rhs[1] +
             self[2][0] * rhs[2] + self[3][0] * rhs[3]),
            (self[0][1] * rhs[0] + self[1][1] * rhs[1] +
             self[2][1] * rhs[2] + self[3][1] * rhs[3]),
            (self[0][2] * rhs[0] + self[1][2] * rhs[1] +
             self[2][2] * rhs[2] + self[3][2] * rhs[3]),
            (self[0][3] * rhs[0] + self[1][3] * rhs[1] +
             self[2][3] * rhs[2] + self[3][3] * rhs[3]),
        ])
    }
}
