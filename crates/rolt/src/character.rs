use joltc_sys::*;

use crate::{BodyId, FromJolt, IntoJolt, IntoRolt, Quat, RVec3, Vec3};

/// Settings for [`CharacterVirtual::extended_update`].
///
/// See also: Jolt's [`ExtendedUpdateSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/struct_character_virtual_1_1_extended_update_settings.html) struct.
pub struct ExtendedUpdateSettings(pub JPC_ExtendedUpdateSettings);

impl ExtendedUpdateSettings {
    pub fn step_down(&self) -> Vec3 { Vec3::from_jolt(self.0.StickToFloorStepDown) }
    pub fn set_step_down(&mut self, v: Vec3) { self.0.StickToFloorStepDown = v.into_jolt(); }

    pub fn step_up(&self) -> Vec3 { Vec3::from_jolt(self.0.WalkStairsStepUp) }
    pub fn set_step_up(&mut self, v: Vec3) { self.0.WalkStairsStepUp = v.into_jolt(); }

    pub fn min_step_forward(&self) -> f32 { self.0.WalkStairsMinStepForward }
    pub fn set_min_step_forward(&mut self, v: f32) { self.0.WalkStairsMinStepForward = v; }

    pub fn step_forward_test(&self) -> f32 { self.0.WalkStairsStepForwardTest }
    pub fn set_step_forward_test(&mut self, v: f32) { self.0.WalkStairsStepForwardTest = v; }

    pub fn cos_angle_forward_contact(&self) -> f32 { self.0.WalkStairsCosAngleForwardContact }
    pub fn set_cos_angle_forward_contact(&mut self, v: f32) { self.0.WalkStairsCosAngleForwardContact = v; }

    pub fn step_down_extra(&self) -> Vec3 { Vec3::from_jolt(self.0.WalkStairsStepDownExtra) }
    pub fn set_step_down_extra(&mut self, v: Vec3) { self.0.WalkStairsStepDownExtra = v.into_jolt(); }
}

impl Default for ExtendedUpdateSettings {
    fn default() -> Self {
        let mut raw = unsafe { std::mem::zeroed::<JPC_ExtendedUpdateSettings>() };
        unsafe { JPC_ExtendedUpdateSettings_default(&mut raw) };
        Self(raw)
    }
}

/// See also: Jolt's [`CharacterVirtual`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_character_virtual.html) class.
pub struct CharacterVirtual {
    raw: *mut JPC_CharacterVirtual,
}

impl CharacterVirtual {
    /// Create a new character virtual.
    ///
    /// # Safety
    /// `physics_system` must be valid and outlive this character.
    pub unsafe fn new(settings: &JPC_CharacterVirtualSettings, position: RVec3, rotation: Quat, user_data: u64, physics_system: *mut JPC_PhysicsSystem) -> Self {
        let raw = JPC_CharacterVirtual_new(settings, position.into_jolt(), rotation.into_jolt(), user_data, physics_system);
        Self { raw }
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
        unsafe { JPC_CharacterVirtual_CancelVelocityTowardsSteepSlopes(self.raw, desired_velocity.into_jolt()).into_rolt() }
    }

    // --- up direction ---

    pub fn up(&self) -> Vec3 { unsafe { JPC_CharacterVirtual_GetUp(self.raw).into_rolt() } }
    pub fn set_up(&mut self, up: Vec3) {
        unsafe { JPC_CharacterVirtual_SetUp(self.raw, up.into_jolt()) }
    }

    // --- ground state ---

    pub fn ground_state(&self) -> JPC_GroundState {
        unsafe { JPC_CharacterVirtual_GetGroundState(self.raw) }
    }

    pub fn is_supported(&self) -> bool { unsafe { JPC_CharacterVirtual_IsSupported(self.raw) } }
    pub fn ground_position(&self) -> RVec3 { unsafe { JPC_CharacterVirtual_GetGroundPosition(self.raw).into_rolt() } }
    pub fn ground_normal(&self) -> Vec3 { unsafe { JPC_CharacterVirtual_GetGroundNormal(self.raw).into_rolt() } }
    pub fn ground_velocity(&self) -> Vec3 { unsafe { JPC_CharacterVirtual_GetGroundVelocity(self.raw).into_rolt() } }
    pub fn ground_body_id(&self) -> BodyId { BodyId::new(unsafe { JPC_CharacterVirtual_GetGroundBodyID(self.raw) }) }
    pub fn ground_user_data(&self) -> u64 { unsafe { JPC_CharacterVirtual_GetGroundUserData(self.raw) } }

