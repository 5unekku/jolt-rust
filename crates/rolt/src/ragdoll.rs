use joltc_sys::*;

use crate::{BodyId, FromJolt, IntoJolt, IntoRolt, Mat4, Ref, Vec3};

/// Settings for a ragdoll — a collection of bodies connected by constraints.
///
/// See also: Jolt's [`RagdollSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_ragdoll_settings.html) class.
pub struct RagdollSettings {
    raw: *mut JPC_RagdollSettings,
}

impl RagdollSettings {
    pub fn new() -> Self {
        Self { raw: unsafe { JPC_RagdollSettings_new() } }
    }

    pub fn part_count(&self) -> u32 {
        unsafe { JPC_RagdollSettings_GetPartCount(self.raw) as u32 }
    }

    pub fn add_part(&mut self, settings: &JPC_BodyCreationSettings) {
        unsafe { JPC_RagdollSettings_AddPart(self.raw, settings) }
    }

    pub fn set_part_fixed_constraint(&mut self, part_index: u32, settings: &JPC_FixedConstraintSettings) {
        unsafe { JPC_RagdollSettings_SetPartFixedConstraint(self.raw, part_index as _, settings) }
    }

    pub fn set_part_hinge_constraint(&mut self, part_index: u32, settings: &JPC_HingeConstraintSettings) {
        unsafe { JPC_RagdollSettings_SetPartHingeConstraint(self.raw, part_index as _, settings) }
    }

    pub fn set_part_slider_constraint(&mut self, part_index: u32, settings: &JPC_SliderConstraintSettings) {
        unsafe { JPC_RagdollSettings_SetPartSliderConstraint(self.raw, part_index as _, settings) }
    }

    pub fn set_part_swing_twist_constraint(&mut self, part_index: u32, settings: &JPC_SwingTwistConstraintSettings) {
        unsafe { JPC_RagdollSettings_SetPartSwingTwistConstraint(self.raw, part_index as _, settings) }
    }

    pub fn set_part_point_constraint(&mut self, part_index: u32, settings: &JPC_PointConstraintSettings) {
        unsafe { JPC_RagdollSettings_SetPartPointConstraint(self.raw, part_index as _, settings) }
    }

    /// Create a ragdoll and add it to the given physics system.
    /// # Safety
    /// `physics_system` must be valid and outlive the returned `Ragdoll`.
    pub unsafe fn create_ragdoll(
        &self,
        collision_group: JPC_GroupID,
        user_data: u64,
        physics_system: *mut JPC_PhysicsSystem,
    ) -> Option<Ragdoll> {
        let raw = JPC_RagdollSettings_CreateRagdoll(
            self.raw,
            collision_group,
            user_data,
            physics_system,
        );
        if raw.is_null() { None } else { Some(Ragdoll { raw }) }
    }

    pub fn raw(&self) -> *mut JPC_RagdollSettings {
        self.raw
    }
}

impl Default for RagdollSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for RagdollSettings {
    fn drop(&mut self) {
        unsafe { JPC_RagdollSettings_delete(self.raw) }
    }
}

/// A live ragdoll instance in a physics simulation.
///
/// See also: Jolt's [`Ragdoll`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_ragdoll.html) class.
pub struct Ragdoll {
    raw: *mut JPC_Ragdoll,
}

impl Ragdoll {
    pub fn add_to_physics_system(&mut self, activation: JPC_Activation, lock_bodies: bool) {
        unsafe { JPC_Ragdoll_AddToPhysicsSystem(self.raw, activation, lock_bodies) }
    }

    pub fn remove_from_physics_system(&mut self, lock_bodies: bool) {
        unsafe { JPC_Ragdoll_RemoveFromPhysicsSystem(self.raw, lock_bodies) }
    }

    pub fn activate(&mut self, lock_bodies: bool) {
        unsafe { JPC_Ragdoll_Activate(self.raw, lock_bodies) }
    }

