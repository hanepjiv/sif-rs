// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/05/12
//  @date 2018/06/18

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use std::error::Error as StdError;
// ----------------------------------------------------------------------------
use uuid::Uuid;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Error
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Error {
    /// Insert
    Insert(Uuid),
}
// ============================================================================
impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        <Self as ::std::fmt::Debug>::fmt(self, f)
    }
}
// ============================================================================
impl StdError for Error {
    // ========================================================================
    fn description(&self) -> &str {
        match *self {
            Error::Insert(_) => "sif::manager: insert failed",
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
