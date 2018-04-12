// -*- mode:rust; coding:utf-8-unix; -*-

//! frame.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/06
//  @date 2018/04/12

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use gl::types::*;
// ----------------------------------------------------------------------------
use super::{gl_result, Bind, GLError, Render, Texture};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Frame
#[derive(Debug, Default)]
pub struct Frame {
    /// id
    id: GLuint,
}
// ============================================================================
impl Frame {
    // ========================================================================
    /// new
    pub fn new() -> Self {
        match gl_result(|| -> Result<GLuint, ()> {
            let mut id = 0;
            unsafe {
                ::gl::GenFramebuffers(1, &mut id);
            }
            Ok(id)
        }) {
            Ok(id) => Frame { id },
            _ => panic!("Frame::new"),
        }
    }
    // ========================================================================
    /// attach_2d
    pub fn attach_2d(
        &self,
        attatchment: GLenum,
        textarget: GLenum,
        texture: &Texture,
    ) -> Result<(), GLError<(), GLenum>> {
        self.bind_with(|| {
            unwrap!(gl_result(|| -> Result<(), ()> {
                unsafe {
                    Ok(::gl::FramebufferTexture2D(
                        ::gl::FRAMEBUFFER,
                        attatchment,
                        textarget,
                        texture.id(),
                        0,
                    ))
                }
            }));
            gl_result(|| -> Result<(), GLenum> {
                unsafe {
                    match ::gl::CheckFramebufferStatus(::gl::FRAMEBUFFER) {
                        ::gl::FRAMEBUFFER_COMPLETE => Ok(()),
                        x => Err(x),
                    }
                }
            })
        })
    }
    // ========================================================================
    /// attach_render
    pub fn attach_render(
        &self,
        attatchment: GLenum,
        renderbuffer: &Render,
    ) -> Result<(), GLError<(), GLenum>> {
        self.bind_with(|| {
            unwrap!(gl_result(|| -> Result<(), ()> {
                unsafe {
                    Ok(::gl::FramebufferRenderbuffer(
                        ::gl::FRAMEBUFFER,
                        attatchment,
                        ::gl::RENDERBUFFER,
                        renderbuffer.id(),
                    ))
                }
            }));
            gl_result(|| -> Result<(), GLenum> {
                unsafe {
                    match ::gl::CheckFramebufferStatus(::gl::FRAMEBUFFER) {
                        ::gl::FRAMEBUFFER_COMPLETE => Ok(()),
                        x => Err(x),
                    }
                }
            })
        })
    }
}
// ============================================================================
impl Drop for Frame {
    fn drop(&mut self) {
        gl_result(|| -> Result<(), ()> {
            unsafe { Ok(::gl::DeleteFramebuffers(1, &self.id)) }
        }).expect("Frame::drop");
    }
}
// ============================================================================
impl Bind for Frame {
    // ========================================================================
    fn id(&self) -> GLuint {
        self.id
    }
    // ========================================================================
    /// bind
    fn bind(&self) {
        gl_result(|| -> Result<(), ()> {
            unsafe {
                Ok(::gl::BindFramebuffer(
                    ::gl::FRAMEBUFFER,
                    self.id,
                ))
            }
        }).expect("Frame::bind");
    }
    // ========================================================================
    /// unbind
    fn unbind(&self) {
        gl_result(|| -> Result<(), ()> {
            unsafe { Ok(::gl::BindFramebuffer(::gl::FRAMEBUFFER, 0)) }
        }).expect("Frame::unbind");
    }
}
