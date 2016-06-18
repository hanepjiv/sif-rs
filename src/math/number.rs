/* -*- mode:rust; coding:utf-8-unix; -*- */

//! number.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/08
//  @date 2016/06/18

/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
use super::{ Epsilon, };
/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
/// trait Number
pub trait Number
    where Self: ::std::default::Default + ::std::ops::Neg +
    ::std::ops::Add + ::std::ops::AddAssign +
    ::std::ops::Sub + ::std::ops::SubAssign +
    ::std::ops::Mul + ::std::ops::MulAssign +
    ::std::ops::Div + ::std::ops::DivAssign +
    Epsilon {}
/* ========================================================================== */
impl < T > Number for T
    where T:    ::std::default::Default + ::std::ops::Neg +
    ::std::ops::Add + ::std::ops::AddAssign +
    ::std::ops::Sub + ::std::ops::SubAssign +
    ::std::ops::Mul + ::std::ops::MulAssign +
    ::std::ops::Div + ::std::ops::DivAssign +
    Epsilon {}
