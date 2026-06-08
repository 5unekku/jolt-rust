use joltc_sys::*;

use crate::{conversions::IntoRolt, FromJolt, IntoJolt, ObjectLayer, Quat, RefConst, RVec3, Vec3};

/// See also: Jolt's [`SpringSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/struct_spring_settings.html) struct.
#[repr(transparent)]
pub struct SpringSettings(pub JPC_SpringSettings);

impl SpringSettings {
    pub fn mode(&self) -> JPC_SpringMode { self.0.Mode }
    pub fn set_mode(&mut self, mode: JPC_SpringMode) { self.0.Mode = mode; }
    pub fn frequency_or_stiffness(&self) -> f32 { self.0.FrequencyOrStiffness }
    pub fn set_frequency_or_stiffness(&mut self, value: f32) { self.0.FrequencyOrStiffness = value; }
    pub fn damping(&self) -> f32 { self.0.Damping }
    pub fn set_damping(&mut self, value: f32) { self.0.Damping = value; }
}

impl Default for SpringSettings {
    fn default() -> Self {
        let mut raw = unsafe { std::mem::zeroed::<JPC_SpringSettings>() };
        unsafe { JPC_SpringSettings_default(&mut raw) };
        Self(raw)
    }
}

/// See also: Jolt's [`MotorSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/struct_motor_settings.html) struct.
#[repr(transparent)]
pub struct MotorSettings(pub JPC_MotorSettings);

impl MotorSettings {
    pub fn spring_settings(&self) -> SpringSettings { SpringSettings(self.0.SpringSettings) }
    pub fn set_spring_settings(&mut self, settings: SpringSettings) { self.0.SpringSettings = settings.0; }
    pub fn min_force_limit(&self) -> f32 { self.0.MinForceLimit }
    pub fn set_min_force_limit(&mut self, v: f32) { self.0.MinForceLimit = v; }
    pub fn max_force_limit(&self) -> f32 { self.0.MaxForceLimit }
    pub fn set_max_force_limit(&mut self, v: f32) { self.0.MaxForceLimit = v; }
    pub fn min_torque_limit(&self) -> f32 { self.0.MinTorqueLimit }
    pub fn set_min_torque_limit(&mut self, v: f32) { self.0.MinTorqueLimit = v; }
    pub fn max_torque_limit(&self) -> f32 { self.0.MaxTorqueLimit }
    pub fn set_max_torque_limit(&mut self, v: f32) { self.0.MaxTorqueLimit = v; }
}

impl Default for MotorSettings {
    fn default() -> Self {
        let mut raw = unsafe { std::mem::zeroed::<JPC_MotorSettings>() };
        unsafe { JPC_MotorSettings_default(&mut raw) };
        Self(raw)
    }
}

/// Determines which pairs of bodies can collide via a group filter.
/// A null `group_filter` means no filtering (all bodies in the group can collide).
///
/// See also: Jolt's [`CollisionGroup`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_collision_group.html) class.
#[repr(transparent)]
pub struct CollisionGroup(pub JPC_CollisionGroup);

impl CollisionGroup {
    pub fn group_id(&self) -> u32 { self.0.GroupID }
    pub fn set_group_id(&mut self, id: u32) { self.0.GroupID = id; }
    pub fn sub_group_id(&self) -> u32 { self.0.SubGroupID }
    pub fn set_sub_group_id(&mut self, id: u32) { self.0.SubGroupID = id; }
    /// Raw pointer to the group filter vtable object.  Must outlive self if non-null.
    pub fn group_filter(&self) -> *const JPC_GroupFilter { self.0.GroupFilter }
    pub fn set_group_filter(&mut self, filter: *const JPC_GroupFilter) { self.0.GroupFilter = filter; }
}

impl Default for CollisionGroup {
    fn default() -> Self {
        // zero-init: null filter = no filtering
        Self(unsafe { std::mem::zeroed() })
    }
}

/// Owns the shared soft body settings pointer and all creation parameters.
///
/// See also: Jolt's [`SoftBodyCreationSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_soft_body_creation_settings.html) class.
#[repr(transparent)]
pub struct SoftBodyCreationSettings(pub JPC_SoftBodyCreationSettings);

