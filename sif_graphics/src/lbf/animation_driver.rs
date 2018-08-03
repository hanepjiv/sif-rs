// -*- mode:rust; coding:utf-8-unix; -*-

//! animation_friver.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/08/02
//  @date 2018/08/05

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::marker::PhantomData;
// ----------------------------------------------------------------------------
use gl::types::*;
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_manager::{ManagedValue, Manager};
// ----------------------------------------------------------------------------
use super::{
    Animation, Armature, Error, GraphicsAnimationDriver, GraphicsModel,
    GraphicsObject, GraphicsResult, GraphicsScene, IntoGraphics,
};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Driver
#[derive(Debug, Clone)]
pub(crate) struct Driver<'a, 'b> {
    /// uuid
    uuid: Uuid,
    /// name
    name: String,
    /// animation
    animation: Uuid,
    /// object
    object: Uuid,
    /// phantom0
    phantom0: PhantomData<&'a ()>,
    /// phantom1
    phantom1: PhantomData<&'b ()>,
}
// ============================================================================
impl<'a, 'b> AsRef<Uuid> for Driver<'a, 'b> {
    fn as_ref(&self) -> &Uuid {
        &self.uuid
    }
}
// ============================================================================
impl<'a, 'b> AsRef<String> for Driver<'a, 'b> {
    fn as_ref(&self) -> &String {
        &self.name
    }
}
// ============================================================================
impl<'a, 'b> Driver<'a, 'b> {
    /// fn new
    pub fn new(
        uuid: impl Into<Uuid>,
        name: impl Into<String>,
        animation: impl Into<Uuid>,
        object: impl Into<Uuid>,
    ) -> Self {
        Self {
            uuid: uuid.into(),
            name: name.into(),
            animation: animation.into(),
            object: object.into(),
            phantom0: PhantomData::default(),
            phantom1: PhantomData::default(),
        }
    }
}
// ============================================================================
impl<'a, 'b> IntoGraphics for Driver<'a, 'b>
where
    GraphicsScene: AsRef<Manager<Animation<GLfloat>>>,
    GraphicsModel: AsRef<Option<ManagedValue<Armature<GLfloat>>>>,
{
    type Target = GraphicsAnimationDriver;
    type Param = (
        &'a GraphicsScene,
        &'b Manager<Animation<GLfloat>>,
        &'b Manager<GraphicsObject>,
    );
    // ========================================================================
    fn into_graphics(
        self,
        (scene, animations, objects): Self::Param,
    ) -> GraphicsResult<(Self::Target, Self::Param)> {
        Ok((
            GraphicsAnimationDriver::new(
                self.uuid,
                self.name,
                if let Some(x) = {
                    if let Some(x) = animations.get(&self.animation) {
                        Some(x)
                    } else {
                        AsRef::<Manager<Animation<GLfloat>>>::as_ref(scene)
                            .get(&self.animation)
                    }
                } {
                    x.clone()
                } else {
                    return Err(Error::OptNone(
                        format!(
                            "lbf::AnimationDriver: into_graphics: \
                             animation.get: {}",
                            self.animation,
                        ).to_string(),
                    ).into());
                },
                if let Some(x) = {
                    if let Some(x) = objects.get(&self.object) {
                        Some(x)
                    } else {
                        AsRef::<Manager<GraphicsObject>>::as_ref(scene)
                            .get(&self.object)
                    }
                } {
                    x.clone()
                } else {
                    return Err(Error::OptNone(
                        format!(
                            "lbf::AnimationDriver: into_graphics: \
                             object.get: {}",
                            self.object
                        ).to_string(),
                    ).into());
                },
            )?,
            (scene, animations, objects),
        ))
    }
}
