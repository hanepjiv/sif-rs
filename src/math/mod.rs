// -*- mode:rust; coding:utf-8-unix; -*-

//! mod.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/19
//  @date 2017/04/10

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
pub use super::{Error, Result};
// ----------------------------------------------------------------------------
pub use self::number::Number;
pub use self::cleanup::Cleanup;
pub use self::vector::{Vector2, Vector3, Vector4};
pub use self::matrix::{Matrix2x2, Matrix2x3, Matrix3x2, Matrix3x3, Matrix3x4,
                       Matrix4x3, Matrix4x4};
pub use self::quaternion::Quaternion;
// ============================================================================
pub mod cleanup;
pub mod matrix;
pub mod number;
pub mod quaternion;
pub mod vector;
