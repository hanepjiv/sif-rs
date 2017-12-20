// -*- mode:rust; coding:utf-8-unix; -*-

//! buffer.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/06
//  @date 2017/04/20

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use gl::types::*;
// ----------------------------------------------------------------------------
use super::{gl_result, GLError, TBind};
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
    ) -> Result<Self, String> {
        let result_id = gl_result(|| -> Result<GLuint, ()> {
            unsafe {
                let mut id = 0;
                ::gl::GenBuffers(1, &mut id);
                Ok(id)
            }
        });

        let result_buffer = match result_id {
            Err(_) => Err(String::from("Buffer::new: ::gl::GenBuffers")),
            Ok(id) => Ok(Buffer {
                id: id,
                target: target,
                usage: usage,
            }),
        };

        match result_buffer {
            Ok(buffer) => match gl_result(|| -> Result<(), ()> {
                unsafe {
                    let _binder = buffer.binder();
                    Ok(::gl::BufferData(
                        target,
                        size as GLsizeiptr,
                        data,
                        usage,
                    ))
                }
            }) {
                Ok(_) => Ok(buffer),
                _ => Err(String::from("Buffer::new: ::gl::BufferData")),
            },
            _ => result_buffer,
        }
    }
    // ========================================================================
    /// new_vertices
    pub fn new_vertices<T: ?Sized + AsRef<[GLfloat]>>(
        data: &T,
        usage: GLenum,
    ) -> Result<Self, String> {
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
    ) -> Result<Self, String> {
        Buffer::new(
            ::gl::ELEMENT_ARRAY_BUFFER,
            data.as_ref().len() * ::std::mem::size_of::<GLuint>(),
            data.as_ref() as *const _ as *const GLvoid,
            usage,
        )
    }
    // ========================================================================
    /// sub_data
    pub fn sub_data<T>(&self, offset: isize, size: usize, data: *const T) {
        let _binder = self.binder();
        gl_result(|| -> Result<(), ()> {
            unsafe {
                Ok(::gl::BufferSubData(
                    self.target,
                    offset as GLintptr,
                    size as GLsizeiptr,
                    data as *const GLvoid,
                ))
            }
        }).expect("Buffer::sub_data");
    }
    // ========================================================================
    /// draw_elements
    pub fn draw_elements(
        &self,
        mode: GLenum,
        count: GLsizei,
    ) -> Result<(), GLError<(), ()>> {
        let _binder = self.binder();
        gl_result(|| -> Result<(), ()> {
            unsafe {
                Ok(::gl::DrawElements(
                    mode,
                    count,
                    ::gl::UNSIGNED_INT,
                    ::std::ptr::null(),
                ))
            }
        })
    }
    // ========================================================================
    /// draw_arrays
    pub fn draw_arrays(
        &self,
        mode: GLenum,
        first: GLint,
        count: GLsizei,
    ) -> Result<(), GLError<(), ()>> {
        let _binder = self.binder();
        gl_result(|| -> Result<(), ()> {
            unsafe { Ok(::gl::DrawArrays(mode, first, count)) }
        })
    }
}
// ============================================================================
impl Drop for Buffer {
    fn drop(&mut self) {
        gl_result(|| -> Result<(), ()> {
            unsafe { Ok(::gl::DeleteBuffers(1, &self.id)) }
        }).expect("Buffer::drop");
    }
}
// ============================================================================
impl TBind for Buffer {
    // ========================================================================
    fn id(&self) -> GLuint {
        self.id
    }
    // ========================================================================
    fn bind(&self) {
        gl_result(|| -> Result<(), ()> {
            unsafe { Ok(::gl::BindBuffer(self.target, self.id)) }
        }).expect("Buffer::bind");
    }
    // ========================================================================
    fn unbind(&self) {
        gl_result(|| -> Result<(), ()> {
            unsafe { Ok(::gl::BindBuffer(self.target, 0)) }
        }).expect("Buffer::unbind");
    }
}
