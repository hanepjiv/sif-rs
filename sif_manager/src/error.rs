// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/05/12
//  @date 2018/05/12

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use std::error::Error as StdError;
// ----------------------------------------------------------------------------
use uuid::Uuid;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Error
#[allow(missing_copy_implementations)]
#[derive(Debug)]
pub enum Error {
    /// Insert
    Insert(Uuid),
}
// ============================================================================
impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ref e @ Error::Insert(_) => write!(f, "{:?}", e),
        }
    }
}
// ============================================================================
impl StdError for Error {
    // ========================================================================
    fn description(&self) -> &str {
        match *self {
            Error::Insert(_) => "manager: insert failed",
        }
    }
    // ========================================================================
    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Insert(_) => None,
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// type Result
pub type Result<T> = ::std::result::Result<T, Error>;
