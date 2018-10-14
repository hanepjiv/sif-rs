// -*- mode:rust; coding:utf-8-unix; -*-

//! render.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/06
//  @date 2018/05/16

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use gl::types::*;
// ----------------------------------------------------------------------------
use super::{gl_result, Bind, Error, Result};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Render
#[derive(Debug)]
pub struct Render {
    /// id
    id: GLuint,
}
// ============================================================================
impl Render {
    // ========================================================================
    /// new
    pub fn new(
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    ) -> Result<Self> {
        let id = gl_result(|| -> Result<GLuint> {
            let mut id = 0;
            unsafe {
                ::gl::GenRenderbuffers(1, &mut id);
            }
            Ok(id)
        })?;
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::BindRenderbuffer(::gl::RENDERBUFFER, id);
                ::gl::RenderbufferStorage(
                    ::gl::RENDERBUFFER,
                    internalformat,
                    width,
                    height,
                );
                ::gl::BindRenderbuffer(::gl::RENDERBUFFER, 0);
            }
            Ok(())
        })?;
        Ok(Render { id })
    }
}
// ============================================================================
impl Drop for Render {
    fn drop(&mut self) {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::DeleteRenderbuffers(1, &self.id);
            }
            Ok(())
        })
        .expect("Render::drop");
    }
}
// ============================================================================
impl Bind for Render {
    // ========================================================================
    type BindError = Error;
    // ========================================================================
    fn id(&self) -> GLuint {
        self.id
    }
    // ========================================================================
    fn bind(&self) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::BindRenderbuffer(::gl::RENDERBUFFER, self.id);
            }
            Ok(())
        })?;
        Ok(())
    }
    // ========================================================================
    fn unbind(&self) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::BindRenderbuffer(::gl::RENDERBUFFER, 0);
            }
            Ok(())
        })?;
        Ok(())
    }
}
