// -*- mode:rust; coding:utf-8-unix; -*-

//! bind.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/04/11
//  @date 2018/04/11

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use gl::types::*;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// trait TBind
pub trait TBind: ::std::fmt::Debug {
    // ========================================================================
    /// id
    fn id(&self) -> GLuint;
    // ========================================================================
    /// bind
    fn bind(&self);
    // ========================================================================
    /// unbind
    fn unbind(&self);
    // ========================================================================
    /// binder
    fn binder<'a>(&'a self) -> Binder<'a>
    where
        Self: Sized,
    {
        Binder::new(self)
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Binder
#[derive(Debug)]
pub struct Binder<'a> {
    /// bind
    bind: &'a TBind,
}
// ============================================================================
impl<'a> Binder<'a> {
    // ========================================================================
    /// new
    pub fn new(bind: &'a TBind) -> Self {
        bind.bind();
        Binder { bind }
    }
}
// ============================================================================
impl<'a> Drop for Binder<'a> {
    fn drop(&mut self) {
        self.bind.unbind();
    }
}
