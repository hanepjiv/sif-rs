// -*- mode:rust; coding:utf-8-unix; -*-

//! mod.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/02/23
//  @date 2017/01/17

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
pub use super::Result;
pub use self::result::{gl_result, info_log, GLError};
pub use self::bind::{Binder, TBind};
pub use self::buffer::Buffer;
pub use self::texture::Texture;
pub use self::frame::Frame;
pub use self::render::Render;
pub use self::shader::{Shader, ShaderSrc};
pub use self::program::Program;
// mod  =======================================================================
pub mod result;
pub mod bind;
pub mod buffer;
pub mod texture;
pub mod frame;
pub mod render;
pub mod shader;
#[macro_use]
pub mod program;
