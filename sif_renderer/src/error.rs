// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/05/09
//  @date 2019/07/09

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::{error::Error as StdError, fmt::Debug};
// ----------------------------------------------------------------------------
use super::GLError;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Error
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Error {
    /// OptNone
    OptNone(String),
    /// Renderer
    Renderer(String),
    /// Utf8
    Utf8(String),
    /// FromUtf8
    FromUtf8(String),
    /// FFINul
    FFINul(String),
    /// Image
    Image(String),
    /// GL
    GL(String),
}
// ============================================================================
impl From<::std::str::Utf8Error> for Error {
    fn from(e: ::std::str::Utf8Error) -> Self {
        Error::Utf8(format!("{}", e))
    }
}
// ----------------------------------------------------------------------------
impl From<::std::string::FromUtf8Error> for Error {
    fn from(e: ::std::string::FromUtf8Error) -> Self {
        Error::FromUtf8(format!("{}", e))
    }
}
// ----------------------------------------------------------------------------
impl From<::std::ffi::NulError> for Error {
    fn from(e: ::std::ffi::NulError) -> Self {
        Error::FFINul(format!("{}", e))
    }
}
// ----------------------------------------------------------------------------
impl From<::image::ImageError> for Error {
    fn from(e: ::image::ImageError) -> Self {
        Error::Image(format!("{}", e))
    }
}
// ----------------------------------------------------------------------------
impl<R, E> From<GLError<R, E>> for Error
where
    R: Debug + Clone + PartialOrd + PartialEq,
    E: Debug + Clone + PartialOrd + PartialEq,
{
    fn from(e: GLError<R, E>) -> Self {
        Error::GL(format!("{}", e))
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
    fn description(&self) -> &str {
        match *self {
            Error::OptNone(_) => "sif::renderer::Error::OptNone",
            Error::Renderer(_) => "sif::renderer::Error::Renderer",
            Error::Utf8(_) => "sif::renderer::Error::Utf8",
            Error::FromUtf8(_) => "sif::renderer::Error::FromUtf8",
            Error::FFINul(_) => "sif::renderer::Error::FFINul",
            Error::Image(_) => "sif::renderer::Error::Image",
            Error::GL(_) => "sif::renderer::Error::GL",
        }
    }
    // ========================================================================
    fn cause(&self) -> Option<&dyn StdError> {
        match *self {
            Error::OptNone(_) => None,
            Error::Renderer(_) => None,
            Error::Utf8(_) => None,
            Error::FromUtf8(_) => None,
            Error::FFINul(_) => None,
            Error::Image(_) => None,
            Error::GL(_) => None,
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// type Result
pub type Result<T> = ::std::result::Result<T, Error>;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
#[cfg(test)]
mod tests {
    // use  ===================================================================
    use super::{Error, Result};
    // ========================================================================
    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Error>();
        assert_send::<Result<()>>();
    }
    // ------------------------------------------------------------------------
    #[test]
    fn test_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Error>();
        assert_sync::<Result<()>>();
    }
}
