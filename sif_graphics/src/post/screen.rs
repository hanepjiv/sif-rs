// -*- mode:rust; coding:utf-8-unix; -*-

//! screen.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/01/17
//  @date 2018/05/12

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::result::Result as StdResult;
// ----------------------------------------------------------------------------
use gl::types::*;
// ----------------------------------------------------------------------------
use sif_renderer::{gl_result, Bind, Frame, Render, Texture};
// ----------------------------------------------------------------------------
use super::Result;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Screen
#[derive(Debug)]
pub struct Screen {
    /// size
    size: [GLsizei; 2],
    /// frame
    frame: Frame,
    /// colors
    colors: Vec<Texture>,
    /// depth
    depth: Option<Render>,
}
// ============================================================================
impl Screen {
    // ========================================================================
    /// new
    pub fn new(
        wrap_s: GLenum,
        wrap_t: GLenum,
        filter_mag: GLenum,
        filter_min: GLenum,
        mipmap: bool,
        width: GLsizei,
        height: GLsizei,
        size_colors: usize,
        is_depth: bool,
    ) -> Result<Self> {
        if 16 < size_colors {
            panic!("Screen::new: invalid size_color,");
        }
        let frame = Frame::new();
        let mut colors = Vec::default();
        for i in 0..size_colors {
            colors.push(Texture::new_2d(
                wrap_s,
                wrap_t,
                filter_mag,
                filter_min,
                mipmap,
                ::gl::RGBA,
                ::gl::UNSIGNED_BYTE,
                width,
                height,
                ::std::ptr::null(),
            )?);
            frame.attach_2d(
                ::gl::COLOR_ATTACHMENT0 + i as GLuint,
                ::gl::TEXTURE_2D,
                &colors[i],
            )?;
        }
        let depth = if is_depth {
            let depth = Render::new(::gl::DEPTH_COMPONENT16, width, height)?;
            frame.attach_render(::gl::DEPTH_ATTACHMENT, &depth)?;
            Some(depth)
        } else {
            None
        };
        Ok(Screen {
            size: [width, height],
            frame,
            colors,
            depth,
        })
    }
    // ========================================================================
    /// size
    pub fn size(&self) -> &[GLint; 2] {
        &self.size
    }
    // ========================================================================
    /// as_color
    pub fn as_color(&self, i: usize) -> &Texture {
        &self.colors[i]
    }
    // ------------------------------------------------------------------------
    /// as_depth
    pub fn as_depth(&self) -> &Render {
        if let Some(ref depth) = self.depth {
            depth
        } else {
            panic!("Screen::as_depth.");
        }
    }
}
// ============================================================================
impl AsRef<Frame> for Screen {
    fn as_ref(&self) -> &Frame {
        &self.frame
    }
}
// ============================================================================
impl Bind for Screen {
    // ========================================================================
    fn id(&self) -> GLuint {
        panic!("::graphics::Screen: No id.");
    }
    // ========================================================================
    fn bind(&self) {
        self.frame.bind();
        unwrap!(gl_result(|| -> StdResult<(), ()> {
            unsafe {
                ::gl::Viewport(0, 0, self.size[0], self.size[1]);
            }
            Ok(())
        }));
    }
    // ------------------------------------------------------------------------
    fn unbind(&self) {
        self.frame.unbind();
    }
}
