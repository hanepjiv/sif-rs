// -*- mode:rust; coding:utf-8-unix; -*-

//! frame.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/06
//  @date 2017/01/17

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use ::gl::types::*;
// ----------------------------------------------------------------------------
use super::{ gl_result, GLError, TBind, Texture, Render, };
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
    /// attach_2d
    pub fn attach_2d(&self,
                    attatchment: GLenum, textarget: GLenum, texture: &Texture)
                    -> Result<(), GLError<(), GLenum>> {
        let _binder = self.binder();
        unwrap!(gl_result(|| -> Result<(), ()> { unsafe {
            Ok(::gl::FramebufferTexture2D(::gl::FRAMEBUFFER, attatchment,
                                             textarget, texture.id(), 0))
        } }));
        gl_result(|| -> Result<(), GLenum> { unsafe {
            match ::gl::CheckFramebufferStatus(::gl::FRAMEBUFFER) {
                ::gl::FRAMEBUFFER_COMPLETE      => Ok(()),
                x                               => Err(x),
            }
        } })
    }
    // ========================================================================
    /// attach_render
    pub fn attach_render(&self, attatchment: GLenum, renderbuffer: &Render)
                         -> Result<(), GLError<(), GLenum>> {
        let _binder = self.binder();
        unwrap!(gl_result(|| -> Result<(), ()> { unsafe {
            Ok(::gl::FramebufferRenderbuffer(::gl::FRAMEBUFFER, attatchment,
                                             ::gl::RENDERBUFFER,
                                             renderbuffer.id()))
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
