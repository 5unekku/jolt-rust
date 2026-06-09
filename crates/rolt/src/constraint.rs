use joltc_sys::*;

use crate::{Body, FromJolt, IntoJolt, IntoRolt, Mat4, MotorSettings, Quat, Ref, RefConst, RVec3, SpringSettings, Vec3};

// --- constraint settings ---

/// See also: Jolt's [`FixedConstraintSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/struct_fixed_constraint_settings.html) struct.
pub struct FixedConstraintSettings(pub JPC_FixedConstraintSettings);

impl FixedConstraintSettings {
    pub fn space(&self) -> JPC_ConstraintSpace { self.0.Space }
    pub fn set_space(&mut self, space: JPC_ConstraintSpace) { self.0.Space = space; }
    pub fn auto_detect_point(&self) -> bool { self.0.AutoDetectPoint }
    pub fn set_auto_detect_point(&mut self, v: bool) { self.0.AutoDetectPoint = v; }
    pub fn point1(&self) -> RVec3 { RVec3::from_jolt(self.0.Point1) }
    pub fn set_point1(&mut self, v: RVec3) { self.0.Point1 = v.into_jolt(); }
    pub fn axis_x1(&self) -> Vec3 { Vec3::from_jolt(self.0.AxisX1) }
    pub fn set_axis_x1(&mut self, v: Vec3) { self.0.AxisX1 = v.into_jolt(); }
    pub fn axis_y1(&self) -> Vec3 { Vec3::from_jolt(self.0.AxisY1) }
    pub fn set_axis_y1(&mut self, v: Vec3) { self.0.AxisY1 = v.into_jolt(); }
    pub fn point2(&self) -> RVec3 { RVec3::from_jolt(self.0.Point2) }
    pub fn set_point2(&mut self, v: RVec3) { self.0.Point2 = v.into_jolt(); }
    pub fn axis_x2(&self) -> Vec3 { Vec3::from_jolt(self.0.AxisX2) }
    pub fn set_axis_x2(&mut self, v: Vec3) { self.0.AxisX2 = v.into_jolt(); }
    pub fn axis_y2(&self) -> Vec3 { Vec3::from_jolt(self.0.AxisY2) }
    pub fn set_axis_y2(&mut self, v: Vec3) { self.0.AxisY2 = v.into_jolt(); }
}

impl Default for FixedConstraintSettings {
    fn default() -> Self {
        let mut raw = unsafe { std::mem::zeroed::<JPC_FixedConstraintSettings>() };
        unsafe { JPC_FixedConstraintSettings_default(&mut raw) };
        Self(raw)
    }
}

/// See also: Jolt's [`HingeConstraintSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/struct_hinge_constraint_settings.html) struct.
pub struct HingeConstraintSettings(pub JPC_HingeConstraintSettings);

impl HingeConstraintSettings {
    pub fn space(&self) -> JPC_ConstraintSpace { self.0.Space }
    pub fn set_space(&mut self, space: JPC_ConstraintSpace) { self.0.Space = space; }
    pub fn point1(&self) -> RVec3 { RVec3::from_jolt(self.0.Point1) }
    pub fn set_point1(&mut self, v: RVec3) { self.0.Point1 = v.into_jolt(); }
    pub fn hinge_axis1(&self) -> Vec3 { Vec3::from_jolt(self.0.HingeAxis1) }
    pub fn set_hinge_axis1(&mut self, v: Vec3) { self.0.HingeAxis1 = v.into_jolt(); }
    pub fn normal_axis1(&self) -> Vec3 { Vec3::from_jolt(self.0.NormalAxis1) }
    pub fn set_normal_axis1(&mut self, v: Vec3) { self.0.NormalAxis1 = v.into_jolt(); }
    pub fn point2(&self) -> RVec3 { RVec3::from_jolt(self.0.Point2) }
    pub fn set_point2(&mut self, v: RVec3) { self.0.Point2 = v.into_jolt(); }
    pub fn hinge_axis2(&self) -> Vec3 { Vec3::from_jolt(self.0.HingeAxis2) }
    pub fn set_hinge_axis2(&mut self, v: Vec3) { self.0.HingeAxis2 = v.into_jolt(); }
    pub fn normal_axis2(&self) -> Vec3 { Vec3::from_jolt(self.0.NormalAxis2) }
    pub fn set_normal_axis2(&mut self, v: Vec3) { self.0.NormalAxis2 = v.into_jolt(); }
    pub fn limits_min(&self) -> f32 { self.0.LimitsMin }
    pub fn set_limits_min(&mut self, v: f32) { self.0.LimitsMin = v; }
    pub fn limits_max(&self) -> f32 { self.0.LimitsMax }
    pub fn set_limits_max(&mut self, v: f32) { self.0.LimitsMax = v; }
    pub fn limits_spring_settings(&self) -> SpringSettings { SpringSettings(self.0.LimitsSpringSettings) }
    pub fn set_limits_spring_settings(&mut self, s: SpringSettings) { self.0.LimitsSpringSettings = s.0; }
    pub fn max_friction_torque(&self) -> f32 { self.0.MaxFrictionTorque }
    pub fn set_max_friction_torque(&mut self, v: f32) { self.0.MaxFrictionTorque = v; }
    pub fn motor_settings(&self) -> MotorSettings { MotorSettings(self.0.MotorSettings) }
    pub fn set_motor_settings(&mut self, s: MotorSettings) { self.0.MotorSettings = s.0; }
}

impl Default for HingeConstraintSettings {
    fn default() -> Self {
        let mut raw = unsafe { std::mem::zeroed::<JPC_HingeConstraintSettings>() };
        unsafe { JPC_HingeConstraintSettings_default(&mut raw) };
        Self(raw)
    }
}

/// See also: Jolt's [`SliderConstraintSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/struct_slider_constraint_settings.html) struct.
pub struct SliderConstraintSettings(pub JPC_SliderConstraintSettings);

impl SliderConstraintSettings {
    pub fn space(&self) -> JPC_ConstraintSpace { self.0.Space }
    pub fn set_space(&mut self, space: JPC_ConstraintSpace) { self.0.Space = space; }
    pub fn auto_detect_point(&self) -> bool { self.0.AutoDetectPoint }
    pub fn set_auto_detect_point(&mut self, v: bool) { self.0.AutoDetectPoint = v; }
    pub fn point1(&self) -> RVec3 { RVec3::from_jolt(self.0.Point1) }
    pub fn set_point1(&mut self, v: RVec3) { self.0.Point1 = v.into_jolt(); }
    pub fn slider_axis1(&self) -> Vec3 { Vec3::from_jolt(self.0.SliderAxis1) }
    pub fn set_slider_axis1(&mut self, v: Vec3) { self.0.SliderAxis1 = v.into_jolt(); }
    pub fn normal_axis1(&self) -> Vec3 { Vec3::from_jolt(self.0.NormalAxis1) }
    pub fn set_normal_axis1(&mut self, v: Vec3) { self.0.NormalAxis1 = v.into_jolt(); }
    pub fn point2(&self) -> RVec3 { RVec3::from_jolt(self.0.Point2) }
    pub fn set_point2(&mut self, v: RVec3) { self.0.Point2 = v.into_jolt(); }
    pub fn slider_axis2(&self) -> Vec3 { Vec3::from_jolt(self.0.SliderAxis2) }
    pub fn set_slider_axis2(&mut self, v: Vec3) { self.0.SliderAxis2 = v.into_jolt(); }
    pub fn normal_axis2(&self) -> Vec3 { Vec3::from_jolt(self.0.NormalAxis2) }
    pub fn set_normal_axis2(&mut self, v: Vec3) { self.0.NormalAxis2 = v.into_jolt(); }
    pub fn limits_min(&self) -> f32 { self.0.LimitsMin }
    pub fn set_limits_min(&mut self, v: f32) { self.0.LimitsMin = v; }
    pub fn limits_max(&self) -> f32 { self.0.LimitsMax }
    pub fn set_limits_max(&mut self, v: f32) { self.0.LimitsMax = v; }
    pub fn limits_spring_settings(&self) -> SpringSettings { SpringSettings(self.0.LimitsSpringSettings) }
    pub fn set_limits_spring_settings(&mut self, s: SpringSettings) { self.0.LimitsSpringSettings = s.0; }
    pub fn max_friction_force(&self) -> f32 { self.0.MaxFrictionForce }
    pub fn set_max_friction_force(&mut self, v: f32) { self.0.MaxFrictionForce = v; }
    pub fn motor_settings(&self) -> MotorSettings { MotorSettings(self.0.MotorSettings) }
    pub fn set_motor_settings(&mut self, s: MotorSettings) { self.0.MotorSettings = s.0; }
}

impl Default for SliderConstraintSettings {
    fn default() -> Self {
        let mut raw = unsafe { std::mem::zeroed::<JPC_SliderConstraintSettings>() };
        unsafe { JPC_SliderConstraintSettings_default(&mut raw) };
        Self(raw)
    }
}

/// See also: Jolt's [`DistanceConstraintSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/struct_distance_constraint_settings.html) struct.
pub struct DistanceConstraintSettings(pub JPC_DistanceConstraintSettings);

