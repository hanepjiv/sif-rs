// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/12/03
//  @date 2018/09/11

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::error::Error as StdError;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Error
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Error {
    /// OptNone
    OptNone(String),
    /// Loader
    Loader(String),
    /// Current
    Current(i64, i64, i64),
    /// Insert
    Insert(String),
    /// Type
    Type(String),
    /// ImageDimension
    ImageDimension(String),
    /// ImageUnknown
    ImageUnknown(String),
    /// Material
    Material(String),
    /// Mesh
    Mesh(String),
    /// Polygon
    Polygon(String),
    /// Elem
    Elem(String),
    /// Model
    Model(String),
    /// Object
    Object(String),
    /// UuidParse
    UuidParse(String),
    /// IO
    IO(String),
}
// ============================================================================
impl From<super::loader::LoaderError> for Error {
    // ========================================================================
    fn from(e: super::loader::LoaderError) -> Self {
        Error::Loader(format!("{:?}", e)) // use "{:?}" for lua::ThreadStatus
    }
}
// ============================================================================
impl From<::uuid::parser::ParseError> for Error {
    // ========================================================================
    fn from(e: ::uuid::parser::ParseError) -> Self {
        Error::UuidParse(format!("{}", e))
    }
}
// ============================================================================
impl From<::std::io::Error> for Error {
    // ========================================================================
    fn from(e: ::std::io::Error) -> Self {
        Error::IO(format!("{}", e))
    }
}
// ============================================================================
impl ::std::fmt::Display for Error {
    // ========================================================================
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        <Self as ::std::fmt::Debug>::fmt(self, f)
    }
}
// ============================================================================
impl StdError for Error {
    // ========================================================================
    fn description(&self) -> &str {
        match *self {
            Error::OptNone(_) => "LBF opt none",
            Error::Loader(_) => "LBF load failed",
            Error::Current(_, _, _) => "LBF unsupported version",
            Error::Insert(_) => "LBF insert error",
            Error::Type(_) => "LBF type error",
            Error::ImageDimension(_) => "LBF image dimension error",
            Error::ImageUnknown(_) => "LBF image unknwon error",
            Error::Material(_) => "LBF material error",
            Error::Mesh(_) => "LBF mesh error",
            Error::Polygon(_) => "LBF polygon error",
            Error::Elem(_) => "LBF elem error",
            Error::Model(_) => "LBF model error",
            Error::Object(_) => "LBF object error",
            Error::UuidParse(_) => "LBF uuid parse error",
            Error::IO(_) => "LBF io error",
        }
    }
    // ========================================================================
    fn cause(&self) -> Option<&dyn StdError> {
        match *self {
            Error::OptNone(_) => None,
            Error::Loader(_) => None,
            Error::Current(_, _, _) => None,
            Error::Insert(_) => None,
            Error::Type(_) => None,
            Error::ImageDimension(_) => None,
            Error::ImageUnknown(_) => None,
            Error::Material(_) => None,
            Error::Mesh(_) => None,
            Error::Polygon(_) => None,
            Error::Elem(_) => None,
            Error::Model(_) => None,
            Error::Object(_) => None,
            Error::UuidParse(_) => None,
            Error::IO(_) => None,
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
