use joltc_sys::*;

use crate::{
    BodyFilterImpl, BodyId, BroadPhaseLayerFilterImpl, CharacterContactListenerImpl, FromJolt,
    IntoJolt, IntoRolt, ObjectLayer, ObjectLayerFilterImpl, PhysicsSystem, Quat, RVec3, RefConst,
    ShapeFilterImpl, TempAllocator, Vec3,
};

/// See also: Jolt's [`Character`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_character.html) class.
pub struct Character {
    raw: *mut JPC_Character,
}

impl Character {
    pub fn new(
        settings: &CharacterSettings,
        position: RVec3,
        rotation: Quat,
        user_data: u64,
        physics_system: &PhysicsSystem,
    ) -> Self {
        let raw = unsafe {
            JPC_Character_new(
                &settings.raw,
                position.into_jolt(),
                rotation.into_jolt(),
                user_data,
                physics_system.raw(),
            )
        };
        Self { raw }
    }

    pub fn add_to_physics_system(&mut self, activation: JPC_Activation, lock_bodies: bool) {
        unsafe { JPC_Character_AddToPhysicsSystem(self.raw, activation, lock_bodies) }
    }

    pub fn remove_from_physics_system(&mut self, lock_bodies: bool) {
        unsafe { JPC_Character_RemoveFromPhysicsSystem(self.raw, lock_bodies) }
    }

    pub fn activate(&mut self, lock_bodies: bool) {
        unsafe { JPC_Character_Activate(self.raw, lock_bodies) }
    }

    pub fn post_simulation(&mut self, max_separation_distance: f32, lock_bodies: bool) {
        unsafe { JPC_Character_PostSimulation(self.raw, max_separation_distance, lock_bodies) }
    }

    pub fn body_id(&self) -> BodyId {
        BodyId::new(unsafe { JPC_Character_GetBodyID(self.raw) })
    }

    pub fn position(&self, lock_bodies: bool) -> RVec3 {
        unsafe { JPC_Character_GetPosition(self.raw, lock_bodies).into_rolt() }
    }

    pub fn set_position(&mut self, position: RVec3, activation: JPC_Activation, lock_bodies: bool) {
        unsafe {
            JPC_Character_SetPosition(self.raw, position.into_jolt(), activation, lock_bodies)
        }
    }

    pub fn rotation(&self, lock_bodies: bool) -> Quat {
        unsafe { JPC_Character_GetRotation(self.raw, lock_bodies).into_rolt() }
    }

    pub fn set_rotation(&mut self, rotation: Quat, activation: JPC_Activation, lock_bodies: bool) {
        unsafe {
            JPC_Character_SetRotation(self.raw, rotation.into_jolt(), activation, lock_bodies)
        }
    }

    pub fn center_of_mass_position(&self, lock_bodies: bool) -> RVec3 {
        unsafe { JPC_Character_GetCenterOfMassPosition(self.raw, lock_bodies).into_rolt() }
    }

    pub fn linear_velocity(&self, lock_bodies: bool) -> Vec3 {
        unsafe { JPC_Character_GetLinearVelocity(self.raw, lock_bodies).into_rolt() }
    }

    pub fn set_linear_velocity(&mut self, velocity: Vec3, lock_bodies: bool) {
        unsafe { JPC_Character_SetLinearVelocity(self.raw, velocity.into_jolt(), lock_bodies) }
    }

    pub fn add_linear_velocity(&mut self, velocity: Vec3, lock_bodies: bool) {
        unsafe { JPC_Character_AddLinearVelocity(self.raw, velocity.into_jolt(), lock_bodies) }
    }

    pub fn ground_state(&self) -> JPC_GroundState {
        unsafe { JPC_Character_GetGroundState(self.raw) }
    }

    pub fn is_supported(&self) -> bool {
        unsafe { JPC_Character_IsSupported(self.raw) }
    }
    pub fn ground_position(&self) -> RVec3 {
        unsafe { JPC_Character_GetGroundPosition(self.raw).into_rolt() }
    }
    pub fn ground_normal(&self) -> Vec3 {
        unsafe { JPC_Character_GetGroundNormal(self.raw).into_rolt() }
    }
    pub fn ground_velocity(&self) -> Vec3 {
        unsafe { JPC_Character_GetGroundVelocity(self.raw).into_rolt() }
    }
    pub fn ground_body_id(&self) -> BodyId {
        BodyId::new(unsafe { JPC_Character_GetGroundBodyID(self.raw) })
    }
    pub fn ground_user_data(&self) -> u64 {
        unsafe { JPC_Character_GetGroundUserData(self.raw) }
    }

