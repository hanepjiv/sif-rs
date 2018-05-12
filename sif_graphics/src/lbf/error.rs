// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/12/03
//  @date 2018/05/09

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Error
#[derive(Debug)]
pub enum Error {
    /// OptNone
    OptNone(String),
    /// Load
    Load(::lua::ThreadStatus),
    /// Current
    Current,
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
    UuidParse(::uuid::ParseError),
    /// IO
    IO(::std::io::Error),
}
// ============================================================================
impl From<::uuid::ParseError> for Error {
    // ========================================================================
    fn from(e: ::uuid::ParseError) -> Self {
        Error::UuidParse(e)
    }
}
// ============================================================================
impl From<::std::io::Error> for Error {
    // ========================================================================
    fn from(e: ::std::io::Error) -> Self {
        Error::IO(e)
    }
}
// ============================================================================
impl ::std::fmt::Display for Error {
    // ========================================================================
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ref e @ Error::OptNone(_)
            | ref e @ Error::Load(_)
            | ref e @ Error::Current
            | ref e @ Error::Insert(_)
            | ref e @ Error::Type(_)
            | ref e @ Error::Mesh(_)
            | ref e @ Error::Polygon(_)
            | ref e @ Error::Elem(_) => write!(f, "{:?}", e),
            Error::UuidParse(ref e) => e.fmt(f),
            Error::IO(ref e) => e.fmt(f),
        }
    }
}
// ============================================================================
impl ::std::error::Error for Error {
    // ========================================================================
    fn description(&self) -> &str {
        match *self {
            Error::OptNone(_) => "LBF opt none",
            Error::Load(_) => "LBF load failed",
            Error::Current => "LBF unsupported version",
            Error::Insert(_) => "LBF insert error",
            Error::Type(_) => "LBF type error",
            Error::Mesh(_) => "LBF mesh error",
            Error::Polygon(_) => "LBF polygon error",
            Error::Elem(_) => "LBF elem error",
            Error::UuidParse(_) => "LBG uuid parse error",
            Error::IO(ref e) => e.description(),
        }
    }
    // ========================================================================
    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            Error::OptNone(_) => None,
            Error::Load(_) => None,
            Error::Current => None,
            Error::Insert(_) => None,
            Error::Type(_) => None,
            Error::Mesh(_) => None,
            Error::Polygon(_) => None,
            Error::Elem(_) => None,
            Error::UuidParse(_) => None,
            Error::IO(ref e) => Some(e),
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// type Result
pub type Result<T> = ::std::result::Result<T, Error>;
