// -*- mode:rust; coding:utf-8-unix; -*-

//! layer.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/27
//  @date 2018/06/18

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::f32::consts::PI;
// ----------------------------------------------------------------------------
use gl::types::*;
use num::Float;
// ----------------------------------------------------------------------------
use sif_manager::ManagedValue;
use sif_math::Matrix4x4;
use sif_three::{new_mat4_tra, new_mat4_trarotsca};
// ----------------------------------------------------------------------------
use super::{
    super::{Error, Result},
    Font, Metal, TFontReserve,
};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// trait TLayerAppend
pub trait TLayerAppend: ::std::fmt::Debug {
    // ========================================================================
    /// append
    fn append(&self, layer: &mut Layer);
}
// ============================================================================
impl TLayerAppend for str {
    fn append(&self, layer: &mut Layer) {
        layer.append_str(self)
    }
}
// ============================================================================
impl TLayerAppend for String {
    fn append(&self, layer: &mut Layer) {
        layer.append_str(self)
    }
}
// ============================================================================
impl TLayerAppend for char {
    fn append(&self, layer: &mut Layer) {
        layer.append_char(*self)
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Layer
#[derive(Debug)]
pub struct Layer<'a, 'b> {
    /// string
    string: String,
    /// font
    font: ManagedValue<Font<'a, 'b>>,
    /// width
    width: GLfloat,
    /// height
    height: GLfloat,
    /// size
    size: GLfloat,
    /// shift
    shift: GLfloat,
    /// step
    step: usize,
}
// ============================================================================
impl<'a, 'b> Layer<'a, 'b> {
    // ========================================================================
    /// new
    pub fn new(
        font: ManagedValue<Font<'a, 'b>>,
        width: GLfloat,
        height: GLfloat,
        size: GLfloat,
        shift: GLfloat,
        step: usize,
    ) -> Self {
        Layer {
            string: String::new(),
            font,
            width,
            height,
            size,
            shift,
            step,
        }
    }
    // ========================================================================
    /// font
    pub fn font(&self) -> ManagedValue<Font<'a, 'b>> {
        self.font.clone()
    }
    // ========================================================================
    /// width
    pub fn width(&self) -> GLfloat {
        self.width
    }
    // ------------------------------------------------------------------------
    /// height
    pub fn height(&self) -> GLfloat {
        self.height
    }
    // ------------------------------------------------------------------------
    /// set_width_height
    pub fn set_width_height(
        &mut self,
        width: GLfloat,
        height: GLfloat,
    ) -> &Self {
        self.width = width;
        self.height = height;
        self
    }
    // ========================================================================
    /// append
    pub fn append<S>(&mut self, s: &S)
    where
        S: ?Sized + TLayerAppend,
    {
        s.append(self)
    }
    // ------------------------------------------------------------------------
    /// append_str
    fn append_str<S>(&mut self, s: &S)
    where
        S: ?Sized + AsRef<str> + TFontReserve,
    {
        self.font.as_ref().borrow_mut().reserve(s);
        self.string.push_str(s.as_ref())
    }
    // ------------------------------------------------------------------------
    /// append_char
    fn append_char(&mut self, c: char) {
        self.font.as_ref().borrow_mut().reserve(&c);
        self.string.push(c)
    }
    // ========================================================================
    /// pop
    pub fn pop(&mut self) -> Option<char> {
        self.string.pop()
    }
    // ========================================================================
    /// clear
    pub fn clear(&mut self) {
        self.string.shrink_to_fit();
        self.string.clear()
    }
    // ========================================================================
    /// draw
    pub fn draw(
        &self,
        metal: &Metal,
        matrix: &Matrix4x4<GLfloat>,
    ) -> Result<&Self> {
        let mut x = self.shift;
        let mut y = -self.shift;
        let mut font = self.font.as_ref().borrow_mut();
        let _ = font.update()?;
        let space = font.ttf_line_spacing() as GLfloat * self.size
            / font.ttf_height() as GLfloat;
        let orign = (self.width / 2.0, self.height / 2.0 - space);
        for c in self.string.chars() {
            match c {
                '\r' => {}
                '\n' => {
                    x = self.shift;
                    y -= space;
                }
                _ => {
                    let adv = font.advance(&c).ok_or_else(|| {
                        Error::OptNone(
                            "graphics::text::layer::draw: font.advance"
                                .to_string(),
                        )
                    })? * self.size;
                    if self.width < (x + adv) {
                        x = self.shift;
                        y -= space;
                    }
                    let m0 = &new_mat4_trarotsca(
                        x - orign.0,
                        y + orign.1,
                        -1.0,
                        0.0,
                        0.0,
                        0.0,
                        1.0,
                        self.size,
                        self.size,
                        0.0,
                    );
                    for i in 0..self.step {
                        let rad =
                            2.0 * PI / self.step as GLfloat * i as GLfloat;
                        let _ = font.draw(
                            &c,
                            metal,
                            Some(&[
                                [0.1, 0.1, 0.1, 1.0],
                                [0.1, 0.1, 0.1, 1.0],
                                [0.1, 0.1, 0.1, 1.0],
                                [0.1, 0.1, 0.1, 1.0],
                            ]),
                            &(*matrix
                                * new_mat4_tra(
                                    self.shift * Float::cos(rad),
                                    self.shift * Float::sin(rad),
                                    -::std::f32::EPSILON.sqrt(),
                                ) * *m0),
                        );
                    }
                    let _ = font.draw(
                        &c,
                        metal,
                        Some(&[
                            [0.0, 0.9, 0.9, 1.0],
                            [0.9, 0.0, 0.9, 1.0],
                            [0.9, 0.9, 0.0, 1.0],
                            [0.0, 0.0, 0.9, 1.0],
                        ]),
                        &(*matrix * *m0),
                    );
                    x += adv;
                }
            }
        }
        Ok(self)
    }
}
