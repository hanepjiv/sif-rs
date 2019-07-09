// -*- mode:rust; coding:utf-8-unix; -*-

//! mod.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/23
//  @date 2019/07/09

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_math::Float;
// ----------------------------------------------------------------------------
use super::{Error, Result};
// ----------------------------------------------------------------------------
pub use self::{
    bone::Bone,
    pose::{Flags as PoseFlags, Pose},
};
// mod  =======================================================================
mod bone;
mod pose;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Armature
#[derive(Debug, Clone)]
pub struct Armature<V>
where
    V: Float,
{
    /// uuid
    uuid: Uuid,
    /// name
    name: String,
    /// bones
    bones: Vec<Bone<V>>,
}
// ============================================================================
impl<V> Armature<V>
where
    V: Float,
{
    // ========================================================================
    /// fn new
    pub fn new(
        uuid: Uuid,
        name: impl Into<String>,
        mut bones: Vec<Bone<V>>,
    ) -> Self {
        bones.shrink_to_fit();
        Armature {
            uuid,
            name: name.into(),
            bones,
        }
    }
    // ========================================================================
    /// fn len
    pub fn len(&self) -> usize {
        self.bones.len()
    }
    // ========================================================================
    /// fn is_empty
    pub fn is_empty(&self) -> bool {
        self.bones.is_empty()
    }
    // ========================================================================
    /// fn update
    pub fn update(&self, pose: &mut Pose<V>) -> Result<&Armature<V>> {
        if self.len() != pose.len() {
            return Err(Error::InvalidPose);
        }
        let mut stack = Vec::<usize>::new();
        for i in (0..self.len()).rev() {
            stack.push(i);
        }
        while let Some(c) = stack.pop() {
            if let Some(p) = self.bones[c].parent {
                if pose.flags[p].contains(PoseFlags::DIRTY) {
                    stack.push(c);
                    stack.push(p);
                    continue;
                }
                if !pose.flags[c].contains(PoseFlags::DIRTY)
                    && !pose.flags[p].contains(PoseFlags::UPDATED)
                {
                    continue;
                }
                {
                    // update
                    let bone = &self.bones[c];
                    pose.matrix[c] = pose.matrix[p]
                        * bone.offset_matrix()
                        * pose[c].matrix()
                        * bone.inverse_offset_matrix();
                }
            } else {
                if !pose.flags[c].contains(PoseFlags::DIRTY) {
                    continue;
                }
                let bone = &self.bones[c];
                pose.matrix[c] = bone.offset_matrix()
                    * pose[c].matrix()
                    * bone.inverse_offset_matrix();
            }
            pose.flags[c].remove(PoseFlags::DIRTY);
            pose.flags[c].insert(PoseFlags::UPDATED);
        }
        for flags in &mut pose.flags {
            flags.remove(PoseFlags::DIRTY | PoseFlags::UPDATED);
        }
        Ok(self)
    }
}
// ============================================================================
impl<V> AsRef<Uuid> for Armature<V>
where
    V: Float,
{
    fn as_ref(&self) -> &Uuid {
        &self.uuid
    }
}
// ============================================================================
impl<V> AsRef<String> for Armature<V>
where
    V: Float,
{
    fn as_ref(&self) -> &String {
        &self.name
    }
}
// ============================================================================
impl<V> AsRef<Vec<Bone<V>>> for Armature<V>
where
    V: Float,
{
    fn as_ref(&self) -> &Vec<Bone<V>> {
        &self.bones
    }
}
// ----------------------------------------------------------------------------
impl<V> AsMut<Vec<Bone<V>>> for Armature<V>
where
    V: Float,
{
    fn as_mut(&mut self) -> &mut Vec<Bone<V>> {
        &mut self.bones
    }
}
