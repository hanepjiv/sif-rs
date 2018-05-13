// -*- mode:rust; coding:utf-8-unix; -*-

//! image.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2017/01/06
//  @date 2018/05/12

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::path::{Path, PathBuf};
// ----------------------------------------------------------------------------
use gl::types::*;
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_math::Vector4;
// ----------------------------------------------------------------------------
use super::{Error, Result};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Image
#[derive(Debug, Clone)]
pub enum Image {
    /// File
    File(ImageFile),
    /// Procedual
    Procedual(ImageProcedual),
}
// ============================================================================
impl Image {
    // ========================================================================
    /// new_file
    pub fn new_file(
        uuid: Uuid,
        name: impl Into<String>,
        dimension: u8,
        path: impl Into<PathBuf>,
    ) -> Self {
        Image::File(ImageFile::new(uuid, name, dimension, path))
    }
    // ========================================================================
    /// new_procedual
    pub fn new_procedual(
        uuid: Uuid,
        name: impl Into<String>,
        dimension: u8,
        method: ImageProcedualMethod,
    ) -> Self {
        Image::Procedual(ImageProcedual::new(uuid, name, dimension, method))
    }
}
// ============================================================================
impl AsRef<Uuid> for Image {
    fn as_ref(&self) -> &Uuid {
        match *self {
            Image::File(ref inner) => inner.as_ref(),
            Image::Procedual(ref inner) => inner.as_ref(),
        }
    }
}
// ============================================================================
impl AsRef<String> for Image {
    fn as_ref(&self) -> &String {
        match *self {
            Image::File(ref inner) => inner.as_ref(),
            Image::Procedual(ref inner) => inner.as_ref(),
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct ImageBase
#[derive(Debug, Clone)]
struct ImageBase {
    /// uuid
    uuid: Uuid,
    /// name
    name: String,
    /// dimension
    dimension: u8,
}
// ============================================================================
impl ImageBase {
    // ========================================================================
    pub(crate) fn new(
        uuid: Uuid,
        name: impl Into<String>,
        dimension: u8,
    ) -> Self {
        ImageBase {
            uuid,
            name: name.into(),
            dimension,
        }
    }
    // ========================================================================
    pub(crate) fn as_dimension(&self) -> &u8 {
        &self.dimension
    }
}
// ============================================================================
impl AsRef<Uuid> for ImageBase {
    fn as_ref(&self) -> &Uuid {
        &self.uuid
    }
}
// ============================================================================
impl AsRef<String> for ImageBase {
    fn as_ref(&self) -> &String {
        &self.name
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct ImageFile
#[derive(Debug, Clone)]
pub struct ImageFile {
    /// base
    base: ImageBase,
    /// path
    path: PathBuf,
}
// ============================================================================
impl ImageFile {
    // ========================================================================
    /// new
    pub fn new(
        uuid: Uuid,
        name: impl Into<String>,
        dimension: u8,
        path: impl Into<PathBuf>,
    ) -> Self {
        ImageFile {
            base: ImageBase::new(uuid, name, dimension),
            path: path.into(),
        }
    }
    // ========================================================================
    /// as_dimension
    pub fn as_dimension(&self) -> &u8 {
        self.base.as_dimension()
    }
    // ========================================================================
    /// set_path_base
    pub fn set_path_base(&mut self, path_base: &Path) -> Result<&Path> {
        if AsRef::<Path>::as_ref(&self.path).is_absolute() {
            Err(Error::Path(
                "ImageFile::set_path_base: already absolute".to_string(),
            ))
        } else {
            let n = path_base.join(&self.path).canonicalize()?;
            self.path = n;
            Ok(self.path.as_ref())
        }
    }
}
// ============================================================================
impl AsRef<Uuid> for ImageFile {
    fn as_ref(&self) -> &Uuid {
        self.base.as_ref()
    }
}
// ============================================================================
impl AsRef<String> for ImageFile {
    fn as_ref(&self) -> &String {
        self.base.as_ref()
    }
}
// ============================================================================
impl AsRef<PathBuf> for ImageFile {
    fn as_ref(&self) -> &PathBuf {
        &self.path
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct ImageProcedual
#[derive(Debug, Clone)]
pub struct ImageProcedual {
    /// base
    base: ImageBase,
    /// method
    method: ImageProcedualMethod,
}
// ============================================================================
impl ImageProcedual {
    // ========================================================================
    /// new
    pub fn new(
        uuid: Uuid,
        name: impl Into<String>,
        dimension: u8,
        method: ImageProcedualMethod,
    ) -> Self {
        ImageProcedual {
            base: ImageBase::new(uuid, name, dimension),
            method,
        }
    }
    // ========================================================================
    /// as_dimension
    pub fn as_dimension(&self) -> &u8 {
        self.base.as_dimension()
    }
}
// ============================================================================
impl AsRef<Uuid> for ImageProcedual {
    fn as_ref(&self) -> &Uuid {
        self.base.as_ref()
    }
}
// ============================================================================
impl AsRef<String> for ImageProcedual {
    fn as_ref(&self) -> &String {
        self.base.as_ref()
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum ImageProcedualMethod
#[derive(Debug, Clone, Copy)]
pub enum ImageProcedualMethod {
    /// Color
    Color(Vector4<GLfloat>),
    /// Cloud
    Cloud(Vector4<GLfloat>, Vector4<GLfloat>),
    /// Marble
    Marble(Vector4<GLfloat>, Vector4<GLfloat>),
}
