// -*- mode:rust; coding:utf-8-unix; -*-

//! matrix.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/19
//  @date 2018/04/11

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use super::{Cleanup, Error, Number, Result, Vector2, Vector3, Vector4};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// matrix_define!
macro_rules! matrix_define {
    ($name:ident($vector:ident; $n:expr)) => {
        // ====================================================================
        /// struct $name
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
        pub struct $name<V: Number>([$vector<V>; $n]);
        // ====================================================================
        impl<V> From<[$vector<V>; $n]> for $name<V>
        where
            V: Number,
        {
            fn from(inner: [$vector<V>; $n]) -> Self {
                let mut m = $name(inner);
                *m.cleanup()
            }
        }
        // ====================================================================
        impl<V> ::std::ops::Index<usize> for $name<V>
        where
            V: Number,
        {
            type Output = $vector<V>;
            fn index(&self, index: usize) -> &Self::Output {
                &self.0[index]
            }
        }
        // ====================================================================
        impl<V> ::std::ops::IndexMut<usize> for $name<V>
        where
            V: Number,
        {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.0[index]
            }
        }
        // ====================================================================
        impl<V> ::std::ops::Add<V> for $name<V>
        where
            V: Number,
        {
            type Output = Self;
            fn add(self, rhs: V) -> Self::Output {
                let mut inner = [$vector::<V>::default(); $n];
                for i in 0..$n {
                    inner[i] = self[i] + rhs;
                }
                Self::from(inner)
            }
        }
        // --------------------------------------------------------------------
        impl<V> ::std::ops::AddAssign<V> for $name<V>
        where
            V: Number,
        {
            fn add_assign(&mut self, rhs: V) {
                for i in 0..$n {
                    self.0[i] += rhs;
                }
            }
        }
        // ====================================================================
        impl<V> ::std::ops::Sub<V> for $name<V>
        where
            V: Number,
        {
            type Output = Self;
            fn sub(self, rhs: V) -> Self::Output {
                let mut inner = [$vector::<V>::default(); $n];
                for i in 0..$n {
                    inner[i] = self[i] - rhs;
                }
                Self::from(inner)
            }
        }
        // --------------------------------------------------------------------
        impl<V> ::std::ops::SubAssign<V> for $name<V>
        where
            V: Number,
        {
            fn sub_assign(&mut self, rhs: V) {
                for i in 0..$n {
                    self.0[i] -= rhs;
                }
            }
        }
        // ====================================================================
        impl<V> ::std::ops::Mul<V> for $name<V>
        where
            V: Number,
        {
            type Output = Self;
            fn mul(self, rhs: V) -> Self::Output {
                let mut inner = [$vector::<V>::default(); $n];
                for i in 0..$n {
                    inner[i] = self[i] * rhs;
                }
                Self::from(inner)
            }
        }
        // --------------------------------------------------------------------
        impl<V> ::std::ops::MulAssign<V> for $name<V>
        where
            V: Number,
        {
            fn mul_assign(&mut self, rhs: V) {
                for i in 0..$n {
                    self.0[i] *= rhs;
                }
            }
        }
        // ====================================================================
        impl<V> ::std::ops::Div<V> for $name<V>
        where
            V: Number,
        {
            type Output = Self;
            fn div(self, rhs: V) -> Self::Output {
                let mut inner = [$vector::<V>::default(); $n];
                for i in 0..$n {
                    inner[i] = self[i] / rhs;
                }
                Self::from(inner)
            }
        }
        // --------------------------------------------------------------------
        impl<V> ::std::ops::DivAssign<V> for $name<V>
        where
            V: Number,
        {
            fn div_assign(&mut self, rhs: V) {
                for i in 0..$n {
                    self.0[i] /= rhs;
                }
            }
        }
        // ====================================================================
        impl<V> ::std::ops::Mul<$name<V>> for $name<V>
        where
            V: Number,
        {
            type Output = Self;
            fn mul(self, rhs: $name<V>) -> Self::Output {
                let mut r = Self::from_no_clean([$vector::<V>::default(); $n]);
                for i in 0..$n {
                    for j in 0..$vector::<V>::size() {
                        for k in 0..$vector::<V>::size() {
                            r[i][j] += self[k][j] * rhs[i][k];
                        }
                    }
                }
                *r.cleanup()
            }
        }
        // --------------------------------------------------------------------
        impl<V> ::std::ops::MulAssign<$name<V>> for $name<V>
        where
            V: Number,
        {
            fn mul_assign(&mut self, rhs: $name<V>) {
                *self = *self * rhs;
            }
        }
        // ====================================================================
        impl<V> $name<V>
        where
            V: Number,
        {
            // ================================================================
            /// size
            pub fn size() -> usize {
                $n
            }
            // ================================================================
            /// from_no_clean
            pub fn from_no_clean(inner: [$vector<V>; $n]) -> Self {
                $name(inner)
            }
            // ================================================================
            /// new_identity
            pub fn new_identity() -> Self {
                debug_assert!($n == $vector::<V>::size());
                let mut ret = Self::from_no_clean([$vector::default(); $n]);
                for i in 0..$n {
                    ret[i][i] = V::one();
                }
                ret
            }
            // ================================================================
            /// as_ptr
            pub fn as_ptr(&self) -> *const V {
                self.0[0].as_ptr()
            }
            // ================================================================
            /// as_mut_ptr
            pub fn as_mut_ptr(&mut self) -> *mut V {
                self.0[0].as_mut_ptr()
            }
            // ================================================================
            /// cleanup
            pub fn cleanup(&mut self) -> &mut Self {
                let mut c = Cleanup::new();
                for i in 0..$n {
                    for j in 0..$vector::<V>::size() {
                        c.collect(self[i][j]);
                    }
                }
                for i in 0..$n {
                    for j in 0..$vector::<V>::size() {
                        self[i][j] = c.check(self[i][j]);
                    }
                }
                self
            }
            // ================================================================
            /// transpose
            pub fn transpose(&mut self) -> &mut Self {
                debug_assert!($n == $vector::<V>::size());
                for i in 0..$n {
                    for j in i + 1..$vector::<V>::size() {
                        let t = self[i][j];
                        self[i][j] = self[j][i];
                        self[j][i] = t;
                    }
                }
                self
            }
            // ================================================================
            fn pivot(&self, o: usize) -> usize {
                let mut d = o;
                let mut max = self[o][o].abs();
                for i in o + 1..$n {
                    if max < self[i][o].abs() {
                        d = i;
                        max = self[i][o].abs();
                    }
                }
                d
            }
            // ================================================================
            /// apply_order
            pub fn apply_order(&mut self, order: &[usize; $n]) -> &mut Self {
                debug_assert!($n == $vector::<V>::size());
                let tmp = self.clone();
                for (i, o) in order.iter().enumerate() {
                    if i == *o { continue; }
                    self[i] = tmp[*o];
                }
                self
            }
            // ================================================================
            /// new_inverse
            pub fn new_inverse(&self) -> Result<Self> {
                debug_assert!($n == $vector::<V>::size());

                let mut r = Self::new_identity();
                let mut m = *self.clone().cleanup();

                for o in 0..$n {
                    let d = m.pivot(o);
                    if o != d {
                        {
                            let t = r[o];
                            r[o] = r[d];
                            r[d] = t;
                        }
                        {
                            let t = m[o];
                            m[o] = m[d];
                            m[d] = t;
                        }
                    }

                    if m[o][o].abs() < V::epsilon().sqrt() {
                        return Err(Error::InvalidArguments(String::from(
                            "::sif::math::matrix::new_inverse",
                        )));
                    }

                    r[o] /= m[o][o];
                    m[o] /= m[o][o];

                    for i in 0..o {
                        r[i] -= r[o] * m[i][o];
                        m[i] -= m[o] * m[i][o];
                    }
                    for i in o + 1..$n {
                        r[i] -= r[o] * m[i][o];
                        m[i] -= m[o] * m[i][o];
                    }
                }
                Ok(r)
            }
            // ================================================================
            /// new_decomposition
            pub fn new_decomposition(&self) -> Result<(Self, [usize; $n])> {
                if $n < 2 {
                    return Err(Error::InvalidArguments(String::from(
                        "::sif::math::matrix::new_decomposition: n < 2",
                    )));
                }

                let mut ord = [0usize; $n];
                for i in 0..$n {
                    ord[i] = i;
                }
                let mut m = *self.clone().cleanup();

                for o in 0..$n {
                    let d = m.pivot(o);
                    if o != d {
                        {
                            let t = ord[o];
                            ord[o] = ord[d];
                            ord[d] = t;
                        }
                        {
                            let t = m[o];
                            m[o] = m[d];
                            m[d] = t;
                        }
                    }

                    if m[o][o].abs() < V::epsilon().sqrt() {
                        continue;
                    }

                    for j in o + 1..$vector::<V>::size() {
                        m[o][j] /= m[o][o];
                        for i in o + 1..$n {
                            m[i][j] -= m[o][j] * m[i][o];
                        }
                    }
                }
                Ok((*m.cleanup(), ord))
            }
            // ----------------------------------------------------------------
            /// new_lower       *** row_order ***
            pub fn new_lower(&self) -> Self {
                let mut ret = Self::from_no_clean([$vector::default(); $n]);
                for i in 0..$n {
                    ret[i][i] = V::one();
                    for j in i + 1..$n {
                        ret[i][j] = self[i][j];
                    }
                }
                ret
            }
            // ----------------------------------------------------------------
            /// new_upper       *** row_order ***
            pub fn new_upper(&self) -> Self {
                let mut ret = Self::from_no_clean([$vector::default(); $n]);
                for i in 0..$n {
                    for j in 0..i + 1 {
                        ret[i][j] = self[i][j];
                    }
                }
                ret
            }
        }
    };
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
matrix_define!(Matrix2x2(Vector2; 2));
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
matrix_define!(Matrix2x3(Vector3; 2));
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
matrix_define!(Matrix3x2(Vector2; 3));
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
matrix_define!(Matrix3x3(Vector3; 3));
// ============================================================================
impl<V> Default for Matrix3x3<V>
where
    V: Number,
{
    fn default() -> Self {
        Matrix3x3::from_no_clean([
            Vector3::<V>::from_no_clean([V::one(), V::zero(), V::zero()]),
            Vector3::<V>::from_no_clean([V::zero(), V::one(), V::zero()]),
            Vector3::<V>::from_no_clean([V::zero(), V::zero(), V::one()]),
        ])
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
matrix_define!(Matrix3x4(Vector4; 3));
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
matrix_define!(Matrix4x3(Vector3; 4));
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
matrix_define!(Matrix4x4(Vector4; 4));
// ============================================================================
impl<V> Default for Matrix4x4<V>
where
    V: Number,
{
    fn default() -> Self {
        Matrix4x4::from_no_clean([
            Vector4::<V>::from_no_clean([
                V::one(),
                V::zero(),
                V::zero(),
                V::zero(),
            ]),
            Vector4::<V>::from_no_clean([
                V::zero(),
                V::one(),
                V::zero(),
                V::zero(),
            ]),
            Vector4::<V>::from_no_clean([
                V::zero(),
                V::zero(),
                V::one(),
                V::zero(),
            ]),
            Vector4::<V>::from_no_clean([
                V::zero(),
                V::zero(),
                V::zero(),
                V::one(),
            ]),
        ])
    }
}
// ============================================================================
impl<V> ::std::ops::Mul<Vector4<V>> for Matrix4x4<V>
where
    V: Number,
{
    type Output = Vector4<V>;
    fn mul(self, rhs: Vector4<V>) -> Self::Output {
        Vector4::<V>::from([
            (self[0][0] * rhs[0] + self[1][0] * rhs[1] + self[2][0] * rhs[2]
                + self[3][0] * rhs[3]),
            (self[0][1] * rhs[0] + self[1][1] * rhs[1] + self[2][1] * rhs[2]
                + self[3][1] * rhs[3]),
            (self[0][2] * rhs[0] + self[1][2] * rhs[1] + self[2][2] * rhs[2]
                + self[3][2] * rhs[3]),
            (self[0][3] * rhs[0] + self[1][3] * rhs[1] + self[2][3] * rhs[2]
                + self[3][3] * rhs[3]),
        ])
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
#[cfg(test)]
mod tests {
    // use  ===================================================================
    use super::*;
    // ========================================================================
    #[test]
    fn inverse() {
        let m = Matrix4x4::from([
            Vector4::from_no_clean([1.0f32, 1.0, 1.0, -1.0]),
            Vector4::from_no_clean([1.0, 1.0, -1.0, 1.0]),
            Vector4::from_no_clean([1.0, -1.0, 1.0, 1.0]),
            Vector4::from_no_clean([-1.0, 1.0, 1.0, 1.0]),
        ]);
        let im = m.new_inverse().unwrap();
        assert_eq!(Matrix4x4::<f32>::new_identity(), m * im);
        assert_eq!(Matrix4x4::<f32>::new_identity(), im * m);
    }
    // ========================================================================
    #[test]
    fn decomposition() {
        let m = Matrix4x4::from([
            Vector4::from_no_clean([10.0f32, 0.0, 5.0, 0.0]),
            Vector4::from_no_clean([2.1, 1.0, 0.0, 0.0]),
            Vector4::from_no_clean([3.0, 2.0, 1.0, 0.0]),
            Vector4::from_no_clean([0.0, 1.0, 0.0, 1.0]),
        ]);
        let (lu, ord) = m.new_decomposition().unwrap();
        let l = lu.new_lower();
        let u = lu.new_upper();
        assert_eq!(m, *(l * u).apply_order(&ord));
    }
}