    // --- physics properties ---

    pub fn mass(&self) -> f32 { unsafe { JPC_CharacterVirtual_GetMass(self.raw) } }
    pub fn set_mass(&mut self, mass: f32) { unsafe { JPC_CharacterVirtual_SetMass(self.raw, mass) } }

    pub fn max_strength(&self) -> f32 { unsafe { JPC_CharacterVirtual_GetMaxStrength(self.raw) } }
    pub fn set_max_strength(&mut self, strength: f32) {
        unsafe { JPC_CharacterVirtual_SetMaxStrength(self.raw, strength) }
    }

    pub fn penetration_recovery_speed(&self) -> f32 {
        unsafe { JPC_CharacterVirtual_GetPenetrationRecoverySpeed(self.raw) }
    }

    pub fn set_penetration_recovery_speed(&mut self, speed: f32) {
        unsafe { JPC_CharacterVirtual_SetPenetrationRecoverySpeed(self.raw, speed) }
    }

    pub fn max_num_hits(&self) -> u32 { unsafe { JPC_CharacterVirtual_GetMaxNumHits(self.raw) } }
    pub fn set_max_num_hits(&mut self, max_hits: u32) {
        unsafe { JPC_CharacterVirtual_SetMaxNumHits(self.raw, max_hits) }
    }

    pub fn max_hits_exceeded(&self) -> bool { unsafe { JPC_CharacterVirtual_GetMaxHitsExceeded(self.raw) } }

    // --- user data ---

    pub fn user_data(&self) -> u64 { unsafe { JPC_CharacterVirtual_GetUserData(self.raw) } }
    pub fn set_user_data(&mut self, data: u64) {
        unsafe { JPC_CharacterVirtual_SetUserData(self.raw, data) }
    }

    pub fn inner_body_id(&self) -> BodyId {
        BodyId::new(unsafe { JPC_CharacterVirtual_GetInnerBodyID(self.raw) })
    }

    // --- simulation ---

    /// Basic update — moves the character and resolves collisions.
    ///
    /// # Safety
    /// `temp_allocator` must be valid for the duration of the call.
    pub unsafe fn update(&mut self, args: JPC_CharacterVirtual_UpdateArgs) {
        let mut args = args;
        JPC_CharacterVirtual_Update(self.raw, &mut args)
    }

    pub fn can_walk_stairs(&self, linear_velocity: Vec3) -> bool {
        unsafe { JPC_CharacterVirtual_CanWalkStairs(self.raw, linear_velocity.into_jolt()) }
    }

    /// # Safety
    /// `temp_allocator` must be valid.
    pub unsafe fn walk_stairs(&mut self, args: JPC_CharacterVirtual_WalkStairsArgs) -> bool {
        let mut args = args;
        JPC_CharacterVirtual_WalkStairs(self.raw, &mut args)
    }

    /// # Safety
    /// `temp_allocator` must be valid.
    pub unsafe fn stick_to_floor(&mut self, args: JPC_CharacterVirtual_StickToFloorArgs) -> bool {
        let mut args = args;
        JPC_CharacterVirtual_StickToFloor(self.raw, &mut args)
    }

    /// Extended update that handles stairs and floor sticking.
    ///
    /// # Safety
    /// `temp_allocator` must be valid.
    pub unsafe fn extended_update(&mut self, args: JPC_CharacterVirtual_ExtendedUpdateArgs) {
        let mut args = args;
        JPC_CharacterVirtual_ExtendedUpdate(self.raw, &mut args)
    }

    pub fn update_ground_velocity(&mut self) {
        unsafe { JPC_CharacterVirtual_UpdateGroundVelocity(self.raw) }
    }

    pub fn raw(&self) -> *mut JPC_CharacterVirtual { self.raw }
}

impl Drop for CharacterVirtual {
    fn drop(&mut self) {
        unsafe { JPC_CharacterVirtual_delete(self.raw) }
    }
}
