use std::mem;
use std::ops::Deref;

use joltc_sys::*;

/// Rust version of Jolt's [`RefTarget`](https://jrouwe.github.io/JoltPhysicsDocs/5.1.0/class_ref_target.html)
/// CRTP.
///
/// # Safety
///
/// The `value` pointers provided must be live. This trait should only be
/// implemented for C++ types that inherit from `RefTarget<Self>`.
#[allow(clippy::missing_safety_doc)]
pub unsafe trait RefTarget {
    unsafe fn add_ref(value: *const Self);
    unsafe fn release(value: *const Self);
}

/// Enables upcasting references safely.
///
/// # Safety
///
/// Self must be a base class of T. Self and T must have the same RefTarget base
/// class.
pub unsafe trait RefCast<T> {
    fn cast(value: *const Self) -> *const T;
    fn cast_mut(value: *mut Self) -> *mut T;
}

unsafe impl RefTarget for JPC_Shape {
    unsafe fn add_ref(value: *const Self) {
        JPC_Shape_AddRef(value);
    }

    unsafe fn release(value: *const Self) {
        JPC_Shape_Release(value);
    }
}

unsafe impl RefCast<JPC_Shape> for JPC_MutableCompoundShape {
    fn cast(value: *const Self) -> *const JPC_Shape {
        value.cast::<JPC_Shape>()
    }

    fn cast_mut(value: *mut Self) -> *mut JPC_Shape {
        value.cast::<JPC_Shape>()
    }
}

unsafe impl RefTarget for JPC_MutableCompoundShape {
    unsafe fn add_ref(value: *const Self) {
        JPC_Shape_AddRef(value.cast::<JPC_Shape>());
    }

    unsafe fn release(value: *const Self) {
        JPC_Shape_Release(value.cast::<JPC_Shape>());
    }
}

unsafe impl RefTarget for JPC_Constraint {
    unsafe fn add_ref(value: *const Self) {
        JPC_Constraint_AddRef(value);
    }

    unsafe fn release(value: *const Self) {
        JPC_Constraint_Release(value);
    }
}

/// Implement RefTarget for a concrete constraint type by casting to JPC_Constraint.
macro_rules! impl_constraint_ref_target {
    ($t:ty) => {
        unsafe impl RefTarget for $t {
            unsafe fn add_ref(value: *const Self) {
                JPC_Constraint_AddRef(value.cast::<JPC_Constraint>());
            }
            unsafe fn release(value: *const Self) {
                JPC_Constraint_Release(value.cast::<JPC_Constraint>());
            }
        }
        unsafe impl RefCast<JPC_Constraint> for $t {
            fn cast(value: *const Self) -> *const JPC_Constraint { value.cast() }
            fn cast_mut(value: *mut Self) -> *mut JPC_Constraint { value.cast() }
        }
    };
}

impl_constraint_ref_target!(JPC_HingeConstraint);
impl_constraint_ref_target!(JPC_SliderConstraint);
impl_constraint_ref_target!(JPC_DistanceConstraint);
impl_constraint_ref_target!(JPC_PointConstraint);
impl_constraint_ref_target!(JPC_ConeConstraint);
impl_constraint_ref_target!(JPC_PulleyConstraint);
impl_constraint_ref_target!(JPC_GearConstraint);
impl_constraint_ref_target!(JPC_RackAndPinionConstraint);
impl_constraint_ref_target!(JPC_SwingTwistConstraint);
impl_constraint_ref_target!(JPC_PathConstraint);

unsafe impl RefTarget for JPC_SoftBodySharedSettings {
    unsafe fn add_ref(value: *const Self) {
        JPC_SoftBodySharedSettings_AddRef(value);
    }
    unsafe fn release(value: *const Self) {
        JPC_SoftBodySharedSettings_Release(value);
    }
}

unsafe impl RefTarget for JPC_PhysicsMaterial {
    unsafe fn add_ref(value: *const Self) {
        JPC_PhysicsMaterial_AddRef(value);
    }
    unsafe fn release(value: *const Self) {
        JPC_PhysicsMaterial_Release(value);
    }
}

unsafe impl RefTarget for JPC_RagdollSettings {
    unsafe fn add_ref(value: *const Self) {
        JPC_RagdollSettings_AddRef(value);
    }
    unsafe fn release(value: *const Self) {
        JPC_RagdollSettings_Release(value);
    }
}

