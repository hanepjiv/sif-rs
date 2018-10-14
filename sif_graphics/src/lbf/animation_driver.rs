// -*- mode:rust; coding:utf-8-unix; -*-

//! animation_friver.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/08/02
//  @date 2018/08/27

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::marker::PhantomData;
// ----------------------------------------------------------------------------
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_manager::{ManagedValue, Manager};
use sif_math::{Float, Integer};
// ----------------------------------------------------------------------------
use super::{
    Animation, Armature, Error, GraphicsAnimationDriver, GraphicsModel,
    GraphicsObject, GraphicsResult, GraphicsScene, IntoGraphics,
};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Driver
#[derive(Debug, Clone)]
pub(crate) struct Driver<'a, 'b, VF, VI>
where
    VF: 'a + 'b + Float,
    VI: 'a + 'b + Integer,
{
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
    /// phantom2
    phantom2: PhantomData<fn() -> VF>,
    /// phantom3
    phantom3: PhantomData<fn() -> VI>,
}
// ============================================================================
impl<'a, 'b, VF, VI> AsRef<Uuid> for Driver<'a, 'b, VF, VI>
where
    VF: 'a + 'b + Float,
    VI: 'a + 'b + Integer,
{
    fn as_ref(&self) -> &Uuid {
        &self.uuid
    }
}
// ============================================================================
impl<'a, 'b, VF, VI> AsRef<String> for Driver<'a, 'b, VF, VI>
where
    VF: 'a + 'b + Float,
    VI: 'a + 'b + Integer,
{
    fn as_ref(&self) -> &String {
        &self.name
    }
}
// ============================================================================
impl<'a, 'b, VF, VI> Driver<'a, 'b, VF, VI>
where
    VF: 'a + 'b + Float,
    VI: 'a + 'b + Integer,
{
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
            phantom2: PhantomData::default(),
            phantom3: PhantomData::default(),
        }
    }
}
// ============================================================================
impl<'a, 'b, VF, VI> IntoGraphics for Driver<'a, 'b, VF, VI>
where
    VF: 'a + 'b + Float,
    VI: 'a + 'b + Integer,
    GraphicsScene<VF, VI>: AsRef<Manager<Animation<VF>>>,
    GraphicsModel: AsRef<Option<ManagedValue<Armature<VF>>>>,
{
    type Target = GraphicsAnimationDriver<VF>;
    type Param = (
        &'a GraphicsScene<VF, VI>,
        &'b Manager<Animation<VF>>,
        &'b Manager<GraphicsObject<VF>>,
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
                        AsRef::<Manager<Animation<VF>>>::as_ref(scene)
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
                        )
                        .to_string(),
                    )
                    .into());
                },
                if let Some(x) = {
                    if let Some(x) = objects.get(&self.object) {
                        Some(x)
                    } else {
                        AsRef::<Manager<GraphicsObject<VF>>>::as_ref(scene)
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
                        )
                        .to_string(),
                    )
                    .into());
                },
            )?,
            (scene, animations, objects),
        ))
    }
}
