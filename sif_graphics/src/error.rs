// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/12/10
//  @date 2018/05/12

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
        match *self {
            ref e @ Error::OptNone(_)
            | ref e @ Error::InvalidEnum
            | ref e @ Error::InvalidImage
            | ref e @ Error::ManagedNotFound(_)
            | ref e @ Error::Path(_)
            | ref e @ Error::Mesh(_) => write!(f, "{:?}", e),
            Error::LBF(ref e) => e.fmt(f),
            Error::IO(ref e) => e.fmt(f),
            Error::Sif(ref e) => e.fmt(f),
            Error::SifManager(ref e) => e.fmt(f),
            Error::SifRenderer(ref e) => e.fmt(f),
            Error::SifThree(ref e) => e.fmt(f),
            Error::GL(ref e) => e.fmt(f),
            Error::SDL2TTFFont(ref e) => e.fmt(f),
        }
    }
}
// ============================================================================
impl ::std::error::Error for Error {
    // ========================================================================
    fn description(&self) -> &str {
        match *self {
            Error::OptNone(_) => "::graphics::OptNone",
            Error::InvalidEnum => "::graphics::InvalidImage",
            Error::InvalidImage => "::graphics::InvalidImage",
            Error::ManagedNotFound(_) => "::graphics::ManagedNotFound",
            Error::Path(_) => "::graphics::Path",
            Error::Mesh(_) => "::graphics::Mesh",
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
            Error::InvalidEnum => None,
            Error::InvalidImage => None,
            Error::ManagedNotFound(_) => None,
            Error::Path(_) => None,
            Error::Mesh(_) => None,
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
