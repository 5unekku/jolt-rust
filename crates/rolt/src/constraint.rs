use joltc_sys::*;

use crate::{Body, IntoRolt, Ref, Vec3};

/// Base constraint wrapper.  Use [`Constraint::raw`] to pass to physics system.
///
/// See also: Jolt's [`Constraint`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_constraint.html) class.
pub struct Constraint(pub(crate) Ref<JPC_Constraint>);

impl Constraint {
    pub fn is_enabled(&self) -> bool { unsafe { JPC_Constraint_GetEnabled(*self.0) } }
    pub fn set_enabled(&self, enabled: bool) { unsafe { JPC_Constraint_SetEnabled(*self.0, enabled) } }
    pub fn user_data(&self) -> u64 { unsafe { JPC_Constraint_GetUserData(*self.0) } }
    pub fn set_user_data(&self, data: u64) { unsafe { JPC_Constraint_SetUserData(*self.0, data) } }
    pub fn constraint_priority(&self) -> u32 { unsafe { JPC_Constraint_GetConstraintPriority(*self.0) } }
    pub fn set_constraint_priority(&self, priority: u32) { unsafe { JPC_Constraint_SetConstraintPriority(*self.0, priority) } }

    /// The raw pointer — pass to [`PhysicsSystem::add_constraint`][crate::PhysicsSystem::add_constraint].
    pub fn raw(&self) -> *mut JPC_Constraint { *self.0 }
}

// --- fixed constraint ---

/// See also: Jolt's [`FixedConstraint`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_fixed_constraint.html) class.
pub struct FixedConstraint(pub(crate) Ref<JPC_Constraint>);

impl FixedConstraint {
    pub fn create(settings: &JPC_FixedConstraintSettings, body1: &Body<'_>, body2: &Body<'_>) -> Self {
        unsafe {
            let raw = JPC_FixedConstraintSettings_Create(settings, body1.raw(), body2.raw());
            Self(Ref::from_active(raw))
        }
    }

    pub fn as_constraint(&self) -> Constraint { Constraint(self.0.clone()) }
    pub fn raw(&self) -> *mut JPC_Constraint { *self.0 }
}

// --- distance constraint ---

/// See also: Jolt's [`DistanceConstraint`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_distance_constraint.html) class.
pub struct DistanceConstraint(pub(crate) Ref<JPC_DistanceConstraint>);

impl DistanceConstraint {
    pub fn create(settings: &JPC_DistanceConstraintSettings, body1: &Body<'_>, body2: &Body<'_>) -> Self {
        unsafe {
            let raw = JPC_DistanceConstraintSettings_Create(settings, body1.raw(), body2.raw());
            Self(Ref::from_active(raw))
        }
    }

    pub fn set_distance(&self, min_distance: f32, max_distance: f32) {
        unsafe { JPC_DistanceConstraint_SetDistance(*self.0, min_distance, max_distance) }
    }

    pub fn min_distance(&self) -> f32 { unsafe { JPC_DistanceConstraint_GetMinDistance(*self.0) } }
    pub fn max_distance(&self) -> f32 { unsafe { JPC_DistanceConstraint_GetMaxDistance(*self.0) } }

    pub fn as_constraint(&self) -> Constraint { Constraint(self.0.clone().cast()) }
    pub fn raw(&self) -> *mut JPC_DistanceConstraint { *self.0 }
}

// --- hinge constraint ---

/// See also: Jolt's [`HingeConstraint`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_hinge_constraint.html) class.
pub struct HingeConstraint(pub(crate) Ref<JPC_HingeConstraint>);

impl HingeConstraint {
    pub fn create(settings: &JPC_HingeConstraintSettings, body1: &Body<'_>, body2: &Body<'_>) -> Self {
        unsafe {
            let raw = JPC_HingeConstraintSettings_Create(settings, body1.raw(), body2.raw());
            Self(Ref::from_active(raw))
        }
    }