impl DistanceConstraintSettings {
    pub fn space(&self) -> JPC_ConstraintSpace { self.0.Space }
    pub fn set_space(&mut self, space: JPC_ConstraintSpace) { self.0.Space = space; }
    pub fn point1(&self) -> RVec3 { RVec3::from_jolt(self.0.Point1) }
    pub fn set_point1(&mut self, v: RVec3) { self.0.Point1 = v.into_jolt(); }
    pub fn point2(&self) -> RVec3 { RVec3::from_jolt(self.0.Point2) }
    pub fn set_point2(&mut self, v: RVec3) { self.0.Point2 = v.into_jolt(); }
    pub fn min_distance(&self) -> f32 { self.0.MinDistance }
    pub fn set_min_distance(&mut self, v: f32) { self.0.MinDistance = v; }
    pub fn max_distance(&self) -> f32 { self.0.MaxDistance }
    pub fn set_max_distance(&mut self, v: f32) { self.0.MaxDistance = v; }
    pub fn limits_spring_settings(&self) -> SpringSettings { SpringSettings(self.0.LimitsSpringSettings) }
    pub fn set_limits_spring_settings(&mut self, s: SpringSettings) { self.0.LimitsSpringSettings = s.0; }
}

impl Default for DistanceConstraintSettings {
    fn default() -> Self {
        let mut raw = unsafe { std::mem::zeroed::<JPC_DistanceConstraintSettings>() };
        unsafe { JPC_DistanceConstraintSettings_default(&mut raw) };
        Self(raw)
    }
}

/// See also: Jolt's [`SixDOFConstraintSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/struct_six_d_o_f_constraint_settings.html) struct.
pub struct SixDofConstraintSettings(pub JPC_SixDOFConstraintSettings);

impl SixDofConstraintSettings {
    pub fn space(&self) -> JPC_ConstraintSpace { self.0.Space }
    pub fn set_space(&mut self, space: JPC_ConstraintSpace) { self.0.Space = space; }
    pub fn position1(&self) -> RVec3 { RVec3::from_jolt(self.0.Position1) }
    pub fn set_position1(&mut self, v: RVec3) { self.0.Position1 = v.into_jolt(); }
    pub fn axis_x1(&self) -> Vec3 { Vec3::from_jolt(self.0.AxisX1) }
    pub fn set_axis_x1(&mut self, v: Vec3) { self.0.AxisX1 = v.into_jolt(); }
    pub fn axis_y1(&self) -> Vec3 { Vec3::from_jolt(self.0.AxisY1) }
    pub fn set_axis_y1(&mut self, v: Vec3) { self.0.AxisY1 = v.into_jolt(); }
    pub fn position2(&self) -> RVec3 { RVec3::from_jolt(self.0.Position2) }
    pub fn set_position2(&mut self, v: RVec3) { self.0.Position2 = v.into_jolt(); }
    pub fn axis_x2(&self) -> Vec3 { Vec3::from_jolt(self.0.AxisX2) }
    pub fn set_axis_x2(&mut self, v: Vec3) { self.0.AxisX2 = v.into_jolt(); }
    pub fn axis_y2(&self) -> Vec3 { Vec3::from_jolt(self.0.AxisY2) }
    pub fn set_axis_y2(&mut self, v: Vec3) { self.0.AxisY2 = v.into_jolt(); }
    pub fn max_friction(&self) -> &[f32; 6] { &self.0.MaxFriction }
    pub fn set_max_friction(&mut self, v: [f32; 6]) { self.0.MaxFriction = v; }
    pub fn limit_min(&self) -> &[f32; 6] { &self.0.LimitMin }
    pub fn set_limit_min(&mut self, v: [f32; 6]) { self.0.LimitMin = v; }
    pub fn limit_max(&self) -> &[f32; 6] { &self.0.LimitMax }
    pub fn set_limit_max(&mut self, v: [f32; 6]) { self.0.LimitMax = v; }
}

impl Default for SixDofConstraintSettings {
    fn default() -> Self {
        let mut raw = unsafe { std::mem::zeroed::<JPC_SixDOFConstraintSettings>() };
        unsafe { JPC_SixDOFConstraintSettings_default(&mut raw) };
        Self(raw)
    }
}

/// See also: Jolt's [`PointConstraintSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/struct_point_constraint_settings.html) struct.
pub struct PointConstraintSettings(pub JPC_PointConstraintSettings);

impl PointConstraintSettings {
    pub fn space(&self) -> JPC_ConstraintSpace { self.0.Space }
    pub fn set_space(&mut self, space: JPC_ConstraintSpace) { self.0.Space = space; }
    pub fn point1(&self) -> RVec3 { RVec3::from_jolt(self.0.Point1) }
    pub fn set_point1(&mut self, v: RVec3) { self.0.Point1 = v.into_jolt(); }
    pub fn point2(&self) -> RVec3 { RVec3::from_jolt(self.0.Point2) }
    pub fn set_point2(&mut self, v: RVec3) { self.0.Point2 = v.into_jolt(); }
}

impl Default for PointConstraintSettings {
    fn default() -> Self {
        let mut raw = unsafe { std::mem::zeroed::<JPC_PointConstraintSettings>() };
        unsafe { JPC_PointConstraintSettings_default(&mut raw) };
        Self(raw)
    }
}

/// See also: Jolt's [`ConeConstraintSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/struct_cone_constraint_settings.html) struct.
pub struct ConeConstraintSettings(pub JPC_ConeConstraintSettings);

impl ConeConstraintSettings {
    pub fn space(&self) -> JPC_ConstraintSpace { self.0.Space }
    pub fn set_space(&mut self, space: JPC_ConstraintSpace) { self.0.Space = space; }
    pub fn point1(&self) -> RVec3 { RVec3::from_jolt(self.0.Point1) }
    pub fn set_point1(&mut self, v: RVec3) { self.0.Point1 = v.into_jolt(); }
    pub fn twist_axis1(&self) -> Vec3 { Vec3::from_jolt(self.0.TwistAxis1) }
    pub fn set_twist_axis1(&mut self, v: Vec3) { self.0.TwistAxis1 = v.into_jolt(); }
    pub fn point2(&self) -> RVec3 { RVec3::from_jolt(self.0.Point2) }
    pub fn set_point2(&mut self, v: RVec3) { self.0.Point2 = v.into_jolt(); }
    pub fn twist_axis2(&self) -> Vec3 { Vec3::from_jolt(self.0.TwistAxis2) }
    pub fn set_twist_axis2(&mut self, v: Vec3) { self.0.TwistAxis2 = v.into_jolt(); }
    pub fn half_cone_angle(&self) -> f32 { self.0.HalfConeAngle }
    pub fn set_half_cone_angle(&mut self, v: f32) { self.0.HalfConeAngle = v; }
}

impl Default for ConeConstraintSettings {
    fn default() -> Self {
        let mut raw = unsafe { std::mem::zeroed::<JPC_ConeConstraintSettings>() };
        unsafe { JPC_ConeConstraintSettings_default(&mut raw) };
        Self(raw)
    }
}

/// See also: Jolt's [`PulleyConstraintSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/struct_pulley_constraint_settings.html) struct.
pub struct PulleyConstraintSettings(pub JPC_PulleyConstraintSettings);

impl PulleyConstraintSettings {
    pub fn space(&self) -> JPC_ConstraintSpace { self.0.Space }
    pub fn set_space(&mut self, space: JPC_ConstraintSpace) { self.0.Space = space; }
    pub fn body_point1(&self) -> RVec3 { RVec3::from_jolt(self.0.BodyPoint1) }
    pub fn set_body_point1(&mut self, v: RVec3) { self.0.BodyPoint1 = v.into_jolt(); }
    pub fn fixed_point1(&self) -> RVec3 { RVec3::from_jolt(self.0.FixedPoint1) }
    pub fn set_fixed_point1(&mut self, v: RVec3) { self.0.FixedPoint1 = v.into_jolt(); }
    pub fn body_point2(&self) -> RVec3 { RVec3::from_jolt(self.0.BodyPoint2) }
    pub fn set_body_point2(&mut self, v: RVec3) { self.0.BodyPoint2 = v.into_jolt(); }
    pub fn fixed_point2(&self) -> RVec3 { RVec3::from_jolt(self.0.FixedPoint2) }
    pub fn set_fixed_point2(&mut self, v: RVec3) { self.0.FixedPoint2 = v.into_jolt(); }
    pub fn ratio(&self) -> f32 { self.0.Ratio }
    pub fn set_ratio(&mut self, v: f32) { self.0.Ratio = v; }
    pub fn min_length(&self) -> f32 { self.0.MinLength }
    pub fn set_min_length(&mut self, v: f32) { self.0.MinLength = v; }
    pub fn max_length(&self) -> f32 { self.0.MaxLength }
    pub fn set_max_length(&mut self, v: f32) { self.0.MaxLength = v; }
}

impl Default for PulleyConstraintSettings {
    fn default() -> Self {
        let mut raw = unsafe { std::mem::zeroed::<JPC_PulleyConstraintSettings>() };
        unsafe { JPC_PulleyConstraintSettings_default(&mut raw) };
        Self(raw)
    }
}

/// See also: Jolt's [`SwingTwistConstraintSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/struct_swing_twist_constraint_settings.html) struct.
pub struct SwingTwistConstraintSettings(pub JPC_SwingTwistConstraintSettings);