    pub fn body_count(&self) -> u32 {
        unsafe { JPC_Ragdoll_GetBodyCount(self.raw) as u32 }
    }

    pub fn body_id(&self, index: u32) -> BodyId {
        BodyId::new(unsafe { JPC_Ragdoll_GetBodyID(self.raw, index as _) })
    }

    pub fn body_ids(&self) -> Vec<BodyId> {
        let count = self.body_count();
        let mut ids = vec![0u32; count as usize];
        let mut out_count = count as _;
        unsafe { JPC_Ragdoll_GetBodyIDs(self.raw, ids.as_mut_ptr(), &mut out_count) }
        ids.into_iter().map(BodyId::new).collect()
    }

    pub fn is_active(&self, lock_bodies: bool) -> bool {
        unsafe { JPC_Ragdoll_IsActive(self.raw, lock_bodies) }
    }

    pub fn reset_warm_start(&mut self) {
        unsafe { JPC_Ragdoll_ResetWarmStart(self.raw) }
    }

    pub fn set_group_id(&mut self, group_id: JPC_GroupID, lock_bodies: bool) {
        unsafe { JPC_Ragdoll_SetGroupID(self.raw, group_id, lock_bodies) }
    }

    pub fn set_linear_velocity(&mut self, velocity: Vec3, lock_bodies: bool) {
        unsafe { JPC_Ragdoll_SetLinearVelocity(self.raw, velocity.into_jolt(), lock_bodies) }
    }

    pub fn add_linear_velocity(&mut self, velocity: Vec3, lock_bodies: bool) {
        unsafe { JPC_Ragdoll_AddLinearVelocity(self.raw, velocity.into_jolt(), lock_bodies) }
    }

    pub fn add_impulse(&mut self, impulse: Vec3, lock_bodies: bool) {
        unsafe { JPC_Ragdoll_AddImpulse(self.raw, impulse.into_jolt(), lock_bodies) }
    }

    pub fn constraint_count(&self) -> u32 {
        unsafe { JPC_Ragdoll_GetConstraintCount(self.raw) as u32 }
    }

    /// Set joint poses (one `Mat4` per joint).
    pub fn set_pose(&mut self, root_offset: crate::RVec3, joint_matrices: &[Mat4], lock_bodies: bool) {
        let raw_mats: Vec<JPC_Mat44> = joint_matrices.iter().copied().map(IntoJolt::into_jolt).collect();
        unsafe {
            JPC_Ragdoll_SetPose(
                self.raw,
                root_offset.into_jolt(),
                raw_mats.as_ptr(),
                raw_mats.len() as u32,
                lock_bodies,
            )
        }
    }

    /// Get joint poses.  Returns `(root_offset, joint_matrices)`.
    pub fn get_pose(&mut self, joint_count: u32, lock_bodies: bool) -> (crate::RVec3, Vec<Mat4>) {
        let mut root_offset = unsafe { std::mem::zeroed::<JPC_RVec3>() };
        let mut raw_mats = vec![unsafe { std::mem::zeroed::<JPC_Mat44>() }; joint_count as usize];
        unsafe {
            JPC_Ragdoll_GetPose(
                self.raw,
                &mut root_offset,
                raw_mats.as_mut_ptr(),
                joint_count,
                lock_bodies,
            )
        }
        let mats = raw_mats.into_iter().map(Mat4::from_jolt).collect();
        (root_offset.into_rolt(), mats)
    }

    pub fn raw(&self) -> *mut JPC_Ragdoll {
        self.raw
    }
}

impl Drop for Ragdoll {
    fn drop(&mut self) {
        unsafe { JPC_Ragdoll_delete(self.raw) }
    }
}

/// A single ragdoll part wrapping a `JPC_RagdollSettings` ref with part-constraint helpers.
/// Extend by keeping a `Ref<JPC_RagdollSettings>` via `RefTarget` impl.
pub struct RagdollSettingsRef(pub Ref<JPC_RagdollSettings>);
