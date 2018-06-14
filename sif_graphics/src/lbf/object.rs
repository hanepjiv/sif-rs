// -*- mode:rust; coding:utf-8-unix; -*-

//! object.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/06/13
//  @date 2018/06/13

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use gl::types::*;
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_three::TraRotSca;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Object
#[derive(Debug, Clone)]
pub struct Object {
    /// uuid
    pub uuid: Uuid,
    /// name
    pub name: String,
    /// parent
    pub parent: Option<Uuid>,
    /// data_type
    pub data_type: String,
    /// data_uuid
    pub data_uuid: Uuid,
    /// trarotsca
    pub trarotsca: TraRotSca<GLfloat>,
}
// ============================================================================
impl Object {
    // ========================================================================
    /// new
    pub fn new(
        uuid: Uuid,
        name: impl Into<String>,
        parent: Option<Uuid>,
        data_type: impl Into<String>,
        data_uuid: Uuid,
        trarotsca: TraRotSca<GLfloat>,
    ) -> Self {
        Object {
            uuid,
            name: name.into(),
            parent,
            data_type: data_type.into(),
            data_uuid,
            trarotsca,
        }
    }
}
// ============================================================================
impl AsRef<Uuid> for Object {
    fn as_ref(&self) -> &Uuid {
        &self.uuid
    }
}
// ============================================================================
impl AsRef<String> for Object {
    fn as_ref(&self) -> &String {
        &self.name
    }
}