impl SwingTwistConstraintSettings {
    pub fn space(&self) -> JPC_ConstraintSpace { self.0.Space }
    pub fn set_space(&mut self, space: JPC_ConstraintSpace) { self.0.Space = space; }
    pub fn position1(&self) -> RVec3 { RVec3::from_jolt(self.0.Position1) }
    pub fn set_position1(&mut self, v: RVec3) { self.0.Position1 = v.into_jolt(); }
    pub fn twist_axis1(&self) -> Vec3 { Vec3::from_jolt(self.0.TwistAxis1) }
    pub fn set_twist_axis1(&mut self, v: Vec3) { self.0.TwistAxis1 = v.into_jolt(); }
    pub fn plane_axis1(&self) -> Vec3 { Vec3::from_jolt(self.0.PlaneAxis1) }
    pub fn set_plane_axis1(&mut self, v: Vec3) { self.0.PlaneAxis1 = v.into_jolt(); }
    pub fn position2(&self) -> RVec3 { RVec3::from_jolt(self.0.Position2) }
    pub fn set_position2(&mut self, v: RVec3) { self.0.Position2 = v.into_jolt(); }
    pub fn twist_axis2(&self) -> Vec3 { Vec3::from_jolt(self.0.TwistAxis2) }
    pub fn set_twist_axis2(&mut self, v: Vec3) { self.0.TwistAxis2 = v.into_jolt(); }
    pub fn plane_axis2(&self) -> Vec3 { Vec3::from_jolt(self.0.PlaneAxis2) }
    pub fn set_plane_axis2(&mut self, v: Vec3) { self.0.PlaneAxis2 = v.into_jolt(); }
    pub fn swing_type(&self) -> JPC_SwingType { self.0.SwingType }
    pub fn set_swing_type(&mut self, v: JPC_SwingType) { self.0.SwingType = v; }
    pub fn normal_half_cone_angle(&self) -> f32 { self.0.NormalHalfConeAngle }
    pub fn set_normal_half_cone_angle(&mut self, v: f32) { self.0.NormalHalfConeAngle = v; }
    pub fn plane_half_cone_angle(&self) -> f32 { self.0.PlaneHalfConeAngle }
    pub fn set_plane_half_cone_angle(&mut self, v: f32) { self.0.PlaneHalfConeAngle = v; }
    pub fn twist_min_angle(&self) -> f32 { self.0.TwistMinAngle }
    pub fn set_twist_min_angle(&mut self, v: f32) { self.0.TwistMinAngle = v; }
    pub fn twist_max_angle(&self) -> f32 { self.0.TwistMaxAngle }
    pub fn set_twist_max_angle(&mut self, v: f32) { self.0.TwistMaxAngle = v; }
    pub fn max_friction_torque(&self) -> f32 { self.0.MaxFrictionTorque }
    pub fn set_max_friction_torque(&mut self, v: f32) { self.0.MaxFrictionTorque = v; }
    pub fn swing_motor_settings(&self) -> MotorSettings { MotorSettings(self.0.SwingMotorSettings) }
    pub fn set_swing_motor_settings(&mut self, s: MotorSettings) { self.0.SwingMotorSettings = s.0; }
    pub fn twist_motor_settings(&self) -> MotorSettings { MotorSettings(self.0.TwistMotorSettings) }
    pub fn set_twist_motor_settings(&mut self, s: MotorSettings) { self.0.TwistMotorSettings = s.0; }
}

impl Default for SwingTwistConstraintSettings {
    fn default() -> Self {
        let mut raw = unsafe { std::mem::zeroed::<JPC_SwingTwistConstraintSettings>() };
        unsafe { JPC_SwingTwistConstraintSettings_default(&mut raw) };
        Self(raw)
    }
}

/// See also: Jolt's [`PathConstraintSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/struct_path_constraint_settings.html) struct.
pub struct PathConstraintSettings(pub JPC_PathConstraintSettings);

impl PathConstraintSettings {
    pub fn path(&self) -> *mut JPC_PathConstraintPath { self.0.Path }
    pub fn set_path(&mut self, path: Option<&Ref<JPC_PathConstraintPath>>) {
        self.0.Path = path.map_or(std::ptr::null_mut(), |r| r.get());
    }
    pub fn path_position(&self) -> Vec3 { Vec3::from_jolt(self.0.PathPosition) }
    pub fn set_path_position(&mut self, v: Vec3) { self.0.PathPosition = v.into_jolt(); }
    pub fn path_rotation(&self) -> Quat { Quat::from_jolt(self.0.PathRotation) }
    pub fn set_path_rotation(&mut self, v: Quat) { self.0.PathRotation = v.into_jolt(); }
    pub fn path_fraction(&self) -> f32 { self.0.PathFraction }
    pub fn set_path_fraction(&mut self, v: f32) { self.0.PathFraction = v; }
    pub fn max_friction_force(&self) -> f32 { self.0.MaxFrictionForce }
    pub fn set_max_friction_force(&mut self, v: f32) { self.0.MaxFrictionForce = v; }
    pub fn position_motor_settings(&self) -> MotorSettings { MotorSettings(self.0.PositionMotorSettings) }
    pub fn set_position_motor_settings(&mut self, s: MotorSettings) { self.0.PositionMotorSettings = s.0; }
    pub fn rotation_constraint_type(&self) -> JPC_PathRotationConstraintType { self.0.RotationConstraintType }
    pub fn set_rotation_constraint_type(&mut self, v: JPC_PathRotationConstraintType) { self.0.RotationConstraintType = v; }
}

impl Default for PathConstraintSettings {
    fn default() -> Self {
        let mut raw = unsafe { std::mem::zeroed::<JPC_PathConstraintSettings>() };
        unsafe { JPC_PathConstraintSettings_default(&mut raw) };
        Self(raw)
    }
}

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

    pub fn notify_shape_changed(&self, body_id: crate::BodyId, delta_com: Vec3) {
        unsafe { JPC_Constraint_NotifyShapeChanged(*self.0, body_id.raw(), delta_com.into_jolt()) }
    }

    pub fn constraint_settings_base(&self) -> JPC_ConstraintSettings {
        unsafe {
            let obj = JPC_Constraint_GetConstraintSettings(*self.0);
            let base = JPC_ConstraintSettingsObj_GetBaseSettings(obj);
            JPC_ConstraintSettingsObj_Release(obj);
            base
        }
    }

    pub fn with_raw<R>(&self, f: impl FnOnce(*mut JPC_Constraint) -> R) -> R {
        self.0.with_raw(f)
    }

    pub fn raw(&self) -> *mut JPC_Constraint { *self.0 }
}

// --- fixed constraint ---

/// See also: Jolt's [`FixedConstraint`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_fixed_constraint.html) class.
pub struct FixedConstraint(pub(crate) Ref<JPC_Constraint>);

impl FixedConstraint {
    pub fn create(settings: &FixedConstraintSettings, body1: &Body<'_>, body2: &Body<'_>) -> Self {
        unsafe {
            let raw = JPC_FixedConstraintSettings_Create(&settings.0, body1.raw(), body2.raw());
            Self(Ref::from_active(raw))
        }
    }

    pub fn total_lambda_position(&self) -> Vec3 {
        unsafe { JPC_FixedConstraint_GetTotalLambdaPosition((*self.0).cast::<JPC_FixedConstraint>()).into_rolt() }
    }

    pub fn total_lambda_rotation(&self) -> Vec3 {
        unsafe { JPC_FixedConstraint_GetTotalLambdaRotation((*self.0).cast::<JPC_FixedConstraint>()).into_rolt() }
    }

    pub fn body1(&self) -> Body<'_> {
        unsafe { Body::new(JPC_TwoBodyConstraint_GetBody1((*self.0).cast::<JPC_TwoBodyConstraint>())) }
    }

    pub fn body2(&self) -> Body<'_> {
        unsafe { Body::new(JPC_TwoBodyConstraint_GetBody2((*self.0).cast::<JPC_TwoBodyConstraint>())) }
    }

    pub fn constraint_to_body1_matrix(&self) -> Mat4 {
        unsafe { JPC_TwoBodyConstraint_GetConstraintToBody1Matrix((*self.0).cast::<JPC_TwoBodyConstraint>()).into_rolt() }
    }

    pub fn constraint_to_body2_matrix(&self) -> Mat4 {
        unsafe { JPC_TwoBodyConstraint_GetConstraintToBody2Matrix((*self.0).cast::<JPC_TwoBodyConstraint>()).into_rolt() }
    }

    pub fn as_constraint(&self) -> Constraint { Constraint(self.0.clone()) }

    pub fn with_raw<R>(&self, f: impl FnOnce(*mut JPC_Constraint) -> R) -> R {
        self.0.with_raw(f)
    }

    pub fn raw(&self) -> *mut JPC_Constraint { *self.0 }
}

// --- distance constraint ---

/// See also: Jolt's [`DistanceConstraint`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_distance_constraint.html) class.
pub struct DistanceConstraint(pub(crate) Ref<JPC_DistanceConstraint>);

impl DistanceConstraint {
    pub fn create(settings: &DistanceConstraintSettings, body1: &Body<'_>, body2: &Body<'_>) -> Self {
        unsafe {
            let raw = JPC_DistanceConstraintSettings_Create(&settings.0, body1.raw(), body2.raw());
            Self(Ref::from_active(raw))
        }
    }

    pub fn set_distance(&self, min_distance: f32, max_distance: f32) {
        unsafe { JPC_DistanceConstraint_SetDistance(*self.0, min_distance, max_distance) }
    }

