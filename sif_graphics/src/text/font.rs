// -*- mode:rust; coding:utf-8-unix; -*-

//! font.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/27
//  @date 2018/05/23

// ////////////////////////////////////////////////////////////////////////////
// const  =====================================================================
const PADDING: GLsizei = 3;
// use  =======================================================================
use std::{
    borrow::Borrow,
    collections::BTreeMap,
    fmt::{Debug, Formatter},
    hash::Hash,
};
// ----------------------------------------------------------------------------
use gl::types::*;
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_math::Matrix4x4;
use sif_renderer::Texture;
use sif_three::new_mat4_tra;
// ----------------------------------------------------------------------------
use super::{
    super::{Error, Result},
    glyph::Glyph,
    Metal,
};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// trait TFontReserve
pub trait TFontReserve: ::std::fmt::Debug {
    // ========================================================================
    /// reserve
    fn reserve(&self, layer: &mut Font);
}
// ============================================================================
impl TFontReserve for str {
    fn reserve(&self, layer: &mut Font) {
        layer.reserve_str(self)
    }
}
// ============================================================================
impl TFontReserve for String {
    fn reserve(&self, layer: &mut Font) {
        layer.reserve_str(self)
    }
}
// ============================================================================
impl TFontReserve for char {
    fn reserve(&self, layer: &mut Font) {
        layer.reserve_char(self)
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Font
pub struct Font<'a, 'b> {
    /// uuid
    uuid: Uuid,
    /// ttf_font
    ttf_font: ::sdl2::ttf::Font<'a, 'b>,
    /// glyphs
    glyphs: BTreeMap<char, Glyph>,
    /// textures
    textures: Vec<Texture>,
    /// added
    added: Vec<char>,
    /// cursor
    cursor: (GLsizei, GLsizei),
    /// width
    width: GLsizei,
    /// height
    height: GLsizei,
}
// ============================================================================
impl<'a, 'b> Debug for Font<'a, 'b> {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        write!(f, "graphics::text::Font{{ uuid: {:?}, .. }}", self.uuid)
    }
}
// ============================================================================
impl<'a, 'b> AsRef<Uuid> for Font<'a, 'b> {
    fn as_ref(&self) -> &Uuid {
        &self.uuid
    }
}
// ============================================================================
impl<'a, 'b> Font<'a, 'b> {
    // ========================================================================
    /// new
    pub fn new(
        uuid: Uuid,
        ttf_font: ::sdl2::ttf::Font<'a, 'b>,
        width: GLsizei,
        height: GLsizei,
    ) -> Result<Self> {
        Ok(Font {
            uuid,
            ttf_font,
            glyphs: BTreeMap::new(),
            textures: vec![Self::new_texture(width, height)?],
            added: Vec::new(),
            cursor: (PADDING, PADDING),
            width,
            height,
        })
    }
    // ========================================================================
    /// new_texture
    fn new_texture(width: GLsizei, height: GLsizei) -> Result<Texture> {
        Ok(Texture::new_2d(
            ::gl::CLAMP_TO_EDGE,
            ::gl::CLAMP_TO_EDGE,
            ::gl::LINEAR,
            ::gl::LINEAR,
            false,
            ::gl::RGBA,
            ::gl::UNSIGNED_BYTE,
            width,
            height,
            ::std::ptr::null(),
        )?)
    }
    // ========================================================================
    /// texture_height
    pub fn texture_height(&self) -> GLsizei {
        self.height
    }
    // ------------------------------------------------------------------------
    /// texture_width
    pub fn texture_width(&self) -> GLsizei {
        self.width
    }
    // ========================================================================
    /// ttf_height
    pub fn ttf_height(&self) -> GLsizei {
        self.ttf_font.height() as GLsizei
    }
    // ------------------------------------------------------------------------
    /// ttf_line_spacing
    pub fn ttf_line_spacing(&self) -> GLsizei {
        self.ttf_font.recommended_line_spacing() as GLsizei
    }
    // ------------------------------------------------------------------------
    /// ttf_descent
    pub fn ttf_descent(&self) -> GLsizei {
        self.ttf_font.descent() as GLsizei
    }
    // ========================================================================
    /// glyph
    pub fn glyph<C>(&self, c: &C) -> Option<&Glyph>
    where
        char: Borrow<C>,
        C: ?Sized + Hash + Ord,
    {
        self.glyphs.get(c)
    }
    // ========================================================================
    /// reserve
    pub fn reserve<S>(&mut self, s: &S)
    where
        S: ?Sized + TFontReserve,
    {
        s.reserve(self)
    }
    // ------------------------------------------------------------------------
    /// reserve_str
    pub fn reserve_str<S>(&mut self, s: &S)
    where
        S: ?Sized + AsRef<str>,
    {
        for c in s.as_ref().chars() {
            self.reserve_char(&c);
        }
    }
    // ------------------------------------------------------------------------
    /// reserve_char
    pub fn reserve_char(&mut self, c: &char) {
        if self.glyphs.contains_key(c) {
            return;
        }
        self.added.push(*c)
    }
    // ========================================================================
    /// update
    pub fn update(&mut self) -> Result<&mut Self> {
        if self.added.is_empty() {
            return Ok(self);
        }
        self.added.shrink_to_fit();

        let ttf_height = self.ttf_height();

        while let Some(ref c) = self.added.pop() {
            let mut s = String::new();
            s.push(*c);
            let surface = self
                .ttf_font
                .render(&s)
                .blended(::sdl2::pixels::Color::RGBA(0xFF, 0xFF, 0xFF, 0xFF))?;
            let rect = surface.rect();

            if self.width < (self.cursor.0 + rect.width() as GLsizei + PADDING)
            {
                self.cursor.0 = PADDING;
                self.cursor.1 += ttf_height + PADDING;
                if self.height
                    < (self.cursor.1 + ttf_height as GLsizei + PADDING)
                {
                    self.cursor.1 = PADDING;
                    self.textures
                        .push(Self::new_texture(self.width, self.height)?);
                }
            }

            let _ = surface.with_lock(|pxs| -> Result<&Texture> {
                Ok(self
                    .textures
                    .last()
                    .ok_or_else(|| {
                        Error::OptNone(
                            "graphics::text::font::update: self.textures.last"
                                .to_string(),
                        )
                    })?
                    .sub_image_2d(
                        0,
                        self.cursor.0,
                        self.cursor.1,
                        rect.width() as GLsizei,
                        rect.height() as GLsizei,
                        pxs.as_ptr() as *const ::std::os::raw::c_void,
                    )?)
            })?;

            {
                // glyph
                let metrics =
                    self.ttf_font.find_glyph_metrics(*c).ok_or_else(|| {
                        Error::OptNone(
                            "graphics::text::font::update: find_glyph_metrics"
                                .to_string(),
                        )
                    })?;
                let mut glyph = Glyph::new(c);
                glyph.texid = self.textures.len() - 1;
                glyph.coords = [
                    [
                        (self.cursor.0 as GLfloat) / self.width as GLfloat,
                        ((self.cursor.1 as GLfloat + rect.height() as GLfloat)
                            / self.height as GLfloat),
                    ],
                    [
                        ((self.cursor.0 as GLfloat + rect.width() as GLfloat)
                            / self.width as GLfloat),
                        (self.cursor.1 as GLfloat) / self.height as GLfloat,
                    ],
                ];
                glyph.advance =
                    metrics.advance as GLfloat / ttf_height as GLfloat;
                let _ = self.glyphs.insert(*c, glyph);
            }

            self.cursor.0 += rect.width() as GLsizei + PADDING;
        }

        Ok(self)
    }
    // ========================================================================
    /// advance
    pub fn advance<C>(&self, c: &C) -> Option<GLfloat>
    where
        char: Borrow<C>,
        C: ?Sized + Hash + Ord,
    {
        self.glyph(c).map(|g| -> GLfloat { g.advance })
    }
    // ========================================================================
    /// draw
    pub fn draw<C>(
        &mut self,
        c: &C,
        metal: &Metal,
        color: Option<&[[GLfloat; 4]; 4]>,
        matrix: &Matrix4x4<GLfloat>,
    ) -> Result<GLfloat>
    where
        char: Borrow<C>,
        C: ?Sized + Hash + Ord,
    {
        let _ = self.update()?;
        let glyph = self.glyph(c).ok_or_else(|| {
            Error::OptNone(
                "graphics::text::font::Font::draw: glyph ".to_string(),
            )
        })?;
        let d = self.ttf_descent() as GLfloat / self.ttf_height() as GLfloat;
        let _ = metal.draw(
            &self.textures[glyph.texid],
            &glyph.coords,
            color,
            &(*matrix * new_mat4_tra(0.0, d, 0.0)),
        );
        Ok(glyph.advance)
    }
}