    pub fn layer(&self) -> ObjectLayer {
        ObjectLayer::new(unsafe { JPC_Character_GetLayer(self.raw) })
    }

    pub fn set_layer(&mut self, layer: ObjectLayer, lock_bodies: bool) {
        unsafe { JPC_Character_SetLayer(self.raw, layer.raw(), lock_bodies) }
    }

    pub fn up(&self) -> Vec3 {
        unsafe { JPC_Character_GetUp(self.raw).into_rolt() }
    }
    pub fn set_up(&mut self, up: Vec3) {
        unsafe { JPC_Character_SetUp(self.raw, up.into_jolt()) }
    }

    pub fn set_shape(
        &mut self,
        shape: &RefConst<JPC_Shape>,
        max_penetration_depth: f32,
        lock_bodies: bool,
    ) -> bool {
        unsafe { JPC_Character_SetShape(self.raw, shape.get(), max_penetration_depth, lock_bodies) }
    }

    pub fn add_impulse(&mut self, impulse: Vec3) {
        unsafe { JPC_Character_AddImpulse(self.raw, impulse.into_jolt()) }
    }

    pub fn position_and_rotation(&self, lock_bodies: bool) -> (RVec3, Quat) {
        let mut position = unsafe { std::mem::zeroed::<JPC_RVec3>() };
        let mut rotation = unsafe { std::mem::zeroed::<JPC_Quat>() };
        unsafe {
            JPC_Character_GetPositionAndRotation(
                self.raw,
                &mut position,
                &mut rotation,
                lock_bodies,
            )
        }
        (position.into_rolt(), rotation.into_rolt())
    }

    pub fn set_linear_and_angular_velocity(
        &mut self,
        linear: Vec3,
        angular: Vec3,
        lock_bodies: bool,
    ) {
        unsafe {
            JPC_Character_SetLinearAndAngularVelocity(
                self.raw,
                linear.into_jolt(),
                angular.into_jolt(),
                lock_bodies,
            )
        }
    }

    pub fn world_transform(&self, lock_bodies: bool) -> JPC_RMat44 {
        unsafe { JPC_Character_GetWorldTransform(self.raw, lock_bodies) }
    }

    pub fn with_raw<R>(&self, f: impl FnOnce(*mut JPC_Character) -> R) -> R {
        f(self.raw)
    }
    pub fn raw(&self) -> *mut JPC_Character {
        self.raw
    }
}

impl Drop for Character {
    fn drop(&mut self) {
        unsafe { JPC_Character_delete(self.raw) }
    }
}

/// Settings for creating a [`Character`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_character.html).
///
/// Owns the shape so the pointer inside the JPC struct is always valid.
#[derive(Default)]
pub struct CharacterSettings {
    pub(crate) raw: JPC_CharacterSettings,
    _shape: Option<RefConst<JPC_Shape>>,
}

