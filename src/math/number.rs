// -*- mode:rust; coding:utf-8-unix; -*-

//! number.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/08
//  @date 2017/03/15

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// trait Number
pub trait Number
where
    Self: ::std::fmt::Debug
        + ::std::default::Default
        + ::std::ops::Neg
        + ::std::ops::Add
        + ::std::ops::AddAssign
        + ::std::ops::Sub
        + ::std::ops::SubAssign
        + ::std::ops::Mul
        + ::std::ops::MulAssign
        + ::std::ops::Div
        + ::std::ops::DivAssign
        + ::num::Float,
{
}
// ============================================================================
impl<T> Number for T
where
    T: ::std::fmt::Debug
        + ::std::default::Default
        + ::std::ops::Neg
        + ::std::ops::Add
        + ::std::ops::AddAssign
        + ::std::ops::Sub
        + ::std::ops::SubAssign
        + ::std::ops::Mul
        + ::std::ops::MulAssign
        + ::std::ops::Div
        + ::std::ops::DivAssign
        + ::num::Float,
{
}
