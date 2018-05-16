// -*- mode:rust; coding:utf-8-unix; -*-

//! square_buffer.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/03/06
//  @date 2018/05/16

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::mem::size_of;
// ----------------------------------------------------------------------------
use gl::types::*;
// ----------------------------------------------------------------------------
use sif_renderer::{Buffer, Program};
// ----------------------------------------------------------------------------
use super::Result;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
pub(crate) const VERSION: &str = r##"#version 100
#line 2 0

#ifndef GL_ES
# define GL_ES
#endif

#ifdef GL_ES
precision       mediump         float;
precision       mediump         int;
# define        EPSILON         (0.03125)                // sqrt(pow(2, -10));
#else
precision       highp           float;
precision       highp           int;
# define        EPSILON         (0.00034526698300124393) // sqrt(pow(2, -23));
#endif
"##;
// ============================================================================
pub(crate) const UNIFORM: &str = r##"
#line 2 3

// ////////////////////////////////////////////////////////////////////////////
// varying  ===================================================================
varying         vec2            vf_Coord;
"##;
// ============================================================================
pub(crate) const VERTEX: &str = r##"
#line 2 4

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
attribute       vec2            iv_Coord;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
void main(void) {
        vf_Coord                = iv_Coord;
        gl_Position             = vec4(iv_Coord * 2.0 - 1.0, -1.0, 1.0);
}
"##;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct SquareBuffer
#[derive(Debug)]
pub struct SquareBuffer {
    /// vertices
    vertices: Buffer,
}
// ============================================================================
impl SquareBuffer {
    // ========================================================================
    /// new
    pub fn new() -> Result<Self> {
        Ok(SquareBuffer {
            vertices: Buffer::new_vertices(
                &[0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0],
                ::gl::STATIC_DRAW,
            )?,
        })
    }
    // ========================================================================
    /// set_vertices
    pub fn set_vertices(&self, program: &Program) -> Result<&Self> {
        Program::set_attribute(
            sif_renderer_program_location!(program, "iv_Coord"),
            &self.vertices,
            2,
            ::gl::FLOAT,
            ::gl::FALSE,
            2 * size_of::<GLfloat>(),
            0, // 0 * size_of::<GLfloat>(),
        )?;
        Ok(self)
    }
    // ========================================================================
    /// draw
    pub fn draw(&self) -> Result<&Self> {
        let _ = self.vertices.draw_arrays(::gl::TRIANGLES, 0, 6)?;
        Ok(self)
    }
}
