use std::ptr;

use joltc_sys::*;

use crate::{
    BodyActivationListenerImpl, BodyDrawSettings, BodyInterface, BodyLockInterface,
    BroadPhaseLayerInterfaceImpl, BroadPhaseQuery, ContactListenerImpl, DebugRendererSimpleImpl,
    JobSystem, NarrowPhaseQuery, ObjectLayerPairFilterImpl, ObjectVsBroadPhaseLayerFilterImpl,
    SimShapeFilterImpl, SoftBodyContactListenerImpl, StateRecorder, TempAllocator,
    VehicleConstraint,
};
use crate::{BodyId, FromJolt, IntoJolt, IntoRolt, Vec3};

/// The root of everything for a physics simulation.
///
/// See also: Jolt's [`PhysicsSystem`](https://jrouwe.github.io/JoltPhysicsDocs/5.1.0/class_physics_system.html) class.
pub struct PhysicsSystem {
    raw: *mut JPC_PhysicsSystem,
    broad_phase_layer_interface: Option<BroadPhaseLayerInterfaceImpl<'static>>,
    object_vs_broad_phase_layer_filter: Option<ObjectVsBroadPhaseLayerFilterImpl<'static>>,
    object_layer_pair_filter: Option<ObjectLayerPairFilterImpl<'static>>,
    sim_shape_filter: Option<SimShapeFilterImpl<'static>>,
    contact_listener: Option<ContactListenerImpl<'static>>,
    body_activation_listener: Option<BodyActivationListenerImpl<'static>>,
    soft_body_contact_listener: Option<SoftBodyContactListenerImpl<'static>>,
}

impl PhysicsSystem {
    pub fn new() -> Self {
        unsafe {
            Self {
                raw: JPC_PhysicsSystem_new(),
                broad_phase_layer_interface: None,
                object_vs_broad_phase_layer_filter: None,
                object_layer_pair_filter: None,
                sim_shape_filter: None,
                contact_listener: None,
                body_activation_listener: None,
                soft_body_contact_listener: None,
            }
        }
    }

    pub fn init(
        &mut self,
        max_bodies: u32,
        num_body_mutexes: u32,
        max_body_pairs: u32,
        max_contact_constraints: u32,
        broad_phase_layer_interface: impl Into<BroadPhaseLayerInterfaceImpl<'static>>,
        object_vs_broad_phase_layer_filter: impl Into<ObjectVsBroadPhaseLayerFilterImpl<'static>>,
        object_layer_pair_filter: impl Into<ObjectLayerPairFilterImpl<'static>>,
    ) {
        let bpli = broad_phase_layer_interface.into();
        let bpli_raw = bpli.raw();
        self.broad_phase_layer_interface = Some(bpli);

        let ovbplf = object_vs_broad_phase_layer_filter.into();
        let ovbplf_raw = ovbplf.raw();
        self.object_vs_broad_phase_layer_filter = Some(ovbplf);

        let olpf = object_layer_pair_filter.into();
        let olpf_raw = olpf.raw();
        self.object_layer_pair_filter = Some(olpf);

        unsafe {
            JPC_PhysicsSystem_Init(
                self.raw,
                max_bodies,
                num_body_mutexes,
                max_body_pairs,
                max_contact_constraints,
                bpli_raw,
                ovbplf_raw,
                olpf_raw,
            );
        }
    }

    pub fn set_sim_shape_filter(
        &mut self,
        sim_shape_filter: impl Into<SimShapeFilterImpl<'static>>,
    ) {
        let sim_shape_filter = sim_shape_filter.into();
        let raw = sim_shape_filter.raw();
        self.sim_shape_filter = Some(sim_shape_filter);

        unsafe {
            JPC_PhysicsSystem_SetSimShapeFilter(self.raw, raw);
        }
    }

