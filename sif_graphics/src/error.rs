// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/12/10
//  @date 2018/06/01

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use uuid::Uuid;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Error
#[derive(Debug)]
pub enum Error {
    /// OptNone
    OptNone(String),
    /// InvalidArg
    InvalidArg(String),
    /// InvalidEnum
    InvalidEnum,
    /// InvalidImage
    InvalidImage,
    /// ManagedNotFound,
    ManagedNotFound(Uuid),
    /// Path
    Path(String),
    /// Mesh
    Mesh(String),
    /// Light
    Light(String),
    /// LBF
    LBF(super::lbf::Error),
    /// IO
    IO(::std::io::Error),
    /// Sif
    Sif(::sif_error::Error),
    /// SifManager
    SifManager(::sif_manager::Error),
    /// SifRenderer
    SifRenderer(::sif_renderer::Error),
    /// SifThree
    SifThree(::sif_three::Error),
    /// GL
    GL(Box<::std::error::Error>),
    /// SDL2TTFFont
    SDL2TTFFont(::sdl2::ttf::FontError),
}
// ============================================================================
impl From<super::lbf::Error> for Error {
    fn from(e: super::lbf::Error) -> Self {
        Error::LBF(e)
    }
}
// ----------------------------------------------------------------------------
impl From<::std::io::Error> for Error {
    fn from(e: ::std::io::Error) -> Self {
        Error::IO(e)
    }
}
// ----------------------------------------------------------------------------
impl From<::sif_error::Error> for Error {
    fn from(e: ::sif_error::Error) -> Self {
        Error::Sif(e)
    }
}
// ----------------------------------------------------------------------------
impl From<::sif_manager::Error> for Error {
    fn from(e: ::sif_manager::Error) -> Self {
        Error::SifManager(e)
    }
}
// ----------------------------------------------------------------------------
impl From<::sif_renderer::Error> for Error {
    fn from(e: ::sif_renderer::Error) -> Self {
        Error::SifRenderer(e)
    }
}
// ----------------------------------------------------------------------------
impl From<::sif_three::Error> for Error {
    fn from(e: ::sif_three::Error) -> Self {
        Error::SifThree(e)
    }
}
// ----------------------------------------------------------------------------
impl<R, E> From<::sif_renderer::GLError<R, E>> for Error
where
    R: ::std::fmt::Debug + 'static,
    E: ::std::fmt::Debug + 'static,
{
    fn from(e: ::sif_renderer::GLError<R, E>) -> Self {
        Error::GL(Box::new(e))
    }
}
// ----------------------------------------------------------------------------
impl From<::sdl2::ttf::FontError> for Error {
    fn from(e: ::sdl2::ttf::FontError) -> Self {
        Error::SDL2TTFFont(e)
    }
}
// ============================================================================
impl ::std::fmt::Display for Error {
    // ========================================================================
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
// ============================================================================
impl ::std::error::Error for Error {
    // ========================================================================
    fn description(&self) -> &str {
        match *self {
            Error::OptNone(_) => "::sif_graphics::Error::OptNone",
            Error::InvalidArg(_) => "::sif_graphics::Error::InvalidArg",
            Error::InvalidEnum => "::sif_graphics::Error::InvalidImage",
            Error::InvalidImage => "::sif_graphics::Error::InvalidImage",
            Error::ManagedNotFound(_) => {
                "::sif_graphics::Error::ManagedNotFound"
            }
            Error::Path(_) => "::sif_graphics::Error::Path",
            Error::Mesh(_) => "::sif_graphics::Error::Mesh",
            Error::Light(_) => "::sif_graphics::Error::Light",
            Error::LBF(ref e) => e.description(),
            Error::IO(ref e) => e.description(),
            Error::Sif(ref e) => e.description(),
            Error::SifManager(ref e) => e.description(),
            Error::SifRenderer(ref e) => e.description(),
            Error::SifThree(ref e) => e.description(),
            Error::GL(ref e) => e.description(),
            Error::SDL2TTFFont(ref e) => e.description(),
        }
    }
    // ========================================================================
    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            Error::OptNone(_) => None,
            Error::InvalidArg(_) => None,
            Error::InvalidEnum => None,
            Error::InvalidImage => None,
            Error::ManagedNotFound(_) => None,
            Error::Path(_) => None,
            Error::Mesh(_) => None,
            Error::Light(_) => None,
            Error::LBF(ref e) => Some(e),
            Error::IO(ref e) => Some(e),
            Error::Sif(ref e) => Some(e),
            Error::SifManager(ref e) => Some(e),
            Error::SifRenderer(ref e) => Some(e),
            Error::SifThree(ref e) => Some(e),
            Error::GL(ref e) => Some(e.as_ref()),
            Error::SDL2TTFFont(ref e) => Some(e),
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
