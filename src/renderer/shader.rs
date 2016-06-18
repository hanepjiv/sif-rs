/* -*- mode:rust; coding:utf-8-unix; -*- */

//! shader.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/08
//  @date 2016/06/18

/* ////////////////////////////////////////////////////////////////////////// */
/* use  ===================================================================== */
use ::std::vec::{ Vec, };
use ::std::string::{ String, };
use ::gl::types::*;
/* -------------------------------------------------------------------------- */
use super::{ gl_result, info_log, };
/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
/// struct ShderSrc
#[derive( Debug, )]
pub struct ShaderSrc<'a> {
    /// type_
    type_:      GLenum,
    /// srcs
    srcs:       Vec<&'a str>,
}
/* ========================================================================== */
impl <'a> ShaderSrc<'a> {
    /* ====================================================================== */
    /// new
    pub fn new(type_: GLenum, srcs: Vec<&'a str>) -> Self { ShaderSrc {
        type_:  type_,
        srcs:   srcs,
    } }
}
/* ////////////////////////////////////////////////////////////////////////// */
/* ========================================================================== */
/// struct Shader
#[derive( Debug, )]
pub struct Shader {
    /// id
    id:         GLuint,
}
/* ========================================================================== */
impl Shader {
    /* ====================================================================== */
    /// new
    pub fn new(src: &ShaderSrc) -> Result<Self, String> {
        let id = gl_result(|| -> Result<GLuint, ()> { unsafe {
            Ok(::gl::CreateShader(src.type_))
        } }).expect("Shader::new: CreateShader");

        {
            let mut v = ::std::vec::Vec::new();
            let mut l = ::std::vec::Vec::new();
            for i in src.srcs.iter() {
                v.push(i.as_ptr());
                l.push(i.len());
            }
            gl_result(|| -> Result<(), ()> { unsafe {
                Ok(::gl::ShaderSource(id, v.len() as GLsizei,
                                      v.as_ptr() as *const *const GLchar,
                                      l.as_ptr() as *const GLint))
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
    /* ====================================================================== */
    /// id
    pub fn id(&self) -> GLuint { self.id }
}
/* ========================================================================== */
impl Drop for Shader {
    fn drop(&mut self) {
        gl_result(|| -> Result<(), ()> { unsafe {
            Ok(::gl::DeleteShader(self.id))
        } }).expect("Shader::drop");
    }
}
