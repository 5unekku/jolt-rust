use joltc_sys::*;

use crate::{Body, BodyId, FromJolt, IntoJolt, IntoRolt, Mat4, ObjectLayer, RMat4, Vec3};

/// Heap-allocated wheel settings for a wheeled vehicle.
///
/// See also: Jolt's [`WheelSettingsWV`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_wheel_settings_w_v.html) class.
pub struct WheelSettingsWV {
    raw: *mut JPC_WheelSettingsWV,
}

impl WheelSettingsWV {
    pub fn new() -> Self {
        Self {
            raw: unsafe { JPC_WheelSettingsWV_new() },
        }
    }

    pub fn position(&self) -> Vec3 {
        Vec3::from_jolt(unsafe { JPC_WheelSettingsWV_GetPosition(self.raw) })
    }
    pub fn set_position(&mut self, v: Vec3) {
        unsafe { JPC_WheelSettingsWV_SetPosition(self.raw, v.into_jolt()) }
    }
    pub fn suspension_direction(&self) -> Vec3 {
        Vec3::from_jolt(unsafe { JPC_WheelSettingsWV_GetSuspensionDirection(self.raw) })
    }
    pub fn set_suspension_direction(&mut self, v: Vec3) {
        unsafe { JPC_WheelSettingsWV_SetSuspensionDirection(self.raw, v.into_jolt()) }
    }
    pub fn steering_axis(&self) -> Vec3 {
        Vec3::from_jolt(unsafe { JPC_WheelSettingsWV_GetSteeringAxis(self.raw) })
    }
    pub fn set_steering_axis(&mut self, v: Vec3) {
        unsafe { JPC_WheelSettingsWV_SetSteeringAxis(self.raw, v.into_jolt()) }
    }
    pub fn wheel_up(&self) -> Vec3 {
        Vec3::from_jolt(unsafe { JPC_WheelSettingsWV_GetWheelUp(self.raw) })
    }
    pub fn set_wheel_up(&mut self, v: Vec3) {
        unsafe { JPC_WheelSettingsWV_SetWheelUp(self.raw, v.into_jolt()) }
    }
    pub fn wheel_forward(&self) -> Vec3 {
        Vec3::from_jolt(unsafe { JPC_WheelSettingsWV_GetWheelForward(self.raw) })
    }
    pub fn set_wheel_forward(&mut self, v: Vec3) {
        unsafe { JPC_WheelSettingsWV_SetWheelForward(self.raw, v.into_jolt()) }
    }
    pub fn suspension_min_length(&self) -> f32 {
        unsafe { JPC_WheelSettingsWV_GetSuspensionMinLength(self.raw) }
    }
    pub fn set_suspension_min_length(&mut self, v: f32) {
        unsafe { JPC_WheelSettingsWV_SetSuspensionMinLength(self.raw, v) }
    }
    pub fn suspension_max_length(&self) -> f32 {
        unsafe { JPC_WheelSettingsWV_GetSuspensionMaxLength(self.raw) }
    }
    pub fn set_suspension_max_length(&mut self, v: f32) {
        unsafe { JPC_WheelSettingsWV_SetSuspensionMaxLength(self.raw, v) }
    }
    pub fn suspension_preload_length(&self) -> f32 {
        unsafe { JPC_WheelSettingsWV_GetSuspensionPreloadLength(self.raw) }
    }
    pub fn set_suspension_preload_length(&mut self, v: f32) {
        unsafe { JPC_WheelSettingsWV_SetSuspensionPreloadLength(self.raw, v) }
    }
    pub fn radius(&self) -> f32 {
        unsafe { JPC_WheelSettingsWV_GetRadius(self.raw) }
    }
    pub fn set_radius(&mut self, v: f32) {
        unsafe { JPC_WheelSettingsWV_SetRadius(self.raw, v) }
    }
    pub fn width(&self) -> f32 {
        unsafe { JPC_WheelSettingsWV_GetWidth(self.raw) }
    }
    pub fn set_width(&mut self, v: f32) {
        unsafe { JPC_WheelSettingsWV_SetWidth(self.raw, v) }
    }
    pub fn max_steer_angle(&self) -> f32 {
        unsafe { JPC_WheelSettingsWV_GetMaxSteerAngle(self.raw) }
    }
    pub fn set_max_steer_angle(&mut self, v: f32) {
        unsafe { JPC_WheelSettingsWV_SetMaxSteerAngle(self.raw, v) }
    }
    pub fn max_brake_torque(&self) -> f32 {
        unsafe { JPC_WheelSettingsWV_GetMaxBrakeTorque(self.raw) }
    }
    pub fn set_max_brake_torque(&mut self, v: f32) {
        unsafe { JPC_WheelSettingsWV_SetMaxBrakeTorque(self.raw, v) }
    }
    pub fn max_hand_brake_torque(&self) -> f32 {
        unsafe { JPC_WheelSettingsWV_GetMaxHandBrakeTorque(self.raw) }
    }
    pub fn set_max_hand_brake_torque(&mut self, v: f32) {
        unsafe { JPC_WheelSettingsWV_SetMaxHandBrakeTorque(self.raw, v) }
    }