    pub fn motor_state(&self) -> JPC_MotorState { unsafe { JPC_HingeConstraint_GetMotorState(*self.0) } }
    pub fn set_motor_state(&self, state: JPC_MotorState) { unsafe { JPC_HingeConstraint_SetMotorState(*self.0, state) } }
    pub fn target_angular_velocity(&self) -> f32 { unsafe { JPC_HingeConstraint_GetTargetAngularVelocity(*self.0) } }
    pub fn set_target_angular_velocity(&self, v: f32) { unsafe { JPC_HingeConstraint_SetTargetAngularVelocity(*self.0, v) } }
    pub fn target_angle(&self) -> f32 { unsafe { JPC_HingeConstraint_GetTargetAngle(*self.0) } }
    pub fn set_target_angle(&self, angle: f32) { unsafe { JPC_HingeConstraint_SetTargetAngle(*self.0, angle) } }
    pub fn current_angle(&self) -> f32 { unsafe { JPC_HingeConstraint_GetCurrentAngle(*self.0) } }
    pub fn set_limits(&self, min: f32, max: f32) { unsafe { JPC_HingeConstraint_SetLimits(*self.0, min, max) } }
    pub fn limits_min(&self) -> f32 { unsafe { JPC_HingeConstraint_GetLimitsMin(*self.0) } }
    pub fn limits_max(&self) -> f32 { unsafe { JPC_HingeConstraint_GetLimitsMax(*self.0) } }
    pub fn has_limits(&self) -> bool { unsafe { JPC_HingeConstraint_HasLimits(*self.0) } }
    pub fn max_friction_torque(&self) -> f32 { unsafe { JPC_HingeConstraint_GetMaxFrictionTorque(*self.0) } }
    pub fn set_max_friction_torque(&self, torque: f32) { unsafe { JPC_HingeConstraint_SetMaxFrictionTorque(*self.0, torque) } }

    pub fn as_constraint(&self) -> Constraint { Constraint(self.0.clone().cast()) }
    pub fn raw(&self) -> *mut JPC_HingeConstraint { *self.0 }
}

// --- slider constraint ---

/// See also: Jolt's [`SliderConstraint`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_slider_constraint.html) class.
pub struct SliderConstraint(pub(crate) Ref<JPC_SliderConstraint>);

impl SliderConstraint {
    pub fn create(settings: &JPC_SliderConstraintSettings, body1: &Body<'_>, body2: &Body<'_>) -> Self {
        unsafe {
            let raw = JPC_SliderConstraintSettings_Create(settings, body1.raw(), body2.raw());
            Self(Ref::from_active(raw))
        }
    }

    pub fn motor_state(&self) -> JPC_MotorState { unsafe { JPC_SliderConstraint_GetMotorState(*self.0) } }
    pub fn set_motor_state(&self, state: JPC_MotorState) { unsafe { JPC_SliderConstraint_SetMotorState(*self.0, state) } }
    pub fn target_velocity(&self) -> f32 { unsafe { JPC_SliderConstraint_GetTargetVelocity(*self.0) } }
    pub fn set_target_velocity(&self, v: f32) { unsafe { JPC_SliderConstraint_SetTargetVelocity(*self.0, v) } }
    pub fn target_position(&self) -> f32 { unsafe { JPC_SliderConstraint_GetTargetPosition(*self.0) } }
    pub fn set_target_position(&self, p: f32) { unsafe { JPC_SliderConstraint_SetTargetPosition(*self.0, p) } }
    pub fn current_position(&self) -> f32 { unsafe { JPC_SliderConstraint_GetCurrentPosition(*self.0) } }
    pub fn set_limits(&self, min: f32, max: f32) { unsafe { JPC_SliderConstraint_SetLimits(*self.0, min, max) } }
    pub fn limits_min(&self) -> f32 { unsafe { JPC_SliderConstraint_GetLimitsMin(*self.0) } }
    pub fn limits_max(&self) -> f32 { unsafe { JPC_SliderConstraint_GetLimitsMax(*self.0) } }
    pub fn has_limits(&self) -> bool { unsafe { JPC_SliderConstraint_HasLimits(*self.0) } }
    pub fn max_friction_force(&self) -> f32 { unsafe { JPC_SliderConstraint_GetMaxFrictionForce(*self.0) } }
    pub fn set_max_friction_force(&self, force: f32) { unsafe { JPC_SliderConstraint_SetMaxFrictionForce(*self.0, force) } }

    pub fn as_constraint(&self) -> Constraint { Constraint(self.0.clone().cast()) }
    pub fn raw(&self) -> *mut JPC_SliderConstraint { *self.0 }
}

// --- point constraint ---

/// See also: Jolt's [`PointConstraint`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_point_constraint.html) class.
pub struct PointConstraint(pub(crate) Ref<JPC_PointConstraint>);

impl PointConstraint {
    pub fn create(settings: &JPC_PointConstraintSettings, body1: &Body<'_>, body2: &Body<'_>) -> Self {
        unsafe {
            let raw = JPC_PointConstraintSettings_Create(settings, body1.raw(), body2.raw());
            Self(Ref::from_active(raw))
        }
    }

    pub fn local_space_point1(&self) -> Vec3 { unsafe { JPC_PointConstraint_GetLocalSpacePoint1(*self.0).into_rolt() } }
    pub fn local_space_point2(&self) -> Vec3 { unsafe { JPC_PointConstraint_GetLocalSpacePoint2(*self.0).into_rolt() } }

