//! Safe Rust wrapper around [Jolt Physics](github.com/jrouwe/JoltPhysics) using
//! [JoltC](https://github.com/5unekku/JoltC).
//!
//! These bindings target Jolt Physics 5.5.0. You can view the C++ documentation
//! for this version of Jolt Physics here:
//!
//! <https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/>

use joltc_sys::*;

mod body;
mod body_interface;
mod body_lock;
mod character;
mod constraint;
mod conversions;
mod math;
mod narrow_phase;
mod physics_system;
mod reference;
mod remote_drop;
mod settings;
mod shape;
mod shape_settings;
mod simple_types;
mod ragdoll;
mod soft_body;
mod state_recorder;
mod traits;
mod vehicle;

pub use crate::body::*;
pub use crate::body_interface::*;
pub use crate::body_lock::*;
pub use crate::character::*;
pub use crate::constraint::*;
pub use crate::conversions::*;
pub use crate::math::*;
pub use crate::narrow_phase::*;
pub use crate::physics_system::*;
pub use crate::reference::*;
pub use crate::settings::*;
pub use crate::shape::*;
pub use crate::shape_settings::*;
pub use crate::simple_types::*;
pub use crate::ragdoll::*;
pub use crate::soft_body::*;
pub use crate::state_recorder::*;
pub use crate::traits::*;
pub use crate::vehicle::*;

/// [`JPH::RegisterDefaultAllocator`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/_memory_8h.html)
pub fn register_default_allocator() {
    unsafe { JPC_RegisterDefaultAllocator() }
}

/// Creates a new global factory. Required before registering types.
///
/// See also: Jolt's [`Factory`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_factory.html) class.
pub fn factory_init() {
    unsafe { JPC_FactoryInit() }
}

/// Deletes the globally registered factory.
pub fn factory_delete() {
    unsafe { JPC_FactoryDelete() }
}

/// Register all Jolt shape types and related factories.
///
/// See also: [`JPH::RegisterTypes`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/_register_types_8h.html)
pub fn register_types() {
    unsafe { JPC_RegisterTypes() }
}

/// Unregister all Jolt shape types.
pub fn unregister_types() {
    unsafe { JPC_UnregisterTypes() }
}