    pub fn raw(&self) -> *const JPC_WheelSettingsWV {
        self.raw
    }
}

impl Default for WheelSettingsWV {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for WheelSettingsWV {
    fn drop(&mut self) {
        unsafe { JPC_WheelSettingsWV_delete(self.raw) }
    }
}

/// See also: Jolt's [`VehicleEngineSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/struct_vehicle_engine_settings.html) struct.
#[repr(transparent)]
pub struct VehicleEngineSettings(pub JPC_VehicleEngineSettings);

impl VehicleEngineSettings {
    pub fn max_torque(&self) -> f32 {
        self.0.MaxTorque
    }
    pub fn set_max_torque(&mut self, v: f32) {
        self.0.MaxTorque = v;
    }
    pub fn min_rpm(&self) -> f32 {
        self.0.MinRPM
    }
    pub fn set_min_rpm(&mut self, v: f32) {
        self.0.MinRPM = v;
    }
    pub fn max_rpm(&self) -> f32 {
        self.0.MaxRPM
    }
    pub fn set_max_rpm(&mut self, v: f32) {
        self.0.MaxRPM = v;
    }
    pub fn inertia(&self) -> f32 {
        self.0.Inertia
    }
    pub fn set_inertia(&mut self, v: f32) {
        self.0.Inertia = v;
    }
    pub fn angular_damping(&self) -> f32 {
        self.0.AngularDamping
    }
    pub fn set_angular_damping(&mut self, v: f32) {
        self.0.AngularDamping = v;
    }
}

impl Default for VehicleEngineSettings {
    fn default() -> Self {
        Self(JPC_VehicleEngineSettings::default())
    }
}

/// See also: Jolt's [`VehicleTransmissionSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/struct_vehicle_transmission_settings.html) struct.
pub struct VehicleTransmissionSettings(pub JPC_VehicleTransmissionSettings);

impl VehicleTransmissionSettings {
    pub fn mode(&self) -> JPC_VehicleTransmissionMode {
        self.0.Mode
    }
    pub fn set_mode(&mut self, v: JPC_VehicleTransmissionMode) {
        self.0.Mode = v;
    }
    pub fn switch_up_rpm(&self) -> f32 {
        self.0.SwitchUpRPM
    }
    pub fn set_switch_up_rpm(&mut self, v: f32) {
        self.0.SwitchUpRPM = v;
    }
    pub fn switch_down_rpm(&self) -> f32 {
        self.0.SwitchDownRPM
    }
    pub fn set_switch_down_rpm(&mut self, v: f32) {
        self.0.SwitchDownRPM = v;
    }
    pub fn switch_time(&self) -> f32 {
        self.0.SwitchTime
    }
    pub fn set_switch_time(&mut self, v: f32) {
        self.0.SwitchTime = v;
    }
    pub fn clutch_release_time(&self) -> f32 {
        self.0.ClutchReleaseTime
    }
    pub fn set_clutch_release_time(&mut self, v: f32) {
        self.0.ClutchReleaseTime = v;
    }
    pub fn switch_latency(&self) -> f32 {
        self.0.SwitchLatency
    }
    pub fn set_switch_latency(&mut self, v: f32) {
        self.0.SwitchLatency = v;
    }
    pub fn clutch_strength(&self) -> f32 {
        self.0.ClutchStrength
    }
    pub fn set_clutch_strength(&mut self, v: f32) {
        self.0.ClutchStrength = v;
    }
    pub fn gear_ratios(&self) -> &[f32] {
        &self.0.GearRatios[..self.0.GearRatiosLen as usize]
    }
    pub fn set_gear_ratios(&mut self, ratios: &[f32]) {
        let n = ratios.len().min(self.0.GearRatios.len());
        self.0.GearRatios[..n].copy_from_slice(&ratios[..n]);
        self.0.GearRatiosLen = n as u32;
    }
    pub fn reverse_gear_ratios(&self) -> &[f32] {
        &self.0.ReverseGearRatios[..self.0.ReverseGearRatiosLen as usize]
    }
    pub fn set_reverse_gear_ratios(&mut self, ratios: &[f32]) {
        let n = ratios.len().min(self.0.ReverseGearRatios.len());
        self.0.ReverseGearRatios[..n].copy_from_slice(&ratios[..n]);
        self.0.ReverseGearRatiosLen = n as u32;
    }
}

impl Default for VehicleTransmissionSettings {
    fn default() -> Self {
        Self(JPC_VehicleTransmissionSettings::default())
    }
}

/// See also: Jolt's [`VehicleDifferentialSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/struct_vehicle_differential_settings.html) struct.
#[repr(transparent)]
pub struct VehicleDifferentialSettings(pub JPC_VehicleDifferentialSettings);

impl VehicleDifferentialSettings {
    pub fn left_wheel(&self) -> i32 {
        self.0.LeftWheel
    }
    pub fn set_left_wheel(&mut self, v: i32) {
        self.0.LeftWheel = v;
    }
    pub fn right_wheel(&self) -> i32 {
        self.0.RightWheel
    }
    pub fn set_right_wheel(&mut self, v: i32) {
        self.0.RightWheel = v;
    }
    pub fn differential_ratio(&self) -> f32 {
        self.0.DifferentialRatio
    }
    pub fn set_differential_ratio(&mut self, v: f32) {
        self.0.DifferentialRatio = v;
    }
    pub fn left_right_split(&self) -> f32 {
        self.0.LeftRightSplit
    }
    pub fn set_left_right_split(&mut self, v: f32) {
        self.0.LeftRightSplit = v;
    }
    pub fn limited_slip_ratio(&self) -> f32 {
        self.0.LimitedSlipRatio
    }
    pub fn set_limited_slip_ratio(&mut self, v: f32) {
        self.0.LimitedSlipRatio = v;
    }
    pub fn engine_torque_ratio(&self) -> f32 {
        self.0.EngineTorqueRatio
    }
    pub fn set_engine_torque_ratio(&mut self, v: f32) {
        self.0.EngineTorqueRatio = v;
    }
}

impl Default for VehicleDifferentialSettings {
    fn default() -> Self {
        Self(JPC_VehicleDifferentialSettings::default())
    }
}

/// See also: Jolt's [`VehicleAntiRollBar`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/struct_vehicle_anti_roll_bar.html) struct.
#[repr(transparent)]
pub struct VehicleAntiRollBar(pub JPC_VehicleAntiRollBar);

impl VehicleAntiRollBar {
    pub fn new(left_wheel: i32, right_wheel: i32, stiffness: f32) -> Self {
        Self(JPC_VehicleAntiRollBar {
            LeftWheel: left_wheel,
            RightWheel: right_wheel,
            Stiffness: stiffness,
        })
    }
    pub fn left_wheel(&self) -> i32 {
        self.0.LeftWheel
    }
    pub fn set_left_wheel(&mut self, v: i32) {
        self.0.LeftWheel = v;
    }
    pub fn right_wheel(&self) -> i32 {
        self.0.RightWheel
    }
    pub fn set_right_wheel(&mut self, v: i32) {
        self.0.RightWheel = v;
    }
    pub fn stiffness(&self) -> f32 {
        self.0.Stiffness
    }
    pub fn set_stiffness(&mut self, v: f32) {
        self.0.Stiffness = v;
    }
}

/// Settings builder for a wheeled vehicle.
///
/// See also: Jolt's [`VehicleConstraintSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_vehicle_constraint_settings.html) class.
pub struct VehicleConstraintSettings {
    raw: *mut JPC_VehicleConstraintSettings,
}

impl VehicleConstraintSettings {
    pub fn new() -> Self {
        Self {
            raw: unsafe { JPC_VehicleConstraintSettings_new() },
        }
    }