    pub fn set_contact_listener(
        &mut self,
        contact_listener: Option<impl Into<ContactListenerImpl<'static>>>,
    ) {
        if let Some(contact_listener) = contact_listener {
            let contact_listener = contact_listener.into();
            let raw = contact_listener.raw();
            self.contact_listener = Some(contact_listener);

            unsafe {
                JPC_PhysicsSystem_SetContactListener(self.raw, raw);
            }
        } else {
            unsafe {
                JPC_PhysicsSystem_SetContactListener(self.raw, ptr::null_mut());
            }
        }
    }

    /// rebuild the broadphase AABBs — call once after adding all initial bodies, before the first [`update`][Self::update].
    pub fn optimize_broad_phase(&self) {
        unsafe { JPC_PhysicsSystem_OptimizeBroadPhase(self.raw) }
    }

    pub fn gravity(&self) -> crate::Vec3 {
        unsafe { JPC_PhysicsSystem_GetGravity(self.raw).into_rolt() }
    }

    pub fn set_gravity(&self, gravity: crate::Vec3) {
        unsafe { JPC_PhysicsSystem_SetGravity(self.raw, gravity.into_jolt()) }
    }

    pub fn num_bodies(&self) -> u32 {
        unsafe { JPC_PhysicsSystem_GetNumBodies(self.raw) }
    }

    pub fn num_active_bodies(&self, body_type: JPC_BodyType) -> u32 {
        unsafe { JPC_PhysicsSystem_GetNumActiveBodies(self.raw, body_type) }
    }

    /// Return all body IDs in the simulation.
    pub fn bodies(&self) -> Vec<crate::BodyId> {
        let mut count = 0u32;
        unsafe { JPC_PhysicsSystem_GetBodies(self.raw, ptr::null_mut(), &mut count) }
        let mut ids = vec![0u32; count as usize];
        unsafe { JPC_PhysicsSystem_GetBodies(self.raw, ids.as_mut_ptr(), &mut count) }
        ids.into_iter().map(crate::BodyId::new).collect()
    }

    /// Return all active body IDs of the given type.
    pub fn active_bodies(&self, body_type: JPC_BodyType) -> Vec<crate::BodyId> {
        let mut count = 0u32;
        unsafe {
            JPC_PhysicsSystem_GetActiveBodies(self.raw, body_type, ptr::null_mut(), &mut count)
        }
        let mut ids = vec![0u32; count as usize];
        unsafe {
            JPC_PhysicsSystem_GetActiveBodies(self.raw, body_type, ids.as_mut_ptr(), &mut count)
        }
        ids.into_iter().map(crate::BodyId::new).collect()
    }

    pub fn update(
        &self,
        delta_time: f32,
        collision_steps: i32,
        temp_allocator: &TempAllocator,
        job_system: &dyn JobSystem,
    ) {
        unsafe {
            JPC_PhysicsSystem_Update(
                self.raw,
                delta_time,
                collision_steps,
                temp_allocator.raw(),
                job_system.raw_job_system(),
            );
        }
    }

    pub fn add_constraint(&self, constraint: &crate::Constraint) {
        unsafe { JPC_PhysicsSystem_AddConstraint(self.raw, constraint.raw()) }
    }

    pub fn remove_constraint(&self, constraint: &crate::Constraint) {
        unsafe { JPC_PhysicsSystem_RemoveConstraint(self.raw, constraint.raw()) }
    }

    pub fn draw_bodies(
        &self,
        settings: &mut BodyDrawSettings,
        renderer: &DebugRendererSimpleImpl<'_>,
    ) {
        unsafe {
            JPC_PhysicsSystem_DrawBodies(self.raw, &mut settings.0, renderer.raw(), ptr::null());
        }
    }

