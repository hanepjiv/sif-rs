// -*- mode:rust; coding:utf-8-unix; -*-

//! mod.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/05/23
//  @date 2018/07/31

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::{borrow::Borrow, fmt::Debug, hash::Hash, iter::IntoIterator};
// ----------------------------------------------------------------------------
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_manager::{ManagedValue, Manager, ManagerIter, ManagerIterMut};
use sif_math::Number;
// ----------------------------------------------------------------------------
use super::{Error, Result};
// ----------------------------------------------------------------------------
pub use self::node::{
    AsNodeHolder, Flags as NodeFlags, Node, NodeHolder, NodeHolderField,
};
// mod  =======================================================================
#[macro_use]
mod node;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// type GraphIter
type GraphIter<'a, V> = ManagerIter<'a, Node<V>>;
// ============================================================================
/// type GraphIterMut
type GraphIterMut<'a, V> = ManagerIterMut<'a, Node<V>>;
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
            })?.clone();
        Ok(Graph { uuid, nodes, root })
    }
    // ========================================================================
    /// iter
    pub fn iter(&self) -> GraphIter<V> {
        self.nodes.iter()
    }
    // ------------------------------------------------------------------------
    /// iter_mut
    pub fn iter_mut(&mut self) -> GraphIterMut<V> {
        self.nodes.iter_mut()
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
    pub fn insert(&mut self, v: ManagedValue<Node<V>>) -> Result<Uuid> {
        Ok(self.nodes.insert_managed(v)?)
    }
    // ------------------------------------------------------------------------
    /// emplace
    pub fn emplace(
        &mut self,
        uuid: Uuid,
        parent: Option<ManagedValue<Node<V>>>,
    ) -> Result<Uuid> {
        let p = parent.or_else(|| Some(self.root.clone()));
        Ok(self.nodes.insert(Node::<V>::new(uuid, p))?)
    }
    // ========================================================================
    /// contains_key
    pub fn contains_key<U>(&self, uuid: &U) -> bool
    where
        Uuid: Borrow<U>,
        U: ?Sized + Hash + Ord,
    {
        self.nodes.contains_key(uuid)
    }
    // ========================================================================
    /// get
    pub fn get<U>(&self, uuid: &U) -> Option<ManagedValue<Node<V>>>
    where
        Uuid: Borrow<U>,
        U: ?Sized + Hash + Ord,
    {
        self.nodes.get(uuid).cloned()
    }
    // ========================================================================
    /// remove
    pub fn remove<U>(&mut self, uuid: &U) -> Option<ManagedValue<Node<V>>>
    where
        Uuid: Borrow<U>,
        U: ?Sized + Hash + Ord,
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
// ============================================================================
impl<'a, V> IntoIterator for &'a Graph<V>
where
    V: Debug + Number,
{
    type Item = <GraphIter<'a, V> as IntoIterator>::Item;
    type IntoIter = GraphIter<'a, V>;
    // ========================================================================
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
// ----------------------------------------------------------------------------
impl<'a, V> IntoIterator for &'a mut Graph<V>
where
    V: Debug + Number,
{
    type Item = <GraphIterMut<'a, V> as IntoIterator>::Item;
    type IntoIter = GraphIterMut<'a, V>;
    // ========================================================================
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