    pub fn set_up(&mut self, up: Vec3) {
        unsafe { JPC_VehicleConstraintSettings_SetUp(self.raw, up.into_jolt()) }
    }

    pub fn set_forward(&mut self, forward: Vec3) {
        unsafe { JPC_VehicleConstraintSettings_SetForward(self.raw, forward.into_jolt()) }
    }

    pub fn set_max_pitch_roll_angle(&mut self, angle: f32) {
        unsafe { JPC_VehicleConstraintSettings_SetMaxPitchRollAngle(self.raw, angle) }
    }

    pub fn add_wheel(&mut self, wheel: &WheelSettingsWV) {
        unsafe { JPC_VehicleConstraintSettings_AddWheel(self.raw, wheel.raw()) }
    }

    pub fn add_anti_roll_bar(&mut self, bar: VehicleAntiRollBar) {
        unsafe { JPC_VehicleConstraintSettings_AddAntiRollBar(self.raw, bar.0) }
    }

    pub fn set_controller(&mut self, controller: &WheeledVehicleControllerSettings) {
        unsafe { JPC_VehicleConstraintSettings_SetController(self.raw, controller.raw) }
    }

    /// Consume the settings and create a vehicle constraint.
    /// The `body` must be the vehicle's rigid body.
    pub fn create(self, body: &Body<'_>) -> VehicleConstraint {
        let vc = unsafe { JPC_VehicleConstraintSettings_Create(self.raw, body.raw()) };
        VehicleConstraint { raw: vc }
    }

