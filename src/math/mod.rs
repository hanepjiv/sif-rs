// -*- mode:rust; coding:utf-8-unix; -*-

//! mod.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/19
//  @date 2017/01/09

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
pub use self::number::Number;
pub use self::error::MathError;
pub use self::cleanup::Cleanup;
pub use self::vector::{ Vector2, Vector3, Vector4, };
pub use self::matrix::Matrix4x4;
pub use self::quaternion::Quaternion;
// ============================================================================
pub mod number;
pub mod error;
pub mod cleanup;
pub mod vector;
pub mod matrix;
pub mod quaternion;
