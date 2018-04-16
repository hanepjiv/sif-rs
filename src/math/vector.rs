// -*- mode:rust; coding:utf-8-unix; -*-

//! vector.rs

//  Copyight 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/19
//  @date 2018/04/16

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use super::{Cleanup, Number};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// vector_define!
macro_rules! vector_define {
    ($name:ident, $n:expr) => {
        // ////////////////////////////////////////////////////////////////////
        // ====================================================================
        /// struct $name
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd)]
        pub struct $name<V: Number>([V; $n]);
        // ====================================================================
        impl<V> From<[V; $n]> for $name<V>
        where
            V: Number,
        {
            fn from(src: [V; $n]) -> Self {
                *$name(src).cleanup()
            }
        }
        // ====================================================================
        impl<'a, V> From<&'a [V]> for $name<V>
        where
            V: Number,
        {
            fn from(src: &'a [V]) -> Self {
                let mut inner = <[V; $n]>::default();
                inner.copy_from_slice(src);
                $name::from(inner)
            }
        }
        // ====================================================================
        impl<V> ::std::ops::Index<usize> for $name<V>
        where
            V: Number,
        {
            type Output = V;
            fn index(&self, index: usize) -> &Self::Output {
                &self.0[index]
            }
        }
        // --------------------------------------------------------------------
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
                let mut inner = [V::default(); $n];
                for i in 0..$n {
                    inner[i] = self.0[i] + rhs;
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
                let mut inner = [V::default(); $n];
                for i in 0..$n {
                    inner[i] = self.0[i] - rhs;
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
                let mut inner = [V::default(); $n];
                for i in 0..$n {
                    inner[i] = self.0[i] * rhs;
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
                let mut inner = [V::default(); $n];
                for i in 0..$n {
                    inner[i] = self.0[i] / rhs;
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
        impl<V> ::std::ops::Add for $name<V>
        where
            V: Number,
        {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                let mut inner = [V::default(); $n];
                for i in 0..$n {
                    inner[i] = self.0[i] + rhs.0[i];
                }
                Self::from(inner)
            }
        }
        // --------------------------------------------------------------------
        impl<V> ::std::ops::AddAssign for $name<V>
        where
            V: Number,
        {
            fn add_assign(&mut self, rhs: Self) {
                for i in 0..$n {
                    self.0[i] += rhs.0[i];
                }
            }
        }
        // ====================================================================
        impl<V> ::std::ops::Sub for $name<V>
        where
            V: Number,
        {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                let mut inner = [V::default(); $n];
                for i in 0..$n {
                    inner[i] = self.0[i] - rhs.0[i];
                }
                Self::from(inner)
            }
        }
        // --------------------------------------------------------------------
        impl<V> ::std::ops::SubAssign for $name<V>
        where
            V: Number,
        {
            fn sub_assign(&mut self, rhs: Self) {
                for i in 0..$n {
                    self.0[i] -= rhs.0[i];
                }
            }
        }
        // ====================================================================
        impl<V> ::std::ops::Mul for $name<V>
        where
            V: Number,
        {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                let mut inner = [V::default(); $n];
                for i in 0..$n {
                    inner[i] = self.0[i] * rhs.0[i];
                }
                Self::from(inner)
            }
        }
        // --------------------------------------------------------------------
        impl<V> ::std::ops::MulAssign for $name<V>
        where
            V: Number,
        {
            fn mul_assign(&mut self, rhs: Self) {
                for i in 0..$n {
                    self.0[i] *= rhs.0[i];
                }
            }
        }
        // ====================================================================
        impl<V> ::std::ops::Div for $name<V>
        where
            V: Number,
        {
            type Output = Self;
            fn div(self, rhs: Self) -> Self::Output {
                let mut inner = [V::default(); $n];
                for i in 0..$n {
                    inner[i] = self.0[i] / rhs.0[i];
                }
                Self::from(inner)
            }
        }
        // --------------------------------------------------------------------
        impl<V> ::std::ops::DivAssign for $name<V>
        where
            V: Number,
        {
            fn div_assign(&mut self, rhs: Self) {
                for i in 0..$n {
                    self.0[i] /= rhs.0[i];
                }
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
            pub fn from_no_clean(inner: [V; $n]) -> Self {
                $name(inner)
            }
            // ================================================================
            /// as_ptr
            pub fn as_ptr(&self) -> *const V {
                self.0.as_ptr()
            }
            // ================================================================
            /// as_mut_ptr
            pub fn as_mut_ptr(&mut self) -> *mut V {
                self.0.as_mut_ptr()
            }
            // ================================================================
            /// cleanup
            pub fn cleanup(&mut self) -> &mut Self {
                let mut c = Cleanup::default();
                for i in &self.0 {
                    c.collect(*i);
                }
                for i in &mut self.0 {
                    *i = c.check(*i);
                }
                self
            }
            // ================================================================
            /// dot
            ///
            /// # Examples
            ///
            /// ```
            /// use ::sif::math::Vector3;
            ///
            /// assert_eq!(Vector3::from([1.0f32, 0.0, 0.0]).
            ///            dot(&Vector3::from([0.0f32, 1.0, 0.0])),
            ///            0.0);
            /// ```
            pub fn dot(&self, rhs: &Self) -> V {
                let mut ret = V::zero();
                for i in 0..$n {
                    ret += self.0[i] * rhs.0[i]
                }
                ret
            }
            // ================================================================
            /// length2
            pub fn length2(&self) -> V {
                self.dot(self)
            }
            // ----------------------------------------------------------------
            /// length
            pub fn length(&self) -> V {
                self.length2().sqrt()
            }
            // ================================================================
            /// normalize
            pub fn normalize(&mut self) -> &mut Self {
                let l = self.length();
                if l < V::epsilon().sqrt() {
                    for i in &mut self.0 {
                        *i = V::zero();
                    }
                } else {
                    for i in &mut self.0 {
                        *i /= l;
                    }
                }
                self
            }
        }
    };
}

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
vector_define!(Vector2, 2);
// ============================================================================
impl<V: Number> Vector2<V> {
    // ========================================================================
    /// new
    pub fn new(x: V, y: V) -> Self {
        Vector2::<V>([x, y])
    }
}
// ============================================================================
impl<V: Number> From<Vector3<V>> for Vector2<V> {
    fn from(src: Vector3<V>) -> Self {
        Vector2::new(src[0], src[1])
    }
}
// ============================================================================
impl<V: Number> From<Vector4<V>> for Vector2<V> {
    fn from(src: Vector4<V>) -> Self {
        Vector2::new(src[0], src[1])
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
vector_define!(Vector3, 3);
// ============================================================================
impl<V: Number> Vector3<V> {
    // ========================================================================
    /// new
    pub fn new(x: V, y: V, z: V) -> Self {
        Vector3::<V>([x, y, z])
    }
    // ========================================================================
    /// from_vector2
    pub fn from_vector2(src: &Vector2<V>, z: V) -> Self {
        Vector3::new(src[0], src[1], z)
    }
    // ========================================================================
    /// cross
    ///
    /// # Examples
    ///
    /// ```
    /// use ::sif::math::Vector3;
    ///
    /// assert_eq!(Vector3::from([1.0f32, 0.0, 0.0]).
    ///            cross(&Vector3::from([0.0f32, 1.0, 0.0])),
    ///            Vector3::from([0.0f32, 0.0, 1.0]));
    /// ```
    pub fn cross(&self, r: &Self) -> Self {
        Vector3::new(
            self[1] * r[2] - self[2] * r[1],
            self[2] * r[0] - self[0] * r[2],
            self[0] * r[1] - self[1] * r[0],
        )
    }
}
// ============================================================================
impl<V: Number> From<Vector4<V>> for Vector3<V> {
    fn from(src: Vector4<V>) -> Self {
        Vector3::new(src[0], src[1], src[2])
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
vector_define!(Vector4, 4);
// ============================================================================
impl<V: Number> Vector4<V> {
    // ========================================================================
    /// new
    pub fn new(x: V, y: V, z: V, w: V) -> Self {
        Vector4::<V>([x, y, z, w])
    }
    // ========================================================================
    /// from_vector2
    pub fn from_vector2(src: &Vector2<V>, z: V, w: V) -> Self {
        Vector4::new(src[0], src[1], z, w)
    }
    // ========================================================================
    /// from_vector3
    pub fn from_vector3(src: &Vector3<V>, w: V) -> Self {
        Vector4::new(src[0], src[1], src[2], w)
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
    fn normalize() {
        let mut v0 = Vector4::new(0.0f32, 0.0, 0.0, 0.0);
        assert_eq!(0.0f32, v0.normalize().length());

        let mut v1 = Vector4::new(1.0f32, 1.0, 1.0, 1.0);
        assert_eq!(1.0f32, v1.normalize().length());
    }
}
