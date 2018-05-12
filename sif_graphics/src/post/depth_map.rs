// -*- mode:rust; coding:utf-8-unix; -*-

//! depth_map.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/02/13
//  @date 2018/05/12

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use gl::types::*;
// ----------------------------------------------------------------------------
use sif_manager::ManagedValue;
use sif_math::Matrix4x4;
use sif_renderer::{gl_result, Bind, Program, ShaderSrc, Texture};
use sif_three::NodeHolder;
// ----------------------------------------------------------------------------
use super::{Result, Screen, super::Object};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
const VERSION: &str = r##"#version 100
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
const UNIFORM: &str = r##"
#line 2 1

// ////////////////////////////////////////////////////////////////////////////
// struct  ====================================================================
struct Material {
  float                         alpha;
  bool                          is_map_diffuse;
};
// ////////////////////////////////////////////////////////////////////////////
// uniform  ===================================================================
uniform         float           u_Near;
uniform         float           u_Far;
uniform         mat4            u_Mat4_ProjViewModel;
uniform         bool            u_Skinning;
uniform         mat4            u_Bones[BONE_MAX_NUM];
uniform         Material        u_Material;
// ----------------------------------------------------------------------------
uniform         sampler2D       u_Texture_Diffuse;              // 00;
// ////////////////////////////////////////////////////////////////////////////
// varying  ===================================================================
varying         vec2            vf_Coord;
varying         float           vf_Depth;
"##;
// ============================================================================
const VERTEX: &str = r##"
#line 2 2

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
attribute       vec3            iv_Position;
attribute       vec2            iv_Coord;
attribute       vec4            iv_BoneIdx;
attribute       vec4            iv_Weight;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
void main(void) {
        vf_Coord        = iv_Coord;
        gl_Position     = vec4(iv_Position, 1.0);
        if (u_Skinning && iv_BoneIdx[0] >= 0.0) {
          gl_Position = skinning(u_Bones, iv_BoneIdx, iv_Weight) * gl_Position;
        }
        gl_Position     = u_Mat4_ProjViewModel * gl_Position;
        vf_Depth        = (gl_Position.w - u_Near) / (u_Far - u_Near);
}
"##;
// ============================================================================
const FRAGMENT: &str = r##"
#line 2 3

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
void main(void) {
        float alpha = u_Material.alpha;
        alpha *= (u_Material.is_map_diffuse ?
                        texture2D(u_Texture_Diffuse, vf_Coord).a : 1.0);
        if (alpha < EPSILON) {
                discard;
        }
        gl_FragData[0] = float2rgba(vf_Depth);
}
"##;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct DepthMapParam
#[derive(Debug, Clone, Copy)]
pub struct DepthMapParam {
    /// near
    pub near: GLfloat,
    /// far
    pub far: GLfloat,
    /// projection * view matrix
    pub mat4_proj_view: Matrix4x4<GLfloat>,
}
// ============================================================================
impl Default for DepthMapParam {
    // ========================================================================
    fn default() -> Self {
        DepthMapParam {
            near: 0.1,
            far: 100.0,
            mat4_proj_view: Matrix4x4::<GLfloat>::default(),
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct DepthMap
#[derive(Debug)]
pub struct DepthMap {
    /// screen
    screen: Screen,
}
// ============================================================================
impl DepthMap {
    // ========================================================================
    /// new_program
    pub fn new_program() -> Result<Program> {
        let common = include_str!("../assets/glsl/common.glsl");
        Ok(Program::new(&[
            ShaderSrc::new(
                ::gl::VERTEX_SHADER,
                vec![VERSION, common, UNIFORM, VERTEX],
            ),
            ShaderSrc::new(
                ::gl::FRAGMENT_SHADER,
                vec![VERSION, common, UNIFORM, FRAGMENT],
            ),
        ])?)
    }
    // ========================================================================
    /// new
    pub fn new(width: GLsizei, height: GLsizei) -> Result<Self> {
        Ok(DepthMap {
            screen: Screen::new(
                ::gl::CLAMP_TO_EDGE,
                ::gl::CLAMP_TO_EDGE,
                ::gl::NEAREST,
                ::gl::NEAREST,
                false,
                width,
                height,
                1,
                true,
            )?,
        })
    }
    // ========================================================================
    /// emit
    pub fn emit(
        &self,
        program: &Program,
        param: &DepthMapParam,
        managed_obj: &ManagedValue<Object>,
    ) -> Result<&Self> {
        program.bind_with(|| {
            Program::set_uniform1f(
                sif_renderer_program_location!(program, "u_Near"),
                param.near,
            );
            Program::set_uniform1f(
                sif_renderer_program_location!(program, "u_Far"),
                param.far,
            );
            let mut obj = managed_obj.as_ref().borrow_mut();
            if let Ok(rc) = obj.as_node() {
                let node = &*rc.borrow();
                Program::set_uniform_matrix4fv(
                    sif_renderer_program_location!(
                        program,
                        "u_Mat4_ProjViewModel"
                    ),
                    1,
                    ::gl::FALSE,
                    (param.mat4_proj_view * *node.as_matrix()).as_ptr(),
                );
            }
            obj.draw_silhouette(program)?;
            Ok(self)
        })
    }
    // ========================================================================
    /// size
    pub fn size(&self) -> &[GLint; 2] {
        self.screen.size()
    }
    // ========================================================================
    /// as_color
    pub fn as_color(&self) -> &Texture {
        self.screen.as_color(0)
    }
}
// ============================================================================
impl Bind for DepthMap {
    // ========================================================================
    fn id(&self) -> GLuint {
        panic!("::graphics::DepthMap: No id.");
    }
    // ========================================================================
    fn bind(&self) {
        self.screen.bind();
        unwrap!(gl_result(|| -> Result<()> {
            unsafe {
                ::gl::Enable(::gl::DEPTH_TEST);
                ::gl::DepthFunc(::gl::LEQUAL);
                ::gl::DepthMask(::gl::TRUE);
                ::gl::DepthRangef(0.0, 1.0);
                ::gl::Disable(::gl::BLEND);
                ::gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                ::gl::Clear(::gl::COLOR_BUFFER_BIT | ::gl::DEPTH_BUFFER_BIT);
            }
            Ok(())
        }));
    }
    // ------------------------------------------------------------------------
    fn unbind(&self) {
        self.screen.unbind();
    }
}
