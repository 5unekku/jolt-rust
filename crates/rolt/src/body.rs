use std::marker::PhantomData;

use joltc_sys::*;

use crate::{BodyId, BroadPhaseLayer, IntoJolt, IntoRolt, Mat4, ObjectLayer, Quat, RVec3, Vec3, RMat4};

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

    /// sensor bodies detect overlaps without applying collision forces.
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

    /// whether Jolt is allowed to put this body to sleep when it stops moving.
    pub fn allow_sleeping(&self) -> bool { unsafe { JPC_Body_GetAllowSleeping(self.inner) } }
    pub fn set_allow_sleeping(&mut self, allow: bool) {
        unsafe { JPC_Body_SetAllowSleeping(self.inner, allow) }
    }

    /// restart the inactivity timer so sleep is not triggered on the next step.
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

    /// total force accumulated this step via [`add_force`][Self::add_force] calls.
    pub fn accumulated_force(&self) -> Vec3 {
        unsafe { JPC_Body_GetAccumulatedForce(self.inner).into_rolt() }
    }

    /// total torque accumulated this step via [`add_torque`][Self::add_torque] calls.
    pub fn accumulated_torque(&self) -> Vec3 {
        unsafe { JPC_Body_GetAccumulatedTorque(self.inner).into_rolt() }
    }

    pub fn reset_force(&mut self) { unsafe { JPC_Body_ResetForce(self.inner) } }
    pub fn reset_torque(&mut self) { unsafe { JPC_Body_ResetTorque(self.inner) } }
    /// clear accumulated forces, torques, and velocities.
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

    pub fn can_be_kinematic_or_dynamic(&self) -> bool {
        unsafe { JPC_Body_CanBeKinematicOrDynamic(self.inner) }
    }

    pub fn collide_kinematic_vs_non_dynamic(&self) -> bool {
        unsafe { JPC_Body_GetCollideKinematicVsNonDynamic(self.inner) }
    }

    pub fn set_collide_kinematic_vs_non_dynamic(&mut self, collide: bool) {
        unsafe { JPC_Body_SetCollideKinematicVsNonDynamic(self.inner, collide) }
    }

    /// `true` if manifold reduction is enabled for collisions with `other`.
    pub fn use_manifold_reduction_with_body(&self, other: &Body<'_>) -> bool {
        unsafe { JPC_Body_GetUseManifoldReductionWithBody(self.inner, other.inner) }
    }

    /// `true` if enhanced internal edge removal is enabled for collisions with `other`.
    pub fn enhanced_internal_edge_removal_with_body(&self, other: &Body<'_>) -> bool {
        unsafe { JPC_Body_GetEnhancedInternalEdgeRemovalWithBody(self.inner, other.inner) }
    }

    /// like [`set_linear_velocity`][Self::set_linear_velocity] but clamps to max linear velocity.
    pub fn set_linear_velocity_clamped(&mut self, velocity: Vec3) {
        unsafe { JPC_Body_SetLinearVelocityClamped(self.inner, velocity.into_jolt()) }
    }

    /// like [`set_angular_velocity`][Self::set_angular_velocity] but clamps to max angular velocity.
    pub fn set_angular_velocity_clamped(&mut self, velocity: Vec3) {
        unsafe { JPC_Body_SetAngularVelocityClamped(self.inner, velocity.into_jolt()) }
    }

    /// velocity of a point in body-local space (offset from center of mass).
    pub fn point_velocity_com(&self, point: Vec3) -> Vec3 {
        unsafe { JPC_Body_GetPointVelocityCOM(self.inner, point.into_jolt()).into_rolt() }
    }

    /// 4x4 inverse inertia tensor in world space.
    pub fn inverse_inertia(&self) -> Mat4 {
        let mut out = unsafe { std::mem::zeroed() };
        unsafe { JPC_Body_GetInverseInertia(self.inner, &mut out) }
        out.into_rolt()
    }

    pub fn add_impulse_at_point(&mut self, impulse: Vec3, point: RVec3) {
        unsafe { JPC_Body_AddImpulse2(self.inner, impulse.into_jolt(), point.into_jolt()) }
    }

    /// teleport a kinematic body to the target pose over `delta_time`, computing
    /// the velocity needed to reach it in one step (used for interpolation).
    pub fn move_kinematic(&mut self, target_position: RVec3, target_rotation: Quat, delta_time: f32) {
        unsafe { JPC_Body_MoveKinematic(self.inner, target_position.into_jolt(), target_rotation.into_jolt(), delta_time) }
    }

    /// apply a buoyancy impulse for a body partially submerged in fluid.
    ///
    /// `buoyancy` > 1 makes the body float, < 1 makes it sink.
    /// returns `true` if any part of the body was below the surface.
    pub fn apply_buoyancy_impulse(
        &mut self,
        surface_position: RVec3,
        surface_normal: Vec3,
        buoyancy: f32,
        linear_drag: f32,
        angular_drag: f32,
        fluid_velocity: Vec3,
        gravity: Vec3,
        delta_time: f32,
    ) -> bool {
        unsafe {
            JPC_Body_ApplyBuoyancyImpulse(
                self.inner,
                surface_position.into_jolt(),
                surface_normal.into_jolt(),
                buoyancy,
                linear_drag,
                angular_drag,
                fluid_velocity.into_jolt(),
                gravity.into_jolt(),
                delta_time,
            )
        }
    }

    /// whether the body is currently registered with the broadphase.
    pub fn is_in_broad_phase(&self) -> bool { unsafe { JPC_Body_IsInBroadPhase(self.inner) } }
    /// `true` if the contact cache was explicitly invalidated and needs rebuilding.
    pub fn is_collision_cache_invalid(&self) -> bool { unsafe { JPC_Body_IsCollisionCacheInvalid(self.inner) } }

    pub fn inverse_center_of_mass_transform(&self) -> RMat4 {
        unsafe { JPC_Body_GetInverseCenterOfMassTransform(self.inner).into_rolt() }
    }

    /// surface normal at `position` on the given sub-shape, in world space.
    pub fn world_space_surface_normal(&self, sub_shape_id: JPC_SubShapeID, position: RVec3) -> Vec3 {
        unsafe { JPC_Body_GetWorldSpaceSurfaceNormal(self.inner, sub_shape_id, position.into_jolt()).into_rolt() }
    }

    pub fn collision_group(&self) -> JPC_CollisionGroup {
        unsafe { JPC_Body_GetCollisionGroup(self.inner) }
    }

    pub fn set_collision_group(&mut self, group: &JPC_CollisionGroup) {
        unsafe { JPC_Body_SetCollisionGroup(self.inner, group) }
    }

    pub fn with_raw<R>(&self, f: impl FnOnce(*mut JPC_Body) -> R) -> R {
        f(self.inner)
    }

    pub fn raw(&self) -> *mut JPC_Body { self.inner }
}
