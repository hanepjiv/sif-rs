// -*- mode:rust; coding:utf-8-unix; -*-

//! result.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/08
//  @date 2018/04/10

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::fmt::Debug;
// ----------------------------------------------------------------------------
use gl::types::*;
// ////////////////////////////////////////////////////////////////////////////
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
    GL(Result<R, E>, GLenum),
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
pub fn gl_result<R, E, F>(f: F) -> Result<R, GLError<R, E>>
where
    R: Debug,
    E: Debug,
    F: FnOnce() -> Result<R, E>,
{
    let result = f();
    // if cfg!(debug_assertions) {
    match unsafe { ::gl::GetError() } {
        ::gl::NO_ERROR => {
            result.map_err(|e| -> GLError<R, E> { GLError::Function(e) })
        }
        e => Err(GLError::GL(result, e)),
    }
    //} else {
    //    result.map_err(|e| -> GLError<R, E> { GLError::Function(e) })
    //}
}
// ============================================================================
/// get_iv
fn get_iv(type_: GLenum, id: GLuint, pname: GLenum) -> GLint {
    gl_result(|| -> Result<GLint, ()> {
        unsafe {
            let mut params = 0;
            match type_ {
                ::gl::SHADER => ::gl::GetShaderiv(id, pname, &mut params),
                ::gl::PROGRAM => ::gl::GetProgramiv(id, pname, &mut params),
                _ => return Err(()),
            }
            Ok(params)
        }
    }).expect("result::get_iv")
}
// ============================================================================
/// info_log
pub fn info_log(
    type_: GLenum,
    id: GLuint,
    state: GLenum,
) -> Result<(), String> {
    match get_iv(type_, id, state) as GLboolean {
        ::gl::FALSE => gl_result(|| -> Result<(), String> {
            unsafe {
                let loglen = get_iv(type_, id, ::gl::INFO_LOG_LENGTH);
                if 0 >= loglen {
                    return Err(String::from("0 >= loglen"));
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
                        return Err(String::from("invalid type_"));
                    }
                }
                for i in &mut log {
                    if *i > 128 {
                        *i -= 128;
                    }
                }
                let msg = ::std::str::from_utf8(log.as_slice())
                    .expect("info_log: ::std::str::from_utf8");
                println!("{}", msg);
                Err(String::from(msg))
            }
        }).map_err(|e| -> String {
            match e {
                GLError::Function(msg) => msg,
                GLError::GL(_, _) => {
                    String::from("info_log: ::gl::Get*InfoLog")
                }
            }
        }),
        _ => Ok(()),
    }
}
