// -*- mode:rust; coding:utf-8-unix; -*-

//! mod.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/02/25
//  @date 2018/05/11

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Sequence
#[derive(Debug, Clone, Copy)]
pub struct Sequence {
    /// field
    field: i32,
}
// ============================================================================
impl Sequence {
    // ------------------------------------------------------------------------
    /// new
    pub fn new() -> Self {
        Sequence { field: 0 }
    }
}
// ============================================================================
/// TSequence
pub trait TSequence {
    // ------------------------------------------------------------------------
    /// on_event
    fn on_event(&mut self) -> &mut Self;
}
