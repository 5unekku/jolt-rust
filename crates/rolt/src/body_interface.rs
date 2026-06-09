use std::marker::PhantomData;

use joltc_sys::*;

use crate::{Body, BodyId, FromJolt, IntoJolt, IntoRolt, ObjectLayer, Quat, RMat4, RefConst, RVec3, Vec3};

/// Settings used to create a physics body.
///
/// Owns the shape via [`RefConst`] so the raw pointer in the inner JPC struct is always valid,
/// making [`BodyInterface::create_body`] and [`BodyInterface::create_and_add_body`] safe to call.
///
/// See also: Jolt's [`BodyCreationSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_body_creation_settings.html) class.
pub struct BodyCreationSettings {
    pub(crate) raw: JPC_BodyCreationSettings,
    shape: Option<RefConst<JPC_Shape>>,
}

impl BodyCreationSettings {
    /// Convenience constructor: sets the shape, layer, and motion type; everything else is default.
    pub fn new(shape: &RefConst<JPC_Shape>, layer: ObjectLayer, motion_type: JPC_MotionType) -> Self {
        let mut s = Self::default();
        s.set_shape(shape);
        s.raw.ObjectLayer = layer.raw();
        s.raw.MotionType = motion_type;
        s
    }

    /// Set the shape, keeping it alive for the lifetime of these settings.
    pub fn set_shape(&mut self, shape: &RefConst<JPC_Shape>) {
        self.shape = Some(shape.clone());
        self.raw.Shape = shape.get();
    }

    /// Call `f` with the raw JPC settings pointer — for joltc-sys power users.
    pub fn with_raw<R>(&self, f: impl FnOnce(*const JPC_BodyCreationSettings) -> R) -> R {
        f(&self.raw)
    }

