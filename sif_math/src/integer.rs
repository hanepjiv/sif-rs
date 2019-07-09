// -*- mode:rust; coding:utf-8-unix; -*-

//! integer.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/08
//  @date 2018/08/27

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// trait Integer
pub trait Integer:
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
    + ::num::cast::FromPrimitive
    + ::num::cast::NumCast
    + ::num::cast::ToPrimitive
    + ::num::traits::Zero
    + ::num::traits::One
    + ::num::Integer
{
}
// ============================================================================
impl<T> Integer for T where
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
        + ::num::cast::ToPrimitive
        + ::num::cast::FromPrimitive
        + ::num::cast::NumCast
        + ::num::traits::Zero
        + ::num::traits::One
        + ::num::Integer
{
}