impl CharacterSettings {
    pub fn new(shape: &RefConst<JPC_Shape>, layer: ObjectLayer) -> Self {
        let mut this = Self {
            raw: Default::default(),
            _shape: Some(shape.clone()),
        };
        this.raw.Shape = shape.get();
        this.raw.Layer = layer.raw();
        this
    }
    pub fn up(&self) -> Vec3 {
        Vec3::from_jolt(self.raw.Up)
    }
    pub fn set_up(&mut self, v: Vec3) {
        self.raw.Up = v.into_jolt();
    }
    pub fn max_slope_angle(&self) -> f32 {
        self.raw.MaxSlopeAngle
    }
    pub fn set_max_slope_angle(&mut self, v: f32) {
        self.raw.MaxSlopeAngle = v;
    }
    pub fn enhanced_internal_edge_removal(&self) -> bool {
        self.raw.EnhancedInternalEdgeRemoval
    }
    pub fn set_enhanced_internal_edge_removal(&mut self, v: bool) {
        self.raw.EnhancedInternalEdgeRemoval = v;
    }
    pub fn layer(&self) -> ObjectLayer {
        ObjectLayer::new(self.raw.Layer)
    }
    pub fn set_layer(&mut self, v: ObjectLayer) {
        self.raw.Layer = v.raw();
    }
    pub fn mass(&self) -> f32 {
        self.raw.Mass
    }
    pub fn set_mass(&mut self, v: f32) {
        self.raw.Mass = v;
    }
    pub fn friction(&self) -> f32 {
        self.raw.Friction
    }
    pub fn set_friction(&mut self, v: f32) {
        self.raw.Friction = v;
    }
    pub fn gravity_factor(&self) -> f32 {
        self.raw.GravityFactor
    }
    pub fn set_gravity_factor(&mut self, v: f32) {
        self.raw.GravityFactor = v;
    }
    pub fn allowed_dofs(&self) -> JPC_AllowedDOFs {
        self.raw.AllowedDOFs
    }
    pub fn set_allowed_dofs(&mut self, v: JPC_AllowedDOFs) {
        self.raw.AllowedDOFs = v;
    }
    pub fn set_shape(&mut self, shape: &RefConst<JPC_Shape>) {
        self._shape = Some(shape.clone());
        self.raw.Shape = shape.get();
    }
}

/// Settings for creating a [`CharacterVirtual`].
///
/// Owns both the primary shape and the optional inner body shape.
#[derive(Default)]
pub struct CharacterVirtualSettings {
    pub(crate) raw: JPC_CharacterVirtualSettings,
    _shape: Option<RefConst<JPC_Shape>>,
    _inner_body_shape: Option<RefConst<JPC_Shape>>,
}

impl CharacterVirtualSettings {
    pub fn new(shape: &RefConst<JPC_Shape>) -> Self {
        let mut this = Self {
            raw: Default::default(),
            _shape: Some(shape.clone()),
            _inner_body_shape: None,
        };
        this.raw.Shape = shape.get();
        this
    }
    pub fn up(&self) -> Vec3 {
        Vec3::from_jolt(self.raw.Up)
    }
    pub fn set_up(&mut self, v: Vec3) {
        self.raw.Up = v.into_jolt();
    }
    pub fn max_slope_angle(&self) -> f32 {
        self.raw.MaxSlopeAngle
    }
    pub fn set_max_slope_angle(&mut self, v: f32) {
        self.raw.MaxSlopeAngle = v;
    }
    pub fn enhanced_internal_edge_removal(&self) -> bool {
        self.raw.EnhancedInternalEdgeRemoval
    }
    pub fn set_enhanced_internal_edge_removal(&mut self, v: bool) {
        self.raw.EnhancedInternalEdgeRemoval = v;
    }
    pub fn mass(&self) -> f32 {
        self.raw.Mass
    }
    pub fn set_mass(&mut self, v: f32) {
        self.raw.Mass = v;
    }
    pub fn max_strength(&self) -> f32 {
        self.raw.MaxStrength
    }
    pub fn set_max_strength(&mut self, v: f32) {
        self.raw.MaxStrength = v;
    }
    pub fn shape_offset(&self) -> Vec3 {
        Vec3::from_jolt(self.raw.ShapeOffset)
    }
    pub fn set_shape_offset(&mut self, v: Vec3) {
        self.raw.ShapeOffset = v.into_jolt();
    }
    pub fn back_face_mode(&self) -> JPC_BackFaceMode {
        self.raw.BackFaceMode
    }
    pub fn set_back_face_mode(&mut self, v: JPC_BackFaceMode) {
        self.raw.BackFaceMode = v;
    }
    pub fn predictive_contact_distance(&self) -> f32 {
        self.raw.PredictiveContactDistance
    }
    pub fn set_predictive_contact_distance(&mut self, v: f32) {
        self.raw.PredictiveContactDistance = v;
    }
    pub fn max_collision_iterations(&self) -> u32 {
        self.raw.MaxCollisionIterations
    }
    pub fn set_max_collision_iterations(&mut self, v: u32) {
        self.raw.MaxCollisionIterations = v;
    }
    pub fn max_constraint_iterations(&self) -> u32 {
        self.raw.MaxConstraintIterations
    }
    pub fn set_max_constraint_iterations(&mut self, v: u32) {
        self.raw.MaxConstraintIterations = v;
    }
    pub fn min_time_remaining(&self) -> f32 {
        self.raw.MinTimeRemaining
    }
    pub fn set_min_time_remaining(&mut self, v: f32) {
        self.raw.MinTimeRemaining = v;
    }
    pub fn collision_tolerance(&self) -> f32 {
        self.raw.CollisionTolerance
    }
    pub fn set_collision_tolerance(&mut self, v: f32) {
        self.raw.CollisionTolerance = v;
    }
    pub fn character_padding(&self) -> f32 {
        self.raw.CharacterPadding
    }
    pub fn set_character_padding(&mut self, v: f32) {
        self.raw.CharacterPadding = v;
    }
    pub fn max_num_hits(&self) -> u32 {
        self.raw.MaxNumHits
    }
    pub fn set_max_num_hits(&mut self, v: u32) {
        self.raw.MaxNumHits = v;
    }
    pub fn hit_reduction_cos_max_angle(&self) -> f32 {
        self.raw.HitReductionCosMaxAngle
    }
    pub fn set_hit_reduction_cos_max_angle(&mut self, v: f32) {
        self.raw.HitReductionCosMaxAngle = v;
    }
    pub fn penetration_recovery_speed(&self) -> f32 {
        self.raw.PenetrationRecoverySpeed
    }
    pub fn set_penetration_recovery_speed(&mut self, v: f32) {
        self.raw.PenetrationRecoverySpeed = v;
    }
    pub fn inner_body_layer(&self) -> ObjectLayer {
        ObjectLayer::new(self.raw.InnerBodyLayer)
    }
    pub fn set_inner_body_layer(&mut self, v: ObjectLayer) {
        self.raw.InnerBodyLayer = v.raw();
    }
    pub fn set_shape(&mut self, shape: &RefConst<JPC_Shape>) {
        self._shape = Some(shape.clone());
        self.raw.Shape = shape.get();
    }
    pub fn set_inner_body_shape(&mut self, shape: &RefConst<JPC_Shape>) {
        self._inner_body_shape = Some(shape.clone());
        self.raw.InnerBodyShape = shape.get();
    }
}

