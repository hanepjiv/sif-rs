// -*- mode:rust; coding:utf-8-unix; -*-

//! frame.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/06
//  @date 2016/10/13

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use ::gl::types::*;
// ----------------------------------------------------------------------------
use super::{ gl_result, GLError, TBind, };
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Frame
#[derive( Debug, )]
pub struct Frame {
    /// id
    id:         GLuint,
}
// ============================================================================
impl Frame {
    // ========================================================================
    /// new
    pub fn new() -> Self {
        match gl_result(|| -> Result<GLuint, ()> { unsafe {
            let mut id = 0;
            ::gl::GenFramebuffers(1, &mut id);
            Ok(id)
        } }) {
            Err(_)      => panic!("Frame::new"),
            Ok(id)      => { Frame {
                id:     id,
            } },
        }
    }
    // ========================================================================
    /// attach
    pub fn attach(&self, att: GLenum, id: GLuint, level: GLint)
                  -> Result<(), GLError<(), GLenum>>{
        let _binder = self.binder();
        unwrap!(gl_result(|| -> Result<(), ()> { unsafe {
            Ok(::gl::FramebufferTexture(::gl::FRAMEBUFFER, att, id, level))
        } }));
        gl_result(|| -> Result<(), GLenum> { unsafe {
            match ::gl::CheckFramebufferStatus(::gl::FRAMEBUFFER) {
                ::gl::FRAMEBUFFER_COMPLETE      => Ok(()),
                x                               => Err(x),
            }
        } })
    }
}
// ============================================================================
impl Drop for Frame {
    fn drop(&mut self) {
        gl_result(|| -> Result<(), ()> { unsafe {
            Ok(::gl::DeleteFramebuffers(1, &self.id))
        } }).expect("Frame::drop");
    }
}
// ============================================================================
impl TBind for Frame {
    // ========================================================================
    fn id(&self) -> GLuint { self.id }
    // ========================================================================
    /// bind
    fn bind(&self) {
        gl_result(|| -> Result<(), ()> { unsafe {
            Ok(::gl::BindFramebuffer(::gl::FRAMEBUFFER, self.id))
        } }).expect("Frame::bind");
    }
    // ========================================================================
    /// unbind
    fn unbind(&self) {
        gl_result(|| -> Result<(), ()> { unsafe {
            Ok(::gl::BindFramebuffer(::gl::FRAMEBUFFER, 0))
        } }).expect("Frame::unbind");
    }
}
