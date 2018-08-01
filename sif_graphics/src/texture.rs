// -*- mode:rust; coding:utf-8-unix; -*-

//! texture.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/01/06
//  @date 2018/07/31

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::{cell::RefCell, convert::Into, os::raw::c_void, path::Path, rc::Rc};
// ----------------------------------------------------------------------------
use gl::types::*;
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_renderer::Texture as RendererTexture;
// ----------------------------------------------------------------------------
use super::Result;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Texture
#[derive(Debug, Clone)]
pub struct Texture {
    /// uuid
    uuid: Uuid,
    /// name
    name: String,
    /// inner
    inner: Rc<RefCell<RendererTexture>>,
    /// wrap
    wrap: [GLenum; 3],
    /// filter
    filter: [GLenum; 2],
    /// mipmap
    mipmap: bool,
}
// ============================================================================
impl Texture {
    // ========================================================================
    /// new_2d
    pub fn new_2d(
        uuid: Uuid,
        name: impl Into<String>,
        wrap_s: GLenum,
        wrap_t: GLenum,
        filter_mag: GLenum,
        filter_min: GLenum,
        mipmap: bool,
        format: GLenum,
        type_: GLenum,
        width: GLsizei,
        height: GLsizei,
        pixels: *const c_void,
    ) -> Result<Self> {
        Ok(Texture {
            uuid,
            name: name.into(),
            inner: Rc::new(RefCell::new(RendererTexture::new_2d(
                wrap_s, wrap_t, filter_mag, filter_min, mipmap, format, type_,
                width, height, pixels,
            )?)),
            wrap: [wrap_s, wrap_t, 0],
            filter: [filter_mag, filter_min],
            mipmap,
        })
    }
    // ========================================================================
    /// open_2d
    pub fn open_2d(
        uuid: Uuid,
        name: impl Into<String>,
        wrap_s: GLenum,
        wrap_t: GLenum,
        filter_mag: GLenum,
        filter_min: GLenum,
        mipmap: bool,
        path: impl AsRef<Path>,
    ) -> Result<Self> {
        Ok(Texture {
            uuid,
            name: name.into(),
            inner: Rc::new(RefCell::new(RendererTexture::open_2d(
                wrap_s, wrap_t, filter_mag, filter_min, mipmap, path,
            )?)),
            wrap: [wrap_s, wrap_t, 0],
            filter: [filter_mag, filter_min],
            mipmap,
        })
    }
}
// ============================================================================
impl AsRef<Uuid> for Texture {
    fn as_ref(&self) -> &Uuid {
        &self.uuid
    }
}
// ============================================================================
impl AsRef<String> for Texture {
    fn as_ref(&self) -> &String {
        &self.name
    }
}
// ============================================================================
impl AsRef<::std::cell::RefCell<RendererTexture>> for Texture {
    fn as_ref(&self) -> &RefCell<RendererTexture> {
        self.inner.as_ref()
    }
}
