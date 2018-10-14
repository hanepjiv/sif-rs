// -*- mode:rust; coding:utf-8-unix; -*-

//! frame.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/06
//  @date 2018/05/16

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::result::Result as StdResult;
// ----------------------------------------------------------------------------
use gl::types::*;
// ----------------------------------------------------------------------------
use super::{gl_result, Bind, Error, Render, Result, Texture};
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
    pub fn new() -> Result<Self> {
        let id = gl_result(|| -> Result<GLuint> {
            let mut id = 0;
            unsafe {
                ::gl::GenFramebuffers(1, &mut id);
            }
            Ok(id)
        })?;
        Ok(Frame { id })
    }
    // ========================================================================
    /// attach_2d
    pub fn attach_2d(
        &self,
        attatchment: GLenum,
        textarget: GLenum,
        texture: &Texture,
    ) -> Result<&Self> {
        self.bind_with(|| -> Result<()> {
            gl_result(|| -> Result<()> {
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
            })?;
            Ok(())
        })?;
        Ok(self)
    }
    // ========================================================================
    /// attach_render
    pub fn attach_render(
        &self,
        attatchment: GLenum,
        renderbuffer: &Render,
    ) -> Result<&Self> {
        self.bind_with(|| -> Result<()> {
            gl_result(|| -> Result<()> {
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
            })?;
            Ok(())
        })?;
        Ok(self)
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
        })
        .expect("Frame::drop");
    }
}
// ============================================================================
impl Bind for Frame {
    // ========================================================================
    type BindError = Error;
    // ========================================================================
    fn id(&self) -> GLuint {
        self.id
    }
    // ========================================================================
    fn bind(&self) -> Result<()> {
        gl_result(|| -> StdResult<(), ()> {
            unsafe {
                ::gl::BindFramebuffer(::gl::FRAMEBUFFER, self.id);
            }
            Ok(())
        })?;
        Ok(())
    }
    // ========================================================================
    fn unbind(&self) -> Result<()> {
        gl_result(|| -> StdResult<(), ()> {
            unsafe {
                ::gl::BindFramebuffer(::gl::FRAMEBUFFER, 0);
            }
            Ok(())
        })?;
        Ok(())
    }
}