    pub fn min_distance(&self) -> f32 { unsafe { JPC_DistanceConstraint_GetMinDistance(*self.0) } }
    pub fn max_distance(&self) -> f32 { unsafe { JPC_DistanceConstraint_GetMaxDistance(*self.0) } }
    pub fn total_lambda_position(&self) -> f32 { unsafe { JPC_DistanceConstraint_GetTotalLambdaPosition(*self.0) } }
    pub fn limits_spring_settings(&self) -> SpringSettings { unsafe { SpringSettings(JPC_DistanceConstraint_GetLimitsSpringSettings(*self.0)) } }
    pub fn set_limits_spring_settings(&self, settings: &SpringSettings) { unsafe { JPC_DistanceConstraint_SetLimitsSpringSettings(*self.0, &settings.0) } }

    pub fn body1(&self) -> Body<'_> {
        unsafe { Body::new(JPC_TwoBodyConstraint_GetBody1((*self.0).cast::<JPC_TwoBodyConstraint>())) }
    }

    pub fn body2(&self) -> Body<'_> {
        unsafe { Body::new(JPC_TwoBodyConstraint_GetBody2((*self.0).cast::<JPC_TwoBodyConstraint>())) }
    }

    pub fn constraint_to_body1_matrix(&self) -> Mat4 {
        unsafe { JPC_TwoBodyConstraint_GetConstraintToBody1Matrix((*self.0).cast::<JPC_TwoBodyConstraint>()).into_rolt() }
    }

    pub fn constraint_to_body2_matrix(&self) -> Mat4 {
        unsafe { JPC_TwoBodyConstraint_GetConstraintToBody2Matrix((*self.0).cast::<JPC_TwoBodyConstraint>()).into_rolt() }
    }

    pub fn as_constraint(&self) -> Constraint { Constraint(self.0.clone().cast()) }

    pub fn with_raw<R>(&self, f: impl FnOnce(*mut JPC_DistanceConstraint) -> R) -> R {
        self.0.with_raw(f)
    }

    pub fn raw(&self) -> *mut JPC_DistanceConstraint { *self.0 }
}

// --- hinge constraint ---

/// See also: Jolt's [`HingeConstraint`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_hinge_constraint.html) class.
pub struct HingeConstraint(pub(crate) Ref<JPC_HingeConstraint>);

impl HingeConstraint {
    pub fn create(settings: &HingeConstraintSettings, body1: &Body<'_>, body2: &Body<'_>) -> Self {
        unsafe {
            let raw = JPC_HingeConstraintSettings_Create(&settings.0, body1.raw(), body2.raw());
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
    pub fn total_lambda_position(&self) -> Vec3 { unsafe { JPC_HingeConstraint_GetTotalLambdaPosition(*self.0).into_rolt() } }
    pub fn total_lambda_rotation(&self) -> [f32; 2] {
        let v = unsafe { JPC_HingeConstraint_GetTotalLambdaRotation(*self.0) };
        [v.x, v.y]
    }
    pub fn total_lambda_rotation_limits(&self) -> f32 { unsafe { JPC_HingeConstraint_GetTotalLambdaRotationLimits(*self.0) } }
    pub fn total_lambda_motor(&self) -> f32 { unsafe { JPC_HingeConstraint_GetTotalLambdaMotor(*self.0) } }
    pub fn local_space_point1(&self) -> Vec3 { unsafe { JPC_HingeConstraint_GetLocalSpacePoint1(*self.0).into_rolt() } }
    pub fn local_space_point2(&self) -> Vec3 { unsafe { JPC_HingeConstraint_GetLocalSpacePoint2(*self.0).into_rolt() } }
    pub fn local_space_hinge_axis1(&self) -> Vec3 { unsafe { JPC_HingeConstraint_GetLocalSpaceHingeAxis1(*self.0).into_rolt() } }
    pub fn local_space_hinge_axis2(&self) -> Vec3 { unsafe { JPC_HingeConstraint_GetLocalSpaceHingeAxis2(*self.0).into_rolt() } }
    pub fn local_space_normal_axis1(&self) -> Vec3 { unsafe { JPC_HingeConstraint_GetLocalSpaceNormalAxis1(*self.0).into_rolt() } }
    pub fn local_space_normal_axis2(&self) -> Vec3 { unsafe { JPC_HingeConstraint_GetLocalSpaceNormalAxis2(*self.0).into_rolt() } }
    pub fn motor_settings(&self) -> MotorSettings { unsafe { MotorSettings(JPC_HingeConstraint_GetMotorSettings(*self.0)) } }
    pub fn set_motor_settings(&self, settings: &MotorSettings) { unsafe { JPC_HingeConstraint_SetMotorSettings(*self.0, &settings.0) } }
    pub fn set_target_orientation_bs(&self, orientation: Quat) { unsafe { JPC_HingeConstraint_SetTargetOrientationBS(*self.0, orientation.into_jolt()) } }
    pub fn limits_spring_settings(&self) -> SpringSettings { unsafe { SpringSettings(JPC_HingeConstraint_GetLimitsSpringSettings(*self.0)) } }
    pub fn set_limits_spring_settings(&self, settings: &SpringSettings) { unsafe { JPC_HingeConstraint_SetLimitsSpringSettings(*self.0, &settings.0) } }

    pub fn body1(&self) -> Body<'_> {
        unsafe { Body::new(JPC_TwoBodyConstraint_GetBody1((*self.0).cast::<JPC_TwoBodyConstraint>())) }
    }

    pub fn body2(&self) -> Body<'_> {
        unsafe { Body::new(JPC_TwoBodyConstraint_GetBody2((*self.0).cast::<JPC_TwoBodyConstraint>())) }
    }

    pub fn constraint_to_body1_matrix(&self) -> Mat4 {
        unsafe { JPC_TwoBodyConstraint_GetConstraintToBody1Matrix((*self.0).cast::<JPC_TwoBodyConstraint>()).into_rolt() }
    }

    pub fn constraint_to_body2_matrix(&self) -> Mat4 {
        unsafe { JPC_TwoBodyConstraint_GetConstraintToBody2Matrix((*self.0).cast::<JPC_TwoBodyConstraint>()).into_rolt() }
    }

    pub fn as_constraint(&self) -> Constraint { Constraint(self.0.clone().cast()) }

    pub fn with_raw<R>(&self, f: impl FnOnce(*mut JPC_HingeConstraint) -> R) -> R {
        self.0.with_raw(f)
    }

    pub fn raw(&self) -> *mut JPC_HingeConstraint { *self.0 }
}

// --- slider constraint ---

/// See also: Jolt's [`SliderConstraint`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_slider_constraint.html) class.
pub struct SliderConstraint(pub(crate) Ref<JPC_SliderConstraint>);

impl SliderConstraint {
    pub fn create(settings: &SliderConstraintSettings, body1: &Body<'_>, body2: &Body<'_>) -> Self {
        unsafe {
            let raw = JPC_SliderConstraintSettings_Create(&settings.0, body1.raw(), body2.raw());
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
    pub fn total_lambda_position(&self) -> [f32; 2] {
        let v = unsafe { JPC_SliderConstraint_GetTotalLambdaPosition(*self.0) };
        [v.x, v.y]
    }
    pub fn total_lambda_position_limits(&self) -> f32 { unsafe { JPC_SliderConstraint_GetTotalLambdaPositionLimits(*self.0) } }
    pub fn total_lambda_rotation(&self) -> Vec3 { unsafe { JPC_SliderConstraint_GetTotalLambdaRotation(*self.0).into_rolt() } }
    pub fn total_lambda_motor(&self) -> f32 { unsafe { JPC_SliderConstraint_GetTotalLambdaMotor(*self.0) } }
    pub fn motor_settings(&self) -> MotorSettings { unsafe { MotorSettings(JPC_SliderConstraint_GetMotorSettings(*self.0)) } }
    pub fn set_motor_settings(&self, settings: &MotorSettings) { unsafe { JPC_SliderConstraint_SetMotorSettings(*self.0, &settings.0) } }
    pub fn limits_spring_settings(&self) -> SpringSettings { unsafe { SpringSettings(JPC_SliderConstraint_GetLimitsSpringSettings(*self.0)) } }
    pub fn set_limits_spring_settings(&self, settings: &SpringSettings) { unsafe { JPC_SliderConstraint_SetLimitsSpringSettings(*self.0, &settings.0) } }

    pub fn body1(&self) -> Body<'_> {
        unsafe { Body::new(JPC_TwoBodyConstraint_GetBody1((*self.0).cast::<JPC_TwoBodyConstraint>())) }
    }

    pub fn body2(&self) -> Body<'_> {
        unsafe { Body::new(JPC_TwoBodyConstraint_GetBody2((*self.0).cast::<JPC_TwoBodyConstraint>())) }
    }

    pub fn constraint_to_body1_matrix(&self) -> Mat4 {
        unsafe { JPC_TwoBodyConstraint_GetConstraintToBody1Matrix((*self.0).cast::<JPC_TwoBodyConstraint>()).into_rolt() }
    }

    pub fn constraint_to_body2_matrix(&self) -> Mat4 {
        unsafe { JPC_TwoBodyConstraint_GetConstraintToBody2Matrix((*self.0).cast::<JPC_TwoBodyConstraint>()).into_rolt() }
    }

