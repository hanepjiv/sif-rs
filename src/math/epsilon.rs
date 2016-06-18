/* -*- mode:rust; coding:utf-8-unix; -*- */

//! epsilon.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/04
//  @date 2016/06/18

/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
/// trait Epsilon
pub trait Epsilon
    where Self: ::std::fmt::Debug + ::num::Float {
    /* ====================================================================== */
    /// epsilon
    fn epsilon() -> Self { Self::min_positive_value().sqrt() }
}
/* ========================================================================== */
impl < T > Epsilon for T
    where T:    ::std::fmt::Debug + ::num::Float {
}
/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!((f32::epsilon() as f64) > f64::epsilon());
    }
}
