// -*- mode:rust; coding:utf-8-unix; -*-

//! gl_error.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/08
//  @date 2018/05/09

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::fmt::Debug;
use std::result::Result as StdResult;
// ----------------------------------------------------------------------------
use gl::types::*;
// ----------------------------------------------------------------------------
use super::{Error, Result};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// type GLResult
pub type GLResult<R, E> = StdResult<R, GLError<R, E>>;
// ============================================================================
/// enum GLError
#[derive(Debug, Clone)]
pub enum GLError<R, E>
where
    R: Debug,
    E: Debug,
{
    /// Function
    Function(E),
    /// GL
    GL(StdResult<R, E>, GLenum),
}
// ============================================================================
impl<R, E> ::std::fmt::Display for GLError<R, E>
where
    R: Debug,
    E: Debug,
{
    // ========================================================================
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ref e @ GLError::Function(_) | ref e @ GLError::GL(_, _) => {
                write!(f, "{:?}", e)
            }
        }
    }
}
// ============================================================================
impl<R, E> ::std::error::Error for GLError<R, E>
where
    R: Debug,
    E: Debug,
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
// ============================================================================
/// gl_result
pub fn gl_result<R, E, F>(f: F) -> GLResult<R, E>
where
    R: Debug,
    E: Debug,
    F: FnOnce() -> StdResult<R, E>,
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
fn get_iv(type_: GLenum, id: GLuint, pname: GLenum) -> GLint {
    gl_result(|| -> StdResult<GLint, ()> {
        unsafe {
            let mut params = 0;
            match type_ {
                ::gl::SHADER => ::gl::GetShaderiv(id, pname, &mut params),
                ::gl::PROGRAM => ::gl::GetProgramiv(id, pname, &mut params),
                _ => return Err(()),
            }
            Ok(params)
        }
    }).expect("gl_result::get_iv")
}
// ============================================================================
/// info_log
pub fn info_log(type_: GLenum, id: GLuint, state: GLenum) -> Result<()> {
    match get_iv(type_, id, state) as GLboolean {
        ::gl::FALSE => {
            gl_result(|| -> Result<()> {
                unsafe {
                    let loglen = get_iv(type_, id, ::gl::INFO_LOG_LENGTH);
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