    pub fn position(&self) -> RVec3 { self.raw.Position.into_rolt() }
    pub fn set_position(&mut self, v: RVec3) { self.raw.Position = v.into_jolt(); }
    pub fn rotation(&self) -> Quat { Quat::from_jolt(self.raw.Rotation) }
    pub fn set_rotation(&mut self, v: Quat) { self.raw.Rotation = v.into_jolt(); }
    pub fn linear_velocity(&self) -> Vec3 { self.raw.LinearVelocity.into_rolt() }
    pub fn set_linear_velocity(&mut self, v: Vec3) { self.raw.LinearVelocity = v.into_jolt(); }
    pub fn angular_velocity(&self) -> Vec3 { self.raw.AngularVelocity.into_rolt() }
    pub fn set_angular_velocity(&mut self, v: Vec3) { self.raw.AngularVelocity = v.into_jolt(); }
    pub fn user_data(&self) -> u64 { self.raw.UserData }
    pub fn set_user_data(&mut self, v: u64) { self.raw.UserData = v; }
    pub fn object_layer(&self) -> ObjectLayer { ObjectLayer::new(self.raw.ObjectLayer) }
    pub fn set_object_layer(&mut self, v: ObjectLayer) { self.raw.ObjectLayer = v.raw(); }
    pub fn motion_type(&self) -> JPC_MotionType { self.raw.MotionType }
    pub fn set_motion_type(&mut self, v: JPC_MotionType) { self.raw.MotionType = v; }
    pub fn allowed_dofs(&self) -> JPC_AllowedDOFs { self.raw.AllowedDOFs }
    pub fn set_allowed_dofs(&mut self, v: JPC_AllowedDOFs) { self.raw.AllowedDOFs = v; }
    pub fn allow_dynamic_or_kinematic(&self) -> bool { self.raw.AllowDynamicOrKinematic }
    pub fn set_allow_dynamic_or_kinematic(&mut self, v: bool) { self.raw.AllowDynamicOrKinematic = v; }
    pub fn is_sensor(&self) -> bool { self.raw.IsSensor }
    pub fn set_is_sensor(&mut self, v: bool) { self.raw.IsSensor = v; }
    pub fn collide_kinematic_vs_non_dynamic(&self) -> bool { self.raw.CollideKinematicVsNonDynamic }
    pub fn set_collide_kinematic_vs_non_dynamic(&mut self, v: bool) { self.raw.CollideKinematicVsNonDynamic = v; }
    pub fn use_manifold_reduction(&self) -> bool { self.raw.UseManifoldReduction }
    pub fn set_use_manifold_reduction(&mut self, v: bool) { self.raw.UseManifoldReduction = v; }
    pub fn apply_gyroscopic_force(&self) -> bool { self.raw.ApplyGyroscopicForce }
    pub fn set_apply_gyroscopic_force(&mut self, v: bool) { self.raw.ApplyGyroscopicForce = v; }
    pub fn motion_quality(&self) -> JPC_MotionQuality { self.raw.MotionQuality }
    pub fn set_motion_quality(&mut self, v: JPC_MotionQuality) { self.raw.MotionQuality = v; }
    pub fn enhanced_internal_edge_removal(&self) -> bool { self.raw.EnhancedInternalEdgeRemoval }
    pub fn set_enhanced_internal_edge_removal(&mut self, v: bool) { self.raw.EnhancedInternalEdgeRemoval = v; }
    pub fn allow_sleeping(&self) -> bool { self.raw.AllowSleeping }
    pub fn set_allow_sleeping(&mut self, v: bool) { self.raw.AllowSleeping = v; }
    pub fn friction(&self) -> f32 { self.raw.Friction }
    pub fn set_friction(&mut self, v: f32) { self.raw.Friction = v; }
    pub fn restitution(&self) -> f32 { self.raw.Restitution }
    pub fn set_restitution(&mut self, v: f32) { self.raw.Restitution = v; }
    pub fn linear_damping(&self) -> f32 { self.raw.LinearDamping }
    pub fn set_linear_damping(&mut self, v: f32) { self.raw.LinearDamping = v; }
    pub fn angular_damping(&self) -> f32 { self.raw.AngularDamping }
    pub fn set_angular_damping(&mut self, v: f32) { self.raw.AngularDamping = v; }
    pub fn max_linear_velocity(&self) -> f32 { self.raw.MaxLinearVelocity }
    pub fn set_max_linear_velocity(&mut self, v: f32) { self.raw.MaxLinearVelocity = v; }
    pub fn max_angular_velocity(&self) -> f32 { self.raw.MaxAngularVelocity }
    pub fn set_max_angular_velocity(&mut self, v: f32) { self.raw.MaxAngularVelocity = v; }
    pub fn gravity_factor(&self) -> f32 { self.raw.GravityFactor }
    pub fn set_gravity_factor(&mut self, v: f32) { self.raw.GravityFactor = v; }
    pub fn override_mass_properties(&self) -> JPC_OverrideMassProperties { self.raw.OverrideMassProperties }
    pub fn set_override_mass_properties(&mut self, v: JPC_OverrideMassProperties) { self.raw.OverrideMassProperties = v; }
    pub fn inertia_multiplier(&self) -> f32 { self.raw.InertiaMultiplier }
    pub fn set_inertia_multiplier(&mut self, v: f32) { self.raw.InertiaMultiplier = v; }
}

impl Default for BodyCreationSettings {
    fn default() -> Self {
        let mut raw = unsafe { std::mem::zeroed::<JPC_BodyCreationSettings>() };
        unsafe { JPC_BodyCreationSettings_default(&mut raw) };
        Self { raw, shape: None }
    }
}

/// See also: Jolt's [`BodyInterface`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_body_interface.html) class.
pub struct BodyInterface<'physics_system> {
    raw: *mut JPC_BodyInterface,
    _phantom: PhantomData<&'physics_system ()>,
}

impl<'physics_system> BodyInterface<'physics_system> {
    pub(crate) fn new(raw: *mut JPC_BodyInterface) -> Self {
        Self { raw, _phantom: PhantomData }
    }

    // --- body lifecycle ---

