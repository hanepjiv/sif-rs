// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/11/27
//  @date 2018/06/01

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use std::error::Error as StdError;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Error
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Error {
    /// Sif
    Sif(String),
    /// InvalidArgument
    InvalidArgument(String),
}
// ============================================================================
impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
// ============================================================================
impl StdError for Error {
    // ========================================================================
    fn description(&self) -> &str {
        match *self {
            Error::Sif(ref x) => x.as_str(),
            Error::InvalidArgument(ref x) => x.as_str(),
        }
    }
    // ========================================================================
    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Sif(_) => None,
            Error::InvalidArgument(_) => None,
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// type Result
pub type Result<T> = ::std::result::Result<T, Error>;