    pub fn as_constraint(&self) -> Constraint { Constraint(self.0.clone().cast()) }
    pub fn raw(&self) -> *mut JPC_PointConstraint { *self.0 }
}

// --- six DOF constraint ---

/// See also: Jolt's [`SixDOFConstraint`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_six_d_o_f_constraint.html) class.
pub struct SixDofConstraint(pub(crate) Ref<JPC_Constraint>);

impl SixDofConstraint {
    pub fn create(settings: &JPC_SixDOFConstraintSettings, body1: &Body<'_>, body2: &Body<'_>) -> Self {
        unsafe {
            let raw = JPC_SixDOFConstraintSettings_Create(settings, body1.raw(), body2.raw());
            Self(Ref::from_active(raw))
        }
    }

    pub fn as_constraint(&self) -> Constraint { Constraint(self.0.clone()) }
    pub fn raw(&self) -> *mut JPC_Constraint { *self.0 }
}

// --- swing-twist constraint ---

/// See also: Jolt's [`SwingTwistConstraint`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_swing_twist_constraint.html) class.
pub struct SwingTwistConstraint(pub(crate) Ref<JPC_SwingTwistConstraint>);

impl SwingTwistConstraint {
    pub fn create(settings: &JPC_SwingTwistConstraintSettings, body1: &Body<'_>, body2: &Body<'_>) -> Self {
        unsafe {
            let raw = JPC_SwingTwistConstraintSettings_Create(settings, body1.raw(), body2.raw());
            Self(Ref::from_active(raw))
        }
    }

    pub fn as_constraint(&self) -> Constraint { Constraint(self.0.clone().cast()) }
    pub fn raw(&self) -> *mut JPC_SwingTwistConstraint { *self.0 }
}

// --- cone constraint ---

/// See also: Jolt's [`ConeConstraint`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_cone_constraint.html) class.
pub struct ConeConstraint(pub(crate) Ref<JPC_ConeConstraint>);

impl ConeConstraint {
    pub fn create(settings: &JPC_ConeConstraintSettings, body1: &Body<'_>, body2: &Body<'_>) -> Self {
        unsafe {
            let raw = JPC_ConeConstraintSettings_Create(settings, body1.raw(), body2.raw());
            Self(Ref::from_active(raw))
        }
    }

    pub fn set_half_cone_angle(&self, angle: f32) { unsafe { JPC_ConeConstraint_SetHalfConeAngle(*self.0, angle) } }
    pub fn cos_half_cone_angle(&self) -> f32 { unsafe { JPC_ConeConstraint_GetCosHalfConeAngle(*self.0) } }

    pub fn as_constraint(&self) -> Constraint { Constraint(self.0.clone().cast()) }
    pub fn raw(&self) -> *mut JPC_ConeConstraint { *self.0 }
}

// --- pulley constraint ---

/// See also: Jolt's [`PulleyConstraint`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_pulley_constraint.html) class.
pub struct PulleyConstraint(pub(crate) Ref<JPC_PulleyConstraint>);

impl PulleyConstraint {
    pub fn create(settings: &JPC_PulleyConstraintSettings, body1: &Body<'_>, body2: &Body<'_>) -> Self {
        unsafe {
            let raw = JPC_PulleyConstraintSettings_Create(settings, body1.raw(), body2.raw());
            Self(Ref::from_active(raw))
        }
    }

    pub fn current_length(&self) -> f32 { unsafe { JPC_PulleyConstraint_GetCurrentLength(*self.0) } }

    pub fn as_constraint(&self) -> Constraint { Constraint(self.0.clone().cast()) }
    pub fn raw(&self) -> *mut JPC_PulleyConstraint { *self.0 }
}

// --- path constraint ---

/// See also: Jolt's [`PathConstraint`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_path_constraint.html) class.
pub struct PathConstraint(pub(crate) Ref<JPC_PathConstraint>);

impl PathConstraint {
    pub fn create(settings: &JPC_PathConstraintSettings, body1: &Body<'_>, body2: &Body<'_>) -> Self {
        unsafe {
            let raw = JPC_PathConstraintSettings_Create(settings, body1.raw(), body2.raw());
            Self(Ref::from_active(raw))
        }
    }

    pub fn path_fraction(&self) -> f32 { unsafe { JPC_PathConstraint_GetPathFraction(*self.0) } }
    pub fn max_friction_force(&self) -> f32 { unsafe { JPC_PathConstraint_GetMaxFrictionForce(*self.0) } }
    pub fn set_max_friction_force(&self, force: f32) { unsafe { JPC_PathConstraint_SetMaxFrictionForce(*self.0, force) } }

    pub fn as_constraint(&self) -> Constraint { Constraint(self.0.clone().cast()) }
    pub fn raw(&self) -> *mut JPC_PathConstraint { *self.0 }
}
