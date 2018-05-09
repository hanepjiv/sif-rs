// -*- mode:rust; coding:utf-8-unix; -*-

//! frame.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/06
//  @date 2018/05/09

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use gl::types::*;
use std::result::Result as StdResult;
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
        match gl_result(|| -> StdResult<GLuint, ()> {
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
    ) -> StdResult<(), GLError<(), GLenum>> {
        self.bind_with(|| -> StdResult<(), GLError<(), GLenum>> {
            gl_result(|| -> StdResult<(), GLenum> {
                unsafe {
                    ::gl::FramebufferTexture2D(
                        ::gl::FRAMEBUFFER,
                        attatchment,
                        textarget,
                        texture.id(),
                        0,
                    )
                }
                Ok(())
            })?;
            gl_result(|| -> StdResult<(), GLenum> {
                match unsafe {
                    ::gl::CheckFramebufferStatus(::gl::FRAMEBUFFER)
                } {
                    ::gl::FRAMEBUFFER_COMPLETE => Ok(()),
                    x => Err(x),
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
    ) -> StdResult<(), GLError<(), GLenum>> {
        self.bind_with(|| {
            gl_result(|| -> StdResult<(), GLenum> {
                unsafe {
                    ::gl::FramebufferRenderbuffer(
                        ::gl::FRAMEBUFFER,
                        attatchment,
                        ::gl::RENDERBUFFER,
                        renderbuffer.id(),
                    )
                }
                Ok(())
            })?;
            gl_result(|| -> StdResult<(), GLenum> {
                match unsafe {
                    ::gl::CheckFramebufferStatus(::gl::FRAMEBUFFER)
                } {
                    ::gl::FRAMEBUFFER_COMPLETE => Ok(()),
                    x => Err(x),
                }
            })
        })
    }
}
// ============================================================================
impl Drop for Frame {
    fn drop(&mut self) {
        gl_result(|| -> StdResult<(), ()> {
            unsafe {
                ::gl::DeleteFramebuffers(1, &self.id);
            }
            Ok(())
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
        gl_result(|| -> StdResult<(), ()> {
            unsafe {
                ::gl::BindFramebuffer(::gl::FRAMEBUFFER, self.id);
            }
            Ok(())
        }).expect("Frame::bind");
    }
    // ========================================================================
    /// unbind
    fn unbind(&self) {
        gl_result(|| -> StdResult<(), ()> {
            unsafe {
                ::gl::BindFramebuffer(::gl::FRAMEBUFFER, 0);
            }
            Ok(())
        }).expect("Frame::unbind");
    }
}
