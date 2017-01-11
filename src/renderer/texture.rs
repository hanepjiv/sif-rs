// -*- mode:rust; coding:utf-8-unix; -*-

//! texture.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/06
//  @date 2017/01/10

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use ::std::os::raw::c_void;
// ----------------------------------------------------------------------------
use ::gl::types::*;
// ----------------------------------------------------------------------------
use super::super::error::Error;
use super::{ gl_result, GLError, TBind, };
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// max_texture_size
pub fn max_texture_size() -> Result<GLint, GLError<GLint, ()>> {
    gl_result(|| -> Result<_, ()> { unsafe {
        let mut s: GLint = 0;
        ::gl::GetIntegerv(::gl::MAX_TEXTURE_SIZE, &mut s);
        Ok(s)
    } })
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Texture
#[derive( Debug, Clone, )]
pub struct Texture {
    /// id
    id:                 GLuint,
    /// target
    target:             GLenum,
    /// format
    format:             GLenum,
    /// type_
    type_:              GLenum,
}
// ============================================================================
impl Texture {
    // ========================================================================
    /// new_2d
    pub fn new_2d(wrap_s:       GLenum,         wrap_t:         GLenum,
                  filter_mag:   GLenum,         filter_min:     GLenum,
                  mipmap:       bool,
                  format:       GLenum,         type_:          GLenum,
                  width:        GLsizei,        height:         GLsizei,
                  pixels:       *const c_void) -> Result<Self, Error> {
        match gl_result(|| -> Result<GLuint, ()> { unsafe {
            let mut id = 0;
            ::gl::GenTextures(1, &mut id);
            Ok(id)
        } }) {
            Err(_)      => {
                Err(Error::SifError(String::from("Texture::new_2d")))
            },
            Ok(id)      => {
                let texture = Texture {
                    id:         id,
                    target:     ::gl::TEXTURE_2D,
                    format:     format,
                    type_:      type_,
                };
                texture.tex_image_2d(wrap_s, wrap_t, filter_mag, filter_min,
                                     mipmap, width, height, pixels);
                Ok(texture)
            },
        }
    }
    // ========================================================================
    /// open_2d
    pub fn open_2d<P>(wrap_s:           GLenum, wrap_t:         GLenum,
                      filter_mag:       GLenum, filter_min:     GLenum,
                      mipmap:           bool,   path:           P)
                      -> Result<Self, Error>
        where P: AsRef<::std::path::Path> {
        let i = ::image::imageops::flip_vertical(
            &::image::open(path).expect("Texture::open_2d").to_rgba());
        Texture::new_2d(wrap_s, wrap_t, filter_mag, filter_min,
                        mipmap,
                        ::gl::RGBA, ::gl::UNSIGNED_BYTE,
                        i.width() as GLint, i.height() as GLint,
                        i.into_raw().as_ptr() as *const _ as
                        *const c_void)
    }
    // ========================================================================
    /// tex_image_2d
    fn tex_image_2d(&self,
                    wrap_s:     GLenum,         wrap_t:         GLenum,
                    filter_mag: GLenum,         filter_min:     GLenum,
                    mipmap:     bool,
                    width:      GLsizei,        height:         GLsizei,
                    pixels:     *const c_void) {
        debug_assert_eq!(::gl::TEXTURE_2D, self.target);

        let mut pxs = pixels;
        let buffer: Box<Vec<u32>>;
        if pxs == ::std::ptr::null() {
            buffer = Box::new(vec![0; width as usize * height as usize]);
            pxs = buffer.as_ptr() as *const c_void;
        }

        let _binder = self.binder();
        gl_result(|| -> Result<(), ()> { unsafe {
            ::gl::TexParameteri(::gl::TEXTURE_2D, ::gl::TEXTURE_WRAP_S,
                                wrap_s          as GLint);
            ::gl::TexParameteri(::gl::TEXTURE_2D, ::gl::TEXTURE_WRAP_T,
                                wrap_t          as GLint);
            ::gl::TexParameteri(::gl::TEXTURE_2D, ::gl::TEXTURE_MAG_FILTER,
                                filter_mag      as GLint);
            ::gl::TexParameteri(::gl::TEXTURE_2D, ::gl::TEXTURE_MIN_FILTER,
                                filter_min      as GLint);
            Ok(())
        } }).expect("Texture::tex_image_2d: TexParameteri");
        gl_result(|| -> Result<(), ()> { unsafe {
            Ok(::gl::TexImage2D(self.target, 0, self.format as GLint,
                                width, height, 0,
                                self.format, self.type_, pxs))
        } }).expect("Texture::tex_image_2d: TexImage2D");
        if mipmap {
            gl_result(|| -> Result<(), ()> { unsafe {
                Ok(::gl::GenerateMipmap(::gl::TEXTURE_2D))
            } }).expect("Texture::tex_image_2d: GenerateMipmap");
        }
    }
    // ========================================================================
    /// sub_image_2d
    pub fn sub_image_2d(&self,
                        level:          GLint,
                        xoffset:        GLint,
                        yoffset:        GLint,
                        width:          GLsizei,
                        height:         GLsizei,
                        pixels:         *const c_void)
                        -> Result<(), GLError<(), ()>> {
        debug_assert_eq!(::gl::TEXTURE_2D, self.target);
        let _binder = self.binder();
        gl_result(|| -> Result<(), ()> { unsafe {
            Ok(::gl::TexSubImage2D(self.target, level,
                                   xoffset, yoffset, width, height,
                                   self.format, self.type_, pixels))
        } })
    }
}
// ============================================================================
impl Drop for Texture {
    fn drop(&mut self) {
        gl_result(|| -> Result<(), ()> { unsafe {
            Ok(::gl::DeleteTextures(1, &self.id))
        } }).expect("Texture::drop");
    }
}
// ============================================================================
impl TBind for Texture {
    // ========================================================================
    fn id(&self) -> GLuint { self.id }
    // ========================================================================
    fn bind(&self) {
        gl_result(|| -> Result<(), ()> { unsafe {
            Ok(::gl::BindTexture(self.target, self.id))
        } }).expect("Texture::bind");
    }
    // ========================================================================
    fn unbind(&self) {
        gl_result(|| -> Result<(), ()> { unsafe {
            Ok(::gl::BindTexture(self.target, 0))
        } }).expect("Texture::unbind");
    }
}