    pub fn as_constraint(&self) -> Constraint { Constraint(self.0.clone().cast()) }

    pub fn with_raw<R>(&self, f: impl FnOnce(*mut JPC_SliderConstraint) -> R) -> R {
        self.0.with_raw(f)
    }

    pub fn raw(&self) -> *mut JPC_SliderConstraint { *self.0 }
}

// --- point constraint ---

/// See also: Jolt's [`PointConstraint`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_point_constraint.html) class.
pub struct PointConstraint(pub(crate) Ref<JPC_PointConstraint>);

impl PointConstraint {
    pub fn create(settings: &PointConstraintSettings, body1: &Body<'_>, body2: &Body<'_>) -> Self {
        unsafe {
            let raw = JPC_PointConstraintSettings_Create(&settings.0, body1.raw(), body2.raw());
            Self(Ref::from_active(raw))
        }
    }

    pub fn local_space_point1(&self) -> Vec3 { unsafe { JPC_PointConstraint_GetLocalSpacePoint1(*self.0).into_rolt() } }
    pub fn local_space_point2(&self) -> Vec3 { unsafe { JPC_PointConstraint_GetLocalSpacePoint2(*self.0).into_rolt() } }
    pub fn set_point1(&self, space: JPC_ConstraintSpace, point: RVec3) { unsafe { JPC_PointConstraint_SetPoint1(*self.0, space, point.into_jolt()) } }
    pub fn set_point2(&self, space: JPC_ConstraintSpace, point: RVec3) { unsafe { JPC_PointConstraint_SetPoint2(*self.0, space, point.into_jolt()) } }
    pub fn total_lambda_position(&self) -> Vec3 { unsafe { JPC_PointConstraint_GetTotalLambdaPosition(*self.0).into_rolt() } }

    pub fn body1(&self) -> Body<'_> {
        unsafe { Body::new(JPC_TwoBodyConstraint_GetBody1((*self.0).cast::<JPC_TwoBodyConstraint>())) }
    }

    pub fn body2(&self) -> Body<'_> {
        unsafe { Body::new(JPC_TwoBodyConstraint_GetBody2((*self.0).cast::<JPC_TwoBodyConstraint>())) }
    }

    pub fn constraint_to_body1_matrix(&self) -> Mat4 {
        unsafe { JPC_TwoBodyConstraint_GetConstraintToBody1Matrix((*self.0).cast::<JPC_TwoBodyConstraint>()).into_rolt() }
    }

    pub fn constraint_to_body2_matrix(&self) -> Mat4 {
        unsafe { JPC_TwoBodyConstraint_GetConstraintToBody2Matrix((*self.0).cast::<JPC_TwoBodyConstraint>()).into_rolt() }
    }

    pub fn as_constraint(&self) -> Constraint { Constraint(self.0.clone().cast()) }

    pub fn with_raw<R>(&self, f: impl FnOnce(*mut JPC_PointConstraint) -> R) -> R {
        self.0.with_raw(f)
    }

    pub fn raw(&self) -> *mut JPC_PointConstraint { *self.0 }
}

// --- six DOF constraint ---

/// See also: Jolt's [`SixDOFConstraint`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_six_d_o_f_constraint.html) class.
pub struct SixDofConstraint(pub(crate) Ref<JPC_Constraint>);

impl SixDofConstraint {
    pub fn create(settings: &SixDofConstraintSettings, body1: &Body<'_>, body2: &Body<'_>) -> Self {
        unsafe {
            let raw = JPC_SixDOFConstraintSettings_Create(&settings.0, body1.raw(), body2.raw());
            Self(Ref::from_active(raw))
        }
    }

    fn dof(&self) -> *mut JPC_SixDOFConstraint { (*self.0).cast::<JPC_SixDOFConstraint>() }
    fn two_body(&self) -> *const JPC_TwoBodyConstraint { (*self.0).cast::<JPC_TwoBodyConstraint>() }

    pub fn translation_limits_min(&self) -> Vec3 {
        unsafe { JPC_SixDOFConstraint_GetTranslationLimitsMin(self.dof()).into_rolt() }
    }

    pub fn translation_limits_max(&self) -> Vec3 {
        unsafe { JPC_SixDOFConstraint_GetTranslationLimitsMax(self.dof()).into_rolt() }
    }

    pub fn set_translation_limits(&self, min: Vec3, max: Vec3) {
        unsafe { JPC_SixDOFConstraint_SetTranslationLimits(self.dof(), min.into_jolt(), max.into_jolt()) }
    }

    pub fn rotation_limits_min(&self) -> Vec3 {
        unsafe { JPC_SixDOFConstraint_GetRotationLimitsMin(self.dof()).into_rolt() }
    }

    pub fn rotation_limits_max(&self) -> Vec3 {
        unsafe { JPC_SixDOFConstraint_GetRotationLimitsMax(self.dof()).into_rolt() }
    }

    pub fn set_rotation_limits(&self, min: Vec3, max: Vec3) {
        unsafe { JPC_SixDOFConstraint_SetRotationLimits(self.dof(), min.into_jolt(), max.into_jolt()) }
    }

    pub fn limits_min(&self, axis: JPC_SixDOFConstraint_Axis) -> f32 {
        unsafe { JPC_SixDOFConstraint_GetLimitsMin(self.dof(), axis) }
    }

    pub fn limits_max(&self, axis: JPC_SixDOFConstraint_Axis) -> f32 {
        unsafe { JPC_SixDOFConstraint_GetLimitsMax(self.dof(), axis) }
    }

    pub fn is_free_axis(&self, axis: JPC_SixDOFConstraint_Axis) -> bool {
        unsafe { JPC_SixDOFConstraint_IsFreeAxis(self.dof(), axis) }
    }

    pub fn max_friction(&self, axis: JPC_SixDOFConstraint_Axis) -> f32 {
        unsafe { JPC_SixDOFConstraint_GetMaxFriction(self.dof(), axis) }
    }

    pub fn set_max_friction(&self, axis: JPC_SixDOFConstraint_Axis, friction: f32) {
        unsafe { JPC_SixDOFConstraint_SetMaxFriction(self.dof(), axis, friction) }
    }

    pub fn rotation_in_constraint_space(&self) -> Quat {
        unsafe { JPC_SixDOFConstraint_GetRotationInConstraintSpace(self.dof()).into_rolt() }
    }

    pub fn target_velocity_cs(&self) -> Vec3 {
        unsafe { JPC_SixDOFConstraint_GetTargetVelocityCS(self.dof()).into_rolt() }
    }

    pub fn set_target_velocity_cs(&self, velocity: Vec3) {
        unsafe { JPC_SixDOFConstraint_SetTargetVelocityCS(self.dof(), velocity.into_jolt()) }
    }

    pub fn target_angular_velocity_cs(&self) -> Vec3 {
        unsafe { JPC_SixDOFConstraint_GetTargetAngularVelocityCS(self.dof()).into_rolt() }
    }

    pub fn set_target_angular_velocity_cs(&self, velocity: Vec3) {
        unsafe { JPC_SixDOFConstraint_SetTargetAngularVelocityCS(self.dof(), velocity.into_jolt()) }
    }

    pub fn target_position_cs(&self) -> Vec3 {
        unsafe { JPC_SixDOFConstraint_GetTargetPositionCS(self.dof()).into_rolt() }
    }

    pub fn set_target_position_cs(&self, position: Vec3) {
        unsafe { JPC_SixDOFConstraint_SetTargetPositionCS(self.dof(), position.into_jolt()) }
    }

    pub fn target_orientation_cs(&self) -> Quat {
        unsafe { JPC_SixDOFConstraint_GetTargetOrientationCS(self.dof()).into_rolt() }
    }

    pub fn set_target_orientation_cs(&self, orientation: Quat) {
        unsafe { JPC_SixDOFConstraint_SetTargetOrientationCS(self.dof(), orientation.into_jolt()) }
    }

    pub fn set_target_orientation_bs(&self, orientation: Quat) {
        unsafe { JPC_SixDOFConstraint_SetTargetOrientationBS(self.dof(), orientation.into_jolt()) }
    }

    pub fn total_lambda_position(&self) -> Vec3 {
        unsafe { JPC_SixDOFConstraint_GetTotalLambdaPosition(self.dof()).into_rolt() }
    }

    pub fn total_lambda_rotation(&self) -> Vec3 {
        unsafe { JPC_SixDOFConstraint_GetTotalLambdaRotation(self.dof()).into_rolt() }
    }

    pub fn total_lambda_motor_translation(&self) -> Vec3 {
        unsafe { JPC_SixDOFConstraint_GetTotalLambdaMotorTranslation(self.dof()).into_rolt() }
    }

    pub fn total_lambda_motor_rotation(&self) -> Vec3 {
        unsafe { JPC_SixDOFConstraint_GetTotalLambdaMotorRotation(self.dof()).into_rolt() }
    }

    pub fn is_fixed_axis(&self, axis: JPC_SixDOFConstraint_Axis) -> bool {
        unsafe { JPC_SixDOFConstraint_IsFixedAxis(self.dof(), axis) }
    }

    pub fn limits_spring_settings(&self, axis: JPC_SixDOFConstraint_Axis) -> SpringSettings {
        unsafe { SpringSettings(JPC_SixDOFConstraint_GetLimitsSpringSettings(self.dof(), axis)) }
    }