/// Settings for [`CharacterVirtual::extended_update`].
///
/// See also: Jolt's [`ExtendedUpdateSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/struct_character_virtual_1_1_extended_update_settings.html) struct.
pub struct ExtendedUpdateSettings(pub JPC_ExtendedUpdateSettings);

impl ExtendedUpdateSettings {
    pub fn step_down(&self) -> Vec3 {
        Vec3::from_jolt(self.0.StickToFloorStepDown)
    }
    pub fn set_step_down(&mut self, v: Vec3) {
        self.0.StickToFloorStepDown = v.into_jolt();
    }

    pub fn step_up(&self) -> Vec3 {
        Vec3::from_jolt(self.0.WalkStairsStepUp)
    }
    pub fn set_step_up(&mut self, v: Vec3) {
        self.0.WalkStairsStepUp = v.into_jolt();
    }

    pub fn min_step_forward(&self) -> f32 {
        self.0.WalkStairsMinStepForward
    }
    pub fn set_min_step_forward(&mut self, v: f32) {
        self.0.WalkStairsMinStepForward = v;
    }

    pub fn step_forward_test(&self) -> f32 {
        self.0.WalkStairsStepForwardTest
    }
    pub fn set_step_forward_test(&mut self, v: f32) {
        self.0.WalkStairsStepForwardTest = v;
    }

    pub fn cos_angle_forward_contact(&self) -> f32 {
        self.0.WalkStairsCosAngleForwardContact
    }
    pub fn set_cos_angle_forward_contact(&mut self, v: f32) {
        self.0.WalkStairsCosAngleForwardContact = v;
    }

    pub fn step_down_extra(&self) -> Vec3 {
        Vec3::from_jolt(self.0.WalkStairsStepDownExtra)
    }
    pub fn set_step_down_extra(&mut self, v: Vec3) {
        self.0.WalkStairsStepDownExtra = v.into_jolt();
    }
}

impl Default for ExtendedUpdateSettings {
    fn default() -> Self {
        let mut raw = unsafe { std::mem::zeroed::<JPC_ExtendedUpdateSettings>() };
        unsafe { JPC_ExtendedUpdateSettings_default(&mut raw) };
        Self(raw)
    }
}

