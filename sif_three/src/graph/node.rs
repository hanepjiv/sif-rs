// -*- mode:rust; coding:utf-8-unix; -*-

//! node.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/02/25
//  @date 2018/05/12

// ////////////////////////////////////////////////////////////////////////////
// use  =======================================================================
use std::{cell::RefCell, fmt::Debug};
// ----------------------------------------------------------------------------
use uuid::Uuid;
// ----------------------------------------------------------------------------
use sif_manager::{ManagedValue, ManagedWeak};
use sif_math::{Matrix4x4, Number};
// ----------------------------------------------------------------------------
use super::super::{Error, Result, trarotsca::TraRotSca};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// Flags
bitflags! {
    #[allow(missing_docs)]
    pub struct Flags: u32 {
        #[allow(missing_docs)]
        const DIRTY = 0b0000_0000_0000_0000_0000_0000_0000_0001u32;
        #[allow(missing_docs)]
        const UPDATED       = 0b0000_0000_0000_0000_0000_0000_0000_0010u32;
    }
}
// ===========================================================================
impl Default for Flags {
    fn default() -> Self {
        Flags::DIRTY
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Node
#[derive(Debug, Default, Clone)]
pub struct Node<V>
where
    V: Debug + Number,
{
    /// uuid
    uuid: Uuid,
    /// parent
    parent: Option<ManagedWeak<Node<V>>>,
    /// matrix
    matrix: Matrix4x4<V>,
    /// inverse_matrix
    inverse_matrix: Matrix4x4<V>,
    /// trarotsca
    trarotsca: TraRotSca<V>,
    /// flags
    pub flags: Flags,
}
// ============================================================================
impl<V> AsRef<Uuid> for Node<V>
where
    V: Debug + Number,
{
    fn as_ref(&self) -> &Uuid {
        &self.uuid
    }
}
// ============================================================================
impl<V> AsRef<TraRotSca<V>> for Node<V>
where
    V: Debug + Number,
{
    fn as_ref(&self) -> &TraRotSca<V> {
        &self.trarotsca
    }
}
// ----------------------------------------------------------------------------
impl<V> AsMut<TraRotSca<V>> for Node<V>
where
    V: Debug + Number,
{
    fn as_mut(&mut self) -> &mut TraRotSca<V> {
        self.flags.insert(Flags::DIRTY);
        &mut self.trarotsca
    }
}
// ============================================================================
impl<V> Node<V>
where
    V: Debug + Number,
{
    // ========================================================================
    /// new
    pub fn new(uuid: Uuid, parent: Option<ManagedValue<Node<V>>>) -> Node<V> {
        Node {
            uuid,
            parent: parent.map(|v| -> ManagedWeak<Node<V>> { v.downgrade() }),
            matrix: Matrix4x4::<V>::default(),
            inverse_matrix: Matrix4x4::<V>::default(),
            trarotsca: TraRotSca::<V>::default(),
            flags: Flags::default(),
        }
    }
    // ========================================================================
    /// update
    pub fn update(&mut self) -> Result<&Matrix4x4<V>> {
        if let Some(ref parent) = self.parent {
            let prt = parent.upgrade().ok_or_else(|| {
                Error::OptNone(
                    "three::graph::node::Node::update: parent.upgrade"
                        .to_string(),
                )
            })?;
            let p = prt.as_ref();
            if p.borrow().flags.contains(Flags::DIRTY) {
                let _ = p.borrow_mut().update();
            }
            if !self.flags.contains(Flags::DIRTY)
                && !p.borrow().flags.contains(Flags::UPDATED)
            {
                return Ok(&self.matrix);
            }
            self.matrix = p.borrow().matrix * self.trarotsca.matrix();
            self.inverse_matrix =
                self.trarotsca.inverse_matrix() * p.borrow().inverse_matrix;
            self.flags.remove(Flags::DIRTY);
            self.flags.insert(Flags::UPDATED);
        } else {
            if !self.flags.contains(Flags::DIRTY) {
                return Ok(&self.matrix);
            }
            self.matrix = self.trarotsca.matrix();
            self.inverse_matrix = self.trarotsca.inverse_matrix();
            self.flags.remove(Flags::DIRTY);
            self.flags.insert(Flags::UPDATED);
        }
        Ok(&self.matrix)
    }
    // ========================================================================
    /// as_matrix
    pub fn as_matrix(&self) -> &Matrix4x4<V> {
        &self.matrix
    }
    // ------------------------------------------------------------------------
    /// as_inverse_matrix
    pub fn as_inverse_matrix(&self) -> &Matrix4x4<V> {
        &self.inverse_matrix
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// trait NodeHolder
pub trait NodeHolder: Debug {
    /// Number
    type Number: Debug + Number;
    // ========================================================================
    /// set_node
    fn set_node(&mut self, node: Option<ManagedValue<Node<Self::Number>>>);
    // ------------------------------------------------------------------------
    /// peek_node
    fn peek_node(&self) -> &Option<ManagedValue<Node<Self::Number>>>;
    // ------------------------------------------------------------------------
    /// has_node
    fn has_node(&self) -> bool;
    // ------------------------------------------------------------------------
    /// as_node
    fn as_node(&self) -> Result<&RefCell<Node<Self::Number>>>;
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct NodeHolderField
#[derive(Debug, Default, Clone)]
pub struct NodeHolderField<V>
where
    V: Debug + Number,
{
    /// node
    node: Option<ManagedValue<Node<V>>>,
}
// ============================================================================
impl<V> NodeHolder for NodeHolderField<V>
where
    V: Debug + Number,
{
    // ========================================================================
    /// type Number
    type Number = V;
    // ========================================================================
    /// set_node
    fn set_node(&mut self, node: Option<ManagedValue<Node<V>>>) {
        self.node = node
    }
    // ------------------------------------------------------------------------
    /// peek_node
    fn peek_node(&self) -> &Option<ManagedValue<Node<V>>> {
        &self.node
    }
    // ------------------------------------------------------------------------
    /// has_node
    fn has_node(&self) -> bool {
        self.node.is_some()
    }
    // ------------------------------------------------------------------------
    /// as_node
    fn as_node(&self) -> Result<&RefCell<Node<V>>> {
        if let Some(ref x) = self.node {
            Ok(x.as_ref())
        } else {
            Err(Error::NoNode)
        }
    }
}
// ============================================================================
/// trait AsNodeHolder
pub trait AsNodeHolder
where
    Self: Debug,
{
    // ========================================================================
    /// Number
    type Number: Debug + Number;
    // ========================================================================
    /// as_node_holder
    fn as_node_holder(&self) -> &NodeHolderField<Self::Number>;
    // ------------------------------------------------------------------------
    /// as_node_holder_mut
    fn as_node_holder_mut(&mut self) -> &mut NodeHolderField<Self::Number>;
}
// ============================================================================
impl<T> NodeHolder for T
where
    T: AsNodeHolder,
{
    // ========================================================================
    type Number = T::Number;
    // ========================================================================
    fn set_node(&mut self, node: Option<ManagedValue<Node<Self::Number>>>) {
        self.as_node_holder_mut().set_node(node)
    }
    // ------------------------------------------------------------------------
    fn peek_node(&self) -> &Option<ManagedValue<Node<Self::Number>>> {
        self.as_node_holder().peek_node()
    }
    // ------------------------------------------------------------------------
    fn has_node(&self) -> bool {
        self.as_node_holder().has_node()
    }
    // ------------------------------------------------------------------------
    fn as_node(&self) -> Result<&RefCell<Node<Self::Number>>> {
        self.as_node_holder().as_node()
    }
}
