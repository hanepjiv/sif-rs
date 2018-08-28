// -*- mode:rust; coding:utf-8-unix; -*-

//! manager.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/02/27
//  @date 2018/08/21

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use std::{
    borrow::Borrow,
    cell::RefCell,
    collections::{btree_map, BTreeMap},
    fmt::Debug,
    hash::{Hash, Hasher},
    rc::{Rc, Weak},
};
// ----------------------------------------------------------------------------
use uuid::Uuid;
// ----------------------------------------------------------------------------
use super::{Error, Result};
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct ManagedValue
#[derive(Debug)]
pub struct ManagedValue<T: AsRef<Uuid>>(Rc<RefCell<T>>);
// ============================================================================
impl<T> ManagedValue<T>
where
    T: Debug + AsRef<Uuid>,
{
    // ========================================================================
    /// new
    pub fn new(value: T) -> Self {
        ManagedValue(Rc::new(RefCell::new(value)))
    }
    // ========================================================================
    /// downgrade
    pub fn downgrade(&self) -> ManagedWeak<T> {
        ManagedWeak(Rc::<RefCell<T>>::downgrade(&self.0))
    }
}
// ============================================================================
impl<T> Clone for ManagedValue<T>
where
    T: Debug + AsRef<Uuid>,
{
    fn clone(&self) -> Self {
        ManagedValue(self.0.clone())
    }
}
// ============================================================================
impl<T> Hash for ManagedValue<T>
where
    T: Debug + AsRef<Uuid> + Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.0.as_ref().borrow().hash(state)
    }
}
// ============================================================================
impl<T> AsRef<RefCell<T>> for ManagedValue<T>
where
    T: Debug + AsRef<Uuid>,
{
    fn as_ref(&self) -> &RefCell<T> {
        self.0.as_ref()
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct ManagedWeak
#[derive(Debug)]
pub struct ManagedWeak<T: AsRef<Uuid>>(Weak<RefCell<T>>);
// ============================================================================
impl<T> Clone for ManagedWeak<T>
where
    T: Debug + AsRef<Uuid>,
{
    fn clone(&self) -> Self {
        ManagedWeak(self.0.clone())
    }
}
// ============================================================================
impl<T> ManagedWeak<T>
where
    T: Debug + AsRef<Uuid>,
{
    /// upgrade
    pub fn upgrade(&self) -> Option<ManagedValue<T>> {
        match self.0.upgrade() {
            None => None,
            Some(x) => Some(ManagedValue(x)),
        }
    }
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// type ManagerIter
pub type ManagerIter<'a, T> = btree_map::Iter<'a, Uuid, ManagedValue<T>>;
// ============================================================================
/// type ManagerIterMut
pub type ManagerIterMut<'a, T> = btree_map::IterMut<'a, Uuid, ManagedValue<T>>;
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
/// struct Manager
#[derive(Debug, Clone)]
pub struct Manager<T>(BTreeMap<Uuid, ManagedValue<T>>)
where
    T: Debug + AsRef<Uuid>;
// ============================================================================
impl<T> Default for Manager<T>
where
    T: Debug + AsRef<Uuid>,
{
    fn default() -> Self {
        Manager(BTreeMap::<Uuid, ManagedValue<T>>::default())
    }
}
// ============================================================================
impl<T> Manager<T>
where
    T: Debug + AsRef<Uuid>,
{
    // ========================================================================
    /// iter
    pub fn iter(&self) -> ManagerIter<T> {
        self.0.iter()
    }
    // ------------------------------------------------------------------------
    /// iter_mut
    pub fn iter_mut(&mut self) -> ManagerIterMut<T> {
        self.0.iter_mut()
    }
    // ========================================================================
    /// insert
    pub fn insert(&mut self, x: T) -> Result<Uuid> {
        let uuid: Uuid = *x.as_ref();
        if let Some(prev) = self.0.insert(uuid, ManagedValue::new(x)) {
            Err(Error::Insert(*prev.as_ref().borrow().as_ref()))
        } else {
            Ok(uuid)
        }
    }
    // ------------------------------------------------------------------------
    /// insert_managed
    pub fn insert_managed(&mut self, x: ManagedValue<T>) -> Result<Uuid> {
        let uuid: Uuid = *x.as_ref().borrow().as_ref();
        if let Some(prev) = self.0.insert(uuid, x) {
            Err(Error::Insert(*prev.as_ref().borrow().as_ref()))
        } else {
            Ok(uuid)
        }
    }
    // ========================================================================
    /// get
    pub fn get<U>(&self, uuid: &U) -> Option<&ManagedValue<T>>
    where
        Uuid: Borrow<U>,
        U: ?Sized + Hash + Ord,
    {
        self.0.get(uuid)
    }
    // ========================================================================
    /// contains_key
    pub fn contains_key<U>(&self, uuid: &U) -> bool
    where
        Uuid: Borrow<U>,
        U: ?Sized + Hash + Ord,
    {
        self.0.contains_key(uuid)
    }
    // ========================================================================
    /// remove
    pub fn remove<U>(&mut self, uuid: &U) -> Option<ManagedValue<T>>
    where
        Uuid: Borrow<U>,
        U: ?Sized + Hash + Ord,
    {
        self.0.remove(uuid)
    }
    // ========================================================================
    /// append
    pub fn append(&mut self, other: &mut Self) {
        self.0.append(&mut other.0)
    }
}
// ============================================================================
impl<'a, T> IntoIterator for &'a Manager<T>
where
    T: Debug + AsRef<Uuid> + Hash,
{
    type Item = <btree_map::Iter<'a, Uuid, ManagedValue<T>> as Iterator>::Item;
    type IntoIter = btree_map::Iter<'a, Uuid, ManagedValue<T>>;
    // ========================================================================
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
// ----------------------------------------------------------------------------
impl<'a, T> IntoIterator for &'a mut Manager<T>
where
    T: Debug + AsRef<Uuid> + Hash,
{
    type Item =
        <btree_map::IterMut<'a, Uuid, ManagedValue<T>> as Iterator>::Item;
    type IntoIter = btree_map::IterMut<'a, Uuid, ManagedValue<T>>;
    // ========================================================================
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