/// Arguments for [`CharacterVirtual::update`].
pub struct CharacterUpdateArgs<'a> {
    pub delta_time: f32,
    pub gravity: Vec3,
    pub broad_phase_layer_filter: Option<BroadPhaseLayerFilterImpl<'a>>,
    pub object_layer_filter: Option<ObjectLayerFilterImpl<'a>>,
    pub body_filter: Option<BodyFilterImpl<'a>>,
    pub shape_filter: Option<ShapeFilterImpl<'a>>,
    pub temp_allocator: &'a TempAllocator,
}

/// Arguments for [`CharacterVirtual::walk_stairs`].
pub struct CharacterWalkStairsArgs<'a> {
    pub delta_time: f32,
    pub step_up: Vec3,
    pub step_forward: Vec3,
    pub step_forward_test: Vec3,
    pub step_down_extra: Vec3,
    pub broad_phase_layer_filter: Option<BroadPhaseLayerFilterImpl<'a>>,
    pub object_layer_filter: Option<ObjectLayerFilterImpl<'a>>,
    pub body_filter: Option<BodyFilterImpl<'a>>,
    pub shape_filter: Option<ShapeFilterImpl<'a>>,
    pub temp_allocator: &'a TempAllocator,
}

/// Arguments for [`CharacterVirtual::stick_to_floor`].
pub struct CharacterStickToFloorArgs<'a> {
    pub step_down: Vec3,
    pub broad_phase_layer_filter: Option<BroadPhaseLayerFilterImpl<'a>>,
    pub object_layer_filter: Option<ObjectLayerFilterImpl<'a>>,
    pub body_filter: Option<BodyFilterImpl<'a>>,
    pub shape_filter: Option<ShapeFilterImpl<'a>>,
    pub temp_allocator: &'a TempAllocator,
}

/// Arguments for [`CharacterVirtual::refresh_contacts`].
pub struct CharacterRefreshContactsArgs<'a> {
    pub broad_phase_layer_filter: Option<BroadPhaseLayerFilterImpl<'a>>,
    pub object_layer_filter: Option<ObjectLayerFilterImpl<'a>>,
    pub body_filter: Option<BodyFilterImpl<'a>>,
    pub shape_filter: Option<ShapeFilterImpl<'a>>,
    pub temp_allocator: &'a TempAllocator,
}

/// Arguments for [`CharacterVirtual::set_shape`].
pub struct CharacterSetShapeArgs<'a> {
    pub shape: &'a RefConst<JPC_Shape>,
    pub max_penetration_depth: f32,
    pub broad_phase_layer_filter: Option<BroadPhaseLayerFilterImpl<'a>>,
    pub object_layer_filter: Option<ObjectLayerFilterImpl<'a>>,
    pub body_filter: Option<BodyFilterImpl<'a>>,
    pub shape_filter: Option<ShapeFilterImpl<'a>>,
    pub temp_allocator: &'a TempAllocator,
}

/// Arguments for [`CharacterVirtual::extended_update`].
pub struct CharacterExtendedUpdateArgs<'a> {
    pub delta_time: f32,
    pub gravity: Vec3,
    pub settings: ExtendedUpdateSettings,
    pub broad_phase_layer_filter: Option<BroadPhaseLayerFilterImpl<'a>>,
    pub object_layer_filter: Option<ObjectLayerFilterImpl<'a>>,
    pub body_filter: Option<BodyFilterImpl<'a>>,
    pub shape_filter: Option<ShapeFilterImpl<'a>>,
    pub temp_allocator: &'a TempAllocator,
}

/// See also: Jolt's [`CharacterVirtual`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_character_virtual.html) class.
pub struct CharacterVirtual {
    raw: *mut JPC_CharacterVirtual,
    contact_listener: Option<CharacterContactListenerImpl<'static>>,
}

impl CharacterVirtual {
    pub fn new(
        settings: &CharacterVirtualSettings,
        position: RVec3,
        rotation: Quat,
        user_data: u64,
        physics_system: &PhysicsSystem,
    ) -> Self {
        let raw = unsafe {
            JPC_CharacterVirtual_new(
                &settings.raw,
                position.into_jolt(),
                rotation.into_jolt(),
                user_data,
                physics_system.raw(),
            )
        };
        Self {
            raw,
            contact_listener: None,
        }
    }