    pub fn with_raw<R>(&self, f: impl FnOnce(*mut JPC_VehicleConstraintSettings) -> R) -> R {
        f(self.raw)
    }

    pub fn raw(&self) -> *mut JPC_VehicleConstraintSettings {
        self.raw
    }
}

impl Default for VehicleConstraintSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for VehicleConstraintSettings {
    fn drop(&mut self) {
        unsafe { JPC_VehicleConstraintSettings_delete(self.raw) }
    }
}

// --- wheeled vehicle controller settings ---

/// See also: Jolt's [`WheeledVehicleControllerSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_wheeled_vehicle_controller_settings.html) class.
pub struct WheeledVehicleControllerSettings {
    raw: *mut JPC_WheeledVehicleControllerSettings,
}

impl WheeledVehicleControllerSettings {
    pub fn new() -> Self {
        Self {
            raw: unsafe { JPC_WheeledVehicleControllerSettings_new() },
        }
    }

    pub fn set_engine(&mut self, engine: &VehicleEngineSettings) {
        unsafe { JPC_WheeledVehicleControllerSettings_SetEngine(self.raw, &engine.0) }
    }

    pub fn set_transmission(&mut self, transmission: &VehicleTransmissionSettings) {
        unsafe { JPC_WheeledVehicleControllerSettings_SetTransmission(self.raw, &transmission.0) }
    }

    pub fn add_differential(&mut self, differential: &VehicleDifferentialSettings) {
        unsafe { JPC_WheeledVehicleControllerSettings_AddDifferential(self.raw, &differential.0) }
    }

    pub fn with_raw<R>(
        &self,
        f: impl FnOnce(*const JPC_WheeledVehicleControllerSettings) -> R,
    ) -> R {
        f(self.raw)
    }

