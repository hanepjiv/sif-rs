// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/12/10
//  @date 2020/03/19

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::error::Error as StdError;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Error
#[derive(Debug)]
pub enum Error {
    /// OptNone
    OptNone(String),
    /// NoNode
    NoNode,
    /// InvalidPose
    InvalidPose,
    /// IO
    IO(::std::io::Error),
    /// SifManager
    SifManager(::sif_manager::Error),
    /// SifMath
    SifMath(::sif_math::Error),
}
// ============================================================================
impl From<::std::io::Error> for Error {
    fn from(e: ::std::io::Error) -> Self {
        Error::IO(e)
    }
}
// ----------------------------------------------------------------------------
impl From<::sif_manager::Error> for Error {
    fn from(e: ::sif_manager::Error) -> Self {
        Error::SifManager(e)
    }
}
// ----------------------------------------------------------------------------
impl From<::sif_math::Error> for Error {
    fn from(e: ::sif_math::Error) -> Self {
        Error::SifMath(e)
    }
}
// ============================================================================
impl ::std::fmt::Display for Error {
    // ========================================================================
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        <Self as ::std::fmt::Debug>::fmt(self, f)
    }
}
// ============================================================================
impl StdError for Error {
    // ========================================================================
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            Error::OptNone(_) => None,
            Error::NoNode => None,
            Error::InvalidPose => None,
            Error::IO(ref e) => Some(e),
            Error::SifManager(ref e) => Some(e),
            Error::SifMath(ref e) => Some(e),
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// type Result
pub type Result<T> = ::std::result::Result<T, Error>;
