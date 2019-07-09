// -*- mode:rust; coding:utf-8-unix; -*-

//! shader.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/08
//  @date 2019/07/09

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use gl::types::*;
use std::ffi::CString;
use std::result::Result as StdResult;
// ----------------------------------------------------------------------------
use super::{gl_result, info_log, Result};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct ShderSrc
#[derive(Debug)]
pub struct ShaderSrc<'a> {
    /// type_
    type_: GLenum,
    /// srcs
    srcs: Vec<&'a str>,
}
// ============================================================================
impl<'a> ShaderSrc<'a> {
    // ========================================================================
    /// new
    pub fn new(type_: GLenum, srcs: Vec<&'a str>) -> Self {
        ShaderSrc { type_, srcs }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Shader
#[derive(Debug)]
pub struct Shader {
    /// id
    id: GLuint,
}
// ============================================================================
impl Shader {
    // ========================================================================
    /// new
    pub fn new(src: &ShaderSrc<'_>) -> Result<Self> {
        let id = gl_result(|| -> StdResult<GLuint, ()> {
            Ok(unsafe { ::gl::CreateShader(src.type_) })
        })?;

        {
            let mut s = String::new();
            for i in &src.srcs {
                s.push_str(i);
            }
            let cs = CString::new(s)?;
            gl_result(|| -> StdResult<(), ()> {
                unsafe {
                    ::gl::ShaderSource(
                        id,
                        1,
                        &(cs.as_ptr()) as *const *const GLchar,
                        &(cs.as_bytes().len()) as *const usize as *const i32,
                    );
                }
                Ok(())
            })?;
        }

        gl_result(|| -> StdResult<(), ()> {
            unsafe {
                ::gl::CompileShader(id);
            }
            Ok(())
        })?;

        info_log(::gl::SHADER, id, ::gl::COMPILE_STATUS)?;

        Ok(Shader { id })
    }
    // ========================================================================
    /// id
    pub fn id(&self) -> GLuint {
        self.id
    }
}
// ============================================================================
impl Drop for Shader {
    fn drop(&mut self) {
        gl_result(|| -> StdResult<(), ()> {
            unsafe {
                ::gl::DeleteShader(self.id);
            }
            Ok(())
        })
        .expect("Shader::drop");
    }
}
