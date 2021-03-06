// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/12/10
//  @date 2020/03/19

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::{error::Error as StdError, fmt::Debug};
// ----------------------------------------------------------------------------
use uuid::Uuid;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Error
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    /// Object
    Object(String),
    /// AnimationDriver,
    AnimationDriver(String),
    /// LBF
    LBF(super::lbf::Error),
    /// IO
    IO(String),
    /// Sif
    Sif(::sif_error::Error),
    /// SifManager
    SifManager(::sif_manager::Error),
    /// SifThree
    SifThree(::sif_three::Error),
    /// SifRenderer
    SifRenderer(::sif_renderer::Error),
    /// GL
    GL(String),
    /// SDL2TTFFont
    SDL2TTFFont(String),
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
        Error::IO(format!("{}", e))
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
impl From<::sif_three::Error> for Error {
    fn from(e: ::sif_three::Error) -> Self {
        Error::SifThree(e)
    }
}
// ----------------------------------------------------------------------------
impl From<::sif_renderer::Error> for Error {
    fn from(e: ::sif_renderer::Error) -> Self {
        Error::SifRenderer(e)
    }
}
// ----------------------------------------------------------------------------
impl<R, E> From<::sif_renderer::GLError<R, E>> for Error
where
    R: Debug + Clone + PartialOrd + PartialEq,
    E: Debug + Clone + PartialOrd + PartialEq,
{
    fn from(e: ::sif_renderer::GLError<R, E>) -> Self {
        Error::GL(format!("{}", e))
    }
}
// ----------------------------------------------------------------------------
impl From<::sdl2::ttf::FontError> for Error {
    fn from(e: ::sdl2::ttf::FontError) -> Self {
        Error::SDL2TTFFont(format!("{}", e))
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
            Error::InvalidArg(_) => None,
            Error::InvalidEnum => None,
            Error::InvalidImage => None,
            Error::ManagedNotFound(_) => None,
            Error::Path(_) => None,
            Error::Mesh(_) => None,
            Error::Light(_) => None,
            Error::Object(_) => None,
            Error::AnimationDriver(_) => None,
            Error::LBF(ref e) => Some(e),
            Error::IO(_) => None,
            Error::Sif(ref e) => Some(e),
            Error::SifManager(ref e) => Some(e),
            Error::SifThree(ref e) => Some(e),
            Error::SifRenderer(ref e) => Some(e),
            Error::GL(_) => None,
            Error::SDL2TTFFont(_) => None,
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
