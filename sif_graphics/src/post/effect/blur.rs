// -*- mode:rust; coding:utf-8-unix; -*-

//! blur.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/01/19
//  @date 2018/05/12

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use gl::types::*;
use num::Float;
// ----------------------------------------------------------------------------
use sif_renderer::{gl_result, Bind, Frame, Program, ShaderSrc, Texture};
// ----------------------------------------------------------------------------
use super::{
    super::square_buffer::{SquareBuffer, UNIFORM, VERSION, VERTEX}, Effect,
    EffectArgs, Result,
};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
const FRAGMENT: &str = r##"
#line 2 5

// uniform  ===================================================================
#define         LIMIT_DISTANCE  (16)
uniform         sampler2D       u_Texture;
uniform         vec2            u_InvResolution;
uniform         int             u_Distance;     // [0, 16)
uniform         int             u_Direction;    // 0: Vertical, 1: Horizontal
uniform         float           u_Gauss[LIMIT_DISTANCE];
uniform         float           u_InvWeight;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
void main(void) {
        vec2    l_Offset         = u_InvResolution;
        l_Offset[u_Direction]    = 0.0;
        gl_FragData[0]           = texture2D(u_Texture, vf_Coord) *
                                   u_Gauss[0] * u_InvWeight;
        for (int i = u_Distance; i > 0; --i) {
                float l_Weight   = u_Gauss[i];
                l_Weight        *= u_InvWeight;
                gl_FragData[0]  +=
                        texture2D(u_Texture,vf_Coord-(float(i)*l_Offset)) *
                        l_Weight;
                gl_FragData[0]  +=
                        texture2D(u_Texture,vf_Coord+(float(i)*l_Offset)) *
                        l_Weight;
        }
}
"##;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
const LIMIT_DISTANCE: u8 = 16;
// ============================================================================
/// struct Blur
#[derive(Debug)]
pub struct Blur {
    /// program
    program: Program,
    /// color0
    color0: Texture,
    /// frame
    frame: Frame,
    /// size
    size: [GLsizei; 2],
    /// gaussian
    gaussian: Vec<GLfloat>,
}
// ============================================================================
impl Blur {
    // ========================================================================
    /// new
    pub fn new(width: GLsizei, height: GLsizei) -> Result<Self> {
        let frame = Frame::new();
        let color0 = Texture::new_2d(
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
        )?;
        frame.attach_2d(::gl::COLOR_ATTACHMENT0, ::gl::TEXTURE_2D, &color0)?;
        let mut gaussian = vec![0.0; LIMIT_DISTANCE as usize];
        {
            let sigma = 5.0;
            let s2 = sigma * sigma * 2.0;
            let c = 1.0 / Float::sqrt(::std::f32::consts::PI * s2);
            for i in 0..LIMIT_DISTANCE {
                gaussian[i as usize] =
                    c * Float::exp((GLfloat::from(i * i)) / -s2);
            }
        }
        let common = include_str!("../../assets/glsl/common.glsl");
        Ok(Blur {
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
            color0,
            frame,
            size: [width, height],
            gaussian,
        })
    }
    // ========================================================================
    /// size
    pub fn size(&self) -> &[GLint; 2] {
        &self.size
    }
}
// ============================================================================
impl Effect for Blur {
    // ========================================================================
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
        if let EffectArgs::Blur { distance } = *effect_args {
            gl_result(|| -> Result<()> {
                unsafe {
                    ::gl::Disable(::gl::BLEND);
                    ::gl::Disable(::gl::DEPTH_TEST);
                    ::gl::DepthMask(::gl::FALSE);
                }
                Ok(())
            })?;
            self.program.bind_with(|| {
                // vertices
                square_buffer.set_vertices(&self.program);

                // common params
                {
                    // distance
                    let d = if distance >= LIMIT_DISTANCE {
                        LIMIT_DISTANCE - 1
                    } else {
                        distance
                    };

                    Program::set_uniform1i(
                        sif_renderer_program_location!(
                            self.program,
                            "u_Distance"
                        ),
                        GLint::from(d),
                    );

                    // gauss
                    Program::set_uniform1fv(
                        sif_renderer_program_location!(
                            self.program,
                            "u_Gauss[0]"
                        ),
                        1 + GLint::from(d),
                        self.gaussian.as_ptr(),
                    );

                    // weight
                    {
                        let mut weight = self.gaussian[0];
                        for i in 1..1 + d {
                            weight += self.gaussian[i as usize] * 2.0;
                        }
                        Program::set_uniform1f(
                            sif_renderer_program_location!(
                                self.program,
                                "u_InvWeight"
                            ),
                            1.0 / weight,
                        );
                    }
                }

                // 1st pass to texture
                {
                    Program::set_uniform2f(
                        sif_renderer_program_location!(
                            self.program,
                            "u_InvResolution"
                        ),
                        1.0 / (self.size[0] as GLfloat),
                        1.0 / (self.size[1] as GLfloat),
                    );
                    Program::set_uniform1i(
                        sif_renderer_program_location!(
                            self.program,
                            "u_Direction"
                        ),
                        0,
                    );
                    Program::set_texture(
                        sif_renderer_program_location!(
                            self.program,
                            "u_Texture"
                        ),
                        0,
                        texture,
                    );
                    let _ = self.frame.bind_with(|| {
                        gl_result(|| -> Result<()> {
                            unsafe {
                                ::gl::Viewport(
                                    0,
                                    0,
                                    self.size[0],
                                    self.size[1],
                                );
                            }
                            Ok(())
                        })?;
                        square_buffer.draw()
                    })?;
                }

                // 2nd pass to frame (or screen)
                {
                    Program::set_uniform2f(
                        sif_renderer_program_location!(
                            self.program,
                            "u_InvResolution"
                        ),
                        1.0 / (a_width as GLfloat),
                        1.0 / (a_height as GLfloat),
                    );
                    Program::set_uniform1i(
                        sif_renderer_program_location!(
                            self.program,
                            "u_Direction"
                        ),
                        1,
                    );
                    Program::set_texture(
                        sif_renderer_program_location!(
                            self.program,
                            "u_Texture"
                        ),
                        0,
                        &self.color0,
                    );
                    let draw = move || {
                        gl_result(|| -> Result<()> {
                            unsafe {
                                ::gl::Viewport(a_x, a_y, a_width, a_height);
                            }
                            Ok(())
                        })?;
                        square_buffer.draw()
                    };
                    if let Some(f) = frame {
                        let _ = f.bind_with(draw)?;
                    } else {
                        let _ = draw()?;
                    }
                }
                Ok(self)
            })
        } else {
            panic!("::graphics::post::effect::blur::Blur: invalid args.");
        }
    }
}