    // --- transform ---

    pub fn position(&self) -> RVec3 {
        unsafe { JPC_CharacterVirtual_GetPosition(self.raw).into_rolt() }
    }

    pub fn set_position(&mut self, position: RVec3) {
        unsafe { JPC_CharacterVirtual_SetPosition(self.raw, position.into_jolt()) }
    }

    pub fn rotation(&self) -> Quat {
        unsafe { JPC_CharacterVirtual_GetRotation(self.raw).into_rolt() }
    }

    pub fn set_rotation(&mut self, rotation: Quat) {
        unsafe { JPC_CharacterVirtual_SetRotation(self.raw, rotation.into_jolt()) }
    }

    pub fn center_of_mass_position(&self) -> RVec3 {
        unsafe { JPC_CharacterVirtual_GetCenterOfMassPosition(self.raw).into_rolt() }
    }

    // --- velocity ---

    pub fn linear_velocity(&self) -> Vec3 {
        unsafe { JPC_CharacterVirtual_GetLinearVelocity(self.raw).into_rolt() }
    }

    pub fn set_linear_velocity(&mut self, velocity: Vec3) {
        unsafe { JPC_CharacterVirtual_SetLinearVelocity(self.raw, velocity.into_jolt()) }
    }

    pub fn cancel_velocity_towards_steep_slopes(&self, desired_velocity: Vec3) -> Vec3 {
        unsafe {
            JPC_CharacterVirtual_CancelVelocityTowardsSteepSlopes(
                self.raw,
                desired_velocity.into_jolt(),
            )
            .into_rolt()
        }
    }

    // --- up direction ---

    pub fn up(&self) -> Vec3 {
        unsafe { JPC_CharacterVirtual_GetUp(self.raw).into_rolt() }
    }
    pub fn set_up(&mut self, up: Vec3) {
        unsafe { JPC_CharacterVirtual_SetUp(self.raw, up.into_jolt()) }
    }

    // --- ground state ---

    pub fn ground_state(&self) -> JPC_GroundState {
        unsafe { JPC_CharacterVirtual_GetGroundState(self.raw) }
    }

    pub fn is_supported(&self) -> bool {
        unsafe { JPC_CharacterVirtual_IsSupported(self.raw) }
    }
    pub fn ground_position(&self) -> RVec3 {
        unsafe { JPC_CharacterVirtual_GetGroundPosition(self.raw).into_rolt() }
    }
    pub fn ground_normal(&self) -> Vec3 {
        unsafe { JPC_CharacterVirtual_GetGroundNormal(self.raw).into_rolt() }
    }
    pub fn ground_velocity(&self) -> Vec3 {
        unsafe { JPC_CharacterVirtual_GetGroundVelocity(self.raw).into_rolt() }
    }
    pub fn ground_body_id(&self) -> BodyId {
        BodyId::new(unsafe { JPC_CharacterVirtual_GetGroundBodyID(self.raw) })
    }
    pub fn ground_user_data(&self) -> u64 {
        unsafe { JPC_CharacterVirtual_GetGroundUserData(self.raw) }
    }

    // --- physics properties ---

    pub fn mass(&self) -> f32 {
        unsafe { JPC_CharacterVirtual_GetMass(self.raw) }
    }
    pub fn set_mass(&mut self, mass: f32) {
        unsafe { JPC_CharacterVirtual_SetMass(self.raw, mass) }
    }

    pub fn max_strength(&self) -> f32 {
        unsafe { JPC_CharacterVirtual_GetMaxStrength(self.raw) }
    }
    pub fn set_max_strength(&mut self, strength: f32) {
        unsafe { JPC_CharacterVirtual_SetMaxStrength(self.raw, strength) }
    }

    pub fn penetration_recovery_speed(&self) -> f32 {
        unsafe { JPC_CharacterVirtual_GetPenetrationRecoverySpeed(self.raw) }
    }

    pub fn set_penetration_recovery_speed(&mut self, speed: f32) {
        unsafe { JPC_CharacterVirtual_SetPenetrationRecoverySpeed(self.raw, speed) }
    }