/// Rust equivalent to Jolt's [`RefConst`](https://jrouwe.github.io/JoltPhysicsDocs/5.1.0/class_ref_const.html)
pub struct RefConst<T: RefTarget> {
    ptr: *const T,
}

impl<T: RefTarget> RefConst<T> {
    /// Take ownership over a pointer and start reference counting it.
    ///
    /// # Safety
    ///
    /// `ptr` must be valid.
    pub unsafe fn from_active(ptr: *const T) -> Self {
        T::add_ref(ptr);
        Self { ptr }
    }

    /// Take ownership over a pointer that already has a reference counted for you.
    /// Use when the C API already called AddRef before returning the pointer.
    ///
    /// # Safety
    ///
    /// `ptr` must be valid and have an outstanding reference owned by the caller.
    pub unsafe fn from_addrefed(ptr: *const T) -> Self {
        Self { ptr }
    }

    pub fn get(&self) -> *const T {
        self.ptr
    }

    /// Call `f` with the raw pointer, guaranteed alive for the duration of the call.
    ///
    /// Analogous to wgpu's `as_hal` — the refcount is held while `f` runs, so the
    /// pointer is valid.  You still own all the usual JoltC safety obligations inside `f`.
    ///
    /// # Safety
    /// Caller must not destroy the object or violate Jolt's threading rules inside `f`.
    pub unsafe fn with_raw<R>(&self, f: impl FnOnce(*const T) -> R) -> R {
        f(self.ptr)
    }

    pub fn cast<U>(self) -> RefConst<U>
    where
        U: RefTarget,
        T: RefCast<U>,
    {
        let new = RefConst {
            ptr: RefCast::cast(self.ptr),
        };
        mem::forget(self);
        new
    }
}

impl<T: RefTarget> Deref for RefConst<T> {
    type Target = *const T;

    fn deref(&self) -> &Self::Target {
        &self.ptr
    }
}

impl<T: RefTarget> Clone for RefConst<T> {
    fn clone(&self) -> Self {
        unsafe {
            T::add_ref(self.ptr);
        }

        Self { ptr: self.ptr }
    }
}

impl<T: RefTarget> Drop for RefConst<T> {
    fn drop(&mut self) {
        unsafe {
            T::release(self.ptr);
        }
    }
}

/// Rust equivalent to Jolt's [`Ref`](https://jrouwe.github.io/JoltPhysicsDocs/5.1.0/class_ref.html)
pub struct Ref<T: RefTarget> {
    ptr: *mut T,
}

impl<T: RefTarget> Ref<T> {
    /// Take ownership over a pointer and start reference counting it.
    ///
    /// # Safety
    ///
    /// `ptr` must be valid.
    pub unsafe fn from_active(ptr: *mut T) -> Self {
        T::add_ref(ptr);
        Self { ptr }
    }

    /// Take ownership over a pointer that already has a reference counted for you.
    /// Use when the C API already called AddRef before returning the pointer.
    ///
    /// # Safety
    ///
    /// `ptr` must be valid and have an outstanding reference owned by the caller.
    pub unsafe fn from_addrefed(ptr: *mut T) -> Self {
        Self { ptr }
    }

    pub fn get(&self) -> *mut T {
        self.ptr
    }

    /// Call `f` with the raw pointer, guaranteed alive for the duration of the call.
    ///
    /// Analogous to wgpu's `as_hal` — the refcount is held while `f` runs, so the
    /// pointer is valid.  You still own all the usual JoltC safety obligations inside `f`.
    ///
    /// # Safety
    /// Caller must not destroy the object or violate Jolt's threading rules inside `f`.
    pub unsafe fn with_raw<R>(&self, f: impl FnOnce(*mut T) -> R) -> R {
        f(self.ptr)
    }

    pub fn cast<U>(self) -> Ref<U>
    where
        U: RefTarget,
        T: RefCast<U>,
    {
        let new = Ref {
            ptr: RefCast::cast_mut(self.ptr),
        };
        mem::forget(self);
        new
    }
}

impl<T: RefTarget> Deref for Ref<T> {
    type Target = *mut T;

    fn deref(&self) -> &Self::Target {
        &self.ptr
    }
}

impl<T: RefTarget> Clone for Ref<T> {
    fn clone(&self) -> Self {
        unsafe {
            T::add_ref(self.ptr);
        }

        Self { ptr: self.ptr }
    }
}

impl<T: RefTarget> Drop for Ref<T> {
    fn drop(&mut self) {
        unsafe {
            T::release(self.ptr);
        }
    }
}
