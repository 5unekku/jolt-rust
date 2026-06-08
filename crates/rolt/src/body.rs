use std::marker::PhantomData;

use joltc_sys::*;

use crate::{BodyId, BroadPhaseLayer, IntoJolt, IntoRolt, ObjectLayer, RVec3, Vec3, RMat4};

/// See also: Jolt's [`Body`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_body.html) class.
pub struct Body<'interface> {
    inner: *mut JPC_Body,
    _phantom: PhantomData<&'interface ()>,
}

impl<'interface> Body<'interface> {
    pub(crate) fn new(inner: *mut JPC_Body) -> Self {
        assert!(!inner.is_null());
        Self { inner, _phantom: PhantomData }
    }

    pub fn id(&self) -> BodyId {
        BodyId::new(unsafe { JPC_Body_GetID(self.inner) })
    }

    pub fn body_type(&self) -> JPC_BodyType {
        unsafe { JPC_Body_GetBodyType(self.inner) }
    }

    pub fn is_rigid_body(&self) -> bool { unsafe { JPC_Body_IsRigidBody(self.inner) } }
    pub fn is_soft_body(&self) -> bool { unsafe { JPC_Body_IsSoftBody(self.inner) } }
    pub fn is_active(&self) -> bool { unsafe { JPC_Body_IsActive(self.inner) } }
    pub fn is_static(&self) -> bool { unsafe { JPC_Body_IsStatic(self.inner) } }
    pub fn is_kinematic(&self) -> bool { unsafe { JPC_Body_IsKinematic(self.inner) } }
    pub fn is_dynamic(&self) -> bool { unsafe { JPC_Body_IsDynamic(self.inner) } }

    pub fn is_sensor(&self) -> bool { unsafe { JPC_Body_IsSensor(self.inner) } }
    pub fn set_is_sensor(&mut self, sensor: bool) {
        unsafe { JPC_Body_SetIsSensor(self.inner, sensor) }
    }

    pub fn motion_type(&self) -> JPC_MotionType {
        unsafe { JPC_Body_GetMotionType(self.inner) }
    }

    pub fn set_motion_type(&mut self, motion_type: JPC_MotionType) {
        unsafe { JPC_Body_SetMotionType(self.inner, motion_type) }
    }

    pub fn broad_phase_layer(&self) -> BroadPhaseLayer {
        BroadPhaseLayer::new(unsafe { JPC_Body_GetBroadPhaseLayer(self.inner) })
    }

    pub fn object_layer(&self) -> ObjectLayer {
        ObjectLayer::new(unsafe { JPC_Body_GetObjectLayer(self.inner) })
    }

    pub fn allow_sleeping(&self) -> bool { unsafe { JPC_Body_GetAllowSleeping(self.inner) } }
    pub fn set_allow_sleeping(&mut self, allow: bool) {
        unsafe { JPC_Body_SetAllowSleeping(self.inner, allow) }
    }

    pub fn reset_sleep_timer(&mut self) {
        unsafe { JPC_Body_ResetSleepTimer(self.inner) }
    }

    pub fn friction(&self) -> f32 { unsafe { JPC_Body_GetFriction(self.inner) } }
    pub fn set_friction(&mut self, friction: f32) {
        unsafe { JPC_Body_SetFriction(self.inner, friction) }
    }

    pub fn restitution(&self) -> f32 { unsafe { JPC_Body_GetRestitution(self.inner) } }
    pub fn set_restitution(&mut self, restitution: f32) {
        unsafe { JPC_Body_SetRestitution(self.inner, restitution) }
    }

    pub fn linear_velocity(&self) -> Vec3 {
        unsafe { JPC_Body_GetLinearVelocity(self.inner).into_rolt() }
    }

    pub fn set_linear_velocity(&mut self, velocity: Vec3) {
        unsafe { JPC_Body_SetLinearVelocity(self.inner, velocity.into_jolt()) }
    }

    pub fn angular_velocity(&self) -> Vec3 {
        unsafe { JPC_Body_GetAngularVelocity(self.inner).into_rolt() }
    }

    pub fn set_angular_velocity(&mut self, velocity: Vec3) {
        unsafe { JPC_Body_SetAngularVelocity(self.inner, velocity.into_jolt()) }
    }

