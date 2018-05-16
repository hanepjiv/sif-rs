// -*- mode:rust; coding:utf-8-unix; -*-

//! program.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/08
//  @date 2018/05/16

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::collections::BTreeMap;
// ----------------------------------------------------------------------------
use gl::types::*;
// ----------------------------------------------------------------------------
use super::{
    gl_result, info_log, Bind, Buffer, Error, Result, Shader, ShaderSrc,
    Texture,
};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Program
#[derive(Debug)]
pub struct Program {
    /// shaders
    shaders: Vec<Shader>,
    /// location map
    location_map: BTreeMap<String, GLint>,
    /// id
    id: GLuint,
}
// ============================================================================
impl Program {
    // ========================================================================
    /// new
    pub fn new(srcs: &[ShaderSrc]) -> Result<Self> {
        let id = gl_result(|| -> Result<GLuint> {
            Ok(unsafe { ::gl::CreateProgram() })
        })?;

        let mut shaders = Vec::new();
        for i in srcs {
            let shader = Shader::new(i)?;
            gl_result(|| -> Result<()> {
                unsafe { ::gl::AttachShader(id, shader.id()) }
                Ok(())
            })?;
            shaders.push(shader);
        }

        gl_result(|| -> Result<()> {
            unsafe { ::gl::LinkProgram(id) }
            Ok(())
        })?;

        info_log(::gl::PROGRAM, id, ::gl::LINK_STATUS)?;

        let mut location_map = BTreeMap::new();
        {
            // uniform
            let active = gl_result(|| -> Result<GLint> {
                let mut active = 0;
                unsafe {
                    ::gl::GetProgramiv(id, ::gl::ACTIVE_UNIFORMS, &mut active);
                }
                Ok(active)
            })?;

            let max_length = gl_result(|| -> Result<GLint> {
                let mut max_length = 0;
                unsafe {
                    ::gl::GetProgramiv(
                        id,
                        ::gl::ACTIVE_UNIFORM_MAX_LENGTH,
                        &mut max_length,
                    );
                }
                Ok(max_length)
            })?;

            for i in 0..active as GLuint {
                let name = gl_result(|| -> Result<String> {
                    let mut name = vec![0u8; max_length as usize];
                    let mut length = 0;
                    let mut s_ = 0;
                    let mut t_ = 0;
                    unsafe {
                        ::gl::GetActiveUniform(
                            id,
                            i,
                            max_length,
                            &mut length,
                            &mut s_,
                            &mut t_,
                            name.as_mut_ptr() as *mut GLchar,
                        );
                    }
                    name.resize(length as usize, 0u8);
                    Ok(String::from_utf8(name)?)
                })?;
                let location = gl_result(|| -> Result<GLint> {
                    Ok(unsafe {
                        ::gl::GetUniformLocation(
                            id,
                            name.as_ptr() as *const GLchar,
                        )
                    })
                })?;
                info!("Program::new: location: {:?} = {:?}", name, location);
                let _ = location_map.insert(name, location);
            }
        }
        {
            // attribute
            let active = gl_result(|| -> Result<GLint> {
                let mut active = 0;
                unsafe {
                    ::gl::GetProgramiv(
                        id,
                        ::gl::ACTIVE_ATTRIBUTES,
                        &mut active,
                    );
                }
                Ok(active)
            })?;

            let max_length = gl_result(|| -> Result<GLint> {
                let mut max_length = 0;
                unsafe {
                    ::gl::GetProgramiv(
                        id,
                        ::gl::ACTIVE_ATTRIBUTE_MAX_LENGTH,
                        &mut max_length,
                    );
                }
                Ok(max_length)
            })?;

            for i in 0..active as GLuint {
                let name = gl_result(|| -> Result<String> {
                    let mut name = vec![0u8; max_length as usize];
                    let mut length = 0;
                    let mut s_ = 0;
                    let mut t_ = 0;
                    unsafe {
                        ::gl::GetActiveAttrib(
                            id,
                            i,
                            max_length,
                            &mut length,
                            &mut s_,
                            &mut t_,
                            name.as_mut_ptr() as *mut GLchar,
                        );
                    }
                    name.resize(length as usize, 0u8);
                    Ok(String::from_utf8(name)?)
                })?;
                let location = gl_result(|| -> Result<GLint> {
                    Ok(unsafe {
                        ::gl::GetAttribLocation(
                            id,
                            name.as_ptr() as *const GLchar,
                        )
                    })
                })?;
                let _ = location_map.insert(name, location);
            }
        }

        Ok(Program {
            shaders,
            location_map,
            id,
        })
    }
    // ========================================================================
    /// location
    pub fn location<Q: ?Sized>(&self, name: &Q) -> Option<GLint>
    where
        String: ::std::borrow::Borrow<Q>,
        Q: ::std::hash::Hash + Ord,
    {
        self.location_map.get(name).cloned()
    }
    // ========================================================================
    /// set_attribute
    pub fn set_attribute(
        location: GLint,
        buffer: &Buffer,
        size_: usize,
        type_: GLenum,
        normalized: GLboolean,
        stride: usize,
        pointer: usize,
    ) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::EnableVertexAttribArray(location as GLuint);
            }
            Ok(())
        })?;
        buffer.bind_with(|| -> Result<()> {
            gl_result(|| -> Result<()> {
                unsafe {
                    ::gl::VertexAttribPointer(
                        location as GLuint,
                        size_ as GLint,
                        type_,
                        normalized,
                        stride as GLsizei,
                        pointer as *const GLvoid,
                    );
                }
                Ok(())
            })?;
            Ok(())
        })
    }
    // ========================================================================
    /// set_uniform1i
    pub fn set_uniform1i(l: GLint, v0: GLint) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::Uniform1i(l, v0);
            }
            Ok(())
        })?;
        Ok(())
    }
    // ------------------------------------------------------------------------
    /// set_uniform1iv
    pub fn set_uniform1iv(
        l: GLint,
        c: GLsizei,
        v: *const GLint,
    ) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::Uniform1iv(l, c, v);
            }
            Ok(())
        })?;
        Ok(())
    }
    // ------------------------------------------------------------------------
    /// set_uniform1ui
    pub fn set_uniform1ui(l: GLint, v0: GLuint) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe { ::gl::Uniform1ui(l, v0) }
            Ok(())
        })?;
        Ok(())
    }
    // ------------------------------------------------------------------------
    /// set_uniform1uiv
    pub fn set_uniform1uiv(
        l: GLint,
        c: GLsizei,
        v: *const GLuint,
    ) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::Uniform1uiv(l, c, v);
            }
            Ok(())
        })?;
        Ok(())
    }
    // ------------------------------------------------------------------------
    /// set_uniform1f
    pub fn set_uniform1f(l: GLint, v0: GLfloat) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::Uniform1f(l, v0);
            }
            Ok(())
        })?;
        Ok(())
    }
    // ------------------------------------------------------------------------
    /// set_uniform1fv
    pub fn set_uniform1fv(
        l: GLint,
        c: GLsizei,
        v: *const GLfloat,
    ) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::Uniform1fv(l, c, v);
            }
            Ok(())
        })?;
        Ok(())
    }
    // ========================================================================
    /// set_uniform2i
    pub fn set_uniform2i(l: GLint, v0: GLint, v1: GLint) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::Uniform2i(l, v0, v1);
            }
            Ok(())
        })?;
        Ok(())
    }
    // ------------------------------------------------------------------------
    /// set_uniform2iv
    pub fn set_uniform2iv(
        l: GLint,
        c: GLsizei,
        v: *const GLint,
    ) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::Uniform2iv(l, c, v);
            }
            Ok(())
        })?;
        Ok(())
    }
    // ------------------------------------------------------------------------
    /// set_uniform2ui
    pub fn set_uniform2ui(l: GLint, v0: GLuint, v1: GLuint) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::Uniform2ui(l, v0, v1);
            }
            Ok(())
        })?;
        Ok(())
    }
    // ------------------------------------------------------------------------
    /// set_uniform2uiv
    pub fn set_uniform2uiv(
        l: GLint,
        c: GLsizei,
        v: *const GLuint,
    ) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::Uniform2uiv(l, c, v);
            }
            Ok(())
        })?;
        Ok(())
    }
    // ------------------------------------------------------------------------
    /// set_uniform2f
    pub fn set_uniform2f(l: GLint, v0: GLfloat, v1: GLfloat) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::Uniform2f(l, v0, v1);
            }
            Ok(())
        })?;
        Ok(())
    }
    // ------------------------------------------------------------------------
    /// set_uniform2fv
    pub fn set_uniform2fv(
        l: GLint,
        c: GLsizei,
        v: *const GLfloat,
    ) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::Uniform2fv(l, c, v);
            }
            Ok(())
        })?;
        Ok(())
    }
    // ========================================================================
    /// set_uniform3i
    pub fn set_uniform3i(
        l: GLint,
        v0: GLint,
        v1: GLint,
        v2: GLint,
    ) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::Uniform3i(l, v0, v1, v2);
            }
            Ok(())
        })?;
        Ok(())
    }
    // ------------------------------------------------------------------------
    /// set_uniform3iv
    pub fn set_uniform3iv(
        l: GLint,
        c: GLsizei,
        v: *const GLint,
    ) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::Uniform3iv(l, c, v);
            }
            Ok(())
        })?;
        Ok(())
    }
    // ------------------------------------------------------------------------
    /// set_uniform3ui
    pub fn set_uniform3ui(
        l: GLint,
        v0: GLuint,
        v1: GLuint,
        v2: GLuint,
    ) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::Uniform3ui(l, v0, v1, v2);
            }
            Ok(())
        })?;
        Ok(())
    }
    // ------------------------------------------------------------------------
    /// set_uniform3uiv
    pub fn set_uniform3uiv(
        l: GLint,
        c: GLsizei,
        v: *const GLuint,
    ) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::Uniform3uiv(l, c, v);
            }
            Ok(())
        })?;
        Ok(())
    }
    // ------------------------------------------------------------------------
    /// set_uniform3f
    pub fn set_uniform3f(
        l: GLint,
        v0: GLfloat,
        v1: GLfloat,
        v2: GLfloat,
    ) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::Uniform3f(l, v0, v1, v2);
            }
            Ok(())
        })?;
        Ok(())
    }
    // ------------------------------------------------------------------------
    /// set_uniform3fv
    pub fn set_uniform3fv(
        l: GLint,
        c: GLsizei,
        v: *const GLfloat,
    ) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::Uniform3fv(l, c, v);
            }
            Ok(())
        })?;
        Ok(())
    }
    // ========================================================================
    /// set_uniform4i
    pub fn set_uniform4i(
        l: GLint,
        v0: GLint,
        v1: GLint,
        v2: GLint,
        v3: GLint,
    ) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::Uniform4i(l, v0, v1, v2, v3);
            }
            Ok(())
        })?;
        Ok(())
    }
    // ------------------------------------------------------------------------
    /// set_uniform4iv
    pub fn set_uniform4iv(
        l: GLint,
        c: GLsizei,
        v: *const GLint,
    ) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::Uniform4iv(l, c, v);
            }
            Ok(())
        })?;
        Ok(())
    }
    // ------------------------------------------------------------------------
    /// set_uniform4ui
    pub fn set_uniform4ui(
        l: GLint,
        v0: GLuint,
        v1: GLuint,
        v2: GLuint,
        v3: GLuint,
    ) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::Uniform4ui(l, v0, v1, v2, v3);
            }
            Ok(())
        })?;
        Ok(())
    }
    // ------------------------------------------------------------------------
    /// set_uniform4uiv
    pub fn set_uniform4uiv(
        l: GLint,
        c: GLsizei,
        v: *const GLuint,
    ) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::Uniform4uiv(l, c, v);
            }
            Ok(())
        })?;
        Ok(())
    }
    // ------------------------------------------------------------------------
    /// set_uniform4f
    pub fn set_uniform4f(
        l: GLint,
        v0: GLfloat,
        v1: GLfloat,
        v2: GLfloat,
        v3: GLfloat,
    ) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::Uniform4f(l, v0, v1, v2, v3);
            }
            Ok(())
        })?;
        Ok(())
    }
    // ------------------------------------------------------------------------
    /// set_uniform4fv
    pub fn set_uniform4fv(
        l: GLint,
        c: GLsizei,
        v: *const GLfloat,
    ) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::Uniform4fv(l, c, v);
            }
            Ok(())
        })?;
        Ok(())
    }
    // ========================================================================
    /// set_uniform_matrix2fv
    pub fn set_uniform_matrix2fv(
        l: GLint,
        c: GLsizei,
        t: GLboolean,
        v: *const GLfloat,
    ) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::UniformMatrix2fv(l, c, t, v);
            }
            Ok(())
        })?;
        Ok(())
    }
    // ------------------------------------------------------------------------
    /// set_uniform_matrix3fv
    pub fn set_uniform_matrix3fv(
        l: GLint,
        c: GLsizei,
        t: GLboolean,
        v: *const GLfloat,
    ) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::UniformMatrix3fv(l, c, t, v);
            }
            Ok(())
        })?;
        Ok(())
    }
    // ------------------------------------------------------------------------
    /// set_uniform_matrix4fv
    pub fn set_uniform_matrix4fv(
        l: GLint,
        c: GLsizei,
        t: GLboolean,
        v: *const GLfloat,
    ) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::UniformMatrix4fv(l, c, t, v);
            }
            Ok(())
        })?;
        Ok(())
    }
    // ------------------------------------------------------------------------
    /// set_uniform_matrix2x3fv
    pub fn set_uniform_matrix2x3fv(
        l: GLint,
        c: GLsizei,
        t: GLboolean,
        v: *const GLfloat,
    ) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::UniformMatrix2x3fv(l, c, t, v);
            }
            Ok(())
        })?;
        Ok(())
    }
    // ------------------------------------------------------------------------
    /// set_uniform_matrix3x2fv
    pub fn set_uniform_matrix3x2fv(
        l: GLint,
        c: GLsizei,
        t: GLboolean,
        v: *const GLfloat,
    ) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::UniformMatrix3x2fv(l, c, t, v);
            }
            Ok(())
        })?;
        Ok(())
    }
    // ------------------------------------------------------------------------
    /// set_uniform_matrix2x4fv
    pub fn set_uniform_matrix2x4fv(
        l: GLint,
        c: GLsizei,
        t: GLboolean,
        v: *const GLfloat,
    ) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::UniformMatrix2x4fv(l, c, t, v);
            }
            Ok(())
        })?;
        Ok(())
    }
    // ------------------------------------------------------------------------
    /// set_uniform_matrix4x2fv
    pub fn set_uniform_matrix4x2fv(
        l: GLint,
        c: GLsizei,
        t: GLboolean,
        v: *const GLfloat,
    ) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::UniformMatrix4x2fv(l, c, t, v);
            }
            Ok(())
        })?;
        Ok(())
    }
    // ------------------------------------------------------------------------
    /// set_uniform_matrix3x4fv
    pub fn set_uniform_matrix3x4fv(
        l: GLint,
        c: GLsizei,
        t: GLboolean,
        v: *const GLfloat,
    ) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::UniformMatrix3x4fv(l, c, t, v);
            }
            Ok(())
        })?;
        Ok(())
    }
    // ------------------------------------------------------------------------
    /// set_uniform_matrix4x3fv
    pub fn set_uniform_matrix4x3fv(
        l: GLint,
        c: GLsizei,
        t: GLboolean,
        v: *const GLfloat,
    ) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::UniformMatrix4x3fv(l, c, t, v);
            }
            Ok(())
        })?;
        Ok(())
    }
    // ========================================================================
    /// set_texture
    pub fn set_texture(
        l: GLint,
        index: GLint,
        texture: &Texture,
    ) -> Result<()> {
        if 0 > index {
            Program::set_uniform1i(l, -1)
        } else {
            gl_result(|| -> Result<()> {
                unsafe {
                    ::gl::ActiveTexture(::gl::TEXTURE0 + index as GLuint);
                }
                Ok(())
            })?;
            texture.bind()?;
            Program::set_uniform1i(l, index)
        }
    }
}
// ============================================================================
impl Drop for Program {
    fn drop(&mut self) {
        gl_result(|| -> Result<()> {
            unsafe { ::gl::DeleteProgram(self.id) }
            Ok(())
        }).expect("Program::drop");
    }
}
// ============================================================================
impl Bind for Program {
    // ========================================================================
    type BindError = Error;
    // ========================================================================
    fn id(&self) -> GLuint {
        self.id
    }
    // ========================================================================
    fn bind(&self) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::UseProgram(self.id);
            }
            Ok(())
        })?;
        Ok(())
    }
    // ========================================================================
    fn unbind(&self) -> Result<()> {
        gl_result(|| -> Result<()> {
            unsafe {
                ::gl::UseProgram(0);
            }
            Ok(())
        })?;
        Ok(())
    }
}
// ============================================================================
/// sif_renderer_program_location!
#[macro_export]
macro_rules! sif_renderer_program_location {
    ($e:expr, $name:expr)               => {
        unwrap!($e.location($name))
    };
    ($e:expr, $fmt:expr, $($args:tt)+)  => {
        unwrap!($e.location(&format!($fmt, $($args)+)), $fmt, $($args)+)
    };
}