    pub fn set_limits_spring_settings(&self, axis: JPC_SixDOFConstraint_Axis, settings: &SpringSettings) {
        unsafe { JPC_SixDOFConstraint_SetLimitsSpringSettings(self.dof(), axis, &settings.0) }
    }

    pub fn motor_settings(&self, axis: JPC_SixDOFConstraint_Axis) -> MotorSettings {
        unsafe { MotorSettings(JPC_SixDOFConstraint_GetMotorSettings(self.dof(), axis)) }
    }

    pub fn set_motor_settings(&self, axis: JPC_SixDOFConstraint_Axis, settings: &MotorSettings) {
        unsafe { JPC_SixDOFConstraint_SetMotorSettings(self.dof(), axis, &settings.0) }
    }

    pub fn motor_state(&self, axis: JPC_SixDOFConstraint_Axis) -> JPC_MotorState {
        unsafe { JPC_SixDOFConstraint_GetMotorState(self.dof(), axis) }
    }

    pub fn set_motor_state(&self, axis: JPC_SixDOFConstraint_Axis, state: JPC_MotorState) {
        unsafe { JPC_SixDOFConstraint_SetMotorState(self.dof(), axis, state) }
    }

    pub fn body1(&self) -> Body<'_> {
        unsafe { Body::new(JPC_TwoBodyConstraint_GetBody1(self.two_body())) }
    }

    pub fn body2(&self) -> Body<'_> {
        unsafe { Body::new(JPC_TwoBodyConstraint_GetBody2(self.two_body())) }
    }

    pub fn constraint_to_body1_matrix(&self) -> Mat4 {
        unsafe { JPC_TwoBodyConstraint_GetConstraintToBody1Matrix(self.two_body()).into_rolt() }
    }

    pub fn constraint_to_body2_matrix(&self) -> Mat4 {
        unsafe { JPC_TwoBodyConstraint_GetConstraintToBody2Matrix(self.two_body()).into_rolt() }
    }

    pub fn as_constraint(&self) -> Constraint { Constraint(self.0.clone()) }

    pub fn with_raw<R>(&self, f: impl FnOnce(*mut JPC_Constraint) -> R) -> R {
        self.0.with_raw(f)
    }

    pub fn raw(&self) -> *mut JPC_Constraint { *self.0 }
}

// --- swing-twist constraint ---

/// See also: Jolt's [`SwingTwistConstraint`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_swing_twist_constraint.html) class.
pub struct SwingTwistConstraint(pub(crate) Ref<JPC_SwingTwistConstraint>);

impl SwingTwistConstraint {
    pub fn create(settings: &SwingTwistConstraintSettings, body1: &Body<'_>, body2: &Body<'_>) -> Self {
        unsafe {
            let raw = JPC_SwingTwistConstraintSettings_Create(&settings.0, body1.raw(), body2.raw());
            Self(Ref::from_active(raw))
        }
    }

    pub fn normal_half_cone_angle(&self) -> f32 { unsafe { JPC_SwingTwistConstraint_GetNormalHalfConeAngle(*self.0) } }
    pub fn set_normal_half_cone_angle(&self, angle: f32) { unsafe { JPC_SwingTwistConstraint_SetNormalHalfConeAngle(*self.0, angle) } }
    pub fn plane_half_cone_angle(&self) -> f32 { unsafe { JPC_SwingTwistConstraint_GetPlaneHalfConeAngle(*self.0) } }
    pub fn set_plane_half_cone_angle(&self, angle: f32) { unsafe { JPC_SwingTwistConstraint_SetPlaneHalfConeAngle(*self.0, angle) } }
    pub fn twist_min_angle(&self) -> f32 { unsafe { JPC_SwingTwistConstraint_GetTwistMinAngle(*self.0) } }
    pub fn set_twist_min_angle(&self, angle: f32) { unsafe { JPC_SwingTwistConstraint_SetTwistMinAngle(*self.0, angle) } }
    pub fn twist_max_angle(&self) -> f32 { unsafe { JPC_SwingTwistConstraint_GetTwistMaxAngle(*self.0) } }
    pub fn set_twist_max_angle(&self, angle: f32) { unsafe { JPC_SwingTwistConstraint_SetTwistMaxAngle(*self.0, angle) } }
    pub fn max_friction_torque(&self) -> f32 { unsafe { JPC_SwingTwistConstraint_GetMaxFrictionTorque(*self.0) } }
    pub fn set_max_friction_torque(&self, torque: f32) { unsafe { JPC_SwingTwistConstraint_SetMaxFrictionTorque(*self.0, torque) } }
    pub fn swing_motor_state(&self) -> JPC_MotorState { unsafe { JPC_SwingTwistConstraint_GetSwingMotorState(*self.0) } }
    pub fn set_swing_motor_state(&self, state: JPC_MotorState) { unsafe { JPC_SwingTwistConstraint_SetSwingMotorState(*self.0, state) } }
    pub fn twist_motor_state(&self) -> JPC_MotorState { unsafe { JPC_SwingTwistConstraint_GetTwistMotorState(*self.0) } }
    pub fn set_twist_motor_state(&self, state: JPC_MotorState) { unsafe { JPC_SwingTwistConstraint_SetTwistMotorState(*self.0, state) } }
    pub fn target_angular_velocity_cs(&self) -> Vec3 { unsafe { JPC_SwingTwistConstraint_GetTargetAngularVelocityCS(*self.0).into_rolt() } }
    pub fn set_target_angular_velocity_cs(&self, velocity: Vec3) { unsafe { JPC_SwingTwistConstraint_SetTargetAngularVelocityCS(*self.0, velocity.into_jolt()) } }
    pub fn target_orientation_cs(&self) -> Quat { unsafe { JPC_SwingTwistConstraint_GetTargetOrientationCS(*self.0).into_rolt() } }
    pub fn set_target_orientation_cs(&self, orientation: Quat) { unsafe { JPC_SwingTwistConstraint_SetTargetOrientationCS(*self.0, orientation.into_jolt()) } }
    pub fn set_target_orientation_bs(&self, orientation: Quat) { unsafe { JPC_SwingTwistConstraint_SetTargetOrientationBS(*self.0, orientation.into_jolt()) } }
    pub fn rotation_in_constraint_space(&self) -> Quat { unsafe { JPC_SwingTwistConstraint_GetRotationInConstraintSpace(*self.0).into_rolt() } }
    pub fn total_lambda_position(&self) -> Vec3 { unsafe { JPC_SwingTwistConstraint_GetTotalLambdaPosition(*self.0).into_rolt() } }
    pub fn total_lambda_twist(&self) -> f32 { unsafe { JPC_SwingTwistConstraint_GetTotalLambdaTwist(*self.0) } }
    pub fn total_lambda_swing_y(&self) -> f32 { unsafe { JPC_SwingTwistConstraint_GetTotalLambdaSwingY(*self.0) } }
    pub fn total_lambda_swing_z(&self) -> f32 { unsafe { JPC_SwingTwistConstraint_GetTotalLambdaSwingZ(*self.0) } }
    pub fn total_lambda_motor(&self) -> Vec3 { unsafe { JPC_SwingTwistConstraint_GetTotalLambdaMotor(*self.0).into_rolt() } }
    pub fn local_space_position1(&self) -> Vec3 { unsafe { JPC_SwingTwistConstraint_GetLocalSpacePosition1(*self.0).into_rolt() } }
    pub fn local_space_position2(&self) -> Vec3 { unsafe { JPC_SwingTwistConstraint_GetLocalSpacePosition2(*self.0).into_rolt() } }
    pub fn constraint_to_body1(&self) -> Quat { unsafe { JPC_SwingTwistConstraint_GetConstraintToBody1(*self.0).into_rolt() } }
    pub fn constraint_to_body2(&self) -> Quat { unsafe { JPC_SwingTwistConstraint_GetConstraintToBody2(*self.0).into_rolt() } }
    pub fn swing_motor_settings(&self) -> MotorSettings { unsafe { MotorSettings(JPC_SwingTwistConstraint_GetSwingMotorSettings(*self.0)) } }
    pub fn set_swing_motor_settings(&self, settings: &MotorSettings) { unsafe { JPC_SwingTwistConstraint_SetSwingMotorSettings(*self.0, &settings.0) } }
    pub fn twist_motor_settings(&self) -> MotorSettings { unsafe { MotorSettings(JPC_SwingTwistConstraint_GetTwistMotorSettings(*self.0)) } }
    pub fn set_twist_motor_settings(&self, settings: &MotorSettings) { unsafe { JPC_SwingTwistConstraint_SetTwistMotorSettings(*self.0, &settings.0) } }

    pub fn body1(&self) -> Body<'_> {
        unsafe { Body::new(JPC_TwoBodyConstraint_GetBody1((*self.0).cast::<JPC_TwoBodyConstraint>())) }
    }

    pub fn body2(&self) -> Body<'_> {
        unsafe { Body::new(JPC_TwoBodyConstraint_GetBody2((*self.0).cast::<JPC_TwoBodyConstraint>())) }
    }

    pub fn constraint_to_body1_matrix(&self) -> Mat4 {
        unsafe { JPC_TwoBodyConstraint_GetConstraintToBody1Matrix((*self.0).cast::<JPC_TwoBodyConstraint>()).into_rolt() }
    }

    pub fn constraint_to_body2_matrix(&self) -> Mat4 {
        unsafe { JPC_TwoBodyConstraint_GetConstraintToBody2Matrix((*self.0).cast::<JPC_TwoBodyConstraint>()).into_rolt() }
    }

    pub fn as_constraint(&self) -> Constraint { Constraint(self.0.clone().cast()) }

    pub fn with_raw<R>(&self, f: impl FnOnce(*mut JPC_SwingTwistConstraint) -> R) -> R {
        self.0.with_raw(f)
    }

    pub fn raw(&self) -> *mut JPC_SwingTwistConstraint { *self.0 }
}