    pub fn raw(&self) -> *const JPC_WheeledVehicleControllerSettings {
        self.raw
    }
}

impl Default for WheeledVehicleControllerSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for WheeledVehicleControllerSettings {
    fn drop(&mut self) {
        unsafe { JPC_WheeledVehicleControllerSettings_delete(self.raw) }
    }
}

// --- vehicle collision testers ---

/// Opaque vehicle collision tester (ray or sphere cast).
///
/// See also: Jolt's [`VehicleCollisionTester`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_vehicle_collision_tester.html) class.
pub struct VehicleCollisionTester {
    raw: *mut JPC_VehicleCollisionTester,
}

impl VehicleCollisionTester {
    /// Ray-cast tester.
    pub fn ray(layer: ObjectLayer, up: Vec3, max_slope_angle: f32) -> Self {
        Self {
            raw: unsafe {
                JPC_VehicleCollisionTesterRay_new(layer.raw(), up.into_jolt(), max_slope_angle)
            },
        }
    }

    /// Sphere-cast tester.
    pub fn cast_sphere(layer: ObjectLayer, radius: f32, up: Vec3, max_slope_angle: f32) -> Self {
        Self {
            raw: unsafe {
                JPC_VehicleCollisionTesterCastSphere_new(
                    layer.raw(),
                    radius,
                    up.into_jolt(),
                    max_slope_angle,
                )
            },
        }
    }

    /// Cylinder-cast tester.
    pub fn cast_cylinder(layer: ObjectLayer, convex_radius_fraction: f32) -> Self {
        Self {
            raw: unsafe {
                JPC_VehicleCollisionTesterCastCylinder_new(layer.raw(), convex_radius_fraction)
            },
        }
    }

    pub fn with_raw<R>(&self, f: impl FnOnce(*mut JPC_VehicleCollisionTester) -> R) -> R {
        f(self.raw)
    }

    pub fn raw(&self) -> *mut JPC_VehicleCollisionTester {
        self.raw
    }
}

impl Drop for VehicleCollisionTester {
    fn drop(&mut self) {
        unsafe { JPC_VehicleCollisionTester_delete(self.raw) }
    }
}

// --- wheel ---

/// A single wheel on a wheeled vehicle.
///
/// The lifetime is tied to the `VehicleConstraint` that owns it.
pub struct WheelWV<'constraint> {
    raw: *mut JPC_WheelWV,
    _phantom: std::marker::PhantomData<&'constraint ()>,
}

