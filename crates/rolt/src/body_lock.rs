use std::marker::PhantomData;

use joltc_sys::*;

use crate::{Body, BodyId};

/// Provides RAII body locking via [`BodyLockRead`], [`BodyLockWrite`], etc.
///
/// See also: Jolt's [`BodyLockInterface`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_body_lock_interface.html) class.
pub struct BodyLockInterface<'system> {
    raw: *const JPC_BodyLockInterface,
    _phantom: PhantomData<&'system ()>,
}

impl<'system> BodyLockInterface<'system> {
    pub(crate) fn new(raw: *const JPC_BodyLockInterface) -> Self {
        Self { raw, _phantom: PhantomData }
    }

    /// Acquire a read (shared) lock on a single body.
    pub fn read(&self, body_id: BodyId) -> BodyLockRead<'_> {
        BodyLockRead::new(self.raw, body_id)
    }

    /// Acquire a write (exclusive) lock on a single body.
    pub fn write(&self, body_id: BodyId) -> BodyLockWrite<'_> {
        BodyLockWrite::new(self.raw, body_id)
    }

    /// Acquire read (shared) locks on multiple bodies at once.
    pub fn multi_read<'lock>(&'lock self, body_ids: &[BodyId]) -> BodyLockMultiRead<'lock> {
        BodyLockMultiRead::new(self.raw, body_ids)
    }

    /// Acquire write (exclusive) locks on multiple bodies at once.
    pub fn multi_write<'lock>(&'lock self, body_ids: &[BodyId]) -> BodyLockMultiWrite<'lock> {
        BodyLockMultiWrite::new(self.raw, body_ids)
    }
}

/// RAII read lock on a single body. Unlocks when dropped.
///
/// See also: Jolt's [`BodyLockRead`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_body_lock_read.html) class.
pub struct BodyLockRead<'lock> {
    raw: *mut JPC_BodyLockRead,
    _phantom: PhantomData<&'lock ()>,
}

impl<'lock> BodyLockRead<'lock> {
    fn new(interface: *const JPC_BodyLockInterface, body_id: BodyId) -> Self {
        let raw = unsafe { JPC_BodyLockRead_new(interface, body_id.raw()) };
        Self { raw, _phantom: PhantomData }
    }

    pub fn succeeded(&self) -> bool {
        unsafe { JPC_BodyLockRead_Succeeded(self.raw) }
    }

    /// Returns the locked body, or `None` if the lock failed.
    pub fn get(&self) -> Option<Body<'_>> {
        if !self.succeeded() {
            return None;
        }
        // cast_mut: we only expose &Body (immutable) while the read lock is held
        let ptr = unsafe { JPC_BodyLockRead_GetBody(self.raw) }.cast_mut();
        if ptr.is_null() { None } else { Some(Body::new(ptr)) }
    }
}

impl<'lock> Drop for BodyLockRead<'lock> {
    fn drop(&mut self) {
        unsafe { JPC_BodyLockRead_delete(self.raw) }
    }
}

/// RAII write lock on a single body. Unlocks when dropped.
///
/// See also: Jolt's [`BodyLockWrite`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_body_lock_write.html) class.
pub struct BodyLockWrite<'lock> {
    raw: *mut JPC_BodyLockWrite,
    _phantom: PhantomData<&'lock ()>,
}

impl<'lock> BodyLockWrite<'lock> {
    fn new(interface: *const JPC_BodyLockInterface, body_id: BodyId) -> Self {
        let raw = unsafe { JPC_BodyLockWrite_new(interface, body_id.raw()) };
        Self { raw, _phantom: PhantomData }
    }

    pub fn succeeded(&self) -> bool {
        unsafe { JPC_BodyLockWrite_Succeeded(self.raw) }
    }

    /// Returns the locked body, or `None` if the lock failed.
    pub fn get(&mut self) -> Option<Body<'_>> {
        if !self.succeeded() {
            return None;
        }
        let ptr = unsafe { JPC_BodyLockWrite_GetBody(self.raw) };
        if ptr.is_null() { None } else { Some(Body::new(ptr)) }
    }
}

impl<'lock> Drop for BodyLockWrite<'lock> {
    fn drop(&mut self) {
        unsafe { JPC_BodyLockWrite_delete(self.raw) }
    }
}

/// RAII read lock on multiple bodies at once. Unlocks when dropped.
///
/// See also: Jolt's [`BodyLockMultiRead`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_body_lock_multi_read.html) class.
pub struct BodyLockMultiRead<'lock> {
    raw: *mut JPC_BodyLockMultiRead,
    _phantom: PhantomData<&'lock ()>,
}

impl<'lock> BodyLockMultiRead<'lock> {
    fn new(interface: *const JPC_BodyLockInterface, body_ids: &[BodyId]) -> Self {
        let ids: Vec<JPC_BodyID> = body_ids.iter().map(|b| b.raw()).collect();
        let raw = unsafe {
            JPC_BodyLockMultiRead_new(interface, ids.as_ptr(), ids.len() as i32)
        };
        Self { raw, _phantom: PhantomData }
    }

    /// Returns the body at `index` into the slice passed to [`BodyLockInterface::multi_read`].
    pub fn get(&self, index: usize) -> Option<Body<'_>> {
        let ptr = unsafe { JPC_BodyLockMultiRead_GetBody(self.raw, index as i32) }.cast_mut();
        if ptr.is_null() { None } else { Some(Body::new(ptr)) }
    }
}

impl<'lock> Drop for BodyLockMultiRead<'lock> {
    fn drop(&mut self) {
        unsafe { JPC_BodyLockMultiRead_delete(self.raw) }
    }
}

/// RAII write lock on multiple bodies at once. Unlocks when dropped.
///
/// See also: Jolt's [`BodyLockMultiWrite`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_body_lock_multi_write.html) class.
pub struct BodyLockMultiWrite<'lock> {
    raw: *mut JPC_BodyLockMultiWrite,
    _phantom: PhantomData<&'lock ()>,
}

impl<'lock> BodyLockMultiWrite<'lock> {
    fn new(interface: *const JPC_BodyLockInterface, body_ids: &[BodyId]) -> Self {
        let ids: Vec<JPC_BodyID> = body_ids.iter().map(|b| b.raw()).collect();
        let raw = unsafe {
            JPC_BodyLockMultiWrite_new(interface, ids.as_ptr(), ids.len() as i32)
        };
        Self { raw, _phantom: PhantomData }
    }

    /// Returns the body at `index` into the slice passed to [`BodyLockInterface::multi_write`].
    pub fn get(&mut self, index: usize) -> Option<Body<'_>> {
        let ptr = unsafe { JPC_BodyLockMultiWrite_GetBody(self.raw, index as i32) };
        if ptr.is_null() { None } else { Some(Body::new(ptr)) }
    }
}

impl<'lock> Drop for BodyLockMultiWrite<'lock> {
    fn drop(&mut self) {
        unsafe { JPC_BodyLockMultiWrite_delete(self.raw) }
    }
}
