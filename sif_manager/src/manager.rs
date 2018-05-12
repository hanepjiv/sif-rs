// -*- mode:rust; coding:utf-8-unix; -*-

//! manager.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/02/27
//  @date 2018/05/12

// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
use std::{borrow::Borrow, cell::RefCell,
          collections::{BTreeMap, btree_map::Iter}, fmt::Debug,
          hash::{Hash, Hasher}, rc::{Rc, Weak}};
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
/// struct Manager
#[derive(Debug, Clone)]
pub struct Manager<T>
where
    T: Debug + AsRef<Uuid>,
{
    /// map
    map: BTreeMap<Uuid, ManagedValue<T>>,
}
// ============================================================================
impl<T> Default for Manager<T>
where
    T: Debug + AsRef<Uuid>,
{
    fn default() -> Self {
        Manager {
            map: BTreeMap::<Uuid, ManagedValue<T>>::default(),
        }
    }
}
// ============================================================================
impl<T> Manager<T>
where
    T: Debug + AsRef<Uuid>,
{
    // ========================================================================
    /// insert
    pub fn insert(&mut self, x: T) -> Result<Uuid> {
        let uuid: Uuid = *x.as_ref();
        if let Some(prev) = self.map.insert(uuid, ManagedValue::new(x)) {
            Err(Error::Insert(*prev.as_ref().borrow().as_ref()))
        } else {
            Ok(uuid)
        }
    }
    // ------------------------------------------------------------------------
    /// insert_managed
    pub fn insert_managed(&mut self, x: ManagedValue<T>) -> Result<Uuid> {
        let uuid: Uuid = *x.as_ref().borrow().as_ref();
        if let Some(prev) = self.map.insert(uuid, x) {
            Err(Error::Insert(*prev.as_ref().borrow().as_ref()))
        } else {
            Ok(uuid)
        }
    }
    // ========================================================================
    /// get
    pub fn get<U: ?Sized>(&self, uuid: &U) -> Option<&ManagedValue<T>>
    where
        Uuid: Borrow<U>,
        U: Hash + Ord,
    {
        self.map.get(uuid)
    }
    // ========================================================================
    /// remove
    pub fn remove<U: ?Sized>(&mut self, uuid: &U) -> Option<ManagedValue<T>>
    where
        Uuid: Borrow<U>,
        U: Hash + Ord,
    {
        self.map.remove(uuid)
    }
    // ========================================================================
    /// contains_key
    pub fn contains_key<U: ?Sized>(&self, uuid: &U) -> bool
    where
        Uuid: Borrow<U>,
        U: Hash + Ord,
    {
        self.map.contains_key(uuid)
    }
    // ========================================================================
    /// iter
    pub fn iter(&self) -> Iter<Uuid, ManagedValue<T>> {
        self.map.iter()
    }
}
