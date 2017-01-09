// -*- mode:rust; coding:utf-8-unix; -*-

//! shader.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/08
//  @date 2017/01/09

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use ::std::ffi::CString;
use ::gl::types::*;
// ----------------------------------------------------------------------------
use super::{ gl_result, info_log, };
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct ShderSrc
#[derive( Debug, )]
pub struct ShaderSrc<'a> {
    /// type_
    type_:      GLenum,
    /// srcs
    srcs:       Vec<&'a str>,
}
// ============================================================================
impl <'a> ShaderSrc<'a> {
    // ========================================================================
    /// new
    pub fn new(type_: GLenum, srcs: Vec<&'a str>) -> Self { ShaderSrc {
        type_:  type_,
        srcs:   srcs,
    } }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Shader
#[derive( Debug, )]
pub struct Shader {
    /// id
    id:         GLuint,
}
// ============================================================================
impl Shader {
    // ========================================================================
    /// new
    pub fn new(src: &ShaderSrc) -> Result<Self, String> {
        let id = gl_result(|| -> Result<GLuint, ()> { unsafe {
            Ok(::gl::CreateShader(src.type_))
        } }).expect("Shader::new: CreateShader");

        {
            let mut s   = String::new();
            for i in src.srcs.iter() { s.push_str(i); }
            let cs      = unwrap!(CString::new(s));
            gl_result(|| -> Result<(), ()> { unsafe {
                Ok(::gl::ShaderSource(
                    id, 1,
                    &(cs.as_ptr()) as *const *const GLchar,
                    ::std::mem::transmute(&(cs.as_bytes().len()))))
            } }).expect("Shader::new: ShaderSource");
        }

        gl_result(|| -> Result<(), ()> { unsafe {
            Ok(::gl::CompileShader(id))
        } }).expect("Shader::new: CompileShader");

        info_log(::gl::SHADER, id, ::gl::COMPILE_STATUS).
            expect("Shader::new: info_log");

        Ok(Shader {
            id:                 id,
        })
    }
    // ========================================================================
    /// id
    pub fn id(&self) -> GLuint { self.id }
}
// ============================================================================
impl Drop for Shader {
    fn drop(&mut self) {
        gl_result(|| -> Result<(), ()> { unsafe {
            Ok(::gl::DeleteShader(self.id))
        } }).expect("Shader::drop");
    }
}