// --- cone constraint ---

/// See also: Jolt's [`ConeConstraint`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_cone_constraint.html) class.
pub struct ConeConstraint(pub(crate) Ref<JPC_ConeConstraint>);

impl ConeConstraint {
    pub fn create(settings: &ConeConstraintSettings, body1: &Body<'_>, body2: &Body<'_>) -> Self {
        unsafe {
            let raw = JPC_ConeConstraintSettings_Create(&settings.0, body1.raw(), body2.raw());
            Self(Ref::from_active(raw))
        }
    }

    pub fn set_half_cone_angle(&self, angle: f32) { unsafe { JPC_ConeConstraint_SetHalfConeAngle(*self.0, angle) } }
    pub fn cos_half_cone_angle(&self) -> f32 { unsafe { JPC_ConeConstraint_GetCosHalfConeAngle(*self.0) } }
    pub fn total_lambda_position(&self) -> Vec3 { unsafe { JPC_ConeConstraint_GetTotalLambdaPosition(*self.0).into_rolt() } }
    pub fn total_lambda_rotation(&self) -> f32 { unsafe { JPC_ConeConstraint_GetTotalLambdaRotation(*self.0) } }

    pub fn body1(&self) -> Body<'_> {
        unsafe { Body::new(JPC_TwoBodyConstraint_GetBody1((*self.0).cast::<JPC_TwoBodyConstraint>())) }
    }

    pub fn body2(&self) -> Body<'_> {
        unsafe { Body::new(JPC_TwoBodyConstraint_GetBody2((*self.0).cast::<JPC_TwoBodyConstraint>())) }
    }

    pub fn constraint_to_body1_matrix(&self) -> Mat4 {
        unsafe { JPC_TwoBodyConstraint_GetConstraintToBody1Matrix((*self.0).cast::<JPC_TwoBodyConstraint>()).into_rolt() }
    }

    pub fn constraint_to_body2_matrix(&self) -> Mat4 {
        unsafe { JPC_TwoBodyConstraint_GetConstraintToBody2Matrix((*self.0).cast::<JPC_TwoBodyConstraint>()).into_rolt() }
    }

    pub fn as_constraint(&self) -> Constraint { Constraint(self.0.clone().cast()) }

    pub fn with_raw<R>(&self, f: impl FnOnce(*mut JPC_ConeConstraint) -> R) -> R {
        self.0.with_raw(f)
    }

    pub fn raw(&self) -> *mut JPC_ConeConstraint { *self.0 }
}

// --- pulley constraint ---

/// See also: Jolt's [`PulleyConstraint`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_pulley_constraint.html) class.
pub struct PulleyConstraint(pub(crate) Ref<JPC_PulleyConstraint>);

impl PulleyConstraint {
    pub fn create(settings: &PulleyConstraintSettings, body1: &Body<'_>, body2: &Body<'_>) -> Self {
        unsafe {
            let raw = JPC_PulleyConstraintSettings_Create(&settings.0, body1.raw(), body2.raw());
            Self(Ref::from_active(raw))
        }
    }

    pub fn set_length(&self, min_length: f32, max_length: f32) { unsafe { JPC_PulleyConstraint_SetLength(*self.0, min_length, max_length) } }
    pub fn min_length(&self) -> f32 { unsafe { JPC_PulleyConstraint_GetMinLength(*self.0) } }
    pub fn max_length(&self) -> f32 { unsafe { JPC_PulleyConstraint_GetMaxLength(*self.0) } }
    pub fn current_length(&self) -> f32 { unsafe { JPC_PulleyConstraint_GetCurrentLength(*self.0) } }
    pub fn total_lambda_position(&self) -> f32 { unsafe { JPC_PulleyConstraint_GetTotalLambdaPosition(*self.0) } }

    pub fn body1(&self) -> Body<'_> {
        unsafe { Body::new(JPC_TwoBodyConstraint_GetBody1((*self.0).cast::<JPC_TwoBodyConstraint>())) }
    }

    pub fn body2(&self) -> Body<'_> {
        unsafe { Body::new(JPC_TwoBodyConstraint_GetBody2((*self.0).cast::<JPC_TwoBodyConstraint>())) }
    }

    pub fn constraint_to_body1_matrix(&self) -> Mat4 {
        unsafe { JPC_TwoBodyConstraint_GetConstraintToBody1Matrix((*self.0).cast::<JPC_TwoBodyConstraint>()).into_rolt() }
    }

    pub fn constraint_to_body2_matrix(&self) -> Mat4 {
        unsafe { JPC_TwoBodyConstraint_GetConstraintToBody2Matrix((*self.0).cast::<JPC_TwoBodyConstraint>()).into_rolt() }
    }

    pub fn as_constraint(&self) -> Constraint { Constraint(self.0.clone().cast()) }

    pub fn with_raw<R>(&self, f: impl FnOnce(*mut JPC_PulleyConstraint) -> R) -> R {
        self.0.with_raw(f)
    }

    pub fn raw(&self) -> *mut JPC_PulleyConstraint { *self.0 }
}

// --- path constraint ---

/// See also: Jolt's [`PathConstraint`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_path_constraint.html) class.
pub struct PathConstraint(pub(crate) Ref<JPC_PathConstraint>);

impl PathConstraint {
    pub fn create(settings: &PathConstraintSettings, body1: &Body<'_>, body2: &Body<'_>) -> Self {
        unsafe {
            let raw = JPC_PathConstraintSettings_Create(&settings.0, body1.raw(), body2.raw());
            Self(Ref::from_active(raw))
        }
    }

    pub fn set_path(&self, path: &PathConstraintPath, path_fraction: f32) {
        unsafe { JPC_PathConstraint_SetPath(*self.0, path.0.get().cast_mut(), path_fraction) }
    }

    pub fn get_path(&self) -> Option<PathConstraintPath> {
        let raw = unsafe { JPC_PathConstraint_GetPath(*self.0) };
        if raw.is_null() {
            None
        } else {
            Some(PathConstraintPath(unsafe { RefConst::from_active(raw) }))
        }
    }

    pub fn path_fraction(&self) -> f32 { unsafe { JPC_PathConstraint_GetPathFraction(*self.0) } }
    pub fn max_friction_force(&self) -> f32 { unsafe { JPC_PathConstraint_GetMaxFrictionForce(*self.0) } }
    pub fn set_max_friction_force(&self, force: f32) { unsafe { JPC_PathConstraint_SetMaxFrictionForce(*self.0, force) } }
    pub fn position_motor_state(&self) -> JPC_MotorState { unsafe { JPC_PathConstraint_GetPositionMotorState(*self.0) } }
    pub fn set_position_motor_state(&self, state: JPC_MotorState) { unsafe { JPC_PathConstraint_SetPositionMotorState(*self.0, state) } }
    pub fn target_velocity(&self) -> f32 { unsafe { JPC_PathConstraint_GetTargetVelocity(*self.0) } }
    pub fn set_target_velocity(&self, velocity: f32) { unsafe { JPC_PathConstraint_SetTargetVelocity(*self.0, velocity) } }
    pub fn target_path_fraction(&self) -> f32 { unsafe { JPC_PathConstraint_GetTargetPathFraction(*self.0) } }
    pub fn set_target_path_fraction(&self, fraction: f32) { unsafe { JPC_PathConstraint_SetTargetPathFraction(*self.0, fraction) } }
    pub fn total_lambda_position(&self) -> [f32; 2] {
        let v = unsafe { JPC_PathConstraint_GetTotalLambdaPosition(*self.0) };
        [v.x, v.y]
    }
    pub fn total_lambda_position_limits(&self) -> f32 { unsafe { JPC_PathConstraint_GetTotalLambdaPositionLimits(*self.0) } }
    pub fn total_lambda_motor(&self) -> f32 { unsafe { JPC_PathConstraint_GetTotalLambdaMotor(*self.0) } }
    pub fn total_lambda_rotation(&self) -> Vec3 { unsafe { JPC_PathConstraint_GetTotalLambdaRotation(*self.0).into_rolt() } }
    pub fn position_motor_settings(&self) -> MotorSettings { unsafe { MotorSettings(JPC_PathConstraint_GetPositionMotorSettings(*self.0)) } }
    pub fn set_position_motor_settings(&self, settings: &MotorSettings) { unsafe { JPC_PathConstraint_SetPositionMotorSettings(*self.0, &settings.0) } }

