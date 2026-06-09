use std::marker::PhantomData;

use joltc_sys::*;

use crate::{BroadPhaseLayerFilterImpl, FromJolt, IntoJolt, ObjectLayerFilterImpl, Vec3};

/// Opaque handle to Jolt's BroadPhaseQuery interface.
///
/// See also: Jolt's [`BroadPhaseQuery`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_broad_phase_query.html) class.
pub struct BroadPhaseQuery<'physics_system> {
    raw: *const JPC_BroadPhaseQuery,
    _phantom: PhantomData<&'physics_system ()>,
}

impl<'physics_system> BroadPhaseQuery<'physics_system> {
    pub(crate) fn new(raw: *const JPC_BroadPhaseQuery) -> Self {
        Self { raw, _phantom: PhantomData }
    }

    /// Bounding box that contains all bodies in the broad phase.
    pub fn bounds(&self) -> (Vec3, Vec3) {
        let b = unsafe { JPC_BroadPhaseQuery_GetBounds(self.raw) };
        (Vec3::from_jolt(b.Min), Vec3::from_jolt(b.Max))
    }

    /// Cast a ray and collect overlapping body IDs + fraction.
    pub fn cast_ray(
        &self,
        origin: Vec3,
        direction: Vec3,
        collector: &mut BroadPhaseCastCollector<'_>,
        broad_phase_layer_filter: Option<&BroadPhaseLayerFilterImpl<'_>>,
        object_layer_filter: Option<&ObjectLayerFilterImpl<'_>>,
    ) {
        unsafe {
            JPC_BroadPhaseQuery_CastRay(
                self.raw,
                origin.into_jolt(),
                direction.into_jolt(),
                collector.raw,
                broad_phase_layer_filter.map_or(std::ptr::null(), |f| f.raw()),
                object_layer_filter.map_or(std::ptr::null(), |f| f.raw()),
            )
        }
    }

    /// Collide an axis-aligned box and collect overlapping body IDs.
    pub fn collide_aa_box(
        &self,
        box_min: Vec3,
        box_max: Vec3,
        collector: &mut BodyIDCollector<'_>,
        broad_phase_layer_filter: Option<&BroadPhaseLayerFilterImpl<'_>>,
        object_layer_filter: Option<&ObjectLayerFilterImpl<'_>>,
    ) {
        let aa_box = JPC_AABox { Min: box_min.into_jolt(), Max: box_max.into_jolt() };
        unsafe {
            JPC_BroadPhaseQuery_CollideAABox(
                self.raw,
                &aa_box,
                collector.raw,
                broad_phase_layer_filter.map_or(std::ptr::null(), |f| f.raw()),
                object_layer_filter.map_or(std::ptr::null(), |f| f.raw()),
            )
        }
    }

    /// Collide a sphere and collect overlapping body IDs.
    pub fn collide_sphere(
        &self,
        center: Vec3,
        radius: f32,
        collector: &mut BodyIDCollector<'_>,
        broad_phase_layer_filter: Option<&BroadPhaseLayerFilterImpl<'_>>,
        object_layer_filter: Option<&ObjectLayerFilterImpl<'_>>,
    ) {
        unsafe {
            JPC_BroadPhaseQuery_CollideSphere(
                self.raw,
                center.into_jolt(),
                radius,
                collector.raw,
                broad_phase_layer_filter.map_or(std::ptr::null(), |f| f.raw()),
                object_layer_filter.map_or(std::ptr::null(), |f| f.raw()),
            )
        }
    }

    /// Collide a point and collect overlapping body IDs.
    pub fn collide_point(
        &self,
        point: Vec3,
        collector: &mut BodyIDCollector<'_>,
        broad_phase_layer_filter: Option<&BroadPhaseLayerFilterImpl<'_>>,
        object_layer_filter: Option<&ObjectLayerFilterImpl<'_>>,
    ) {
        unsafe {
            JPC_BroadPhaseQuery_CollidePoint(
                self.raw,
                point.into_jolt(),
                collector.raw,
                broad_phase_layer_filter.map_or(std::ptr::null(), |f| f.raw()),
                object_layer_filter.map_or(std::ptr::null(), |f| f.raw()),
            )
        }
    }

    /// Cast an oriented box and collect overlapping body IDs + fraction.
    pub fn cast_aa_box(
        &self,
        box_min: Vec3,
        box_max: Vec3,
        direction: Vec3,
        collector: &mut BroadPhaseCastCollector<'_>,
        broad_phase_layer_filter: Option<&BroadPhaseLayerFilterImpl<'_>>,
        object_layer_filter: Option<&ObjectLayerFilterImpl<'_>>,
    ) {
        let cast = JPC_AABoxCast {
            Box: JPC_AABox { Min: box_min.into_jolt(), Max: box_max.into_jolt() },
            Direction: direction.into_jolt(),
        };
        unsafe {
            JPC_BroadPhaseQuery_CastAABox(
                self.raw,
                &cast,
                collector.raw,
                broad_phase_layer_filter.map_or(std::ptr::null(), |f| f.raw()),
                object_layer_filter.map_or(std::ptr::null(), |f| f.raw()),
            )
        }
    }

