// -*- mode:rust; coding:utf-8-unix; -*-

//! buffer.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/06
//  @date 2018/05/12

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use gl::types::*;
use std::result::Result as StdResult;
// ----------------------------------------------------------------------------
use super::{gl_result, Bind, GLResult, Result};
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
    pub fn new_vertices<T: ?Sized + AsRef<[GLfloat]>>(
        data: &T,
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
    pub fn new_indices<T: ?Sized + AsRef<[GLuint]>>(
        data: &T,
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
    ) -> GLResult<(), ()> {
        gl_result(|| -> StdResult<(), ()> {
            self.bind_with(|| {
                ::gl::BufferSubData(
                    self.target,
                    offset as GLintptr,
                    size as GLsizeiptr,
                    data as *const GLvoid,
                );
                Ok(())
            })
        })
    }
    // ========================================================================
    /// draw_elements
    pub fn draw_elements(
        &self,
        mode: GLenum,
        count: GLsizei,
    ) -> GLResult<(), ()> {
        gl_result(|| -> StdResult<(), ()> {
            self.bind_with(|| {
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
        })
    }
    // ========================================================================
    /// draw_arrays
    pub fn draw_arrays(
        &self,
        mode: GLenum,
        first: GLint,
        count: GLsizei,
    ) -> GLResult<(), ()> {
        gl_result(|| -> StdResult<(), ()> {
            self.bind_with(|| {
                unsafe { ::gl::DrawArrays(mode, first, count) }
                Ok(())
            })
        })
    }
}
// ============================================================================
impl Drop for Buffer {
    fn drop(&mut self) {
        gl_result(|| -> StdResult<(), ()> {
            unsafe { ::gl::DeleteBuffers(1, &self.id) }
            Ok(())
        }).expect("Buffer::drop");
    }
}
// ============================================================================
impl Bind for Buffer {
    // ========================================================================
    fn id(&self) -> GLuint {
        self.id
    }
    // ========================================================================
    fn bind(&self) {
        gl_result(|| -> Result<()> {
            unsafe { ::gl::BindBuffer(self.target, self.id) }
            Ok(())
        }).expect("Buffer::bind");
    }
    // ========================================================================
    fn unbind(&self) {
        gl_result(|| -> Result<()> {
            unsafe { ::gl::BindBuffer(self.target, 0) }
            Ok(())
        }).expect("Buffer::unbind");
    }
}