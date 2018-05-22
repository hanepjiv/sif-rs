// -*- mode:rust; coding:utf-8-unix; -*-

//! buffer.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/06
//  @date 2018/05/23

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use gl::types::*;
// ----------------------------------------------------------------------------
use super::{gl_result, Bind, Error, Result};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Buffer
#[derive(Debug)]
pub struct Buffer {
    /// id
    id: GLuint,
    /// target
    target: GLenum,
    /// usage
    usage: GLenum,
}
// ============================================================================
impl Buffer {
    // ========================================================================
    /// new
    pub fn new(
        target: GLenum,
        size: usize,
        data: *const GLvoid,
        usage: GLenum,
    ) -> Result<Self> {
        let buffer = Buffer {
            id: gl_result(|| -> Result<GLuint> {
                let mut id = 0;
                unsafe {
                    ::gl::GenBuffers(1, &mut id);
                }
                Ok(id)
            })?,
            target,
            usage,
        };
        buffer.bind_with(|| -> Result<()> {
            gl_result(|| -> Result<()> {
                unsafe {
                    ::gl::BufferData(target, size as GLsizeiptr, data, usage)
                }
                Ok(())
            })?;
            Ok(())
        })?;
        Ok(buffer)
    }
    // ========================================================================
    /// new_vertices
    pub fn new_vertices(
        data: impl AsRef<[GLfloat]>,
        usage: GLenum,
    ) -> Result<Self> {
        Buffer::new(
            ::gl::ARRAY_BUFFER,
            data.as_ref().len() * ::std::mem::size_of::<GLfloat>(),
            data.as_ref() as *const _ as *const GLvoid,
            usage,
        )
    }
    // ========================================================================
    /// new_indices
    pub fn new_indices(
        data: impl AsRef<[GLuint]>,
        usage: GLenum,
    ) -> Result<Self> {
        Buffer::new(
            ::gl::ELEMENT_ARRAY_BUFFER,
            data.as_ref().len() * ::std::mem::size_of::<GLuint>(),
            data.as_ref() as *const _ as *const GLvoid,
            usage,
        )
    }
    // ========================================================================
    /// sub_data
    pub unsafe fn sub_data<T>(
        &self,
        offset: isize,
        size: usize,
        data: *const T,
    ) -> Result<&Self> {
        gl_result(|| -> Result<()> {
            self.bind_with(|| -> Result<()> {
                ::gl::BufferSubData(
                    self.target,
                    offset as GLintptr,
                    size as GLsizeiptr,
                    data as *const GLvoid,
                );
                Ok(())
            })
        })?;
        Ok(self)
    }
    // ========================================================================
    /// draw_elements
    pub fn draw_elements(
        &self,
        mode: GLenum,
        count: GLsizei,
    ) -> Result<&Self> {
        gl_result(|| -> Result<()> {
            self.bind_with(|| -> Result<()> {
                unsafe {
                    ::gl::DrawElements(
                        mode,
                        count,
                        ::gl::UNSIGNED_INT,
                        ::std::ptr::null(),
                    )
                }
                Ok(())
            })
        })?;
        Ok(self)
    }
    // ========================================================================
    /// draw_arrays
    pub fn draw_arrays(
        &self,
        mode: GLenum,
        first: GLint,
        count: GLsizei,
    ) -> Result<&Self> {
        gl_result(|| -> Result<()> {
            self.bind_with(|| {
                unsafe { ::gl::DrawArrays(mode, first, count) }
                Ok(())
            })
        })?;
        Ok(self)
    }
}
// ============================================================================
impl Drop for Buffer {
    fn drop(&mut self) {
        gl_result(|| -> Result<()> {
            unsafe { ::gl::DeleteBuffers(1, &self.id) }
            Ok(())
        }).expect("Buffer::drop");
    }
}
// ============================================================================
impl Bind for Buffer {
    // ========================================================================
    type BindError = Error;
    // ========================================================================
    fn id(&self) -> GLuint {
        self.id
    }
    // ========================================================================
    fn bind(&self) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe { ::gl::BindBuffer(self.target, self.id) }
            Ok(())
        })?;
        Ok(())
    }
    // ========================================================================
    fn unbind(&self) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe { ::gl::BindBuffer(self.target, 0) }
            Ok(())
        })?;
        Ok(())
    }
}