    pub fn raw(&self) -> *const JPC_BroadPhaseQuery { self.raw }
}

/// Result from a broad phase cast query (ray or box).
#[derive(Debug, Clone, Copy)]
pub struct BroadPhaseCastResult {
    pub body_id: crate::BodyId,
    pub fraction: f32,
}

/// Collector for broad phase cast results (ray or box cast).
pub struct BroadPhaseCastCollector<'a> {
    raw: *mut JPC_BroadPhaseCastCollector,
    #[allow(dead_code)]
    bridge: Box<BroadPhaseCastBridge>,
    _marker: PhantomData<&'a ()>,
}

struct BroadPhaseCastBridge {
    hits: Vec<BroadPhaseCastResult>,
}

unsafe extern "C" fn cast_reset(self_ptr: *mut std::ffi::c_void) {
    let bridge = &mut *(self_ptr as *mut BroadPhaseCastBridge);
    bridge.hits.clear();
}

unsafe extern "C" fn cast_add_hit(
    self_ptr: *mut std::ffi::c_void,
    _base: *mut JPC_BroadPhaseCastCollector,
    result: *const JPC_BroadPhaseCastResult,
) {
    let bridge = &mut *(self_ptr as *mut BroadPhaseCastBridge);
    bridge.hits.push(BroadPhaseCastResult {
        body_id: crate::BodyId::new((*result).BodyID),
        fraction: (*result).Fraction,
    });
}

impl<'a> BroadPhaseCastCollector<'a> {
    pub fn new() -> Self {
        let mut bridge = Box::new(BroadPhaseCastBridge { hits: Vec::new() });
        let fns = JPC_BroadPhaseCastCollectorFns {
            Reset: Some(cast_reset),
            AddHit: Some(cast_add_hit),
        };
        let raw = unsafe {
            JPC_BroadPhaseCastCollector_new(bridge.as_mut() as *mut BroadPhaseCastBridge as *mut _, fns)
        };
        Self { raw, bridge, _marker: PhantomData }
    }

    pub fn hits(&self) -> &[BroadPhaseCastResult] {
        &self.bridge.hits
    }

    pub fn reset(&mut self) {
        self.bridge.hits.clear();
        unsafe { JPC_BroadPhaseCastCollector_UpdateEarlyOutFraction(self.raw, 1.0 + f32::EPSILON) }
    }
}

impl Default for BroadPhaseCastCollector<'_> {
    fn default() -> Self { Self::new() }
}

impl Drop for BroadPhaseCastCollector<'_> {
    fn drop(&mut self) {
        unsafe { JPC_BroadPhaseCastCollector_delete(self.raw) }
    }
}

/// Collector for broad phase collide results (body IDs only).
pub struct BodyIDCollector<'a> {
    raw: *mut JPC_BodyIDCollector,
    #[allow(dead_code)]
    bridge: Box<BodyIDBridge>,
    _marker: PhantomData<&'a ()>,
}

struct BodyIDBridge {
    hits: Vec<crate::BodyId>,
}

unsafe extern "C" fn body_id_reset(self_ptr: *mut std::ffi::c_void) {
    let bridge = &mut *(self_ptr as *mut BodyIDBridge);
    bridge.hits.clear();
}

unsafe extern "C" fn body_id_add_hit(self_ptr: *mut std::ffi::c_void, body_id: JPC_BodyID) {
    let bridge = &mut *(self_ptr as *mut BodyIDBridge);
    bridge.hits.push(crate::BodyId::new(body_id));
}

impl<'a> BodyIDCollector<'a> {
    pub fn new() -> Self {
        let mut bridge = Box::new(BodyIDBridge { hits: Vec::new() });
        let fns = JPC_BodyIDCollectorFns {
            Reset: Some(body_id_reset),
            AddHit: Some(body_id_add_hit),
        };
        let raw = unsafe {
            JPC_BodyIDCollector_new(bridge.as_mut() as *mut BodyIDBridge as *mut _, fns)
        };
        Self { raw, bridge, _marker: PhantomData }
    }

    pub fn hits(&self) -> &[crate::BodyId] {
        &self.bridge.hits
    }

    pub fn reset(&mut self) {
        self.bridge.hits.clear();
    }
}

impl Default for BodyIDCollector<'_> {
    fn default() -> Self { Self::new() }
}

impl Drop for BodyIDCollector<'_> {
    fn drop(&mut self) {
        unsafe { JPC_BodyIDCollector_delete(self.raw) }
    }
}