    pub fn body_interface(&self) -> BodyInterface<'_> {
        unsafe {
            let raw = JPC_PhysicsSystem_GetBodyInterface(self.raw);
            BodyInterface::new(raw)
        }
    }

    pub fn body_lock_interface(&self) -> BodyLockInterface<'_> {
        unsafe {
            let raw = JPC_PhysicsSystem_GetBodyLockInterface(self.raw);
            BodyLockInterface::new(raw)
        }
    }

    pub fn narrow_phase_query(&self) -> NarrowPhaseQuery<'_> {
        unsafe {
            let raw = JPC_PhysicsSystem_GetNarrowPhaseQuery(self.raw);
            NarrowPhaseQuery::new(raw)
        }
    }

    /// Register a vehicle constraint as a step listener so it receives pre/post-physics callbacks.
    pub fn add_step_listener(&mut self, vehicle: &VehicleConstraint) {
        unsafe { JPC_PhysicsSystem_AddStepListener(self.raw, vehicle.raw()) }
    }

    pub fn remove_step_listener(&mut self, vehicle: &VehicleConstraint) {
        unsafe { JPC_PhysicsSystem_RemoveStepListener(self.raw, vehicle.raw()) }
    }

    /// serialize all physics state for deterministic replay or save/load.
    pub fn save_state(&self, recorder: &mut StateRecorder) {
        unsafe { JPC_PhysicsSystem_SaveState(self.raw, recorder.raw()) }
    }

    /// restore previously saved state — returns `false` on failure.
    pub fn restore_state(&mut self, recorder: &mut StateRecorder) -> bool {
        unsafe { JPC_PhysicsSystem_RestoreState(self.raw, recorder.raw()) }
    }

    pub fn physics_settings(&self) -> JPC_PhysicsSettings {
        unsafe { JPC_PhysicsSystem_GetPhysicsSettings(self.raw) }
    }

    pub fn set_physics_settings(&mut self, settings: JPC_PhysicsSettings) {
        unsafe { JPC_PhysicsSystem_SetPhysicsSettings(self.raw, settings) }
    }

    pub fn body_stats(&self) -> JPC_BodyStats {
        unsafe { JPC_PhysicsSystem_GetBodyStats(self.raw) }
    }

    pub fn add_constraints(&self, constraints: &[&crate::Constraint]) {
        let mut ptrs: Vec<*mut JPC_Constraint> = constraints.iter().map(|c| c.raw()).collect();
        unsafe { JPC_PhysicsSystem_AddConstraints(self.raw, ptrs.as_mut_ptr(), ptrs.len() as i32) }
    }

    pub fn remove_constraints(&self, constraints: &[&crate::Constraint]) {
        let mut ptrs: Vec<*mut JPC_Constraint> = constraints.iter().map(|c| c.raw()).collect();
        unsafe {
            JPC_PhysicsSystem_RemoveConstraints(self.raw, ptrs.as_mut_ptr(), ptrs.len() as i32)
        }
    }

    /// Body interface without body locking — use with care in multithreaded scenarios.
    pub fn body_interface_no_lock(&self) -> BodyInterface<'_> {
        unsafe {
            let raw = JPC_PhysicsSystem_GetBodyInterfaceNoLock(self.raw);
            BodyInterface::new(raw.cast_mut())
        }
    }

    /// Narrow phase query without body locking — use with care.
    pub fn narrow_phase_query_no_lock(&self) -> NarrowPhaseQuery<'_> {
        unsafe {
            let raw = JPC_PhysicsSystem_GetNarrowPhaseQueryNoLock(self.raw);
            NarrowPhaseQuery::new(raw)
        }
    }

    /// Body lock interface without body locking.
    pub fn body_lock_interface_no_lock(&self) -> BodyLockInterface<'_> {
        unsafe {
            let raw = JPC_PhysicsSystem_GetBodyLockInterfaceNoLock(self.raw);
            BodyLockInterface::new(raw)
        }
    }

    /// Returns `true` if the two bodies were in contact during the last simulation step.
    pub fn were_bodies_in_contact(&self, body1: BodyId, body2: BodyId) -> bool {
        unsafe { JPC_PhysicsSystem_WereBodiesInContact(self.raw, body1.raw(), body2.raw()) }
    }

    /// Bounding box containing all bodies.
    pub fn bounds(&self) -> (Vec3, Vec3) {
        let b = unsafe { JPC_PhysicsSystem_GetBounds(self.raw) };
        (Vec3::from_jolt(b.Min), Vec3::from_jolt(b.Max))
    }

    pub fn set_body_activation_listener(
        &mut self,
        listener: Option<impl Into<BodyActivationListenerImpl<'static>>>,
    ) {
        if let Some(listener) = listener {
            let listener = listener.into();
            let raw = listener.raw();
            self.body_activation_listener = Some(listener);
            unsafe { JPC_PhysicsSystem_SetBodyActivationListener(self.raw, raw) }
        } else {
            self.body_activation_listener = None;
            unsafe { JPC_PhysicsSystem_SetBodyActivationListener(self.raw, ptr::null_mut()) }
        }
    }

    /// Returns the currently installed body activation listener, if any.
    pub fn body_activation_listener(&self) -> Option<&BodyActivationListenerImpl<'static>> {
        self.body_activation_listener.as_ref()
    }

    pub fn set_combine_friction(&mut self, function: JPC_CombineFunction) {
        unsafe { JPC_PhysicsSystem_SetCombineFriction(self.raw, function) }
    }

    pub fn set_combine_restitution(&mut self, function: JPC_CombineFunction) {
        unsafe { JPC_PhysicsSystem_SetCombineRestitution(self.raw, function) }
    }

    pub fn broad_phase_query(&self) -> BroadPhaseQuery<'_> {
        unsafe {
            let raw = JPC_PhysicsSystem_GetBroadPhaseQuery(self.raw);
            BroadPhaseQuery::new(raw)
        }
    }

    /// draw constraint limit axes (only available with `JPH_DEBUG_RENDERER`).
    pub fn draw_constraint_limits(&mut self, renderer: &DebugRendererSimpleImpl<'_>) {
        unsafe { JPC_PhysicsSystem_DrawConstraintLimits(self.raw, renderer.raw()) }
    }

    /// draw constraint reference frames (only available with `JPH_DEBUG_RENDERER`).
    pub fn draw_constraint_reference_frame(&mut self, renderer: &DebugRendererSimpleImpl<'_>) {
        unsafe { JPC_PhysicsSystem_DrawConstraintReferenceFrame(self.raw, renderer.raw()) }
    }

    /// draw all constraints (only available with `JPH_DEBUG_RENDERER`).
    pub fn draw_constraints(&mut self, renderer: &DebugRendererSimpleImpl<'_>) {
        unsafe { JPC_PhysicsSystem_DrawConstraints(self.raw, renderer.raw()) }
    }

    pub fn set_soft_body_contact_listener(
        &mut self,
        listener: Option<impl Into<SoftBodyContactListenerImpl<'static>>>,
    ) {
        if let Some(listener) = listener {
            let listener = listener.into();
            let raw = listener.raw();
            self.soft_body_contact_listener = Some(listener);
            unsafe { JPC_PhysicsSystem_SetSoftBodyContactListener(self.raw, raw) }
        } else {
            self.soft_body_contact_listener = None;
            unsafe { JPC_PhysicsSystem_SetSoftBodyContactListener(self.raw, ptr::null_mut()) }
        }
    }

    /// Returns the currently installed soft body contact listener, if any.
    pub fn soft_body_contact_listener(&self) -> Option<&SoftBodyContactListenerImpl<'static>> {
        self.soft_body_contact_listener.as_ref()
    }

    pub fn with_raw<R>(&self, f: impl FnOnce(*mut JPC_PhysicsSystem) -> R) -> R {
        f(self.raw)
    }

    pub fn raw(&self) -> *mut JPC_PhysicsSystem {
        self.raw
    }
}

impl Drop for PhysicsSystem {
    fn drop(&mut self) {
        unsafe {
            JPC_PhysicsSystem_delete(self.raw);
        }
    }
}
