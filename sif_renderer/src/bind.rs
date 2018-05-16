// -*- mode:rust; coding:utf-8-unix; -*-

//! bind.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/11
//  @date 2018/05/16

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::result::Result as StdResult;
// ----------------------------------------------------------------------------
use gl::types::*;
// ----------------------------------------------------------------------------
use super::Error;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// trait Bind
pub trait Bind {
    // ========================================================================
    /// type Error
    type BindError: From<Error>;
    // ========================================================================
    /// fn id
    fn id(&self) -> GLuint;
    // ========================================================================
    /// fn bind
    fn bind(&self) -> StdResult<(), Self::BindError>;
    // ========================================================================
    /// fn unbind
    fn unbind(&self) -> StdResult<(), Self::BindError>;
    // ========================================================================
    /// fn bind_with
    fn bind_with<R, E>(
        &self,
        func: impl FnOnce() -> StdResult<R, E>,
    ) -> StdResult<R, E>
    where
        E: From<Self::BindError>,
    {
        self.bind()?;
        let ret = func()?;
        self.unbind()?;
        Ok(ret)
    }
}