impl SoftBodyCreationSettings {
    pub fn position(&self) -> RVec3 { self.0.Position.into_rolt() }
    pub fn set_position(&mut self, v: RVec3) { self.0.Position = v.into_jolt(); }
    pub fn rotation(&self) -> Quat { Quat::from_jolt(self.0.Rotation) }
    pub fn set_rotation(&mut self, v: Quat) { self.0.Rotation = v.into_jolt(); }
    pub fn user_data(&self) -> u64 { self.0.UserData }
    pub fn set_user_data(&mut self, v: u64) { self.0.UserData = v; }
    pub fn object_layer(&self) -> ObjectLayer { ObjectLayer::new(self.0.ObjectLayer) }
    pub fn set_object_layer(&mut self, v: ObjectLayer) { self.0.ObjectLayer = v.raw(); }
    pub fn num_iterations(&self) -> u32 { self.0.NumIterations }
    pub fn set_num_iterations(&mut self, v: u32) { self.0.NumIterations = v; }
    pub fn linear_damping(&self) -> f32 { self.0.LinearDamping }
    pub fn set_linear_damping(&mut self, v: f32) { self.0.LinearDamping = v; }
    pub fn max_linear_velocity(&self) -> f32 { self.0.MaxLinearVelocity }
    pub fn set_max_linear_velocity(&mut self, v: f32) { self.0.MaxLinearVelocity = v; }
    pub fn restitution(&self) -> f32 { self.0.Restitution }
    pub fn set_restitution(&mut self, v: f32) { self.0.Restitution = v; }
    pub fn friction(&self) -> f32 { self.0.Friction }
    pub fn set_friction(&mut self, v: f32) { self.0.Friction = v; }
    pub fn pressure(&self) -> f32 { self.0.Pressure }
    pub fn set_pressure(&mut self, v: f32) { self.0.Pressure = v; }
    pub fn gravity_factor(&self) -> f32 { self.0.GravityFactor }
    pub fn set_gravity_factor(&mut self, v: f32) { self.0.GravityFactor = v; }
    pub fn vertex_radius(&self) -> f32 { self.0.VertexRadius }
    pub fn set_vertex_radius(&mut self, v: f32) { self.0.VertexRadius = v; }
    pub fn update_position(&self) -> bool { self.0.UpdatePosition }
    pub fn set_update_position(&mut self, v: bool) { self.0.UpdatePosition = v; }
    pub fn make_rotation_identity(&self) -> bool { self.0.MakeRotationIdentity }
    pub fn set_make_rotation_identity(&mut self, v: bool) { self.0.MakeRotationIdentity = v; }
    pub fn allow_sleeping(&self) -> bool { self.0.AllowSleeping }
    pub fn set_allow_sleeping(&mut self, v: bool) { self.0.AllowSleeping = v; }
    pub fn faces_double_sided(&self) -> bool { self.0.FacesDoubleSided }
    pub fn set_faces_double_sided(&mut self, v: bool) { self.0.FacesDoubleSided = v; }
    /// Set the shared settings (raw pointer — must outlive this struct).
    pub fn set_shared_settings(&mut self, shared: *const JPC_SoftBodySharedSettings) { self.0.Settings = shared; }
}

impl Default for SoftBodyCreationSettings {
    fn default() -> Self { Self(JPC_SoftBodyCreationSettings::default()) }
}

// --- query settings ---

/// Settings for shape-cast queries.
///
/// See also: Jolt's [`ShapeCastSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/struct_shape_cast_settings.html) struct.
#[repr(transparent)]
pub struct ShapeCastSettings(pub JPC_ShapeCastSettings);