    pub fn point_velocity(&self, point: RVec3) -> Vec3 {
        unsafe { JPC_Body_GetPointVelocity(self.inner, point.into_jolt()).into_rolt() }
    }

    pub fn add_force(&mut self, force: Vec3) {
        unsafe { JPC_Body_AddForce(self.inner, force.into_jolt()) }
    }

    pub fn add_force_at_point(&mut self, force: Vec3, position: RVec3) {
        unsafe { JPC_Body_AddForceAtPoint(self.inner, force.into_jolt(), position.into_jolt()) }
    }

    pub fn add_torque(&mut self, torque: Vec3) {
        unsafe { JPC_Body_AddTorque(self.inner, torque.into_jolt()) }
    }

    pub fn accumulated_force(&self) -> Vec3 {
        unsafe { JPC_Body_GetAccumulatedForce(self.inner).into_rolt() }
    }

    pub fn accumulated_torque(&self) -> Vec3 {
        unsafe { JPC_Body_GetAccumulatedTorque(self.inner).into_rolt() }
    }

    pub fn reset_force(&mut self) { unsafe { JPC_Body_ResetForce(self.inner) } }
    pub fn reset_torque(&mut self) { unsafe { JPC_Body_ResetTorque(self.inner) } }
    pub fn reset_motion(&mut self) { unsafe { JPC_Body_ResetMotion(self.inner) } }

    pub fn add_impulse(&mut self, impulse: Vec3) {
        unsafe { JPC_Body_AddImpulse(self.inner, impulse.into_jolt()) }
    }

    pub fn add_angular_impulse(&mut self, impulse: Vec3) {
        unsafe { JPC_Body_AddAngularImpulse(self.inner, impulse.into_jolt()) }
    }

    pub fn position(&self) -> RVec3 {
        unsafe { JPC_Body_GetPosition(self.inner).into_rolt() }
    }

    pub fn rotation(&self) -> crate::Quat {
        unsafe { JPC_Body_GetRotation(self.inner).into_rolt() }
    }

    pub fn world_transform(&self) -> RMat4 {
        unsafe { JPC_Body_GetWorldTransform(self.inner).into_rolt() }
    }

    pub fn center_of_mass_position(&self) -> RVec3 {
        unsafe { JPC_Body_GetCenterOfMassPosition(self.inner).into_rolt() }
    }

    pub fn center_of_mass_transform(&self) -> RMat4 {
        unsafe { JPC_Body_GetCenterOfMassTransform(self.inner).into_rolt() }
    }

    pub fn shape(&self) -> *const JPC_Shape {
        unsafe { JPC_Body_GetShape(self.inner) }
    }

    pub fn user_data(&self) -> u64 { unsafe { JPC_Body_GetUserData(self.inner) } }
    pub fn set_user_data(&mut self, data: u64) {
        unsafe { JPC_Body_SetUserData(self.inner, data) }
    }

    pub fn use_manifold_reduction(&self) -> bool {
        unsafe { JPC_Body_GetUseManifoldReduction(self.inner) }
    }

    pub fn set_use_manifold_reduction(&mut self, value: bool) {
        unsafe { JPC_Body_SetUseManifoldReduction(self.inner, value) }
    }

    pub fn apply_gyroscopic_force(&self) -> bool {
        unsafe { JPC_Body_GetApplyGyroscopicForce(self.inner) }
    }

    pub fn set_apply_gyroscopic_force(&mut self, value: bool) {
        unsafe { JPC_Body_SetApplyGyroscopicForce(self.inner, value) }
    }

    pub fn enhanced_internal_edge_removal(&self) -> bool {
        unsafe { JPC_Body_GetEnhancedInternalEdgeRemoval(self.inner) }
    }

    pub fn set_enhanced_internal_edge_removal(&mut self, value: bool) {
        unsafe { JPC_Body_SetEnhancedInternalEdgeRemoval(self.inner, value) }
    }

    /// # Safety
    /// See [`Ref::with_raw`].
    pub unsafe fn with_raw<R>(&self, f: impl FnOnce(*mut JPC_Body) -> R) -> R {
        f(self.inner)
    }

    pub fn raw(&self) -> *mut JPC_Body { self.inner }
}
