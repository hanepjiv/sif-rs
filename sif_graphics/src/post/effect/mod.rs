// -*- mode:rust; coding:utf-8-unix; -*-

//! mod.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/03/06
//  @date 2018/05/12

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use gl::types::*;
// ----------------------------------------------------------------------------
use sif_renderer::{Frame, Texture};
// ----------------------------------------------------------------------------
use super::{square_buffer::SquareBuffer, Result};
// ----------------------------------------------------------------------------
pub use self::{blur::Blur, pass::Pass};
// mod  =======================================================================
mod blur;
mod pass;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum EffectArgs
#[derive(Debug, Clone, Copy)]
pub enum EffectArgs {
    /// Pass
    Pass,
    /// Blur
    Blur {
        /// distance
        distance: u8,
    },
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// trait Effect
pub trait Effect: ::std::fmt::Debug {
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
    ) -> Result<&Self>;
}
