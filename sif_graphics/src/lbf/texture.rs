// -*- mode:rust; coding:utf-8-unix; -*-

//! texture.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/07/31
//  @date 2018/08/01

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::{marker::PhantomData, path::PathBuf};
// ----------------------------------------------------------------------------
use gl::types::*;
use uuid::Uuid;
// ----------------------------------------------------------------------------
use super::{
    Error, GraphicsResult, GraphicsScene, GraphicsTexture, Image,
    IntoGraphics, Manager,
};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Texture
#[derive(Debug, Clone)]
pub struct Texture<'a> {
    /// uuid
    uuid: Uuid,
    /// name
    name: String,
    /// image
    image: Uuid,
    /// wrap
    wrap: [GLenum; 3],
    /// filter
    filter: [GLenum; 2],
    /// mipmap
    mipmap: bool,
    /// phantom
    phantom: PhantomData<&'a ()>,
}
// ============================================================================
impl<'a> Texture<'a> {
    // ========================================================================
    /// new
    pub fn new(
        uuid: Uuid,
        name: impl Into<String>,
        image: Uuid,
        wrap_s: GLenum,
        wrap_t: GLenum,
        filter_mag: GLenum,
        filter_min: GLenum,
        mipmap: bool,
    ) -> Self {
        Texture {
            uuid,
            name: name.into(),
            image: image,
            wrap: [wrap_s, wrap_t, 0],
            filter: [filter_mag, filter_min],
            mipmap,
            phantom: PhantomData::default(),
        }
    }
}
// ============================================================================
impl<'a> AsRef<Uuid> for Texture<'a> {
    fn as_ref(&self) -> &Uuid {
        &self.uuid
    }
}
// ============================================================================
impl<'a> AsRef<String> for Texture<'a> {
    fn as_ref(&self) -> &String {
        &self.name
    }
}
// ============================================================================
impl<'a> IntoGraphics for Texture<'a> {
    type Target = GraphicsTexture;
    type Param = &'a Manager<Image>;
    // ========================================================================
    fn into_graphics(
        self,
        scene: &GraphicsScene,
        images: Self::Param,
    ) -> GraphicsResult<Self::Target> {
        let image = {
            if let Some(x) = images.get(&self.image) {
                Some(x)
            } else {
                AsRef::<Manager<Image>>::as_ref(scene).get(&self.image)
            }
        }.ok_or_else(|| {
            Error::OptNone("lbf::Texture: into_graphics".to_string())
        })?.as_ref()
        .borrow();
        if let Image::File(ref img) = *image {
            match *img.as_dimension() {
                2 => GraphicsTexture::open_2d(
                    *AsRef::<Uuid>::as_ref(&self),
                    AsRef::<String>::as_ref(&self).clone(),
                    self.wrap[0],
                    self.wrap[1],
                    self.filter[0],
                    self.filter[1],
                    self.mipmap,
                    AsRef::<PathBuf>::as_ref(img),
                ),
                x => Err(Error::ImageDimension(
                    format!("lbf::Texture: into_graphics: image has invalid dimension {}", x)
                        .to_string()
                ).into()),
            }
        } else {
            Err(Error::ImageUnknown(
                "lbf::Texture: into_graphics: unknown image".to_string(),
            ).into())
        }
    }
}
