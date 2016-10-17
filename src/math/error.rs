// -*- mode:rust; coding:utf-8-unix; -*-

//! error.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/04
//  @date 2016/10/10

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum MathError
#[derive( Debug, )]
pub enum MathError {
    /// InvalidArguments
    InvaridArguments(String),
}
