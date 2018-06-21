// -*- mode:rust; coding:utf-8-unix; -*-

//! glyph.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/27
//  @date 2018/06/18

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use gl::types::*;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Glyph
#[derive(Debug, Clone, Copy)]
pub struct Glyph {
    /// char
    c: char,
    /// coords
    pub coords: [[GLfloat; 2]; 2],
    /// texid
    pub texid: usize,
    /// advance
    pub advance: GLfloat,
}
// ============================================================================
impl Default for Glyph {
    // ========================================================================
    fn default() -> Self {
        Glyph {
            c: '\0',
            coords: [[0.0, 0.0], [1.0, 1.0]],
            texid: 0,
            advance: 0.0,
        }
    }
}
// ============================================================================
impl Glyph {
    // ========================================================================
    pub fn new(c: char) -> Self {
        Glyph {
            c,
            ..Glyph::default()
        }
    }
}
