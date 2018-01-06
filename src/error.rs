// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/11/27
//  @date 2017/03/09

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use std::error::Error as StdError;
// ----------------------------------------------------------------------------
use super::renderer::GLError;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Error
#[derive(Debug)]
pub enum Error {
    /// Sif
    Sif(String),
    /// InvalidArguments
    InvalidArguments(String),
    /// GL
    GL(Box<StdError>),
}
// ============================================================================
impl<R, E> From<GLError<R, E>> for Error
where
    R: ::std::fmt::Debug + 'static,
    E: ::std::fmt::Debug + 'static,
{
    fn from(e: GLError<R, E>) -> Self {
        Error::GL(Box::new(e))
    }
}
// ============================================================================
impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
// ============================================================================
impl ::std::error::Error for Error {
    // ========================================================================
    fn description(&self) -> &str {
        match *self {
            Error::Sif(ref x) => x.as_str(),
            Error::InvalidArguments(ref x) => x.as_str(),
            Error::GL(_) => "sif::Error::GL",
        }
    }
    // ========================================================================
    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            Error::Sif(_) => None,
            Error::InvalidArguments(_) => None,
            Error::GL(ref e) => Some(e.as_ref()),
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// type Result
pub type Result<T> = ::std::result::Result<T, Error>;
