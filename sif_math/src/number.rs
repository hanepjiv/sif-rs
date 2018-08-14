// -*- mode:rust; coding:utf-8-unix; -*-

//! number.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/08
//  @date 2018/08/11

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// trait Number
pub trait Number:
    ::std::fmt::Debug
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
    + ::num::Float
    + ::num::cast::FromPrimitive
    + ::num::cast::NumCast
    + ::num::cast::ToPrimitive
    + ::num::traits::Zero
    + ::num::traits::One
{
}
// ============================================================================
impl<T> Number for T where
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
        + ::num::Float
        + ::num::cast::ToPrimitive
        + ::num::cast::FromPrimitive
        + ::num::cast::NumCast
        + ::num::traits::Zero
        + ::num::traits::One
{}
