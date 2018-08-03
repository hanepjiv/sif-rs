// -*- mode:rust; coding:utf-8-unix; -*-

//! driver.rs

//  Copyright 2017 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2018/08/01
//  @date 2018/08/11

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use gl::types::*;
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_manager::ManagedValue;
use sif_three::{Armature, Bone, NodeHolder, TraRotSca, TraRotScaType};
// ----------------------------------------------------------------------------
use super::{Animation, Curve, CurveType, Error, Object, ObjectData, Result};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// Flags
bitflags! { pub struct Flags: u32 {
    const ACTIVE                = 0b0000_0000_0000_0000_0000_0000_0000_0001u32;
    const DIRTY                 = 0b0000_0000_0000_0000_0000_0000_0001_0000u32;
} }
// ============================================================================
impl Default for Flags {
    fn default() -> Self {
        Flags::ACTIVE | Flags::DIRTY
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// enum Param
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Param {
    /// Object(trs_type, data_idx)
    Object(TraRotScaType, usize),
    /// Bone(bone_idx, trs_type, data_idx)
    Bone(usize, TraRotScaType, usize),
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Driver
#[derive(Debug, Clone)]
pub struct Driver {
    /// uuid
    uuid: Uuid,
    /// name
    name: String,
    /// params
    params: Vec<Param>,
    /// animation
    animation: ManagedValue<Animation<GLfloat>>,
    /// object
    object: ManagedValue<Object>,
    /// flags
    flags: Flags,
    /// lifetime (millisec)
    lifetime: isize,
    /// duration (millisec)
    duration: isize,
    /// scale
    scale: GLfloat,
}
// ============================================================================
impl AsRef<Uuid> for Driver {
    fn as_ref(&self) -> &Uuid {
        &self.uuid
    }
}
// ============================================================================
impl AsRef<String> for Driver {
    fn as_ref(&self) -> &String {
        &self.name
    }
}
// ============================================================================
impl AsRef<Flags> for Driver {
    fn as_ref(&self) -> &Flags {
        &self.flags
    }
}
// ----------------------------------------------------------------------------
impl AsMut<Flags> for Driver {
    fn as_mut(&mut self) -> &mut Flags {
        &mut self.flags
    }
}
// ============================================================================
impl Driver {
    // ========================================================================
    fn make_param(
        curve: &Curve<GLfloat>,
        obj_data: &ObjectData,
    ) -> Option<Param> {
        match AsRef::<CurveType>::as_ref(curve) {
            CurveType::Translate(data_idx) => {
                Some(Param::Object(TraRotScaType::Translate, *data_idx))
            }
            CurveType::RotateQuaternion(data_idx) => {
                Some(Param::Object(TraRotScaType::Rotate, *data_idx))
            }
            CurveType::Scale(data_idx) => {
                Some(Param::Object(TraRotScaType::Scale, *data_idx))
            }
            CurveType::BoneTranslate(bone_name, data_idx) => {
                Driver::make_param_bone(
                    obj_data,
                    bone_name,
                    TraRotScaType::Translate,
                    *data_idx,
                )
            }
            CurveType::BoneRotateQuaternion(bone_name, data_idx) => {
                Driver::make_param_bone(
                    obj_data,
                    bone_name,
                    TraRotScaType::Rotate,
                    *data_idx,
                )
            }
            CurveType::BoneScale(bone_name, data_idx) => {
                Driver::make_param_bone(
                    obj_data,
                    bone_name,
                    TraRotScaType::Scale,
                    *data_idx,
                )
            }
        }
    }
    // ------------------------------------------------------------------------
    fn make_param_bone(
        obj_data: &ObjectData,
        name: &str,
        trs_type: TraRotScaType,
        data_idx: usize,
    ) -> Option<Param> {
        match obj_data {
            ObjectData::Model(managed_model, _) => {
                if let Some(managed_armature) =
                    AsRef::<Option<ManagedValue<Armature<GLfloat>>>>::as_ref(
                        &*managed_model.as_ref().borrow(),
                    ) {
                    let armature = &*managed_armature.as_ref().borrow();
                    for (i, bone) in
                        AsRef::<Vec<Bone<GLfloat>>>::as_ref(armature)
                            .iter()
                            .enumerate()
                    {
                        if name == AsRef::<str>::as_ref(bone) {
                            return Some(Param::Bone(i, trs_type, data_idx));
                        }
                    }
                    None
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    // ========================================================================
    /// fn new
    pub fn new(
        uuid: Uuid,
        name: impl Into<String>,
        animation: ManagedValue<Animation<GLfloat>>,
        object: ManagedValue<Object>,
    ) -> Result<Self> {
        let (params, duration) = {
            let anim = &*animation.as_ref().borrow();
            let mut params = Vec::default();
            let obj = &*object.as_ref().borrow();
            let obj_data = &*AsRef::<ObjectData>::as_ref(obj);
            for curve in AsRef::<Vec<Curve<GLfloat>>>::as_ref(anim) {
                if let Some(x) = Driver::make_param(curve, obj_data) {
                    params.push(x)
                } else {
                    return Err(Error::AnimationDriver(
                        "sif_graphics::animation::Driver: new: \
                         object has not bones."
                            .to_string(),
                    ));
                }
            }
            params.shrink_to_fit();
            (params, anim.duration())
        };
        Ok(Self {
            uuid,
            name: name.into(),
            params,
            animation,
            object,
            flags: Flags::default(),
            lifetime: 0,
            duration,
            scale: 1.0,
        })
    }
    // ========================================================================
    /// fn duration
    pub fn duration(&self) -> isize {
        self.animation.as_ref().borrow().duration()
    }
    // ========================================================================
    /// fn as_lifetime
    pub fn as_lifetime(&self) -> &isize {
        &self.lifetime
    }
    // ========================================================================
    /// fn as_scale
    pub fn as_scale(&self) -> &GLfloat {
        &self.scale
    }
    // ------------------------------------------------------------------------
    /// fn as_scale_mut
    pub fn as_scale_mut(&mut self) -> &mut GLfloat {
        &mut self.scale
    }
    // ========================================================================
    /// fn elapsed
    pub fn elapsed(&mut self, millisec: isize) -> Result<&mut Self> {
        if 0 == millisec || !self.flags.contains(Flags::ACTIVE) {
            Ok(self)
        } else {
            self.lifetime += (millisec as GLfloat * self.scale) as isize;

            let duration = self.duration();
            while duration < self.lifetime {
                self.lifetime -= duration;
            }

            self.flags.insert(Flags::DIRTY);

            Ok(self)
        }
    }
    // ========================================================================
    /// fn update
    pub fn update(&mut self) -> Result<&mut Self> {
        if self.flags.contains(Flags::DIRTY) {
            let obj = &mut *self.object.as_ref().borrow_mut();
            let anim = &*self.animation.as_ref().borrow();
            let key = self.lifetime as GLfloat / 1000.0 * anim.fps() + 1.0;
            for (i, curve) in AsRef::<Vec<Curve<GLfloat>>>::as_ref(anim)
                .iter()
                .enumerate()
            {
                match AsRef::<CurveType>::as_ref(curve) {
                    CurveType::Translate(_)
                    | CurveType::RotateQuaternion(_)
                    | CurveType::Scale(_) => {
                        Driver::update_obj(
                            &self.params[i],
                            obj,
                            curve.value(key),
                        )?;
                    }
                    CurveType::BoneTranslate(_, _)
                    | CurveType::BoneRotateQuaternion(_, _)
                    | CurveType::BoneScale(_, _) => {
                        Driver::update_pose(
                            &self.params[i],
                            &mut *AsMut::<ObjectData>::as_mut(obj),
                            curve.value(key),
                        )?;
                    }
                }
            }
            self.flags.remove(Flags::DIRTY);
        }
        Ok(self)
    }
    // ------------------------------------------------------------------------
    fn update_obj(param: &Param, obj: &Object, value: GLfloat) -> Result<()> {
        if let Param::Object(trs_type, data_idx) = param {
            let mut node = obj.as_node()?.borrow_mut();
            let trs = AsMut::<TraRotSca<GLfloat>>::as_mut(&mut *node);
            match trs_type {
                TraRotScaType::Translate => {
                    trs.translate[*data_idx] = value;
                }
                TraRotScaType::Rotate => {
                    trs.rotate[*data_idx] = value;
                }
                TraRotScaType::Scale => {
                    trs.scale[*data_idx] = value;
                }
            }
            Ok(())
        } else {
            Err(Error::AnimationDriver(
                "sif_graphics::animation::Driver: \
                 update_obj: driver has not Pram::Object."
                    .to_string(),
            ))
        }
    }
    // ------------------------------------------------------------------------
    fn update_pose(
        param: &Param,
        obj_data: &mut ObjectData,
        value: GLfloat,
    ) -> Result<()> {
        if let Param::Bone(bone_idx, trs_type, data_idx) = param {
            match obj_data {
                ObjectData::Model(_, Some(ref mut pose)) => {
                    match trs_type {
                        TraRotScaType::Translate => {
                            pose[*bone_idx].translate[*data_idx] = value;
                        }
                        TraRotScaType::Rotate => {
                            pose[*bone_idx].rotate[*data_idx] = value;
                        }
                        TraRotScaType::Scale => {
                            pose[*bone_idx].scale[*data_idx] = value;
                        }
                    }
                    Ok(())
                }
                _ => Err(Error::AnimationDriver(
                    "sif_graphics::animation::Driver: \
                     update_pose: object has not pose."
                        .to_string(),
                )),
            }
        } else {
            Err(Error::AnimationDriver(
                "sif_graphics::animation::Driver: \
                 update_pose: driver has Pram::Bone."
                    .to_string(),
            ))
        }
    }
}
