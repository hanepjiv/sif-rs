// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/12/03
//  @date 2018/06/15

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
    /// Mesh
    Mesh(String),
    /// Polygon
    Polygon(String),
    /// Elem
    Elem(String),
    /// UuidParse
    UuidParse(String),
    /// IO
    IO(String),
}
// ============================================================================
impl From<super::loader::LoaderError> for Error {
    // ========================================================================
    fn from(e: super::loader::LoaderError) -> Self {
        Error::Loader(format!("{}", e))
    }
}
// ============================================================================
impl From<::uuid::ParseError> for Error {
    // ========================================================================
    fn from(e: ::uuid::ParseError) -> Self {
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
        write!(f, "{:?}", self)
    }
}
// ============================================================================
impl ::std::error::Error for Error {
    // ========================================================================
    fn description(&self) -> &str {
        match *self {
            Error::OptNone(_) => "LBF opt none",
            Error::Loader(_) => "LBF load failed",
            Error::Current(_, _, _) => "LBF unsupported version",
            Error::Insert(_) => "LBF insert error",
            Error::Type(_) => "LBF type error",
            Error::Mesh(_) => "LBF mesh error",
            Error::Polygon(_) => "LBF polygon error",
            Error::Elem(_) => "LBF elem error",
            Error::UuidParse(_) => "LBF uuid parse error",
            Error::IO(_) => "LBF io error",
        }
    }
    // ========================================================================
    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            Error::OptNone(_) => None,
            Error::Loader(_) => None,
            Error::Current(_, _, _) => None,
            Error::Insert(_) => None,
            Error::Type(_) => None,
            Error::Mesh(_) => None,
            Error::Polygon(_) => None,
            Error::Elem(_) => None,
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
