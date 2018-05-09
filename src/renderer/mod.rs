// -*- mode:rust; coding:utf-8-unix; -*-

//! mod.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/02/23
//  @date 2018/05/09

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
pub use self::bind::Bind;
pub use self::buffer::Buffer;
pub use self::error::{Error, Result};
pub use self::frame::Frame;
pub use self::gl_error::{gl_result, info_log, GLError, GLResult};
pub use self::program::Program;
pub use self::render::Render;
pub use self::shader::{Shader, ShaderSrc};
pub use self::texture::Texture;
// mod  =======================================================================
pub mod bind;
pub mod buffer;
pub mod error;
pub mod frame;
pub mod gl_error;
pub mod render;
pub mod shader;
pub mod texture;
#[macro_use]
pub mod program;