    pub fn max_num_hits(&self) -> u32 {
        unsafe { JPC_CharacterVirtual_GetMaxNumHits(self.raw) }
    }
    pub fn set_max_num_hits(&mut self, max_hits: u32) {
        unsafe { JPC_CharacterVirtual_SetMaxNumHits(self.raw, max_hits) }
    }

    pub fn max_hits_exceeded(&self) -> bool {
        unsafe { JPC_CharacterVirtual_GetMaxHitsExceeded(self.raw) }
    }

    // --- user data ---

    pub fn user_data(&self) -> u64 {
        unsafe { JPC_CharacterVirtual_GetUserData(self.raw) }
    }
    pub fn set_user_data(&mut self, data: u64) {
        unsafe { JPC_CharacterVirtual_SetUserData(self.raw, data) }
    }

    pub fn inner_body_id(&self) -> BodyId {
        BodyId::new(unsafe { JPC_CharacterVirtual_GetInnerBodyID(self.raw) })
    }

    // --- simulation ---

    pub fn update(&mut self, args: CharacterUpdateArgs<'_>) {
        let mut raw = JPC_CharacterVirtual_UpdateArgs {
            DeltaTime: args.delta_time,
            __bindgen_padding_0: 0,
            Gravity: args.gravity.into_jolt(),
            BroadPhaseLayerFilter: args.broad_phase_layer_filter.as_ref().into_jolt(),
            ObjectLayerFilter: args.object_layer_filter.as_ref().into_jolt(),
            BodyFilter: args.body_filter.as_ref().into_jolt(),
            ShapeFilter: args.shape_filter.as_ref().into_jolt(),
            TempAllocator: args.temp_allocator.raw(),
        };
        unsafe { JPC_CharacterVirtual_Update(self.raw, &mut raw) }
    }

    pub fn can_walk_stairs(&self, linear_velocity: Vec3) -> bool {
        unsafe { JPC_CharacterVirtual_CanWalkStairs(self.raw, linear_velocity.into_jolt()) }
    }

    pub fn walk_stairs(&mut self, args: CharacterWalkStairsArgs<'_>) -> bool {
        let mut raw = JPC_CharacterVirtual_WalkStairsArgs {
            DeltaTime: args.delta_time,
            __bindgen_padding_0: 0,
            StepUp: args.step_up.into_jolt(),
            StepForward: args.step_forward.into_jolt(),
            StepForwardTest: args.step_forward_test.into_jolt(),
            StepDownExtra: args.step_down_extra.into_jolt(),
            BroadPhaseLayerFilter: args.broad_phase_layer_filter.as_ref().into_jolt(),
            ObjectLayerFilter: args.object_layer_filter.as_ref().into_jolt(),
            BodyFilter: args.body_filter.as_ref().into_jolt(),
            ShapeFilter: args.shape_filter.as_ref().into_jolt(),
            TempAllocator: args.temp_allocator.raw(),
        };
        unsafe { JPC_CharacterVirtual_WalkStairs(self.raw, &mut raw) }
    }

    pub fn stick_to_floor(&mut self, args: CharacterStickToFloorArgs<'_>) -> bool {
        let mut raw = JPC_CharacterVirtual_StickToFloorArgs {
            StepDown: args.step_down.into_jolt(),
            BroadPhaseLayerFilter: args.broad_phase_layer_filter.as_ref().into_jolt(),
            ObjectLayerFilter: args.object_layer_filter.as_ref().into_jolt(),
            BodyFilter: args.body_filter.as_ref().into_jolt(),
            ShapeFilter: args.shape_filter.as_ref().into_jolt(),
            TempAllocator: args.temp_allocator.raw(),
        };
        unsafe { JPC_CharacterVirtual_StickToFloor(self.raw, &mut raw) }
    }

    pub fn extended_update(&mut self, args: CharacterExtendedUpdateArgs<'_>) {
        let mut raw = JPC_CharacterVirtual_ExtendedUpdateArgs {
            DeltaTime: args.delta_time,
            __bindgen_padding_0: 0,
            Gravity: args.gravity.into_jolt(),
            Settings: args.settings.0,
            BroadPhaseLayerFilter: args.broad_phase_layer_filter.as_ref().into_jolt(),
            ObjectLayerFilter: args.object_layer_filter.as_ref().into_jolt(),
            BodyFilter: args.body_filter.as_ref().into_jolt(),
            ShapeFilter: args.shape_filter.as_ref().into_jolt(),
            TempAllocator: args.temp_allocator.raw(),
        };
        unsafe { JPC_CharacterVirtual_ExtendedUpdate(self.raw, &mut raw) }
    }

