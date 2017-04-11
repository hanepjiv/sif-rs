// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/04
//  @date 2017/03/15

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// type Result
pub type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Error
#[derive( Debug, )]
pub enum Error {
    /// InvalidArguments
    InvalidArguments(String),
}
// ============================================================================
impl ::std::fmt::Display for Error {
    // ========================================================================
    fn fmt(&self, f: &mut ::std::fmt::Formatter)
           -> ::std::fmt::Result { match *self {
        ref e@Error::InvalidArguments(_)        => write!(f, "{:?}", e),
        //Error::SomeError(ref e)               => e.fmt(f),
    } }
}
// ============================================================================
impl ::std::error::Error for Error {
    // ========================================================================
    fn description(&self) -> &str { match *self {
        Error::InvalidArguments(_) => "::sif::math::Error::invalidArguments",
        // Error::SomeError(ref e) => e.description(),
    } }
    // ========================================================================
    fn cause(&self) -> Option<&::std::error::Error> { match *self {
        Error::InvalidArguments(_)              => None,
        // Error::SomeError(ref e)              => Some(e),
    } }
}
