// -*- mode:rust; coding:utf-8-unix; -*-

//! mod.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/23
//  @date 2018/05/12

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::{borrow::Borrow, fmt::Debug, hash::Hash};
// ----------------------------------------------------------------------------
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_manager::{ManagedValue, Manager};
use sif_math::Number;
// ----------------------------------------------------------------------------
use super::{Error, Result};
// ----------------------------------------------------------------------------
pub use self::node::{AsNodeHolder, Flags as NodeFlags, Node, NodeHolder,
                     NodeHolderField};
// mod  =======================================================================
#[macro_use]
mod node;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Graph
#[derive(Debug, Clone)]
pub struct Graph<V>
where
    V: Debug + Number,
{
    /// uuid
    uuid: Uuid,
    /// nodes
    nodes: Manager<Node<V>>,
    /// root
    root: ManagedValue<Node<V>>,
}
// ============================================================================
impl<V> AsRef<Uuid> for Graph<V>
where
    V: Debug + Number,
{
    fn as_ref(&self) -> &Uuid {
        &self.uuid
    }
}
// ============================================================================
impl<V> AsRef<Manager<Node<V>>> for Graph<V>
where
    V: Debug + Number,
{
    fn as_ref(&self) -> &Manager<Node<V>> {
        &self.nodes
    }
}
// ----------------------------------------------------------------------------
impl<V> AsMut<Manager<Node<V>>> for Graph<V>
where
    V: Debug + Number,
{
    fn as_mut(&mut self) -> &mut Manager<Node<V>> {
        &mut self.nodes
    }
}
// ============================================================================
impl<V> Graph<V>
where
    V: Debug + Number,
{
    // ========================================================================
    /// new
    pub fn new(uuid: Uuid) -> Result<Self> {
        let mut nodes = Manager::<Node<V>>::default();
        let _ = nodes.insert(Node::<V>::new(uuid, None))?;
        let root = nodes
            .get(&uuid)
            .ok_or_else(|| {
                Error::OptNone(format!(
                    "three::graph::Graph::new: nodes.get(\"{}\")",
                    uuid
                ))
            })?
            .clone();
        Ok(Graph {
            uuid,
            nodes,
            root,
        })
    }
    // ========================================================================
    /// root
    pub fn root(&self) -> &ManagedValue<Node<V>> {
        &self.root
    }
    // ------------------------------------------------------------------------
    /// root_mut
    pub fn root_mut(&mut self) -> &mut ManagedValue<Node<V>> {
        &mut self.root
    }
    // ========================================================================
    /// insert
    pub fn insert(
        &mut self,
        uuid: Uuid,
        parent: Option<ManagedValue<Node<V>>>,
    ) -> Result<Uuid> {
        let p = parent.or_else(|| Some(self.root.clone()));
        Ok(self.nodes.insert(Node::<V>::new(uuid, p))?)
    }
    // ========================================================================
    /// contains_key
    pub fn contains_key<U: ?Sized>(&self, uuid: &U) -> bool
    where
        Uuid: Borrow<U>,
        U: Hash + Ord,
    {
        self.nodes.contains_key(uuid)
    }
    // ========================================================================
    /// get
    pub fn get<U: ?Sized>(&self, uuid: &U) -> Option<ManagedValue<Node<V>>>
    where
        Uuid: Borrow<U>,
        U: Hash + Ord,
    {
        self.nodes.get(uuid).cloned()
    }
    // ========================================================================
    /// remove
    pub fn remove<U: ?Sized>(
        &mut self,
        uuid: &U,
    ) -> Option<ManagedValue<Node<V>>>
    where
        Uuid: Borrow<U>,
        U: Hash + Ord,
    {
        self.nodes.remove(uuid)
    }
    // ========================================================================
    /// update
    pub fn update(&self) {
        for (_, v) in self.nodes.iter() {
            let _ = v.as_ref().borrow_mut().update();
        }
        for (_, v) in self.nodes.iter() {
            let mut flags = v.as_ref().borrow_mut().flags;
            flags.remove(NodeFlags::DIRTY | NodeFlags::UPDATED);
        }
    }
}
