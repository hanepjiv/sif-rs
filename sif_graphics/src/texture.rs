// -*- mode:rust; coding:utf-8-unix; -*-

//! texture.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/01/06
//  @date 2018/06/16

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::{
    cell::RefCell,
    convert::Into,
    os::raw::c_void,
    path::{Path, PathBuf},
    rc::Rc,
};
// ----------------------------------------------------------------------------
use gl::types::*;
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_manager::Manager;
use sif_renderer::Texture as RendererTexture;
// ----------------------------------------------------------------------------
use super::{Error, Image, Result};
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
    inner: ::std::result::Result<Rc<RefCell<RendererTexture>>, Uuid>,
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
    /// new
    pub fn new(
        uuid: Uuid,
        name: impl Into<String>,
        wrap_s: GLenum,
        wrap_t: GLenum,
        filter_mag: GLenum,
        filter_min: GLenum,
        mipmap: bool,
        image: Uuid,
    ) -> Self {
        Texture {
            uuid,
            name: name.into(),
            inner: Err(image),
            wrap: [wrap_s, wrap_t, 0],
            filter: [filter_mag, filter_min],
            mipmap,
        }
    }
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
            inner: Ok(Rc::new(RefCell::new(RendererTexture::new_2d(
                wrap_s, wrap_t, filter_mag, filter_min, mipmap, format, type_,
                width, height, pixels,
            )?))),
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
            inner: Ok(Rc::new(RefCell::new(RendererTexture::open_2d(
                wrap_s, wrap_t, filter_mag, filter_min, mipmap, path,
            )?))),
            wrap: [wrap_s, wrap_t, 0],
            filter: [filter_mag, filter_min],
            mipmap,
        })
    }
    // ========================================================================
    /// from_lbf
    pub fn from_lbf(
        lbf_texture: Texture,
        images: &Manager<Image>,
    ) -> Result<Self> {
        if let Err(ref uuid) = lbf_texture.inner {
            let lbf_image = images
                .get(uuid)
                .ok_or_else(|| {
                    Error::OptNone(
                        "graphics: texture: from_lbf: images.get".to_string(),
                    )
                })?
                .as_ref()
                .borrow();
            if let Image::File(ref inner) = *lbf_image {
                match *inner.as_dimension() {
                    2 => Texture::open_2d(
                        *AsRef::<Uuid>::as_ref(&lbf_texture),
                        AsRef::<String>::as_ref(&lbf_texture).clone(),
                        lbf_texture.wrap[0],
                        lbf_texture.wrap[1],
                        lbf_texture.filter[0],
                        lbf_texture.filter[1],
                        lbf_texture.mipmap,
                        AsRef::<PathBuf>::as_ref(inner),
                    ),
                    _ => Err(Error::InvalidArg(
                        "graphics: texture: from_lbf: invalid image dimension"
                            .to_string(),
                    )),
                }
            } else {
                Err(Error::InvalidImage)
            }
        } else {
            Ok(lbf_texture)
        }
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
        if let Ok(ref rc) = self.inner {
            rc.as_ref()
        } else {
            panic!("AsRef<RendererTexture>>for texture: {:?}", self)
        }
    }
}
