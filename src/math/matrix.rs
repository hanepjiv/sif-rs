// -*- mode:rust; coding:utf-8-unix; -*-

//! matrix.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/19
//  @date 2017/01/03

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use super::{ Number, Cleanup, Vector2, Vector3, Vector4, };
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// matrix_define!
macro_rules! matrix_define {
    ($name:ident($vector:ident; $i:expr))               => {
        // ====================================================================
        /// struct $name
        #[derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, )]
        pub struct $name<V: Number>([$vector<V>; $i]);
        // ====================================================================
        impl <V> From< [$vector<V>; $i] > for $name<V>
            where V: Number {
            fn from(inner: [$vector<V>; $i]) -> Self {
                let mut m = $name(inner);
                m.cleanup();
                m
            }
        }
        // ====================================================================
        impl <V> ::std::ops::Index< usize, > for $name<V>
            where V: Number {
            type Output = $vector<V>;
            fn index(&self, index: usize) -> &Self::Output {
                let &$name(ref inner) = self;
                &inner[index]
            }
        }
        // ====================================================================
        impl <V> ::std::ops::IndexMut< usize, > for $name<V>
            where V: Number {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                let &mut $name(ref mut inner) = self;
                &mut inner[index]
            }
        }
        // ====================================================================
        impl <V> ::std::ops::Add<V> for $name<V>
            where V: Number {
            type Output = Self;
            fn add(self, rhs: V) -> Self::Output {
                let mut inner = [$vector::<V>::default(); $i];
                for i in 0 .. $i { inner[i] = self[i] + rhs; }
                Self::from(inner)
            }
        }
        // --------------------------------------------------------------------
        impl <V> ::std::ops::AddAssign<V> for $name<V>
            where V: Number {
            fn add_assign(&mut self, rhs: V) {
                let &mut $name(ref mut inner) = self;
                for i in 0 .. $i { inner[i] += rhs; }
            }
        }
        // ====================================================================
        impl <V> ::std::ops::Sub<V> for $name<V>
            where V: Number {
            type Output = Self;
            fn sub(self, rhs: V) -> Self::Output {
                let mut inner = [$vector::<V>::default(); $i];
                for i in 0 .. $i { inner[i] = self[i] - rhs; }
                Self::from(inner)
            }
        }
        // --------------------------------------------------------------------
        impl <V> ::std::ops::SubAssign<V> for $name<V>
            where V: Number {
            fn sub_assign(&mut self, rhs: V) {
                let &mut $name(ref mut inner) = self;
                for i in 0 .. $i { inner[i] -= rhs; }
            }
        }
        // ====================================================================
        impl <V> ::std::ops::Mul<V> for $name<V>
            where V: Number {
            type Output = Self;
            fn mul(self, rhs: V) -> Self::Output {
                let mut inner = [$vector::<V>::default(); $i];
                for i in 0 .. $i { inner[i] = self[i] * rhs; }
                Self::from(inner)
            }
        }
        // --------------------------------------------------------------------
        impl <V> ::std::ops::MulAssign<V> for $name<V>
            where V: Number {
            fn mul_assign(&mut self, rhs: V) {
                let &mut $name(ref mut inner) = self;
                for i in 0 .. $i { inner[i] *= rhs; }
            }
        }
        // ====================================================================
        impl <V> ::std::ops::Div<V> for $name<V>
            where V: Number {
            type Output = Self;
            fn div(self, rhs: V) -> Self::Output {
                let mut inner = [$vector::<V>::default(); $i];
                for i in 0 .. $i { inner[i] = self[i] / rhs; }
                Self::from(inner)
            }
        }
        // --------------------------------------------------------------------
        impl <V> ::std::ops::DivAssign<V> for $name<V>
            where V: Number {
            fn div_assign(&mut self, rhs: V) {
                let &mut $name(ref mut inner) = self;
                for i in 0 .. $i { inner[i] /= rhs; }
            }
        }
        // ====================================================================
        matrix_define_impl!($name($vector; $i));
        // ====================================================================
        impl <V> $name< V, >
            where V:    Number {
            // ================================================================
            /// from_no_clean
            pub fn from_no_clean(inner: [$vector<V>; $i]) -> Self {
                $name(inner)
            }
            // ================================================================
            /// as_ptr
            pub fn as_ptr(&self) -> *const V {
                let &$name(ref inner) = self;
                inner[0].as_ptr()
            }
            // ================================================================
            /// as_mut_ptr
            pub fn as_mut_ptr(&mut self) -> *mut V {
                let &mut $name(ref mut inner) = self;
                inner[0].as_mut_ptr()
            }
            // ================================================================
            /// cleanup
            pub fn cleanup(&mut self) {
                let mut c = Cleanup::new();
                for i in 0 .. $i { for j in 0 .. $vector::<V>::size() {
                    c.collect(self[i][j]);
                } }
                for i in 0 .. $i { for j in 0 .. $vector::<V>::size() {
                    self[i][j] = c.check(self[i][j]);
                } }
            }
            // ================================================================
            matrix_define_inner!($name($vector; $i));
        }
    };
}
// ============================================================================
/// matrix_define_impl!
macro_rules! matrix_define_impl {
    (Matrix4x4($vector:ident; $i:expr))         => {
        // ====================================================================
        impl <V> Default for Matrix4x4<V>
            where V: Number {
            fn default() -> Self { Matrix4x4::from_no_clean([
                $vector::from_no_clean([
                    V::one(), V::zero(), V::zero(), V::zero(),
                ]),
                $vector::from_no_clean([
                    V::zero(), V::one(), V::zero(), V::zero(),
                ]),
                $vector::from_no_clean([
                    V::zero(), V::zero(), V::one(), V::zero(),
                ]),
                $vector::from_no_clean([
                    V::zero(), V::zero(), V::zero(), V::one(),
                ]),
            ]) }
        }
        // ====================================================================
        impl <V> ::std::ops::Mul<Matrix4x4<V>> for Matrix4x4<V>
            where V: Number {
            type Output = Self;
            fn mul(self, rhs: Matrix4x4<V>) -> Self::Output { Matrix4x4::from([
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
            ]) }
        }
        // --------------------------------------------------------------------
        impl <V> ::std::ops::MulAssign<Matrix4x4<V>> for Matrix4x4<V>
            where V: Number {
            fn mul_assign(&mut self, rhs: Matrix4x4<V>) {
                *self = ::std::ops::Mul::<Matrix4x4<V>>::mul(*self, rhs);
            }
        }
        // ====================================================================
        impl <V> ::std::ops::Mul<$vector<V>> for Matrix4x4<V>
            where V: Number {
            type Output = $vector<V>;
            fn mul(self, rhs: $vector<V>) -> Self::Output { $vector::from([
                (self[0][0] * rhs[0] + self[1][0] * rhs[1] +
                 self[2][0] * rhs[2] + self[3][0] * rhs[3]),
                (self[0][1] * rhs[0] + self[1][1] * rhs[1] +
                 self[2][1] * rhs[2] + self[3][1] * rhs[3]),
                (self[0][2] * rhs[0] + self[1][2] * rhs[1] +
                 self[2][2] * rhs[2] + self[3][2] * rhs[3]),
                (self[0][3] * rhs[0] + self[1][3] * rhs[1] +
                 self[2][3] * rhs[2] + self[3][3] * rhs[3]),
            ]) }
        }
    };
    ($name:ident($vector:ident; $i:expr))       => {
    };
}
// ============================================================================
/// matrix_define_inner!
macro_rules! matrix_define_inner {
    (Matrix4x4($vector:ident; $i:expr)) => {
        // ====================================================================
        /// mul
        pub fn mul(lhs: &Matrix4x4< V, >, rhs: &Matrix4x4< V, >)
                   -> Matrix4x4< V, > { Matrix4x4::from([
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
            ]) ])
        }
        // ====================================================================
        /// tra
        pub fn tra(tx: V, ty: V, tz: V) -> Matrix4x4< V, > { Matrix4x4::from([
            $vector::from_no_clean([
                V::one(), V::zero(), V::zero(), V::zero(),
            ]),
            $vector::from_no_clean([
                V::zero(), V::one(), V::zero(), V::zero(),
            ]),
            $vector::from_no_clean([
                V::zero(), V::zero(), V::one(), V::zero(),
            ]),
            $vector::from_no_clean([ tx, ty, tz, V::one(), ]),
        ]) }
        // ====================================================================
        /// sca
        pub fn sca(sx: V, sy: V, sz: V) -> Matrix4x4< V, > { Matrix4x4::from([
            $vector::from_no_clean([
                sx,        V::zero(), V::zero(), V::zero(),
            ]),
            $vector::from_no_clean([
                V::zero(), sy,        V::zero(), V::zero(),
            ]),
            $vector::from_no_clean([
                V::zero(), V::zero(), sz,        V::zero(),
            ]),
            $vector::from_no_clean([
                V::zero(), V::zero(), V::zero(), V::one(),
            ]),
        ]) }
        // ====================================================================
        /// trarotsca
        pub fn trarotsca(tx: V, ty: V, tz: V,
                         rx: V, ry: V, rz: V, rw: V,
                         sx: V, sy: V, sz: V,) -> Matrix4x4< V, > {
            Matrix4x4::from([
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
        // ====================================================================
        /// frustum
        pub fn frustum(near: V, far: V, focus: V, aspect: V)
                       -> Matrix4x4< V, > {
            let c = V::one() / (near - far);
            Matrix4x4::from([
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
                    V::from(2).unwrap() * near * far * c,         V::zero()
                ]),
            ])
        }
        // ====================================================================
        /// ortho
        pub fn ortho(near: V, far: V, width: V, height: V) -> Matrix4x4< V, > {
            let c = V::one() / (near - far);
            Matrix4x4::from([
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
    ($name:ident($vector:ident; $i:expr))       => {
    };
}
// ============================================================================
matrix_define!(Matrix2x2(Vector2; 2));
matrix_define!(Matrix2x3(Vector3; 2));
matrix_define!(Matrix3x2(Vector2; 3));
matrix_define!(Matrix3x3(Vector3; 3));
matrix_define!(Matrix3x4(Vector4; 3));
matrix_define!(Matrix4x3(Vector3; 4));
matrix_define!(Matrix4x4(Vector4; 4));