    pub fn update_ground_velocity(&mut self) {
        unsafe { JPC_CharacterVirtual_UpdateGroundVelocity(self.raw) }
    }

    // --- phase 2 gaps ---

    pub fn id(&self) -> crate::JPC_CharacterID {
        unsafe { JPC_CharacterVirtual_GetID(self.raw) }
    }

    pub fn character_padding(&self) -> f32 {
        unsafe { JPC_CharacterVirtual_GetCharacterPadding(self.raw) }
    }

    pub fn enhanced_internal_edge_removal(&self) -> bool {
        unsafe { JPC_CharacterVirtual_GetEnhancedInternalEdgeRemoval(self.raw) }
    }

    pub fn set_enhanced_internal_edge_removal(&mut self, value: bool) {
        unsafe { JPC_CharacterVirtual_SetEnhancedInternalEdgeRemoval(self.raw, value) }
    }

    pub fn hit_reduction_cos_max_angle(&self) -> f32 {
        unsafe { JPC_CharacterVirtual_GetHitReductionCosMaxAngle(self.raw) }
    }

    pub fn set_hit_reduction_cos_max_angle(&mut self, value: f32) {
        unsafe { JPC_CharacterVirtual_SetHitReductionCosMaxAngle(self.raw, value) }
    }

    pub fn refresh_contacts(&mut self, args: CharacterRefreshContactsArgs<'_>) {
        let mut raw = JPC_CharacterVirtual_RefreshContactsArgs {
            BroadPhaseLayerFilter: args.broad_phase_layer_filter.as_ref().into_jolt(),
            ObjectLayerFilter: args.object_layer_filter.as_ref().into_jolt(),
            BodyFilter: args.body_filter.as_ref().into_jolt(),
            ShapeFilter: args.shape_filter.as_ref().into_jolt(),
            TempAllocator: args.temp_allocator.raw(),
        };
        unsafe { JPC_CharacterVirtual_RefreshContacts(self.raw, &mut raw) }
    }

    pub fn set_shape(&mut self, args: CharacterSetShapeArgs<'_>) -> bool {
        let mut raw = JPC_CharacterVirtual_SetShapeArgs {
            Shape: args.shape.get(),
            MaxPenetrationDepth: args.max_penetration_depth,
            BroadPhaseLayerFilter: args.broad_phase_layer_filter.as_ref().into_jolt(),
            ObjectLayerFilter: args.object_layer_filter.as_ref().into_jolt(),
            BodyFilter: args.body_filter.as_ref().into_jolt(),
            ShapeFilter: args.shape_filter.as_ref().into_jolt(),
            TempAllocator: args.temp_allocator.raw(),
        };
        unsafe { JPC_CharacterVirtual_SetShape(self.raw, &mut raw) }
    }

    pub fn set_listener(
        &mut self,
        listener: Option<impl Into<CharacterContactListenerImpl<'static>>>,
    ) {
        if let Some(listener) = listener {
            let listener = listener.into();
            let raw = listener.raw();
            self.contact_listener = Some(listener);
            unsafe { JPC_CharacterVirtual_SetListener(self.raw, raw) }
        } else {
            self.contact_listener = None;
            unsafe { JPC_CharacterVirtual_SetListener(self.raw, std::ptr::null_mut()) }
        }
    }

    /// Returns the currently installed contact listener, if any.
    pub fn contact_listener(&self) -> Option<&CharacterContactListenerImpl<'static>> {
        self.contact_listener.as_ref()
    }

    pub fn with_raw<R>(&self, f: impl FnOnce(*mut JPC_CharacterVirtual) -> R) -> R {
        f(self.raw)
    }

    pub fn raw(&self) -> *mut JPC_CharacterVirtual {
        self.raw
    }
}

impl Drop for CharacterVirtual {
    fn drop(&mut self) {
        unsafe { JPC_CharacterVirtual_delete(self.raw) }
    }
}
