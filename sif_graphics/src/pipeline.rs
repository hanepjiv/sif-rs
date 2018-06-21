// -*- mode:rust; coding:utf-8-unix; -*-

//! pipeline.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/18
//  @date 2018/06/17

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::{
    borrow::Borrow, cell::RefCell, f32::consts::PI, hash::Hash,
    os::raw::c_void,
};
// ----------------------------------------------------------------------------
use gl::types::*;
// ----------------------------------------------------------------------------
use sif_manager::ManagedValue;
use sif_math::{Matrix4x4, Vector3, Vector4};
use sif_renderer::{Bind, Program, ShaderSrc, Texture as RendererTexture};
use sif_three::NodeHolder;
// ----------------------------------------------------------------------------
use super::{
    light, post::DepthMap, Camera, ColorExponent, ColorIntensity, Error,
    Object, ObjectData, Result,
};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// const PIPELINE_MAX_BONE
pub const PIPELINE_MAX_BONE: usize = 64;
// ----------------------------------------------------------------------------
/// const PIPELINE_MAX_LIGHT
pub const PIPELINE_MAX_LIGHT: usize = 16;
// ============================================================================
const VERSION: &str = r##"#version 100
#ifndef GL_ES
#       define GL_ES
#endif
"##;
// ============================================================================
// Bayer Matrix
const BAYER_MATRIX: [u8; 16 * 16] = [
    0, 128, 32, 160, 8, 136, 40, 168, 2, 130, 34, 162, 10, 138, 42, 170, 192,
    64, 224, 96, 200, 72, 232, 104, 194, 66, 226, 98, 202, 74, 234, 106, 48,
    176, 16, 144, 56, 184, 24, 152, 50, 178, 18, 146, 58, 186, 26, 154, 240,
    112, 208, 80, 248, 120, 216, 88, 242, 114, 210, 82, 250, 122, 218, 90, 12,
    140, 44, 172, 4, 132, 36, 164, 14, 142, 46, 174, 6, 134, 38, 166, 204, 76,
    236, 108, 196, 68, 228, 100, 206, 78, 238, 110, 198, 70, 230, 102, 60,
    188, 28, 156, 52, 180, 20, 148, 62, 190, 30, 158, 54, 182, 22, 150, 252,
    124, 220, 92, 244, 116, 212, 84, 254, 126, 222, 94, 246, 118, 214, 86, 3,
    131, 35, 163, 11, 139, 43, 171, 1, 129, 33, 161, 9, 137, 41, 169, 195, 67,
    227, 99, 203, 75, 235, 107, 193, 65, 225, 97, 201, 73, 233, 105, 51, 179,
    19, 147, 59, 187, 27, 155, 49, 177, 17, 145, 57, 185, 25, 153, 243, 115,
    211, 83, 251, 123, 219, 91, 241, 113, 209, 81, 249, 121, 217, 89, 15, 143,
    47, 175, 7, 135, 39, 167, 13, 141, 45, 173, 5, 133, 37, 165, 207, 79, 239,
    111, 199, 71, 231, 103, 205, 77, 237, 109, 197, 69, 229, 101, 63, 191, 31,
    159, 55, 183, 23, 151, 61, 189, 29, 157, 53, 181, 21, 149, 255, 127, 223,
    95, 247, 119, 215, 87, 253, 125, 221, 93, 245, 117, 213, 85,
];
// ============================================================================
// Texture_ShadowMap
const TEXTURE_SHADOWMAP: [&str; PIPELINE_MAX_LIGHT] = [
    "u_Texture_ShadowMap_00",
    "u_Texture_ShadowMap_01",
    "u_Texture_ShadowMap_02",
    "u_Texture_ShadowMap_03",
    "u_Texture_ShadowMap_04",
    "u_Texture_ShadowMap_05",
    "u_Texture_ShadowMap_06",
    "u_Texture_ShadowMap_07",
    "u_Texture_ShadowMap_08",
    "u_Texture_ShadowMap_09",
    "u_Texture_ShadowMap_10",
    "u_Texture_ShadowMap_11",
    "u_Texture_ShadowMap_12",
    "u_Texture_ShadowMap_13",
    "u_Texture_ShadowMap_14",
    "u_Texture_ShadowMap_15",
];
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// Flags
bitflags! {
    #[allow(missing_docs)]
    pub struct Flags: u32 {
    #[allow(missing_docs)]
    const AMBIENT               = 0b0000_0000_0000_0000_0000_0000_0000_0001u32;
    #[allow(missing_docs)]
    const FOG                   = 0b0000_0000_0000_0000_0000_0000_0000_0010u32;
    #[allow(missing_docs)]
    const INK                   = 0b0000_0000_0000_0000_0000_0000_0000_0100u32;
    #[allow(missing_docs)]
    const RIM                   = 0b0000_0000_0000_0000_0000_0000_0000_1000u32;
    #[allow(missing_docs)]
    const DIRTY_LIGHTS          = 0b0000_0000_0000_0000_0000_0000_0001_0000u32;
    #[allow(missing_docs)]
    const DIRTY_AMBIENTS        = 0b0000_0000_0000_0000_0000_0000_0010_0000u32;
    #[allow(missing_docs)]
    const DO_NOT_USE            = 0b1000_0000_0000_0000_0000_0000_0000_0000u32;
    }
}
// ============================================================================
impl Default for Flags {
    fn default() -> Self {
        Flags::AMBIENT
            | Flags::FOG
            | Flags::INK
            | Flags::RIM
            | Flags::DIRTY_AMBIENTS
            | Flags::DIRTY_LIGHTS
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct PipelineParam
#[derive(Debug)]
pub struct PipelineParam {
    /// camera
    pub camera: Option<ManagedValue<Object>>,
    /// lights
    lights: Vec<Option<ManagedValue<Object>>>,
    /// model
    pub model: Option<ManagedValue<Object>>,
    /// ambient
    pub ambient: ColorIntensity,
    /// ink
    pub ink: ColorExponent,
    /// rim
    pub rim: ColorExponent,
    /// flags
    pub flags: Flags,
}
// ============================================================================
impl Default for PipelineParam {
    // ========================================================================
    fn default() -> Self {
        PipelineParam {
            camera: None,
            lights: vec![
                None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None,
            ],
            model: None,
            ambient: ColorIntensity::new(1.0, 1.0, 1.0, 0.2),
            ink: ColorExponent::new(1.0, 1.0, 1.0, 12.0),
            rim: ColorExponent::new(1.0, 1.0, 1.0, 4.0),
            flags: Flags::default(),
        }
    }
}
// ============================================================================
impl AsRef<Vec<Option<ManagedValue<Object>>>> for PipelineParam {
    fn as_ref(&self) -> &Vec<Option<ManagedValue<Object>>> {
        &self.lights
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Vec<Option<ManagedValue<Object>>>> for PipelineParam {
    fn as_mut(&mut self) -> &mut Vec<Option<ManagedValue<Object>>> {
        self.flags.insert(Flags::DIRTY_LIGHTS);
        &mut self.lights
    }
}
// ============================================================================
impl PipelineParam {
    // ========================================================================
    /// as_lights
    pub fn as_lights(&self) -> &Vec<Option<ManagedValue<Object>>> {
        self.as_ref()
    }
    // ------------------------------------------------------------------------
    /// as_lights_mut
    pub fn as_lights_mut(&mut self) -> &mut Vec<Option<ManagedValue<Object>>> {
        self.as_mut()
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Pipeline
#[derive(Debug)]
pub struct Pipeline {
    /// Program
    program: Program,
    /// depth_map_program
    depth_map_program: Program,
    /// bayer_matrix
    bayer_matrix: RendererTexture,
}
// ============================================================================
impl Pipeline {
    // ========================================================================
    /// new
    pub fn new() -> Result<Self> {
        let common = include_str!("./assets/glsl/common.glsl");
        let uniform = include_str!("./assets/glsl/pipeline/uniform.glsl");
        Ok(Pipeline {
            program: Program::new(&[
                ShaderSrc::new(
                    ::gl::VERTEX_SHADER,
                    vec![
                        VERSION,
                        common,
                        uniform,
                        include_str!("./assets/glsl/pipeline/vertex.glsl"),
                    ],
                ),
                ShaderSrc::new(
                    ::gl::FRAGMENT_SHADER,
                    vec![
                        VERSION,
                        common,
                        uniform,
                        include_str!("./assets/glsl/pipeline/fragment.glsl"),
                    ],
                ),
            ])?,
            depth_map_program: DepthMap::new_program()?,
            bayer_matrix: RendererTexture::new_2d(
                ::gl::REPEAT,
                ::gl::REPEAT,
                ::gl::NEAREST,
                ::gl::NEAREST,
                false,
                ::gl::RGBA,
                ::gl::UNSIGNED_BYTE,
                8,
                8,
                BAYER_MATRIX.as_ptr() as *const c_void,
            )?,
        })
    }
    // ========================================================================
    /// set_matrix4
    fn set_matrix4<Q>(
        &self,
        name: &Q,
        matrix: &Matrix4x4<GLfloat>,
    ) -> Result<&Self>
    where
        String: Borrow<Q>,
        Q: ?Sized + Hash + Ord,
    {
        Program::set_uniform_matrix4fv(
            sif_renderer_program_location!(self.program, name),
            1,
            ::gl::FALSE,
            matrix.as_ptr(),
        )?;
        Ok(self)
    }
    // ========================================================================
    /// set_light
    fn set_light(
        &self,
        i: usize,
        opt: &Option<ManagedValue<Object>>,
        mat4_view: &Matrix4x4<GLfloat>,
    ) -> Result<&Self> {
        if if let Some(ref managed_object) = *opt {
            let obj = managed_object.as_ref().borrow();
            if let ObjectData::Light(ref managed_light) = *obj.as_ref() {
                {
                    let pos = *mat4_view
                        * Vector4::from_vector3(&obj.position()?, 1.0);
                    Program::set_uniform3fv(
                        sif_renderer_program_location!(
                            self.program,
                            "u_Lights[{}].position",
                            i
                        ),
                        1,
                        pos.as_ptr(),
                    )?;
                }
                {
                    let mut dir = Vector3::from(
                        *mat4_view * Vector4::from_vector3(&obj.front()?, 0.0),
                    );
                    let _ = dir.normalize();
                    Program::set_uniform3fv(
                        sif_renderer_program_location!(
                            self.program,
                            "u_Lights[{}].direction",
                            i
                        ),
                        1,
                        dir.as_ptr(),
                    )?;
                }
                {
                    let l = managed_light.as_ref().borrow();
                    {
                        let is_enable = l.flags.contains(light::Flags::ENABLE);
                        Program::set_uniform1i(
                            sif_renderer_program_location!(
                                self.program,
                                "u_Lights[{}].is_enable",
                                i
                            ),
                            if is_enable { 1 } else { 0 },
                        )?;
                        if !is_enable {
                            return Ok(self);
                        }
                    }
                    Program::set_uniform1i(
                        sif_renderer_program_location!(
                            self.program,
                            "u_Lights[{}].is_point",
                            i
                        ),
                        if l.flags.contains(light::Flags::POINT) {
                            1
                        } else {
                            0
                        },
                    )?;
                    Program::set_uniform3fv(
                        sif_renderer_program_location!(
                            self.program,
                            "u_Lights[{}].color",
                            i
                        ),
                        1,
                        (l.color * l.intensity).as_ptr(),
                    )?;

                    Program::set_uniform1f(
                        sif_renderer_program_location!(
                            self.program,
                            "u_Lights[{}].exponent",
                            i
                        ),
                        l.exponent,
                    )?;
                    Program::set_uniform3fv(
                        sif_renderer_program_location!(
                            self.program,
                            "u_Lights[{}].kcklkq",
                            i
                        ),
                        1,
                        l.kcklkq.as_ptr(),
                    )?;
                    Program::set_uniform1f(
                        sif_renderer_program_location!(
                            self.program,
                            "u_Lights[{}].cutoff",
                            i
                        ),
                        if l.flags.contains(light::Flags::SPOT) {
                            l.cutoff
                        } else {
                            -1.0
                        },
                    )?;

                    if l.flags.contains(light::Flags::SHADOW) {
                        Program::set_uniform1i(
                            sif_renderer_program_location!(
                                self.program,
                                "u_Lights[{}].is_shadow",
                                i
                            ),
                            1,
                        )?;
                        Program::set_texture(
                            sif_renderer_program_location!(
                                self.program,
                                TEXTURE_SHADOWMAP[i]
                            ),
                            (15 + i) as GLint,
                            l.as_shadow_color(),
                        )?;
                        let shadow_param = l.as_shadow_param();
                        {
                            let mat4_proj = Camera::frustum(
                                shadow_param.near,
                                shadow_param.far,
                                Camera::alpha2focus(PI / 180.0 * 90.0),
                                1.0,
                            );
                            let mat4_light = {
                                let n = obj.as_node()?.borrow_mut();
                                *n.as_inverse_matrix()
                            };
                            Program::set_uniform_matrix4fv(
                                sif_renderer_program_location!(
                                    self.program,
                                    "u_Lights[{}].Mat4_ProjLight",
                                    i
                                ),
                                1,
                                ::gl::FALSE,
                                (mat4_proj * mat4_light).as_ptr(),
                            )?;
                        }
                        Program::set_uniform1f(
                            sif_renderer_program_location!(
                                self.program,
                                "u_Lights[{}].shadow_near",
                                i
                            ),
                            shadow_param.near,
                        )?;
                        Program::set_uniform1f(
                            sif_renderer_program_location!(
                                self.program,
                                "u_Lights[{}].shadow_far",
                                i
                            ),
                            shadow_param.far,
                        )?;
                    } else {
                        Program::set_uniform1i(
                            sif_renderer_program_location!(
                                self.program,
                                "u_Lights[{}].is_shadow",
                                i
                            ),
                            0,
                        )?;
                    }
                }
                false
            } else {
                true
            }
        } else {
            true
        } {
            Program::set_uniform1i(
                sif_renderer_program_location!(
                    self.program,
                    "u_Lights[{}].is_enable",
                    i
                ),
                0,
            )?;
        }
        Ok(self)
    }
    // ========================================================================
    /// emit
    pub fn emit(&mut self, param: &PipelineParam) -> Result<()> {
        self.program.bind_with(|| -> Result<()> {
            {
                // BayerMatrix
                Program::set_texture(
                    sif_renderer_program_location!(
                        self.program,
                        "u_Texture_BayerMatrix"
                    ),
                    4,
                    &self.bayer_matrix,
                )?;
                let mut shift = ::rand::random::<[GLfloat; 2]>();
                shift[0] *= 16.0;
                shift[1] *= 16.0;
                Program::set_uniform2fv(
                    sif_renderer_program_location!(
                        self.program,
                        "u_AlphaDitherShift"
                    ),
                    1,
                    shift.as_ptr(),
                )?;
            }
            {
                // ambient
                Program::set_uniform3fv(
                    sif_renderer_program_location!(
                        self.program,
                        "u_Ambient.color"
                    ),
                    1,
                    (param.ambient.color * param.ambient.intensity).as_ptr(),
                )?;
                Program::set_uniform1i(
                    sif_renderer_program_location!(
                        self.program,
                        "u_Ambient.is_enable"
                    ),
                    if param.flags.contains(Flags::AMBIENT) {
                        1
                    } else {
                        0
                    },
                )?;
                Program::set_uniform3fv(
                    sif_renderer_program_location!(
                        self.program,
                        "u_Ink.color"
                    ),
                    1,
                    param.ink.color.as_ptr(),
                )?;
                Program::set_uniform1f(
                    sif_renderer_program_location!(
                        self.program,
                        "u_Ink.exponent"
                    ),
                    if param.flags.contains(Flags::INK) {
                        param.ink.exponent
                    } else {
                        0.0
                    },
                )?;
                Program::set_uniform3fv(
                    sif_renderer_program_location!(
                        self.program,
                        "u_Rim.color"
                    ),
                    1,
                    param.rim.color.as_ptr(),
                )?;
                Program::set_uniform1f(
                    sif_renderer_program_location!(
                        self.program,
                        "u_Rim.exponent"
                    ),
                    if param.flags.contains(Flags::RIM) {
                        param.rim.exponent
                    } else {
                        0.0
                    },
                )?;
            }

            let view = if let Some(ref m) = param.camera {
                let obj = &*m.as_ref().borrow();
                {
                    let c = AsRef::<RefCell<Camera>>::as_ref(obj).borrow_mut();
                    let _ = self
                        .set_matrix4("u_Mat4_Proj", &c.projection_matrix())?;
                }

                let n = obj.as_node()?.borrow();
                // inverse_view    = camera matrix
                // self.set_matrix4("u_Mat4_Inverse_View", n.as_matrix());
                // view        = inverse camera matrix
                *n.as_inverse_matrix()
            } else {
                return Err(Error::InvalidArg(
                    "::graphics::pipeline::Pipeline::emit: no Camera"
                        .to_string(),
                ));
            };

            if param.flags.contains(Flags::DIRTY_LIGHTS) {
                // light
                for i in 0..param.lights.len() {
                    let _ = self.set_light(i, &param.lights[i], &view)?;
                }
            }

            if let Some(ref m) = param.model {
                // model
                let mut obj = m.as_ref().borrow_mut();
                if let Ok(rc) = obj.as_node() {
                    let n = &*rc.borrow();
                    let _ = self
                        .set_matrix4("u_Mat4_Model", n.as_matrix())?
                        .set_matrix4(
                            "u_Mat4_ViewModel",
                            &(view * *n.as_matrix()),
                        )?;
                }
                obj.draw(&self.program)?;
            }

            Ok(())
        })
    }
}
