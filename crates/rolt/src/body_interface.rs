use std::marker::PhantomData;

use joltc_sys::*;

use crate::{Body, BodyId, IntoJolt, IntoRolt, ObjectLayer, Quat, RMat4, RVec3, Vec3};

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

    /// # Safety
    /// `settings` must be initialized and valid, with a valid `Shape` pointer.
    pub unsafe fn create_body(&self, settings: &JPC_BodyCreationSettings) -> Option<Body<'physics_system>> {
        let raw = JPC_BodyInterface_CreateBody(self.raw, settings);
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
    /// # Safety
    /// `settings` must be initialized and valid, with a valid `Shape` pointer.
    pub unsafe fn create_and_add_body(&self, settings: &JPC_BodyCreationSettings, activation: JPC_Activation) -> BodyId {
        BodyId::new(JPC_BodyInterface_CreateAndAddBody(self.raw, settings, activation))
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

    /// # Safety
    /// `shape` must be a valid shape.
    pub unsafe fn set_shape(&self, body_id: BodyId, shape: *const JPC_Shape, update_mass_properties: bool, activation: JPC_Activation) {
        JPC_BodyInterface_SetShape(self.raw, body_id.raw(), shape, update_mass_properties, activation)
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
    /// # Safety
    /// `settings` must be initialized and valid with a valid `JPC_SoftBodySharedSettings` pointer.
    pub unsafe fn create_soft_body(&self, settings: &JPC_SoftBodyCreationSettings) -> Option<Body<'physics_system>> {
        let raw = JPC_BodyInterface_CreateSoftBody(self.raw, settings);
        if raw.is_null() { None } else { Some(Body::new(raw)) }
    }

    /// Create a soft body with a specific ID (does not add it).
    /// # Safety
    /// `settings` must be initialized and valid.
    pub unsafe fn create_soft_body_with_id(&self, body_id: BodyId, settings: &JPC_SoftBodyCreationSettings) -> Option<Body<'physics_system>> {
        let raw = JPC_BodyInterface_CreateSoftBodyWithID(self.raw, body_id.raw(), settings);
        if raw.is_null() { None } else { Some(Body::new(raw)) }
    }

    /// Create, add, and activate a soft body in one call.
    /// # Safety
    /// `settings` must be initialized and valid.
    pub unsafe fn create_and_add_soft_body(&self, settings: &JPC_SoftBodyCreationSettings, activation: JPC_Activation) -> BodyId {
        BodyId::new(JPC_BodyInterface_CreateAndAddSoftBody(self.raw, settings, activation))
    }

    pub fn raw(&self) -> *mut JPC_BodyInterface { self.raw }
}