    pub fn create_body(&self, settings: &BodyCreationSettings) -> Option<Body<'physics_system>> {
        let raw = unsafe { JPC_BodyInterface_CreateBody(self.raw, &settings.raw) };
        if raw.is_null() { None } else { Some(Body::new(raw)) }
    }

    pub fn add_body(&self, body_id: BodyId, activation: JPC_Activation) {
        unsafe { JPC_BodyInterface_AddBody(self.raw, body_id.raw(), activation) }
    }

    pub fn remove_body(&self, body_id: BodyId) {
        unsafe { JPC_BodyInterface_RemoveBody(self.raw, body_id.raw()) }
    }

    pub fn destroy_body(&self, body_id: BodyId) {
        unsafe { JPC_BodyInterface_DestroyBody(self.raw, body_id.raw()) }
    }

    /// Create, add, and return the body ID in one call.
    pub fn create_and_add_body(&self, settings: &BodyCreationSettings, activation: JPC_Activation) -> BodyId {
        BodyId::new(unsafe { JPC_BodyInterface_CreateAndAddBody(self.raw, &settings.raw, activation) })
    }

    pub fn is_added(&self, body_id: BodyId) -> bool {
        unsafe { JPC_BodyInterface_IsAdded(self.raw, body_id.raw()) }
    }

    pub fn is_active(&self, body_id: BodyId) -> bool {
        unsafe { JPC_BodyInterface_IsActive(self.raw, body_id.raw()) }
    }

    pub fn activate_body(&self, body_id: BodyId) {
        unsafe { JPC_BodyInterface_ActivateBody(self.raw, body_id.raw()) }
    }

    pub fn deactivate_body(&self, body_id: BodyId) {
        unsafe { JPC_BodyInterface_DeactivateBody(self.raw, body_id.raw()) }
    }

    // --- shape ---

    pub fn get_shape(&self, body_id: BodyId) -> *const JPC_Shape {
        unsafe { JPC_BodyInterface_GetShape(self.raw, body_id.raw()) }
    }

    pub fn set_shape(&self, body_id: BodyId, shape: &RefConst<JPC_Shape>, update_mass_properties: bool, activation: JPC_Activation) {
        unsafe { JPC_BodyInterface_SetShape(self.raw, body_id.raw(), shape.get(), update_mass_properties, activation) }
    }

    pub fn notify_shape_changed(&self, body_id: BodyId, old_com: Vec3, update_mass_properties: bool, activation: JPC_Activation) {
        unsafe {
            JPC_BodyInterface_NotifyShapeChanged(self.raw, body_id.raw(), old_com.into_jolt(), update_mass_properties, activation)
        }
    }

    // --- transform ---

    pub fn position(&self, body_id: BodyId) -> RVec3 {
        unsafe { JPC_BodyInterface_GetPosition(self.raw, body_id.raw()).into_rolt() }
    }

    pub fn set_position(&self, body_id: BodyId, position: RVec3, activation: JPC_Activation) {
        unsafe { JPC_BodyInterface_SetPosition(self.raw, body_id.raw(), position.into_jolt(), activation) }
    }

    pub fn rotation(&self, body_id: BodyId) -> Quat {
        unsafe { JPC_BodyInterface_GetRotation(self.raw, body_id.raw()).into_rolt() }
    }

    pub fn set_rotation(&self, body_id: BodyId, rotation: Quat, activation: JPC_Activation) {
        unsafe { JPC_BodyInterface_SetRotation(self.raw, body_id.raw(), rotation.into_jolt(), activation) }
    }

    pub fn position_and_rotation(&self, body_id: BodyId) -> (RVec3, Quat) {
        let mut pos = unsafe { std::mem::zeroed() };
        let mut rot = unsafe { std::mem::zeroed() };
        unsafe { JPC_BodyInterface_GetPositionAndRotation(self.raw, body_id.raw(), &mut pos, &mut rot) }
        (pos.into_rolt(), rot.into_rolt())
    }

    pub fn set_position_and_rotation(&self, body_id: BodyId, position: RVec3, rotation: Quat, activation: JPC_Activation) {
        unsafe {
            JPC_BodyInterface_SetPositionAndRotation(self.raw, body_id.raw(), position.into_jolt(), rotation.into_jolt(), activation)
        }
    }

    pub fn set_position_and_rotation_when_changed(&self, body_id: BodyId, position: RVec3, rotation: Quat, activation: JPC_Activation) {
        unsafe {
            JPC_BodyInterface_SetPositionAndRotationWhenChanged(self.raw, body_id.raw(), position.into_jolt(), rotation.into_jolt(), activation)
        }
    }

    pub fn center_of_mass_position(&self, body_id: BodyId) -> RVec3 {
        unsafe { JPC_BodyInterface_GetCenterOfMassPosition(self.raw, body_id.raw()).into_rolt() }
    }

    pub fn world_transform(&self, body_id: BodyId) -> RMat4 {
        unsafe { JPC_BodyInterface_GetWorldTransform(self.raw, body_id.raw()).into_rolt() }
    }

    pub fn center_of_mass_transform(&self, body_id: BodyId) -> RMat4 {
        unsafe { JPC_BodyInterface_GetCenterOfMassTransform(self.raw, body_id.raw()).into_rolt() }
    }

    pub fn move_kinematic(&self, body_id: BodyId, target_position: RVec3, target_rotation: Quat, delta_time: f32) {
        unsafe {
            JPC_BodyInterface_MoveKinematic(self.raw, body_id.raw(), target_position.into_jolt(), target_rotation.into_jolt(), delta_time)
        }
    }

    // --- velocity ---

    pub fn linear_velocity(&self, body_id: BodyId) -> Vec3 {
        unsafe { JPC_BodyInterface_GetLinearVelocity(self.raw, body_id.raw()).into_rolt() }
    }

    pub fn set_linear_velocity(&self, body_id: BodyId, velocity: Vec3) {
        unsafe { JPC_BodyInterface_SetLinearVelocity(self.raw, body_id.raw(), velocity.into_jolt()) }
    }

    pub fn angular_velocity(&self, body_id: BodyId) -> Vec3 {
        unsafe { JPC_BodyInterface_GetAngularVelocity(self.raw, body_id.raw()).into_rolt() }
    }

    pub fn set_angular_velocity(&self, body_id: BodyId, velocity: Vec3) {
        unsafe { JPC_BodyInterface_SetAngularVelocity(self.raw, body_id.raw(), velocity.into_jolt()) }
    }

    pub fn linear_and_angular_velocity(&self, body_id: BodyId) -> (Vec3, Vec3) {
        let mut linear = unsafe { std::mem::zeroed() };
        let mut angular = unsafe { std::mem::zeroed() };
        unsafe { JPC_BodyInterface_GetLinearAndAngularVelocity(self.raw, body_id.raw(), &mut linear, &mut angular) }
        (linear.into_rolt(), angular.into_rolt())
    }

    pub fn set_linear_and_angular_velocity(&self, body_id: BodyId, linear: Vec3, angular: Vec3) {
        unsafe {
            JPC_BodyInterface_SetLinearAndAngularVelocity(self.raw, body_id.raw(), linear.into_jolt(), angular.into_jolt())
        }
    }

    pub fn add_linear_velocity(&self, body_id: BodyId, delta: Vec3) {
        unsafe { JPC_BodyInterface_AddLinearVelocity(self.raw, body_id.raw(), delta.into_jolt()) }
    }

    pub fn point_velocity(&self, body_id: BodyId, point: RVec3) -> Vec3 {
        unsafe { JPC_BodyInterface_GetPointVelocity(self.raw, body_id.raw(), point.into_jolt()).into_rolt() }
    }

    // --- forces and impulses ---

    pub fn add_force(&self, body_id: BodyId, force: Vec3) {
        unsafe { JPC_BodyInterface_AddForce(self.raw, body_id.raw(), force.into_jolt()) }
    }

    pub fn add_force_at_point(&self, body_id: BodyId, force: Vec3, point: RVec3) {
        unsafe { JPC_BodyInterface_AddForceAtPoint(self.raw, body_id.raw(), force.into_jolt(), point.into_jolt()) }
    }

    pub fn add_torque(&self, body_id: BodyId, torque: Vec3) {
        unsafe { JPC_BodyInterface_AddTorque(self.raw, body_id.raw(), torque.into_jolt()) }
    }

    pub fn add_force_and_torque(&self, body_id: BodyId, force: Vec3, torque: Vec3) {
        unsafe { JPC_BodyInterface_AddForceAndTorque(self.raw, body_id.raw(), force.into_jolt(), torque.into_jolt()) }
    }

    pub fn add_impulse(&self, body_id: BodyId, impulse: Vec3) {
        unsafe { JPC_BodyInterface_AddImpulse(self.raw, body_id.raw(), impulse.into_jolt()) }
    }

    pub fn add_impulse_at_point(&self, body_id: BodyId, impulse: Vec3, point: RVec3) {
        unsafe { JPC_BodyInterface_AddImpulse3(self.raw, body_id.raw(), impulse.into_jolt(), point.into_jolt()) }
    }

    pub fn add_angular_impulse(&self, body_id: BodyId, impulse: Vec3) {
        unsafe { JPC_BodyInterface_AddAngularImpulse(self.raw, body_id.raw(), impulse.into_jolt()) }
    }

    // --- motion type and quality ---

    pub fn body_type(&self, body_id: BodyId) -> JPC_BodyType {
        unsafe { JPC_BodyInterface_GetBodyType(self.raw, body_id.raw()) }
    }

    pub fn motion_type(&self, body_id: BodyId) -> JPC_MotionType {
        unsafe { JPC_BodyInterface_GetMotionType(self.raw, body_id.raw()) }
    }

    pub fn set_motion_type(&self, body_id: BodyId, motion_type: JPC_MotionType, activation: JPC_Activation) {
        unsafe { JPC_BodyInterface_SetMotionType(self.raw, body_id.raw(), motion_type, activation) }
    }

    pub fn motion_quality(&self, body_id: BodyId) -> JPC_MotionQuality {
        unsafe { JPC_BodyInterface_GetMotionQuality(self.raw, body_id.raw()) }
    }

    pub fn set_motion_quality(&self, body_id: BodyId, quality: JPC_MotionQuality) {
        unsafe { JPC_BodyInterface_SetMotionQuality(self.raw, body_id.raw(), quality) }
    }

    // --- material properties ---

    pub fn friction(&self, body_id: BodyId) -> f32 {
        unsafe { JPC_BodyInterface_GetFriction(self.raw, body_id.raw()) }
    }

    pub fn set_friction(&self, body_id: BodyId, friction: f32) {
        unsafe { JPC_BodyInterface_SetFriction(self.raw, body_id.raw(), friction) }
    }

    pub fn restitution(&self, body_id: BodyId) -> f32 {
        unsafe { JPC_BodyInterface_GetRestitution(self.raw, body_id.raw()) }
    }

    pub fn set_restitution(&self, body_id: BodyId, restitution: f32) {
        unsafe { JPC_BodyInterface_SetRestitution(self.raw, body_id.raw(), restitution) }
    }

    pub fn gravity_factor(&self, body_id: BodyId) -> f32 {
        unsafe { JPC_BodyInterface_GetGravityFactor(self.raw, body_id.raw()) }
    }

    pub fn set_gravity_factor(&self, body_id: BodyId, factor: f32) {
        unsafe { JPC_BodyInterface_SetGravityFactor(self.raw, body_id.raw(), factor) }
    }

    // --- object layer ---

    pub fn object_layer(&self, body_id: BodyId) -> ObjectLayer {
        ObjectLayer::new(unsafe { JPC_BodyInterface_GetObjectLayer(self.raw, body_id.raw()) })
    }

    pub fn set_object_layer(&self, body_id: BodyId, layer: ObjectLayer) {
        unsafe { JPC_BodyInterface_SetObjectLayer(self.raw, body_id.raw(), layer.raw()) }
    }

    // --- user data ---

    pub fn user_data(&self, body_id: BodyId) -> u64 {
        unsafe { JPC_BodyInterface_GetUserData(self.raw, body_id.raw()) }
    }

    pub fn set_user_data(&self, body_id: BodyId, data: u64) {
        unsafe { JPC_BodyInterface_SetUserData(self.raw, body_id.raw(), data) }
    }

    // --- misc ---

    pub fn invalidate_contact_cache(&self, body_id: BodyId) {
        unsafe { JPC_BodyInterface_InvalidateContactCache(self.raw, body_id.raw()) }
    }

    // --- soft body ---

    /// Create a soft body (does not add it — call `add_body` separately).
    pub fn create_soft_body(&self, settings: &crate::SoftBodyCreationSettings) -> Option<Body<'physics_system>> {
        let raw = unsafe { JPC_BodyInterface_CreateSoftBody(self.raw, &settings.0) };
        if raw.is_null() { None } else { Some(Body::new(raw)) }
    }

    /// Create a soft body with a specific ID (does not add it).
    pub fn create_soft_body_with_id(&self, body_id: BodyId, settings: &crate::SoftBodyCreationSettings) -> Option<Body<'physics_system>> {
        let raw = unsafe { JPC_BodyInterface_CreateSoftBodyWithID(self.raw, body_id.raw(), &settings.0) };
        if raw.is_null() { None } else { Some(Body::new(raw)) }
    }

    /// Create, add, and activate a soft body in one call.
    pub fn create_and_add_soft_body(&self, settings: &crate::SoftBodyCreationSettings, activation: JPC_Activation) -> BodyId {
        BodyId::new(unsafe { JPC_BodyInterface_CreateAndAddSoftBody(self.raw, &settings.0, activation) })
    }

    pub fn raw(&self) -> *mut JPC_BodyInterface { self.raw }
}