    pub fn body1(&self) -> Body<'_> {
        unsafe { Body::new(JPC_TwoBodyConstraint_GetBody1((*self.0).cast::<JPC_TwoBodyConstraint>())) }
    }

    pub fn body2(&self) -> Body<'_> {
        unsafe { Body::new(JPC_TwoBodyConstraint_GetBody2((*self.0).cast::<JPC_TwoBodyConstraint>())) }
    }

    pub fn constraint_to_body1_matrix(&self) -> Mat4 {
        unsafe { JPC_TwoBodyConstraint_GetConstraintToBody1Matrix((*self.0).cast::<JPC_TwoBodyConstraint>()).into_rolt() }
    }

    pub fn constraint_to_body2_matrix(&self) -> Mat4 {
        unsafe { JPC_TwoBodyConstraint_GetConstraintToBody2Matrix((*self.0).cast::<JPC_TwoBodyConstraint>()).into_rolt() }
    }

    pub fn as_constraint(&self) -> Constraint { Constraint(self.0.clone().cast()) }

    pub fn with_raw<R>(&self, f: impl FnOnce(*mut JPC_PathConstraint) -> R) -> R {
        self.0.with_raw(f)
    }

    pub fn raw(&self) -> *mut JPC_PathConstraint { *self.0 }
}

// --- gear constraint ---

/// See also: Jolt's [`GearConstraintSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/struct_gear_constraint_settings.html) struct.
pub struct GearConstraintSettings(pub JPC_GearConstraintSettings);

impl GearConstraintSettings {
    pub fn space(&self) -> JPC_ConstraintSpace { self.0.Space }
    pub fn set_space(&mut self, space: JPC_ConstraintSpace) { self.0.Space = space; }
    pub fn hinge_axis1(&self) -> Vec3 { Vec3::from_jolt(self.0.HingeAxis1) }
    pub fn set_hinge_axis1(&mut self, v: Vec3) { self.0.HingeAxis1 = v.into_jolt(); }
    pub fn hinge_axis2(&self) -> Vec3 { Vec3::from_jolt(self.0.HingeAxis2) }
    pub fn set_hinge_axis2(&mut self, v: Vec3) { self.0.HingeAxis2 = v.into_jolt(); }
    pub fn ratio(&self) -> f32 { self.0.Ratio }
    pub fn set_ratio(&mut self, v: f32) { self.0.Ratio = v; }
}

impl Default for GearConstraintSettings {
    fn default() -> Self {
        let mut raw = unsafe { std::mem::zeroed::<JPC_GearConstraintSettings>() };
        unsafe { JPC_GearConstraintSettings_default(&mut raw) };
        Self(raw)
    }
}

/// See also: Jolt's [`GearConstraint`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_gear_constraint.html) class.
pub struct GearConstraint(pub(crate) Ref<JPC_GearConstraint>);

impl GearConstraint {
    pub fn create(settings: &GearConstraintSettings, body1: &Body<'_>, body2: &Body<'_>) -> Self {
        unsafe {
            let raw = JPC_GearConstraintSettings_Create(&settings.0, body1.raw(), body2.raw());
            Self(Ref::from_active(raw))
        }
    }

    pub fn set_constraints(&self, gear1: &Constraint, gear2: &Constraint) {
        unsafe { JPC_GearConstraint_SetConstraints(*self.0, *gear1.0, *gear2.0) }
    }

    pub fn total_lambda(&self) -> f32 { unsafe { JPC_GearConstraint_GetTotalLambda(*self.0) } }

    pub fn body1(&self) -> Body<'_> {
        unsafe { Body::new(JPC_TwoBodyConstraint_GetBody1((*self.0).cast::<JPC_TwoBodyConstraint>())) }
    }

    pub fn body2(&self) -> Body<'_> {
        unsafe { Body::new(JPC_TwoBodyConstraint_GetBody2((*self.0).cast::<JPC_TwoBodyConstraint>())) }
    }

    pub fn constraint_to_body1_matrix(&self) -> Mat4 {
        unsafe { JPC_TwoBodyConstraint_GetConstraintToBody1Matrix((*self.0).cast::<JPC_TwoBodyConstraint>()).into_rolt() }
    }

    pub fn constraint_to_body2_matrix(&self) -> Mat4 {
        unsafe { JPC_TwoBodyConstraint_GetConstraintToBody2Matrix((*self.0).cast::<JPC_TwoBodyConstraint>()).into_rolt() }
    }

    pub fn as_constraint(&self) -> Constraint { Constraint(self.0.clone().cast()) }

    pub fn with_raw<R>(&self, f: impl FnOnce(*mut JPC_GearConstraint) -> R) -> R {
        self.0.with_raw(f)
    }

    pub fn raw(&self) -> *mut JPC_GearConstraint { *self.0 }
}

// --- rack and pinion constraint ---

/// See also: Jolt's [`RackAndPinionConstraintSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/struct_rack_and_pinion_constraint_settings.html) struct.
pub struct RackAndPinionConstraintSettings(pub JPC_RackAndPinionConstraintSettings);

impl RackAndPinionConstraintSettings {
    pub fn space(&self) -> JPC_ConstraintSpace { self.0.Space }
    pub fn set_space(&mut self, space: JPC_ConstraintSpace) { self.0.Space = space; }
    pub fn hinge_axis(&self) -> Vec3 { Vec3::from_jolt(self.0.HingeAxis) }
    pub fn set_hinge_axis(&mut self, v: Vec3) { self.0.HingeAxis = v.into_jolt(); }
    pub fn slider_axis(&self) -> Vec3 { Vec3::from_jolt(self.0.SliderAxis) }
    pub fn set_slider_axis(&mut self, v: Vec3) { self.0.SliderAxis = v.into_jolt(); }
    pub fn ratio(&self) -> f32 { self.0.Ratio }
    pub fn set_ratio(&mut self, v: f32) { self.0.Ratio = v; }
}

impl Default for RackAndPinionConstraintSettings {
    fn default() -> Self {
        let mut raw = unsafe { std::mem::zeroed::<JPC_RackAndPinionConstraintSettings>() };
        unsafe { JPC_RackAndPinionConstraintSettings_default(&mut raw) };
        Self(raw)
    }
}

/// See also: Jolt's [`RackAndPinionConstraint`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_rack_and_pinion_constraint.html) class.
pub struct RackAndPinionConstraint(pub(crate) Ref<JPC_RackAndPinionConstraint>);

impl RackAndPinionConstraint {
    pub fn create(settings: &RackAndPinionConstraintSettings, body1: &Body<'_>, body2: &Body<'_>) -> Self {
        unsafe {
            let raw = JPC_RackAndPinionConstraintSettings_Create(&settings.0, body1.raw(), body2.raw());
            Self(Ref::from_active(raw))
        }
    }

    pub fn set_constraints(&self, pinion: &Constraint, rack: &Constraint) {
        unsafe { JPC_RackAndPinionConstraint_SetConstraints(*self.0, *pinion.0, *rack.0) }
    }

    pub fn total_lambda(&self) -> f32 { unsafe { JPC_RackAndPinionConstraint_GetTotalLambda(*self.0) } }

    pub fn body1(&self) -> Body<'_> {
        unsafe { Body::new(JPC_TwoBodyConstraint_GetBody1((*self.0).cast::<JPC_TwoBodyConstraint>())) }
    }

    pub fn body2(&self) -> Body<'_> {
        unsafe { Body::new(JPC_TwoBodyConstraint_GetBody2((*self.0).cast::<JPC_TwoBodyConstraint>())) }
    }

    pub fn constraint_to_body1_matrix(&self) -> Mat4 {
        unsafe { JPC_TwoBodyConstraint_GetConstraintToBody1Matrix((*self.0).cast::<JPC_TwoBodyConstraint>()).into_rolt() }
    }

    pub fn constraint_to_body2_matrix(&self) -> Mat4 {
        unsafe { JPC_TwoBodyConstraint_GetConstraintToBody2Matrix((*self.0).cast::<JPC_TwoBodyConstraint>()).into_rolt() }
    }

    pub fn as_constraint(&self) -> Constraint { Constraint(self.0.clone().cast()) }

    pub fn with_raw<R>(&self, f: impl FnOnce(*mut JPC_RackAndPinionConstraint) -> R) -> R {
        self.0.with_raw(f)
    }

    pub fn raw(&self) -> *mut JPC_RackAndPinionConstraint { *self.0 }
}

// --- path constraint path ---

/// See also: Jolt's [`PathConstraintPath`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_path_constraint_path.html) class.
pub struct PathConstraintPath(pub(crate) RefConst<JPC_PathConstraintPath>);

impl PathConstraintPath {
    pub fn raw(&self) -> *const JPC_PathConstraintPath { self.0.get() }
}

/// See also: Jolt's [`PathConstraintPathHermite`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_path_constraint_path_hermite.html) class.
pub struct PathConstraintPathHermite(pub(crate) Ref<JPC_PathConstraintPathHermite>);

impl PathConstraintPathHermite {
    pub fn new() -> Self {
        unsafe { Self(Ref::from_active(JPC_PathConstraintPathHermite_new())) }
    }

    pub fn add_point(&self, position: Vec3, tangent: Vec3, normal: Vec3) {
        unsafe {
            JPC_PathConstraintPathHermite_AddPoint(*self.0, position.into_jolt(), tangent.into_jolt(), normal.into_jolt())
        }
    }

    pub fn as_path(&self) -> PathConstraintPath {
        PathConstraintPath(unsafe { RefConst::from_active((*self.0).cast::<JPC_PathConstraintPath>()) })
    }

    pub fn raw(&self) -> *mut JPC_PathConstraintPathHermite { *self.0 }
}

impl Default for PathConstraintPathHermite {
    fn default() -> Self { Self::new() }
}
