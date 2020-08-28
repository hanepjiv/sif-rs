// -*- mode:rust; coding:utf-8-unix; -*-

//! pass.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/02/13
//  @date 2020/03/19

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::result::Result as StdResult;
// ----------------------------------------------------------------------------
use gl::types::*;
// ----------------------------------------------------------------------------
use sif_renderer::{
    gl_result, sif_renderer_program_location, Bind, Frame, Program, ShaderSrc,
    Texture,
};
// ----------------------------------------------------------------------------
use super::{
    super::square_buffer::{SquareBuffer, UNIFORM, VERSION, VERTEX},
    Effect, EffectArgs, Error, Result,
};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
const FRAGMENT: &str = r##"
#line 2 5
// ////////////////////////////////////////////////////////////////////////////
// uniform  ===================================================================
uniform         sampler2D       u_Texture;
// ////////////////////////////////////////////////////////////////////////////
// const  =====================================================================
const vec3 COLOR_WHITE          = vec3(1.0);
const vec3 COLOR_SEPIA          = vec3(0.48, 0.33, 0.19);
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
float intensity(const in vec3 color) {
        return (color.r * 0.30 + color.g * 0.58 + color.b * 0.12);
}
// ============================================================================
vec3 mono3(const in vec3 color, const in vec3 modulate) {
        return vec3(intensity(color)) * modulate;
}
// ----------------------------------------------------------------------------
vec4 mono4(const in vec4 color, const in vec3 modulate) {
        return vec4(mono3(color.rgb, modulate), color.a);
}
// ============================================================================
vec4 color_shift(const in sampler2D a_Texture,
                 const in vec2 a_Coord, const in vec2 a_Shift) {
        vec4 ret                = texture2D(a_Texture, a_Coord);
        ret.r                   = texture2D(a_Texture, a_Coord - a_Shift).r;
        ret.b                   = texture2D(a_Texture, a_Coord + a_Shift).b;
        return ret;
}
// ============================================================================
void main(void) {
//  float f             = rgba2float(texture2D(u_Texture, vf_Coord));
//  gl_FragData[0]      = vec4(vec3(f), 1.0);

  gl_FragData[0]      = texture2D(u_Texture, vf_Coord);

//  gl_FragData[0]      = texture2D(u_Texture, floor(vf_Coord*256.0) / 256.0);

//  gl_FragData[0]      = color_shift(u_Texture, vf_Coord, vec2(0.0039));

//  gl_FragData[0]      = mono4(texture2D(u_Texture, vf_Coord), COLOR_SEPIA);
}
"##;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Pass
#[derive(Debug)]
pub struct Pass {
    /// program
    program: Program,
}
// ============================================================================
impl Pass {
    // ========================================================================
    /// new
    pub fn new() -> Result<Self> {
        let common = include_str!("../../assets/glsl/common.glsl");
        Ok(Pass {
            program: Program::new(&[
                ShaderSrc::new(
                    ::gl::VERTEX_SHADER,
                    vec![VERSION, common, UNIFORM, VERTEX],
                ),
                ShaderSrc::new(
                    ::gl::FRAGMENT_SHADER,
                    vec![VERSION, common, UNIFORM, FRAGMENT],
                ),
            ])?,
        })
    }
}
// ============================================================================
impl Effect for Pass {
    // ========================================================================
    /// draw
    fn draw(
        &self,
        frame: Option<&Frame>,
        a_x: GLsizei,
        a_y: GLsizei,
        a_width: GLsizei,
        a_height: GLsizei,
        square_buffer: &SquareBuffer,
        texture: &Texture,
        effect_args: &EffectArgs,
    ) -> Result<&Self> {
        if let EffectArgs::Pass = *effect_args {
            gl_result(|| -> StdResult<(), ()> {
                unsafe {
                    ::gl::Viewport(a_x, a_y, a_width, a_height);
                    ::gl::Disable(::gl::BLEND);
                    ::gl::Disable(::gl::DEPTH_TEST);
                    ::gl::DepthMask(::gl::FALSE);
                    Ok(())
                }
            })?;
            self.program.bind_with(|| {
                Program::set_texture(
                    sif_renderer_program_location!(self.program, "u_Texture"),
                    0,
                    texture,
                )?;
                let _ = square_buffer.set_vertices(&self.program)?;
                if let Some(f) = frame {
                    let _ = f.bind_with(|| square_buffer.draw())?;
                } else {
                    let _ = square_buffer.draw()?;
                }
                Ok(self)
            })
        } else {
            Err(Error::InvalidArg(
                "::graphics::post::effect::blur::Pass".to_string(),
            ))
        }
    }
}