impl<'constraint> WheelWV<'constraint> {
    fn new(raw: *mut JPC_WheelWV) -> Self {
        Self {
            raw,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn has_contact(&self) -> bool {
        unsafe { JPC_WheelWV_HasContact(self.raw) }
    }

    pub fn angular_velocity(&self) -> f32 {
        unsafe { JPC_WheelWV_GetAngularVelocity(self.raw) }
    }
    pub fn set_angular_velocity(&mut self, vel: f32) {
        unsafe { JPC_WheelWV_SetAngularVelocity(self.raw, vel) }
    }

    pub fn rotation_angle(&self) -> f32 {
        unsafe { JPC_WheelWV_GetRotationAngle(self.raw) }
    }
    pub fn set_rotation_angle(&mut self, angle: f32) {
        unsafe { JPC_WheelWV_SetRotationAngle(self.raw, angle) }
    }

    pub fn steer_angle(&self) -> f32 {
        unsafe { JPC_WheelWV_GetSteerAngle(self.raw) }
    }
    pub fn set_steer_angle(&mut self, angle: f32) {
        unsafe { JPC_WheelWV_SetSteerAngle(self.raw, angle) }
    }

    pub fn contact_body_id(&self) -> BodyId {
        BodyId::new(unsafe { JPC_WheelWV_GetContactBodyID(self.raw) })
    }

    pub fn contact_position(&self) -> crate::RVec3 {
        unsafe { JPC_WheelWV_GetContactPosition(self.raw).into_rolt() }
    }

    pub fn contact_normal(&self) -> Vec3 {
        unsafe { JPC_WheelWV_GetContactNormal(self.raw).into_rolt() }
    }

    pub fn contact_point_velocity(&self) -> Vec3 {
        unsafe { JPC_WheelWV_GetContactPointVelocity(self.raw).into_rolt() }
    }

    pub fn contact_longitudinal(&self) -> Vec3 {
        unsafe { JPC_WheelWV_GetContactLongitudinal(self.raw).into_rolt() }
    }

    pub fn contact_lateral(&self) -> Vec3 {
        unsafe { JPC_WheelWV_GetContactLateral(self.raw).into_rolt() }
    }

    pub fn suspension_length(&self) -> f32 {
        unsafe { JPC_WheelWV_GetSuspensionLength(self.raw) }
    }
    pub fn suspension_lambda(&self) -> f32 {
        unsafe { JPC_WheelWV_GetSuspensionLambda(self.raw) }
    }
    pub fn longitudinal_lambda(&self) -> f32 {
        unsafe { JPC_WheelWV_GetLongitudinalLambda(self.raw) }
    }
    pub fn lateral_lambda(&self) -> f32 {
        unsafe { JPC_WheelWV_GetLateralLambda(self.raw) }
    }

    pub fn with_raw<R>(&self, f: impl FnOnce(*mut JPC_WheelWV) -> R) -> R {
        f(self.raw)
    }
}

// --- wheeled vehicle controller ---

/// Runtime controller for a wheeled vehicle.
///
/// Lifetime is tied to the owning `VehicleConstraint`.
pub struct WheeledVehicleController<'constraint> {
    raw: *mut JPC_WheeledVehicleController,
    _phantom: std::marker::PhantomData<&'constraint ()>,
}

impl<'constraint> WheeledVehicleController<'constraint> {
    fn new(raw: *mut JPC_WheeledVehicleController) -> Self {
        Self {
            raw,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn set_driver_input(&mut self, forward: f32, right: f32, brake: f32, hand_brake: f32) {
        unsafe {
            JPC_WheeledVehicleController_SetDriverInput(self.raw, forward, right, brake, hand_brake)
        }
    }

    pub fn forward_input(&self) -> f32 {
        unsafe { JPC_WheeledVehicleController_GetForwardInput(self.raw) }
    }
    pub fn set_forward_input(&mut self, v: f32) {
        unsafe { JPC_WheeledVehicleController_SetForwardInput(self.raw, v) }
    }

    pub fn right_input(&self) -> f32 {
        unsafe { JPC_WheeledVehicleController_GetRightInput(self.raw) }
    }
    pub fn set_right_input(&mut self, v: f32) {
        unsafe { JPC_WheeledVehicleController_SetRightInput(self.raw, v) }
    }

    pub fn brake_input(&self) -> f32 {
        unsafe { JPC_WheeledVehicleController_GetBrakeInput(self.raw) }
    }
    pub fn set_brake_input(&mut self, v: f32) {
        unsafe { JPC_WheeledVehicleController_SetBrakeInput(self.raw, v) }
    }

    pub fn hand_brake_input(&self) -> f32 {
        unsafe { JPC_WheeledVehicleController_GetHandBrakeInput(self.raw) }
    }
    pub fn set_hand_brake_input(&mut self, v: f32) {
        unsafe { JPC_WheeledVehicleController_SetHandBrakeInput(self.raw, v) }
    }

    pub fn with_raw<R>(&self, f: impl FnOnce(*mut JPC_WheeledVehicleController) -> R) -> R {
        f(self.raw)
    }
}

// --- vehicle constraint ---

/// A constraint that drives a vehicle body.
///
/// See also: Jolt's [`VehicleConstraint`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_vehicle_constraint.html) class.
pub struct VehicleConstraint {
    raw: *mut JPC_VehicleConstraint,
}

impl VehicleConstraint {
    pub fn set_vehicle_collision_tester(&mut self, tester: &VehicleCollisionTester) {
        unsafe { JPC_VehicleConstraint_SetVehicleCollisionTester(self.raw, tester.raw) }
    }

    pub fn wheel_count(&self) -> u32 {
        unsafe { JPC_VehicleConstraint_GetWheelCount(self.raw) }
    }

    pub fn wheel(&mut self, index: u32) -> Option<WheelWV<'_>> {
        let ptr = unsafe { JPC_VehicleConstraint_GetWheel(self.raw, index) };
        if ptr.is_null() {
            None
        } else {
            Some(WheelWV::new(ptr))
        }
    }

    pub fn wheeled_controller(&mut self) -> WheeledVehicleController<'_> {
        let raw = unsafe { JPC_VehicleConstraint_GetWheeledController(self.raw) };
        WheeledVehicleController::new(raw)
    }

