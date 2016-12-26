// -*- mode:rust; coding:utf-8-unix; -*-

//! vector.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/19
//  @date 2016/12/22

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use super::{ Number, Cleanup, };
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// vector_define!
macro_rules! vector_define {
    ($name:ident, $number:expr)    => {
        // ////////////////////////////////////////////////////////////////////
        // ====================================================================
        /// struct $name
        #[derive( Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, )]
        pub struct $name<V: Number>([V; $number]);
        // ====================================================================
        impl <V> From< [V; $number] > for $name<V>
            where V: Number {
            fn from(inner: [V; $number]) -> Self {
                let mut m = $name(inner);
                m.cleanup();
                m
            }
        }
        // ====================================================================
        impl <V> From< Vec<V> > for $name<V>
            where V: Number {
            fn from(inner: Vec<V>) -> Self {
                assert_eq!($number, inner.len(), "{}({})", file!(), line!());
                let mut v = [V::zero(); $number];
                for i in 0 .. $number { v[i] = inner[i]; }
                Self::from(v)
            }
        }
        // ====================================================================
        impl <V> ::std::ops::Index< usize, > for $name<V>
            where V: Number {
            type Output         = V;
            fn index(&self, index: usize) -> &Self::Output {
                let &$name(ref inner) = self;
                &inner[index]
            }
        }
        // --------------------------------------------------------------------
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
                let mut inner = [V::default(); $number];
                for i in 0 .. $number { inner[i] = self[i] + rhs; }
                Self::from(inner)
            }
        }
        // --------------------------------------------------------------------
        impl <V> ::std::ops::AddAssign<V> for $name<V>
            where V: Number {
            fn add_assign(&mut self, rhs: V) {
                let &mut $name(ref mut inner) = self;
                for i in 0 .. $number { inner[i] += rhs; }
            }
        }
        // ====================================================================
        impl <V> ::std::ops::Sub<V> for $name<V>
            where V: Number {
            type Output = Self;
            fn sub(self, rhs: V) -> Self::Output {
                let mut inner = [V::default(); $number];
                for i in 0 .. $number { inner[i] = self[i] - rhs; }
                Self::from(inner)
            }
        }
        // --------------------------------------------------------------------
        impl <V> ::std::ops::SubAssign<V> for $name<V>
            where V: Number {
            fn sub_assign(&mut self, rhs: V) {
                let &mut $name(ref mut inner) = self;
                for i in 0 .. $number { inner[i] -= rhs; }
            }
        }
        // ====================================================================
        impl <V> ::std::ops::Mul<V> for $name<V>
            where V: Number {
            type Output = Self;
            fn mul(self, rhs: V) -> Self::Output {
                let mut inner = [V::default(); $number];
                for i in 0 .. $number { inner[i] = self[i] * rhs; }
                Self::from(inner)
            }
        }
        // --------------------------------------------------------------------
        impl <V> ::std::ops::MulAssign<V> for $name<V>
            where V: Number {
            fn mul_assign(&mut self, rhs: V) {
                let &mut $name(ref mut inner) = self;
                for i in 0 .. $number { inner[i] *= rhs; }
            }
        }
        // ====================================================================
        impl <V> ::std::ops::Div<V> for $name<V>
            where V: Number {
            type Output = Self;
            fn div(self, rhs: V) -> Self::Output {
                let mut inner = [V::default(); $number];
                for i in 0 .. $number { inner[i] = self[i] / rhs; }
                Self::from(inner)
            }
        }
        // --------------------------------------------------------------------
        impl <V> ::std::ops::DivAssign<V> for $name<V>
            where V: Number {
            fn div_assign(&mut self, rhs: V) {
                let &mut $name(ref mut inner) = self;
                for i in 0 .. $number { inner[i] /= rhs; }
            }
        }
        // ====================================================================
        impl <V> ::std::ops::Add for $name<V>
            where V: Number {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                let mut inner = [V::default(); $number];
                for i in 0 .. $number { inner[i] = self[i] + rhs[i]; }
                Self::from(inner)
            }
        }
        // --------------------------------------------------------------------
        impl <V> ::std::ops::AddAssign for $name<V>
            where V: Number {
            fn add_assign(&mut self, rhs: Self) {
                let &mut $name(ref mut inner) = self;
                for i in 0 .. $number { inner[i] += rhs[i]; }
            }
        }
        // ====================================================================
        impl <V> ::std::ops::Sub for $name<V>
            where V: Number {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                let mut inner = [V::default(); $number];
                for i in 0 .. $number { inner[i] = self[i] - rhs[i]; }
                Self::from(inner)
            }
        }
        // --------------------------------------------------------------------
        impl <V> ::std::ops::SubAssign for $name<V>
            where V: Number {
            fn sub_assign(&mut self, rhs: Self) {
                let &mut $name(ref mut inner) = self;
                for i in 0 .. $number { inner[i] -= rhs[i]; }
            }
        }
        // ====================================================================
        impl <V> ::std::ops::Mul for $name<V>
            where V: Number {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                let mut inner = [V::default(); $number];
                for i in 0 .. $number { inner[i] = self[i] * rhs[i]; }
                Self::from(inner)
            }
        }
        // --------------------------------------------------------------------
        impl <V> ::std::ops::MulAssign for $name<V>
            where V: Number {
            fn mul_assign(&mut self, rhs: Self) {
                let &mut $name(ref mut inner) = self;
                for i in 0 .. $number { inner[i] *= rhs[i]; }
            }
        }
        // ====================================================================
        impl <V> ::std::ops::Div for $name<V>
            where V: Number {
            type Output = Self;
            fn div(self, rhs: Self) -> Self::Output {
                let mut inner = [V::default(); $number];
                for i in 0 .. $number { inner[i] = self[i] / rhs[i]; }
                Self::from(inner)
            }
        }
        // --------------------------------------------------------------------
        impl <V> ::std::ops::DivAssign for $name<V>
            where V: Number {
            fn div_assign(&mut self, rhs: Self) {
                let &mut $name(ref mut inner) = self;
                for i in 0 .. $number { inner[i] /= rhs[i]; }
            }
        }
        // ====================================================================
        vector_define_impl!($name);
        // ====================================================================
        impl <V> $name< V, >
            where V:    Number {
            // ================================================================
            /// from_no_clean
            pub fn from_no_clean(inner: [V; $number]) -> Self { $name(inner) }
            // ================================================================
            /// size
            pub fn size() -> usize { $number }
            // ================================================================
            /// as_ptr
            pub fn as_ptr(&self) -> *const V {
                let &$name(ref inner) = self;
                inner as *const _ as *const V
            }
            // ================================================================
            /// as_mut_ptr
            pub fn as_mut_ptr(&mut self) -> *mut V {
                let &mut $name(ref mut inner) = self;
                inner as *mut _ as *mut V
            }
            // ================================================================
            /// cleanup
            pub fn cleanup(&mut self) {
                let mut c = Cleanup::new();
                for i in 0 .. $number { c.collect(self[i]); }
                for i in 0 .. $number { self[i] = c.check(self[i]); }
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
            pub fn dot(&self, r: &Self) -> V {
                let mut ret = V::zero();
                for i in 0 .. $number { ret += self[i] * r[i] }
                ret
            }
            // ================================================================
            /// length2
            pub fn length2(&self) -> V { self.dot(self) }
            // ----------------------------------------------------------------
            /// length
            pub fn length(&self) -> V { self.length2().sqrt() }
            // ================================================================
            /// normalize
            pub fn normalize(&mut self) {
                let l = self.length();
                let &mut $name(ref mut inner) = self;
                for i in 0 .. $number { inner[i] /= l; }
            }
            // ================================================================
            vector_define_inner!($name);
        }
    };
}
// ============================================================================
/// vector_define_impl!
macro_rules! vector_define_impl {
    (Vector2)                   => {
    };
    (Vector3)                   => {
        /*
        // ====================================================================
        /// # Examples
        ///
        /// ```
        /// use ::sif::math::{ Vector2, Vector3, };
        ///
        /// let v3 = Vector3::from([0.0f32, 1.0, 4.0]);
        /// {
        ///   let v2_ref: &Vector2<f32> = v3.as_ref();
        ///   assert_eq!(v3[0], v2_ref[0]);
        ///   assert_eq!(v3[1], v2_ref[1]);
        /// }
        /// ```
        impl <V: Number> AsRef<Vector2<V>> for Vector3<V> {
            fn as_ref(&self) -> &Vector2<V> { unsafe {
                ::std::mem::transmute(self)
            } }
        }
        // --------------------------------------------------------------------
        /// # Examples
        ///
        /// ```
        /// use ::sif::math::{ Vector2, Vector3, };
        ///
        /// let mut v3 = Vector3::from([0.0f32, 1.0, 4.0]);
        /// {
        ///   let v2_mut: &mut Vector2<f32> = v3.as_mut();
        ///   v2_mut[0] = 1.0; v2_mut[1] = 2.0;
        /// }
        /// assert_eq!(v3[0], 1.0);
        /// assert_eq!(v3[1], 2.0);
        /// ```
        impl <V: Number> AsMut<Vector2<V>> for Vector3<V> {
            fn as_mut(&mut self) -> &mut Vector2<V> { unsafe {
                ::std::mem::transmute(self)
            } }
        }
         */
    };
    (Vector4)                   => {
        /*
        // ====================================================================
        /// # Examples
        ///
        /// ```
        /// use ::sif::math::{ Vector2, Vector4, };
        ///
        /// let v4 = Vector4::from([0.0f32, 1.0, 4.0, 0.0]);
        /// {
        ///   let v2_ref: &Vector2<f32> = v4.as_ref();
        ///   assert_eq!(v4[0], v2_ref[0]);
        ///   assert_eq!(v4[1], v2_ref[1]);
        /// }
        /// ```
        impl <V: Number> AsRef<Vector2<V>> for Vector4<V> {
            fn as_ref(&self) -> &Vector2<V> { unsafe {
                ::std::mem::transmute(self)
            } }
        }
        // --------------------------------------------------------------------
        /// # Examples
        ///
        /// ```
        /// use ::sif::math::{ Vector2, Vector4, };
        ///
        /// let mut v4 = Vector4::from([0.0f32, 1.0, 4.0, 0.0]);
        /// {
        ///   let v2_mut: &mut Vector2<f32> = v4.as_mut();
        ///   v2_mut[0] = 1.0; v2_mut[1] = 2.0;
        /// }
        /// assert_eq!(v4[0], 1.0);
        /// assert_eq!(v4[1], 2.0);
        /// ```
        impl <V: Number> AsMut<Vector2<V>> for Vector4<V> {
            fn as_mut(&mut self) -> &mut Vector2<V> { unsafe {
                ::std::mem::transmute(self)
            } }
        }
         */
        /*
        // ====================================================================
        /// # Examples
        ///
        /// ```
        /// use ::sif::math::{ Vector3, Vector4, };
        ///
        /// let v4 = Vector4::from([0.0f32, 1.0, 4.0, 0.0]);
        /// {
        ///   let v3_ref: &Vector3<f32> = v4.as_ref();
        ///   assert_eq!(v4[0], v3_ref[0]);
        ///   assert_eq!(v4[1], v3_ref[1]);
        ///   assert_eq!(v4[2], v3_ref[2]);
        /// }
        /// ```
        impl <V: Number> AsRef<Vector3<V>> for Vector4<V> {
            fn as_ref(&self) -> &Vector3<V> { unsafe {
                ::std::mem::transmute(self)
            } }
        }
        // --------------------------------------------------------------------
        /// # Examples
        ///
        /// ```
        /// use ::sif::math::{ Vector3, Vector4, };
        ///
        /// let mut v4 = Vector4::from([0.0f32, 1.0, 4.0, 0.0]);
        /// {
        ///   let v3_mut: &mut Vector3<f32> = v4.as_mut();
        ///   v3_mut[0] = 1.0; v3_mut[1] = 2.0; v3_mut[2] = 8.0;
        /// }
        /// assert_eq!(v4[0], 1.0);
        /// assert_eq!(v4[1], 2.0);
        /// assert_eq!(v4[2], 8.0);
        /// ```
        impl <V: Number> AsMut<Vector3<V>> for Vector4<V> {
            fn as_mut(&mut self) -> &mut Vector3<V> { unsafe {
                ::std::mem::transmute(self)
            } }
        }
        */
    };
    ($name: ident)              => {
    };
}
// ============================================================================
/// vector_define_inner!
macro_rules! vector_define_inner {
    (Vector2)                   => {
        // ====================================================================
        /// new
        pub fn new(x: V, y: V) -> Self { Vector2::<V>([x, y]) }
    };
    (Vector3)                   => {
        // ====================================================================
        /// new
        pub fn new(x: V, y: V, z: V) -> Self { Vector3::<V>([x, y, z]) }
        // ====================================================================
        /// from_vector2
        pub fn from_vector2(src: &Vector2<V>, z: V) -> Self {
            Vector3::from([ src[0], src[1], z, ])
        }
        // ====================================================================
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
            Vector3::from([ self[1] * r[2] - self[2] * r[1],
                            self[2] * r[0] - self[0] * r[2],
                            self[0] * r[1] - self[1] * r[0] ])
        }
    };
    (Vector4)                   => {
        // ====================================================================
        /// new
        pub fn new(x: V, y: V, z: V, w: V) -> Self {
            Vector4::<V>([x, y, z, w])
        }
        // ====================================================================
        /// from_vector2
        pub fn from_vector2(src: &Vector2<V>, z: V, w: V) -> Self {
            Vector4::from([ src[0], src[1], z, w, ])
        }
        // ====================================================================
        /// from_vector3
        pub fn from_vector3(src: &Vector3<V>, w: V) -> Self {
            Vector4::from([ src[0], src[1], src[2], w, ])
        }
    };
    ($name: ident)              => {
    };
}
// ============================================================================
vector_define!(Vector2, 2);
vector_define!(Vector3, 3);
vector_define!(Vector4, 4);
