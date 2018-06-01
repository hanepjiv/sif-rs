// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/05/09
//  @date 2018/06/01

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use super::GLError;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Error
#[derive(Debug)]
pub enum Error {
    /// OptNone
    OptNone(String),
    /// Renderer
    Renderer(String),
    /// Utf8
    Utf8(::std::str::Utf8Error),
    /// FromUtf8
    FromUtf8(::std::string::FromUtf8Error),
    /// FFINul
    FFINul(::std::ffi::NulError),
    /// Image
    Image(::image::ImageError),
    /// GL
    GL(Box<::std::error::Error>),
}
// ============================================================================
impl From<::std::str::Utf8Error> for Error {
    fn from(e: ::std::str::Utf8Error) -> Self {
        Error::Utf8(e)
    }
}
// ----------------------------------------------------------------------------
impl From<::std::string::FromUtf8Error> for Error {
    fn from(e: ::std::string::FromUtf8Error) -> Self {
        Error::FromUtf8(e)
    }
}
// ----------------------------------------------------------------------------
impl From<::std::ffi::NulError> for Error {
    fn from(e: ::std::ffi::NulError) -> Self {
        Error::FFINul(e)
    }
}
// ----------------------------------------------------------------------------
impl From<::image::ImageError> for Error {
    fn from(e: ::image::ImageError) -> Self {
        Error::Image(e)
    }
}
// ----------------------------------------------------------------------------
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
    // ========================================================================
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ref e @ Error::OptNone(_) | ref e @ Error::Renderer(_) => {
                write!(f, "{:?}", e)
            }
            Error::Utf8(ref e) => e.fmt(f),
            Error::FromUtf8(ref e) => e.fmt(f),
            Error::FFINul(ref e) => e.fmt(f),
            Error::Image(ref e) => e.fmt(f),
            Error::GL(ref e) => e.fmt(f),
        }
    }
}
// ============================================================================
impl ::std::error::Error for Error {
    // ========================================================================
    fn description(&self) -> &str {
        match *self {
            Error::OptNone(_) => "::sif::renderer::Error::OptNone",
            Error::Renderer(_) => "::sif::renderer::Error::Renderer",
            Error::Utf8(ref e) => e.description(),
            Error::FromUtf8(ref e) => e.description(),
            Error::FFINul(ref e) => e.description(),
            Error::Image(ref e) => e.description(),
            Error::GL(ref e) => e.description(),
        }
    }
    // ========================================================================
    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            Error::OptNone(_) => None,
            Error::Renderer(_) => None,
            Error::Utf8(ref e) => Some(e),
            Error::FromUtf8(ref e) => Some(e),
            Error::FFINul(ref e) => Some(e),
            Error::Image(ref e) => Some(e),
            Error::GL(ref e) => Some(e.as_ref()),
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
