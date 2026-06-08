use joltc_sys::*;

use crate::{Body, BodyId, FromJolt, IntoJolt, IntoRolt, Mat4, ObjectLayer, RMat4, Vec3};

/// Settings builder for a wheeled vehicle.
///
/// See also: Jolt's [`VehicleConstraintSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_vehicle_constraint_settings.html) class.
pub struct VehicleConstraintSettings {
    raw: *mut JPC_VehicleConstraintSettings,
}

impl VehicleConstraintSettings {
    pub fn new() -> Self {
        Self { raw: unsafe { JPC_VehicleConstraintSettings_new() } }
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

    pub fn add_wheel(&mut self, wheel: &JPC_WheelSettingsWV) {
        unsafe { JPC_VehicleConstraintSettings_AddWheel(self.raw, wheel) }
    }

    pub fn add_anti_roll_bar(&mut self, bar: JPC_VehicleAntiRollBar) {
        unsafe { JPC_VehicleConstraintSettings_AddAntiRollBar(self.raw, bar) }
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

    /// # Safety
    /// See [`Ref::with_raw`].
    pub unsafe fn with_raw<R>(&self, f: impl FnOnce(*mut JPC_VehicleConstraintSettings) -> R) -> R {
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
        Self { raw: unsafe { JPC_WheeledVehicleControllerSettings_new() } }
    }

    pub fn set_engine(&mut self, engine: &JPC_VehicleEngineSettings) {
        unsafe { JPC_WheeledVehicleControllerSettings_SetEngine(self.raw, engine) }
    }

    pub fn set_transmission(&mut self, transmission: &JPC_VehicleTransmissionSettings) {
        unsafe { JPC_WheeledVehicleControllerSettings_SetTransmission(self.raw, transmission) }
    }

    pub fn add_differential(&mut self, differential: &JPC_VehicleDifferentialSettings) {
        unsafe { JPC_WheeledVehicleControllerSettings_AddDifferential(self.raw, differential) }
    }

    /// # Safety
    /// See [`Ref::with_raw`].
    pub unsafe fn with_raw<R>(&self, f: impl FnOnce(*const JPC_WheeledVehicleControllerSettings) -> R) -> R {
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
                JPC_VehicleCollisionTesterCastSphere_new(layer.raw(), radius, up.into_jolt(), max_slope_angle)
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

    /// # Safety
    /// See [`Ref::with_raw`].
    pub unsafe fn with_raw<R>(&self, f: impl FnOnce(*mut JPC_VehicleCollisionTester) -> R) -> R {
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
        Self { raw, _phantom: std::marker::PhantomData }
    }

    pub fn has_contact(&self) -> bool { unsafe { JPC_WheelWV_HasContact(self.raw) } }

    pub fn angular_velocity(&self) -> f32 { unsafe { JPC_WheelWV_GetAngularVelocity(self.raw) } }
    pub fn set_angular_velocity(&mut self, vel: f32) {
        unsafe { JPC_WheelWV_SetAngularVelocity(self.raw, vel) }
    }

    pub fn rotation_angle(&self) -> f32 { unsafe { JPC_WheelWV_GetRotationAngle(self.raw) } }
    pub fn set_rotation_angle(&mut self, angle: f32) {
        unsafe { JPC_WheelWV_SetRotationAngle(self.raw, angle) }
    }

    pub fn steer_angle(&self) -> f32 { unsafe { JPC_WheelWV_GetSteerAngle(self.raw) } }
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

    pub fn suspension_length(&self) -> f32 { unsafe { JPC_WheelWV_GetSuspensionLength(self.raw) } }
    pub fn suspension_lambda(&self) -> f32 { unsafe { JPC_WheelWV_GetSuspensionLambda(self.raw) } }
    pub fn longitudinal_lambda(&self) -> f32 { unsafe { JPC_WheelWV_GetLongitudinalLambda(self.raw) } }
    pub fn lateral_lambda(&self) -> f32 { unsafe { JPC_WheelWV_GetLateralLambda(self.raw) } }

    /// # Safety
    /// See [`Ref::with_raw`].
    pub unsafe fn with_raw<R>(&self, f: impl FnOnce(*mut JPC_WheelWV) -> R) -> R {
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
        Self { raw, _phantom: std::marker::PhantomData }
    }

    pub fn set_driver_input(&mut self, forward: f32, right: f32, brake: f32, hand_brake: f32) {
        unsafe { JPC_WheeledVehicleController_SetDriverInput(self.raw, forward, right, brake, hand_brake) }
    }

    pub fn forward_input(&self) -> f32 { unsafe { JPC_WheeledVehicleController_GetForwardInput(self.raw) } }
    pub fn set_forward_input(&mut self, v: f32) {
        unsafe { JPC_WheeledVehicleController_SetForwardInput(self.raw, v) }
    }

    pub fn right_input(&self) -> f32 { unsafe { JPC_WheeledVehicleController_GetRightInput(self.raw) } }
    pub fn set_right_input(&mut self, v: f32) {
        unsafe { JPC_WheeledVehicleController_SetRightInput(self.raw, v) }
    }

    pub fn brake_input(&self) -> f32 { unsafe { JPC_WheeledVehicleController_GetBrakeInput(self.raw) } }
    pub fn set_brake_input(&mut self, v: f32) {
        unsafe { JPC_WheeledVehicleController_SetBrakeInput(self.raw, v) }
    }

    pub fn hand_brake_input(&self) -> f32 { unsafe { JPC_WheeledVehicleController_GetHandBrakeInput(self.raw) } }
    pub fn set_hand_brake_input(&mut self, v: f32) {
        unsafe { JPC_WheeledVehicleController_SetHandBrakeInput(self.raw, v) }
    }

    /// # Safety
    /// See [`Ref::with_raw`].
    pub unsafe fn with_raw<R>(&self, f: impl FnOnce(*mut JPC_WheeledVehicleController) -> R) -> R {
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
        if ptr.is_null() { None } else { Some(WheelWV::new(ptr)) }
    }

    pub fn wheeled_controller(&mut self) -> WheeledVehicleController<'_> {
        let raw = unsafe { JPC_VehicleConstraint_GetWheeledController(self.raw) };
        WheeledVehicleController::new(raw)
    }

    /// Local-space basis vectors for the given wheel.
    pub fn wheel_local_basis(&self, wheel_index: u32) -> (Vec3, Vec3, Vec3) {
        let mut forward = JPC_Vec3 { x: 0.0, y: 0.0, z: 0.0, _w: 0.0 };
        let mut up = JPC_Vec3 { x: 0.0, y: 0.0, z: 0.0, _w: 0.0 };
        let mut right = JPC_Vec3 { x: 0.0, y: 0.0, z: 0.0, _w: 0.0 };
        unsafe {
            JPC_VehicleConstraint_GetWheelLocalBasis(self.raw, wheel_index, &mut forward, &mut up, &mut right)
        }
        (Vec3::from_jolt(forward), Vec3::from_jolt(up), Vec3::from_jolt(right))
    }

    /// Local-space 4x4 transform for the given wheel.
    pub fn wheel_local_transform(&self, wheel_index: u32, wheel_right: Vec3, wheel_up: Vec3) -> Mat4 {
        unsafe {
            JPC_VehicleConstraint_GetWheelLocalTransform(
                self.raw, wheel_index, wheel_right.into_jolt(), wheel_up.into_jolt()
            ).into_rolt()
        }
    }

    /// World-space 4x4 transform for the given wheel.
    pub fn wheel_world_transform(&self, wheel_index: u32, wheel_right: Vec3, wheel_up: Vec3) -> RMat4 {
        unsafe {
            JPC_VehicleConstraint_GetWheelWorldTransform(
                self.raw, wheel_index, wheel_right.into_jolt(), wheel_up.into_jolt()
            ).into_rolt()
        }
    }

    /// The underlying constraint pointer — required to add to `PhysicsSystem`.
    pub fn as_constraint(&self) -> *mut JPC_Constraint {
        unsafe { JPC_VehicleConstraint_AsConstraint(self.raw) }
    }

    /// # Safety
    /// See [`Ref::with_raw`].
    pub unsafe fn with_raw<R>(&self, f: impl FnOnce(*mut JPC_VehicleConstraint) -> R) -> R {
        f(self.raw)
    }

    pub fn raw(&self) -> *mut JPC_VehicleConstraint {
        self.raw
    }
}
