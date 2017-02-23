// -*- mode:rust; coding:utf-8-unix; -*-

//! render.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/06
//  @date 2017/01/17

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use ::gl::types::*;
// ----------------------------------------------------------------------------
use super::{ gl_result, GLError, TBind, };
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Render
#[derive( Debug, )]
pub struct Render {
    /// id
    id:         GLuint,
}
// ============================================================================
impl Render {
    // ========================================================================
    /// new
    pub fn new(internalformat: GLenum,  width: GLsizei, height: GLsizei)
               -> Result<Self, GLError<GLuint, ()>> {
        match gl_result(|| -> Result<GLuint, ()> { unsafe {
            let mut id = 0;
            ::gl::GenRenderbuffers(1, &mut id);
            Ok(id)
        } }) {
            Err(e)      => Err(e),
            Ok(id)      => {
                match gl_result(|| -> Result<GLuint, ()> { unsafe {
                    ::gl::BindRenderbuffer(::gl::RENDERBUFFER, id);
                    ::gl::RenderbufferStorage(::gl::RENDERBUFFER,
                                              internalformat, width, height);
                    ::gl::BindRenderbuffer(::gl::RENDERBUFFER, 0);
                    Ok(id)
                } }) {
                    Err(e)      => Err(e),
                    Ok(id_)     => Ok(Render {
                        id:     id_,
                    })
                }
            },
        }
    }
}
// ============================================================================
impl Drop for Render {
    fn drop(&mut self) {
        gl_result(|| -> Result<(), ()> { unsafe {
            Ok(::gl::DeleteRenderbuffers(1, &self.id))
        } }).expect("Render::drop");
    }
}
// ============================================================================
impl TBind for Render {
    // ========================================================================
    fn id(&self) -> GLuint { self.id }
    // ========================================================================
    /// bind
    fn bind(&self) {
        gl_result(|| -> Result<(), ()> { unsafe {
            Ok(::gl::BindRenderbuffer(::gl::RENDERBUFFER, self.id))
        } }).expect("Render::bind");
    }
    // ========================================================================
    /// unbind
    fn unbind(&self) {
        gl_result(|| -> Result<(), ()> { unsafe {
            Ok(::gl::BindRenderbuffer(::gl::RENDERBUFFER, 0))
        } }).expect("Render::unbind");
    }
}