    /// Local-space basis vectors for the given wheel.
    pub fn wheel_local_basis(&self, wheel_index: u32) -> (Vec3, Vec3, Vec3) {
        let mut forward = JPC_Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            _w: 0.0,
        };
        let mut up = JPC_Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            _w: 0.0,
        };
        let mut right = JPC_Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            _w: 0.0,
        };
        unsafe {
            JPC_VehicleConstraint_GetWheelLocalBasis(
                self.raw,
                wheel_index,
                &mut forward,
                &mut up,
                &mut right,
            )
        }
        (
            Vec3::from_jolt(forward),
            Vec3::from_jolt(up),
            Vec3::from_jolt(right),
        )
    }

    /// Local-space 4x4 transform for the given wheel.
    pub fn wheel_local_transform(
        &self,
        wheel_index: u32,
        wheel_right: Vec3,
        wheel_up: Vec3,
    ) -> Mat4 {
        unsafe {
            JPC_VehicleConstraint_GetWheelLocalTransform(
                self.raw,
                wheel_index,
                wheel_right.into_jolt(),
                wheel_up.into_jolt(),
            )
            .into_rolt()
        }
    }

    /// World-space 4x4 transform for the given wheel.
    pub fn wheel_world_transform(
        &self,
        wheel_index: u32,
        wheel_right: Vec3,
        wheel_up: Vec3,
    ) -> RMat4 {
        unsafe {
            JPC_VehicleConstraint_GetWheelWorldTransform(
                self.raw,
                wheel_index,
                wheel_right.into_jolt(),
                wheel_up.into_jolt(),
            )
            .into_rolt()
        }
    }

    /// The underlying constraint pointer — required to add to `PhysicsSystem`.
    pub fn as_constraint(&self) -> *mut JPC_Constraint {
        unsafe { JPC_VehicleConstraint_AsConstraint(self.raw) }
    }

    pub fn vehicle_body(&self) -> Body<'_> {
        unsafe { Body::new(JPC_VehicleConstraint_GetVehicleBody(self.raw)) }
    }

    /// Maximum pitch/roll angle in radians.
    pub fn max_pitch_roll_angle(&self) -> f32 {
        unsafe { JPC_VehicleConstraint_GetMaxPitchRollAngle(self.raw) }
    }

    pub fn set_max_pitch_roll_angle(&mut self, angle: f32) {
        unsafe { JPC_VehicleConstraint_SetMaxPitchRollAngle(self.raw, angle) }
    }

    pub fn num_steps_between_collision_test_active(&self) -> u32 {
        unsafe { JPC_VehicleConstraint_GetNumStepsBetweenCollisionTestActive(self.raw) }
    }

    pub fn set_num_steps_between_collision_test_active(&mut self, steps: u32) {
        unsafe { JPC_VehicleConstraint_SetNumStepsBetweenCollisionTestActive(self.raw, steps) }
    }

    pub fn num_steps_between_collision_test_inactive(&self) -> u32 {
        unsafe { JPC_VehicleConstraint_GetNumStepsBetweenCollisionTestInactive(self.raw) }
    }

    pub fn set_num_steps_between_collision_test_inactive(&mut self, steps: u32) {
        unsafe { JPC_VehicleConstraint_SetNumStepsBetweenCollisionTestInactive(self.raw, steps) }
    }

    pub fn with_raw<R>(&self, f: impl FnOnce(*mut JPC_VehicleConstraint) -> R) -> R {
        f(self.raw)
    }

    pub fn raw(&self) -> *mut JPC_VehicleConstraint {
        self.raw
    }
}
