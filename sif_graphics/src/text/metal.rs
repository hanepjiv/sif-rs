// -*- mode:rust; coding:utf-8-unix; -*-

//! metal.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/06/20
//  @date 2018/05/16

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::{mem::size_of, result::Result as StdResult};
// ----------------------------------------------------------------------------
use gl::types::*;
// ----------------------------------------------------------------------------
use sif_math::Matrix4x4;
use sif_renderer::{gl_result, Bind, Buffer, Program, ShaderSrc, Texture};
// ----------------------------------------------------------------------------
use super::Result;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
#[allow(missing_docs, unreachable_pub)]
bitflags! {
    #[allow(missing_docs, unreachable_pub)]
    pub struct Flags: u32 {
    #[allow(missing_docs, unreachable_pub)]
    const DIRTY = 0b0000_0000_0000_0000_0000_0000_0001_0000u32;
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
const VERSION: &str = r##"#version 100
#line 2 0
#ifndef GL_ES
#       define GL_ES
#endif

precision       mediump         float;
precision       mediump         int;
# define        EPSILON         (0.03125)                // sqrt(pow(2, -10));
"##;
// ============================================================================
const VARYING: &str = r##"
#line 2 3
// varying  ===================================================================
varying mediump vec2            vf_Coord;
varying mediump vec4            vf_Color;
"##;
// ============================================================================
const VERTEX: &str = r##"
#line 2 4
// uniform  ===================================================================
uniform         mediump mat4    u_Matrix;
uniform         mediump float   u_Aspect;
// attribute  =================================================================
attribute       vec3            iv_Position;
attribute       vec2            iv_Coord;
attribute       vec4            iv_Color;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
void main(void) {
        vf_Coord          = iv_Coord;
        vf_Color          = iv_Color;
        gl_Position       = vec4(iv_Position, 1.0);
        gl_Position.x    *= u_Aspect;
        gl_Position       = u_Matrix * gl_Position;
}
"##;
// ============================================================================
const FRAGMENT: &str = r##"
#line 2 5
// uniform  ===================================================================
uniform         sampler2D       u_Texture;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
void main(void) {
        gl_FragData[0]  = vf_Color * texture2D(u_Texture, vf_Coord);
}
"##;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Metal
#[derive(Debug)]
pub struct Metal {
    /// program
    program: Program,
    /// positions:
    positions: Buffer,
    /// coords:
    coords: Buffer,
    /// colors:
    colors: Buffer,
    /// indices:
    indices: Buffer,
}
// ============================================================================
impl Metal {
    // ========================================================================
    /// new
    pub fn new() -> Result<Self> {
        Ok(Metal {
            program: Program::new(&[
                ShaderSrc::new(
                    ::gl::VERTEX_SHADER,
                    vec![VERSION, VARYING, VERTEX],
                ),
                ShaderSrc::new(
                    ::gl::FRAGMENT_SHADER,
                    vec![VERSION, VARYING, FRAGMENT],
                ),
            ])?,
            positions: Buffer::new_vertices(
                &[
                    0.0, 0.0, -0.1, 1.0, 0.0, -0.1, 1.0, 1.0, -0.1, 0.0, 1.0,
                    -0.1,
                ],
                ::gl::STATIC_DRAW,
            )?,
            coords: Buffer::new_vertices(
                &[0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0],
                ::gl::DYNAMIC_DRAW,
            )?,
            colors: Buffer::new_vertices(
                &[
                    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
                    1.0, 1.0, 1.0, 1.0, 1.0,
                ],
                ::gl::DYNAMIC_DRAW,
            )?,
            indices: Buffer::new_indices(
                &[0, 1, 2, 0, 2, 3],
                ::gl::STATIC_DRAW,
            )?,
        })
    }
    // ========================================================================
    /// draw
    pub fn draw(
        &self,
        texture: &Texture,
        coords: &[[GLfloat; 2]; 2],
        color: Option<&[[GLfloat; 4]; 4]>,
        matrix: &Matrix4x4<GLfloat>,
    ) -> Result<()> {
        gl_result(|| -> StdResult<(), ()> {
            unsafe {
                ::gl::Enable(::gl::BLEND);
                ::gl::BlendFunc(::gl::SRC_ALPHA, ::gl::ONE_MINUS_SRC_ALPHA);
                ::gl::FrontFace(::gl::CCW);
                ::gl::Enable(::gl::CULL_FACE);
                ::gl::CullFace(::gl::BACK);
                ::gl::Disable(::gl::DEPTH_TEST);
                ::gl::DepthFunc(::gl::LEQUAL);
                ::gl::DepthMask(::gl::FALSE);
            }
            Ok(())
        })?;

        unsafe {
            let _ = self.coords.sub_data(
                0,
                4 * 2 * size_of::<GLfloat>(),
                &[
                    coords[0][0],
                    coords[0][1],
                    coords[1][0],
                    coords[0][1],
                    coords[1][0],
                    coords[1][1],
                    coords[0][0],
                    coords[1][1],
                ] as *const _,
            )?;
        }

        if let Some(c) = color {
            unsafe {
                let _ = self.colors.sub_data(
                    0,
                    4 * 4 * size_of::<GLfloat>(),
                    c as *const _,
                )?;
            }
        }

        self.program.bind_with(|| {
            Program::set_uniform_matrix4fv(
                sif_renderer_program_location!(self.program, "u_Matrix"),
                1,
                ::gl::FALSE,
                matrix.as_ptr(),
            )?;
            Program::set_uniform1f(
                sif_renderer_program_location!(self.program, "u_Aspect"),
                (coords[1][0] - coords[0][0]) / (coords[0][1] - coords[1][1]),
            )?;
            Program::set_texture(
                sif_renderer_program_location!(self.program, "u_Texture"),
                0,
                texture,
            )?;
            Program::set_attribute(
                sif_renderer_program_location!(self.program, "iv_Position"),
                &self.positions,
                3,
                ::gl::FLOAT,
                ::gl::FALSE,
                3 * size_of::<GLfloat>(),
                0, // 0 * size_of::<GLfloat>(),
            )?;
            Program::set_attribute(
                sif_renderer_program_location!(self.program, "iv_Coord"),
                &self.coords,
                2,
                ::gl::FLOAT,
                ::gl::FALSE,
                2 * size_of::<GLfloat>(),
                0, // 0 * size_of::<GLfloat>(),
            )?;
            Program::set_attribute(
                sif_renderer_program_location!(self.program, "iv_Color"),
                &self.colors,
                4,
                ::gl::FLOAT,
                ::gl::FALSE,
                4 * size_of::<GLfloat>(),
                0, // 0 * size_of::<GLfloat>(),
            )?;
            let _ = self.indices.draw_elements(::gl::TRIANGLES, 6)?;
            Ok(())
        })
    }
}
