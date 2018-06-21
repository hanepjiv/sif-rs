// -*- mode:rust; coding:utf-8-unix; -*-

//! gl_error.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/08
//  @date 2018/06/18

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::{fmt::Debug, result::Result as StdResult};
// ----------------------------------------------------------------------------
use gl::types::*;
// ----------------------------------------------------------------------------
use super::{Error, Result};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum GLError
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum GLError<R, E>
where
    R: Debug + Clone + PartialOrd + PartialEq,
    E: Debug + Clone + PartialOrd + PartialEq,
{
    /// Function
    Function(E),
    /// GL
    GL(StdResult<R, E>, GLenum),
}
// ============================================================================
impl<R, E> ::std::fmt::Display for GLError<R, E>
where
    R: Debug + Clone + PartialOrd + PartialEq,
    E: Debug + Clone + PartialOrd + PartialEq,
{
    // ========================================================================
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        <Self as ::std::fmt::Debug>::fmt(self, f)
    }
}
// ============================================================================
impl<R, E> ::std::error::Error for GLError<R, E>
where
    R: Debug + Clone + PartialOrd + PartialEq,
    E: Debug + Clone + PartialOrd + PartialEq,
{
    // ========================================================================
    fn description(&self) -> &str {
        match *self {
            GLError::Function(_) => "::sif::renderer::GLError::Function",
            GLError::GL(_, _) => "::sif::renderer::GLError::GL",
        }
    }
    // ========================================================================
    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            GLError::Function(_) => None,
            GLError::GL(_, _) => None,
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// type GLResult
pub type GLResult<R, E> = StdResult<R, GLError<R, E>>;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// gl_result
pub fn gl_result<R, E>(f: impl FnOnce() -> StdResult<R, E>) -> GLResult<R, E>
where
    R: Debug + Clone + PartialOrd + PartialEq,
    E: Debug + Clone + PartialOrd + PartialEq,
{
    let result = f();
    match unsafe { ::gl::GetError() } {
        ::gl::NO_ERROR => {
            result.map_err(|e| -> GLError<R, E> { GLError::Function(e) })
        }
        e => Err(GLError::GL(result, e)),
    }
}
// ============================================================================
/// get_iv
fn get_iv(type_: GLenum, id: GLuint, pname: GLenum) -> GLResult<GLint, Error> {
    gl_result(|| -> Result<GLint> {
        let mut params = 0;
        unsafe {
            match type_ {
                ::gl::SHADER => ::gl::GetShaderiv(id, pname, &mut params),
                ::gl::PROGRAM => ::gl::GetProgramiv(id, pname, &mut params),
                _ => {
                    return Err(Error::Renderer(format!(
                        "get_iv: invalid type: {}",
                        type_
                    )));
                }
            }
        }
        Ok(params)
    })
}
// ============================================================================
/// info_log
pub fn info_log(type_: GLenum, id: GLuint, state: GLenum) -> Result<()> {
    match get_iv(type_, id, state)? as GLboolean {
        ::gl::FALSE => {
            gl_result(|| -> Result<()> {
                unsafe {
                    let loglen = get_iv(type_, id, ::gl::INFO_LOG_LENGTH)?;
                    if 0 >= loglen {
                        return Err(Error::Renderer("0 >= loglen".to_string()));
                    }
                    let mut log = vec![0u8; loglen as usize];
                    let mut length = 0;
                    match type_ {
                        ::gl::SHADER => ::gl::GetShaderInfoLog(
                            id,
                            loglen,
                            &mut length,
                            log.as_mut_ptr() as *mut i8,
                        ),
                        ::gl::PROGRAM => ::gl::GetProgramInfoLog(
                            id,
                            loglen,
                            &mut length,
                            log.as_mut_ptr() as *mut i8,
                        ),
                        _ => {
                            return Err(Error::Renderer(
                                "invalid type_".to_string(),
                            ));
                        }
                    }
                    for i in &mut log {
                        if *i > 128 {
                            *i -= 128;
                        }
                    }
                    let msg = ::std::str::from_utf8(log.as_slice())?;
                    println!("{}", msg);
                    Err(Error::Renderer(msg.to_string()))
                }
            })?;
            Ok(())
        }
        _ => Ok(()),
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
#[cfg(test)]
mod tests {
    // use  ===================================================================
    use super::{GLError, GLResult};
    // ========================================================================
    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<GLError<(), ()>>();
        assert_send::<GLResult<(), ()>>();
    }
    // ------------------------------------------------------------------------
    #[test]
    fn test_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<GLError<(), ()>>();
        assert_sync::<GLResult<(), ()>>();
    }
}