impl ShapeCastSettings {
    pub fn active_edge_mode(&self) -> JPC_ActiveEdgeMode { self.0.ActiveEdgeMode }
    pub fn set_active_edge_mode(&mut self, v: JPC_ActiveEdgeMode) { self.0.ActiveEdgeMode = v; }
    pub fn collect_faces_mode(&self) -> JPC_CollectFacesMode { self.0.CollectFacesMode }
    pub fn set_collect_faces_mode(&mut self, v: JPC_CollectFacesMode) { self.0.CollectFacesMode = v; }
    pub fn collision_tolerance(&self) -> f32 { self.0.CollisionTolerance }
    pub fn set_collision_tolerance(&mut self, v: f32) { self.0.CollisionTolerance = v; }
    pub fn penetration_tolerance(&self) -> f32 { self.0.PenetrationTolerance }
    pub fn set_penetration_tolerance(&mut self, v: f32) { self.0.PenetrationTolerance = v; }
    pub fn active_edge_movement_direction(&self) -> Vec3 { Vec3::from_jolt(self.0.ActiveEdgeMovementDirection) }
    pub fn set_active_edge_movement_direction(&mut self, v: Vec3) { self.0.ActiveEdgeMovementDirection = v.into_jolt(); }
    pub fn back_face_mode_triangles(&self) -> JPC_BackFaceMode { self.0.BackFaceModeTriangles }
    pub fn set_back_face_mode_triangles(&mut self, v: JPC_BackFaceMode) { self.0.BackFaceModeTriangles = v; }
    pub fn back_face_mode_convex(&self) -> JPC_BackFaceMode { self.0.BackFaceModeConvex }
    pub fn set_back_face_mode_convex(&mut self, v: JPC_BackFaceMode) { self.0.BackFaceModeConvex = v; }
    pub fn use_shrunken_shape_and_convex_radius(&self) -> bool { self.0.UseShrunkenShapeAndConvexRadius }
    pub fn set_use_shrunken_shape_and_convex_radius(&mut self, v: bool) { self.0.UseShrunkenShapeAndConvexRadius = v; }
    pub fn return_deepest_point(&self) -> bool { self.0.ReturnDeepestPoint }
    pub fn set_return_deepest_point(&mut self, v: bool) { self.0.ReturnDeepestPoint = v; }
}

impl Default for ShapeCastSettings {
    fn default() -> Self { Self(JPC_ShapeCastSettings::default()) }
}

/// Settings for collide-shape queries.
///
/// See also: Jolt's [`CollideShapeSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/struct_collide_shape_settings.html) struct.
#[repr(transparent)]
pub struct CollideShapeSettings(pub JPC_CollideShapeSettings);

impl CollideShapeSettings {
    pub fn active_edge_mode(&self) -> JPC_ActiveEdgeMode { self.0.ActiveEdgeMode }
    pub fn set_active_edge_mode(&mut self, v: JPC_ActiveEdgeMode) { self.0.ActiveEdgeMode = v; }
    pub fn collect_faces_mode(&self) -> JPC_CollectFacesMode { self.0.CollectFacesMode }
    pub fn set_collect_faces_mode(&mut self, v: JPC_CollectFacesMode) { self.0.CollectFacesMode = v; }
    pub fn collision_tolerance(&self) -> f32 { self.0.CollisionTolerance }
    pub fn set_collision_tolerance(&mut self, v: f32) { self.0.CollisionTolerance = v; }
    pub fn penetration_tolerance(&self) -> f32 { self.0.PenetrationTolerance }
    pub fn set_penetration_tolerance(&mut self, v: f32) { self.0.PenetrationTolerance = v; }
    pub fn active_edge_movement_direction(&self) -> Vec3 { Vec3::from_jolt(self.0.ActiveEdgeMovementDirection) }
    pub fn set_active_edge_movement_direction(&mut self, v: Vec3) { self.0.ActiveEdgeMovementDirection = v.into_jolt(); }
    pub fn max_separation_distance(&self) -> f32 { self.0.MaxSeparationDistance }
    pub fn set_max_separation_distance(&mut self, v: f32) { self.0.MaxSeparationDistance = v; }
    pub fn back_face_mode(&self) -> JPC_BackFaceMode { self.0.BackFaceMode }
    pub fn set_back_face_mode(&mut self, v: JPC_BackFaceMode) { self.0.BackFaceMode = v; }
}

impl Default for CollideShapeSettings {
    fn default() -> Self { Self(JPC_CollideShapeSettings::default()) }
}

// --- physics material ---

/// Returns the default physics material (singleton, refcounted).
///
/// See also: Jolt's [`PhysicsMaterial::sDefault`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_physics_material.html).
pub fn default_physics_material() -> RefConst<JPC_PhysicsMaterial> {
    unsafe { RefConst::from_active(JPC_PhysicsMaterial_GetDefault()) }
}
