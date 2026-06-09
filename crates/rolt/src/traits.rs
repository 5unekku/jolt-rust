#![allow(non_snake_case)]

use std::ffi::{c_char, c_uint, c_void};
use std::marker::PhantomData;

use joltc_sys::*;
use paste::paste;

use crate::remote_drop::RemoteDrop;
use crate::{Body, BodyId, BroadPhaseLayer, Color, FromJolt, IntoJolt, ObjectLayer, RVec3};

macro_rules! define_impl_struct {
    (
        $mutability:ident
        $base_name:ident {
            $($method:ident),* $(,)?
        }
    ) => {
        paste! {
            #[allow(dead_code)]
            #[doc = "Holds an implementation of the [" $base_name "] trait or the manual vtable equivalent."]
            pub struct [<$base_name Impl>]<'a> {
                raw: *mut [<JPC_ $base_name >],
                remote_this: Option<RemoteDrop>,
                _marker: PhantomData<&'a ()>,
            }

            impl [<$base_name Impl>]<'static> {
                pub fn new<T: $base_name + 'static>(value: T) -> Self {
                    type Bridge<T> = [< $base_name Bridge >]<T>;

                    let fns = [<JPC_ $base_name Fns>] {
                        $(
                            $method: Some(Bridge::<T>::$method as _),
                        )*
                    };

                    let this = Box::into_raw(Box::new(value));

                    let raw = unsafe { [<JPC_ $base_name _new>](this.cast::<c_void>(), fns) };
                    let remote_this = unsafe { RemoteDrop::new(this) };

                    Self {
                        raw,
                        remote_this: Some(remote_this),
                        _marker: PhantomData,
                    }
                }

                pub unsafe fn from_raw(this: *$mutability c_void, fns: [<JPC_ $base_name Fns>]) -> Self {
                    let raw = unsafe { [<JPC_ $base_name _new>](this, fns) };

                    Self {
                        raw,
                        remote_this: None,
                        _marker: PhantomData,
                    }
                }

                pub unsafe fn new_existing(raw: *mut [<JPC_ $base_name>]) -> Self {
                    Self {
                        raw,
                        remote_this: None,
                        _marker: PhantomData,
                    }
                }
            }

            impl<'a> [<$base_name Impl>]<'a> {
                pub fn new_borrowed<T: $base_name + 'a>(value: &'a mut T) -> Self {
                    type Bridge<T> = [< $base_name Bridge >]<T>;

                    let fns = [<JPC_ $base_name Fns>] {
                        $(
                            $method: Some(Bridge::<T>::$method as _),
                        )*
                    };

                    let this = std::ptr::from_mut(value);
                    let raw = unsafe { [<JPC_ $base_name _new>](this.cast::<c_void>(), fns) };

                    Self {
                        raw,
                        remote_this: None,
                        _marker: PhantomData,
                    }
                }

                pub fn raw(&self) -> *mut [<JPC_ $base_name>] {
                    self.raw
                }
            }

            impl<'a> Drop for [<$base_name Impl>]<'a> {
                fn drop(&mut self) {
                    unsafe {
                        [<JPC_ $base_name _delete>](self.raw);
                    }
                }
            }

            impl<'a> IntoJolt for Option<&'a [<$base_name Impl>]<'a>> {
                // FIXME: Should be const
                type Jolt = *mut [<JPC_ $base_name>];

                fn into_jolt(self) -> Self::Jolt {
                    match self {
                        Some(v) => v.raw(),
                        None => std::ptr::null_mut(),
                    }
                }
            }

            impl<T> From<T> for [<$base_name Impl>]<'static>
            where
                T: $base_name + 'static,
            {
                fn from(value: T) -> Self {
                    Self::new(value)
                }
            }
        }
    };
}

/// See also: Jolt's [`ContactListener`](https://jrouwe.github.io/JoltPhysicsDocs/5.1.0/class_contact_listener.html) class.
pub trait ContactListener {
    fn on_contact_validate(
        &self,
        body1: &JPC_Body,
        body2: &JPC_Body,
        base_offset: JPC_RVec3,
        collision_result: &JPC_CollideShapeResult,
    ) -> JPC_ValidateResult;

    fn on_contact_added(
        &self,
        body1: &JPC_Body,
        body2: &JPC_Body,
        manifold: &JPC_ContactManifold,
        settings: &mut JPC_ContactSettings,
    );

    fn on_contact_persisted(
        &self,
        body1: &JPC_Body,
        body2: &JPC_Body,
        manifold: &JPC_ContactManifold,
        settings: &mut JPC_ContactSettings,
    );

    fn on_contact_removed(&self, sub_shape_pair: &JPC_SubShapeIDPair);
}

define_impl_struct!(mut ContactListener {
    OnContactValidate,
    OnContactAdded,
    OnContactPersisted,
    OnContactRemoved,
});

struct ContactListenerBridge<T> {
    _phantom: PhantomData<T>,
}

impl<T: ContactListener> ContactListenerBridge<T> {
    unsafe extern "C" fn OnContactValidate(
        this: *mut c_void,
        body1: *const JPC_Body,
        body2: *const JPC_Body,
        base_offset: JPC_RVec3,
        collision_result: *const JPC_CollideShapeResult,
    ) -> JPC_ValidateResult {
        let this = this.cast::<T>().as_ref().unwrap();
        this.on_contact_validate(&*body1, &*body2, base_offset, &*collision_result)
    }

    unsafe extern "C" fn OnContactAdded(
        this: *mut c_void,
        body1: *const JPC_Body,
        body2: *const JPC_Body,
        manifold: *const JPC_ContactManifold,
        settings: *mut JPC_ContactSettings,
    ) {
        let this = this.cast::<T>().as_ref().unwrap();
        this.on_contact_added(&*body1, &*body2, &*manifold, &mut *settings)
    }

    unsafe extern "C" fn OnContactPersisted(
        this: *mut c_void,
        body1: *const JPC_Body,
        body2: *const JPC_Body,
        manifold: *const JPC_ContactManifold,
        settings: *mut JPC_ContactSettings,
    ) {
        let this = this.cast::<T>().as_ref().unwrap();
        this.on_contact_persisted(&*body1, &*body2, &*manifold, &mut *settings)
    }

    unsafe extern "C" fn OnContactRemoved(
        this: *mut c_void,
        sub_shape_pair: *const JPC_SubShapeIDPair,
    ) {
        let this = this.cast::<T>().as_ref().unwrap();
        this.on_contact_removed(&*sub_shape_pair);
    }
}

/// See also: Jolt's [`GroupFilter`](https://jrouwe.github.io/JoltPhysicsDocs/5.1.0/class_group_filter.html) class.
pub trait GroupFilter {
    fn can_collide(&self, group_1: &JPC_CollisionGroup, group_2: &JPC_CollisionGroup) -> bool;
}

define_impl_struct!(const GroupFilter {
    CanCollide,
});

struct GroupFilterBridge<T> {
    _phantom: PhantomData<T>,
}

impl<T: GroupFilter> GroupFilterBridge<T> {
    unsafe extern "C" fn CanCollide(
        this: *const c_void,
        group_1: *const JPC_CollisionGroup,
        group_2: *const JPC_CollisionGroup,
    ) -> bool {
        let this = this.cast::<T>().as_ref().unwrap();
        let group_1 = &*group_1;
        let group_2 = &*group_2;

        this.can_collide(group_1, group_2)
    }
}

/// See also: Jolt's [`BroadPhaseLayerInterface`](https://jrouwe.github.io/JoltPhysicsDocs/5.1.0/class_broad_phase_layer_interface.html) class.
pub trait BroadPhaseLayerInterface {
    fn get_num_broad_phase_layers(&self) -> u32;
    fn get_broad_phase_layer(&self, layer: ObjectLayer) -> BroadPhaseLayer;
    /// Returns a debug name for the given broad phase layer. Only called when Jolt profiling is enabled.
    fn get_broad_phase_layer_name(&self, _layer: BroadPhaseLayer) -> Option<&'static std::ffi::CStr> { None }
}

define_impl_struct!(const BroadPhaseLayerInterface {
    GetNumBroadPhaseLayers,
    GetBroadPhaseLayer,
    GetBroadPhaseLayerName,
});

struct BroadPhaseLayerInterfaceBridge<T> {
    _phantom: PhantomData<T>,
}

impl<T: BroadPhaseLayerInterface> BroadPhaseLayerInterfaceBridge<T> {
    unsafe extern "C" fn GetNumBroadPhaseLayers(this: *const c_void) -> c_uint {
        let this = this.cast::<T>().as_ref().unwrap();

        this.get_num_broad_phase_layers()
    }

    unsafe extern "C" fn GetBroadPhaseLayer(
        this: *const c_void,
        layer: JPC_ObjectLayer,
    ) -> JPC_BroadPhaseLayer {
        let this = this.cast::<T>().as_ref().unwrap();
        let layer = ObjectLayer::new(layer);

        this.get_broad_phase_layer(layer).raw()
    }

    unsafe extern "C" fn GetBroadPhaseLayerName(
        this: *const c_void,
        layer: JPC_BroadPhaseLayer,
    ) -> *const c_char {
        let this = this.cast::<T>().as_ref().unwrap();
        let layer = BroadPhaseLayer::new(layer);

        match this.get_broad_phase_layer_name(layer) {
            None => std::ptr::null(),
            Some(s) => s.as_ptr(),
        }
    }
}

/// See also: Jolt's [`ObjectVsBroadPhaseLayerFilter`](https://jrouwe.github.io/JoltPhysicsDocs/5.1.0/class_object_vs_broad_phase_layer_filter.html) class.
pub trait ObjectVsBroadPhaseLayerFilter {
    fn should_collide(&self, layer1: ObjectLayer, layer2: BroadPhaseLayer) -> bool;
}

define_impl_struct!(const ObjectVsBroadPhaseLayerFilter { ShouldCollide });

struct ObjectVsBroadPhaseLayerFilterBridge<T> {
    _phantom: PhantomData<T>,
}

impl<T: ObjectVsBroadPhaseLayerFilter> ObjectVsBroadPhaseLayerFilterBridge<T> {
    unsafe extern "C" fn ShouldCollide(
        this: *const c_void,
        layer1: JPC_ObjectLayer,
        layer2: JPC_BroadPhaseLayer,
    ) -> bool {
        let this = this.cast::<T>().as_ref().unwrap();
        let layer1 = ObjectLayer::new(layer1);
        let layer2 = BroadPhaseLayer::new(layer2);

        this.should_collide(layer1, layer2)
    }
}

/// See also: Jolt's [`ObjectLayerPairFilter`](https://jrouwe.github.io/JoltPhysicsDocs/5.1.0/class_object_layer_pair_filter.html) class.
pub trait ObjectLayerPairFilter {
    fn should_collide(&self, layer1: ObjectLayer, layer2: ObjectLayer) -> bool;
}

define_impl_struct!(const ObjectLayerPairFilter { ShouldCollide });

struct ObjectLayerPairFilterBridge<T> {
    _phantom: PhantomData<T>,
}

impl<T: ObjectLayerPairFilter> ObjectLayerPairFilterBridge<T> {
    unsafe extern "C" fn ShouldCollide(
        this: *const c_void,
        layer1: JPC_ObjectLayer,
        layer2: JPC_ObjectLayer,
    ) -> bool {
        let this = this.cast::<T>().as_ref().unwrap();
        let layer1 = ObjectLayer::new(layer1);
        let layer2 = ObjectLayer::new(layer2);

        this.should_collide(layer1, layer2)
    }
}

/// See also: Jolt's [`BroadPhaseLayerFilter`](https://jrouwe.github.io/JoltPhysicsDocs/5.1.0/class_broad_phase_layer_filter.html) class.
pub trait BroadPhaseLayerFilter {
    fn should_collide(&self, layer: BroadPhaseLayer) -> bool;
}

define_impl_struct!(const BroadPhaseLayerFilter { ShouldCollide });

struct BroadPhaseLayerFilterBridge<T> {
    _phantom: PhantomData<T>,
}

impl<T: BroadPhaseLayerFilter> BroadPhaseLayerFilterBridge<T> {
    unsafe extern "C" fn ShouldCollide(this: *const c_void, layer: JPC_BroadPhaseLayer) -> bool {
        let this = this.cast::<T>().as_ref().unwrap();
        let layer = BroadPhaseLayer::new(layer);

        this.should_collide(layer)
    }
}

/// See also: Jolt's [`ObjectLayerFilter`](https://jrouwe.github.io/JoltPhysicsDocs/5.1.0/class_object_layer_filter.html) class.
pub trait ObjectLayerFilter {
    fn should_collide(&self, layer: ObjectLayer) -> bool;
}

define_impl_struct!(const ObjectLayerFilter { ShouldCollide });

struct ObjectLayerFilterBridge<T> {
    _phantom: PhantomData<T>,
}

impl<T: ObjectLayerFilter> ObjectLayerFilterBridge<T> {
    unsafe extern "C" fn ShouldCollide(this: *const c_void, layer: JPC_ObjectLayer) -> bool {
        let this = this.cast::<T>().as_ref().unwrap();
        let layer = ObjectLayer::new(layer);

        this.should_collide(layer)
    }
}

/// See also: Jolt's [`BodyFilter`](https://jrouwe.github.io/JoltPhysicsDocs/5.1.0/class_body_filter.html) class.
pub trait BodyFilter {
    fn should_collide(&self, body_id: BodyId) -> bool;
    fn should_collide_locked(&self, body: &mut Body) -> bool;
}

define_impl_struct!(const BodyFilter {
    ShouldCollide,
    ShouldCollideLocked
});

struct BodyFilterBridge<T> {
    _phantom: PhantomData<T>,
}

impl<T: BodyFilter> BodyFilterBridge<T> {
    unsafe extern "C" fn ShouldCollide(this: *const c_void, body_id: JPC_BodyID) -> bool {
        let this = this.cast::<T>().as_ref().unwrap();
        let body_id = BodyId::new(body_id);

        this.should_collide(body_id)
    }

    unsafe extern "C" fn ShouldCollideLocked(this: *const c_void, body: *const JPC_Body) -> bool {
        let this = this.cast::<T>().as_ref().unwrap();

        // FIXME: cast_mut should not be required here
        let mut body = Body::new(body.cast_mut());

        this.should_collide_locked(&mut body)
    }
}

/// See also: Jolt's [`ShapeFilter`](https://jrouwe.github.io/JoltPhysicsDocs/5.1.0/class_shape_filter.html) class.
#[allow(unused)]
pub trait ShapeFilter {
    fn should_collide(&self, shape2: *const JPC_Shape, subshape_id2: JPC_SubShapeID) -> bool {
        true
    }

    fn should_collide_two_shapes(
        &self,
        shape1: *const JPC_Shape,
        subshape_id1: JPC_SubShapeID,
        shape2: *const JPC_Shape,
        subshape_id2: JPC_SubShapeID,
    ) -> bool {
        true
    }
}

define_impl_struct!(const ShapeFilter {
    ShouldCollide,
    ShouldCollideTwoShapes,
});

struct ShapeFilterBridge<T> {
    _phantom: PhantomData<T>,
}

impl<T: ShapeFilter> ShapeFilterBridge<T> {
    unsafe extern "C" fn ShouldCollide(
        this: *const c_void,
        shape2: *const JPC_Shape,
        subshape_id2: JPC_SubShapeID,
    ) -> bool {
        let this = this.cast::<T>().as_ref().unwrap();

        this.should_collide(shape2, subshape_id2)
    }

    unsafe extern "C" fn ShouldCollideTwoShapes(
        this: *const c_void,
        shape1: *const JPC_Shape,
        subshape_id1: JPC_SubShapeID,
        shape2: *const JPC_Shape,
        subshape_id2: JPC_SubShapeID,
    ) -> bool {
        let this = this.cast::<T>().as_ref().unwrap();

        this.should_collide_two_shapes(shape1, subshape_id1, shape2, subshape_id2)
    }
}

/// See also: Jolt's [`SimShapeFilter`](https://jrouwe.github.io/JoltPhysicsDocs/5.1.0/class_sim_shape_filter.html) class.
pub trait SimShapeFilter {
    fn should_collide(
        &self,
        body1: Body<'_>,
        shape1: *const JPC_Shape,
        subshape1: JPC_SubShapeID,
        body2: Body<'_>,
        shape2: *const JPC_Shape,
        subshape2: JPC_SubShapeID,
    ) -> bool;
}

define_impl_struct!(const SimShapeFilter {
    ShouldCollide,
});

struct SimShapeFilterBridge<T> {
    _phantom: PhantomData<T>,
}

impl<T: SimShapeFilter> SimShapeFilterBridge<T> {
    unsafe extern "C" fn ShouldCollide(
        this: *const c_void,
        body1: *const JPC_Body,
        shape1: *const JPC_Shape,
        subshape1: JPC_SubShapeID,
        body2: *const JPC_Body,
        shape2: *const JPC_Shape,
        subshape2: JPC_SubShapeID,
    ) -> bool {
        let this = this.cast::<T>().as_ref().unwrap();

        // FIXME: `Body` should support a `const` version!
        let body1 = Body::new(body1.cast_mut());
        let body2 = Body::new(body2.cast_mut());

        this.should_collide(body1, shape1, subshape1, body2, shape2, subshape2)
    }
}

/// receives hits from [`NarrowPhaseQuery::cast_shape`][crate::NarrowPhaseQuery::cast_shape].
pub trait CastShapeCollector {
    fn reset(&mut self);
    fn add_hit(&mut self, base: &mut CastShapeBase, result: &JPC_ShapeCastResult);
}

/// handle passed to [`CastShapeCollector::add_hit`] — used to tune early-out.
pub struct CastShapeBase {
    base: *mut JPC_CastShapeCollector,
}

impl CastShapeBase {
    /// set the fraction beyond which further hits are ignored (default 1.0).
    /// call with the current hit's fraction to stop collecting farther hits.
    pub fn update_early_out_fraction(&mut self, fraction: f32) {
        unsafe {
            JPC_CastShapeCollector_UpdateEarlyOutFraction(self.base, fraction);
        }
    }
}

define_impl_struct!(mut CastShapeCollector { Reset, AddHit });

struct CastShapeCollectorBridge<T> {
    _phantom: PhantomData<T>,
}

impl<T: CastShapeCollector> CastShapeCollectorBridge<T> {
    unsafe extern "C" fn AddHit(
        this: *mut c_void,
        base: *mut JPC_CastShapeCollector,
        result: *const JPC_ShapeCastResult,
    ) {
        let this = this.cast::<T>().as_mut().unwrap();
        let mut base = CastShapeBase { base };
        let result = &*result;

        this.add_hit(&mut base, result);
    }

    unsafe extern "C" fn Reset(this: *mut c_void) {
        let this = this.cast::<T>().as_mut().unwrap();

        this.reset();
    }
}

/// receives hits from [`NarrowPhaseQuery::collide_shape`][crate::NarrowPhaseQuery::collide_shape].
pub trait CollideShapeCollector {
    fn reset(&mut self);
    fn add_hit(&mut self, base: &mut CollideShapeBase, result: &JPC_CollideShapeResult);
}

/// handle passed to [`CollideShapeCollector::add_hit`] — used to tune early-out.
pub struct CollideShapeBase {
    base: *mut JPC_CollideShapeCollector,
}

impl CollideShapeBase {
    /// limit the penetration depth beyond which further hits are ignored.
    pub fn update_early_out_fraction(&mut self, fraction: f32) {
        unsafe {
            JPC_CollideShapeCollector_UpdateEarlyOutFraction(self.base, fraction);
        }
    }
}

define_impl_struct!(mut CollideShapeCollector { Reset, AddHit });

struct CollideShapeCollectorBridge<T> {
    _phantom: PhantomData<T>,
}

impl<T: CollideShapeCollector> CollideShapeCollectorBridge<T> {
    unsafe extern "C" fn AddHit(
        this: *mut c_void,
        base: *mut JPC_CollideShapeCollector,
        result: *const JPC_CollideShapeResult,
    ) {
        let this = this.cast::<T>().as_mut().unwrap();
        let mut base = CollideShapeBase { base };
        let result = &*result;

        this.add_hit(&mut base, result);
    }

    unsafe extern "C" fn Reset(this: *mut c_void) {
        let this = this.cast::<T>().as_mut().unwrap();

        this.reset();
    }
}

/// See also: Jolt's [`DebugRendererSimple`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_debug_renderer_simple.html) class.
pub trait DebugRendererSimple {
    fn draw_line(&self, from: RVec3, to: RVec3, color: Color);
}

define_impl_struct!(const DebugRendererSimple { DrawLine });

struct DebugRendererSimpleBridge<T> {
    _phantom: PhantomData<T>,
}

impl<T: DebugRendererSimple> DebugRendererSimpleBridge<T> {
    unsafe extern "C" fn DrawLine(
        this: *const c_void,
        from: JPC_RVec3,
        to: JPC_RVec3,
        color: JPC_Color,
    ) {
        let this = this.cast::<T>().as_ref().unwrap();
        this.draw_line(RVec3::from_jolt(from), RVec3::from_jolt(to), Color::from_jolt(color));
    }
}

/// See also: Jolt's [`PhysicsMaterial`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_physics_material.html) class.
pub trait PhysicsMaterial {
    fn debug_name(&self) -> &str;
    fn debug_color(&self) -> Color;
}

struct PhysicsMaterialBridge<T> {
    _phantom: PhantomData<T>,
}

impl<T: PhysicsMaterial> PhysicsMaterialBridge<T> {
    unsafe extern "C" fn get_debug_name(this: *mut c_void) -> *const std::os::raw::c_char {
        let this = this.cast::<T>().as_ref().unwrap();
        this.debug_name().as_ptr().cast()
    }

    unsafe extern "C" fn get_debug_color(this: *mut c_void) -> JPC_Color {
        let this = this.cast::<T>().as_ref().unwrap();
        this.debug_color().into_jolt()
    }
}

/// Holds an implementation of the [`PhysicsMaterial`] trait.
pub struct PhysicsMaterialImpl<'a> {
    raw: *mut JPC_PhysicsMaterial,
    #[allow(dead_code)]
    remote_this: Option<RemoteDrop>,
    _marker: PhantomData<&'a ()>,
}

impl PhysicsMaterialImpl<'static> {
    pub fn new<T: PhysicsMaterial + 'static>(value: T) -> Self {
        let fns = JPC_PhysicsMaterialFns {
            GetDebugName: Some(PhysicsMaterialBridge::<T>::get_debug_name),
            GetDebugColor: Some(PhysicsMaterialBridge::<T>::get_debug_color),
        };
        let this = Box::into_raw(Box::new(value));
        let raw = unsafe { JPC_PhysicsMaterial_new(this.cast::<c_void>(), fns) };
        let remote_this = unsafe { RemoteDrop::new(this) };
        Self { raw, remote_this: Some(remote_this), _marker: PhantomData }
    }
}

impl<'a> PhysicsMaterialImpl<'a> {
    pub fn raw(&self) -> *mut JPC_PhysicsMaterial { self.raw }
}

impl<'a> Drop for PhysicsMaterialImpl<'a> {
    fn drop(&mut self) {
        unsafe { JPC_PhysicsMaterial_Release(self.raw) }
    }
}

impl<T: PhysicsMaterial + 'static> From<T> for PhysicsMaterialImpl<'static> {
    fn from(value: T) -> Self { Self::new(value) }
}

/// Get the debug name of a pre-existing physics material (e.g. from a shape).
pub fn physics_material_debug_name(material: &crate::RefConst<JPC_PhysicsMaterial>) -> String {
    unsafe {
        let ptr = JPC_PhysicsMaterial_GetDebugName(material.get());
        if ptr.is_null() {
            return String::new();
        }
        std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}

/// Get the singleton default physics material.
pub fn physics_material_default() -> crate::RefConst<JPC_PhysicsMaterial> {
    unsafe { crate::RefConst::from_active(JPC_PhysicsMaterial_GetDefault()) }
}

/// See also: Jolt's [`CharacterContactListener`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_character_contact_listener.html) class.
pub trait CharacterContactListener {
    fn on_adjust_body_velocity(
        &self,
        character: *const JPC_CharacterVirtual,
        body2: *const JPC_Body,
        linear_velocity: &mut JPC_Vec3,
        angular_velocity: &mut JPC_Vec3,
    );

    fn on_contact_validate(
        &self,
        character: *const JPC_CharacterVirtual,
        body_id2: JPC_BodyID,
        sub_shape_id2: JPC_SubShapeID,
    ) -> bool;

    fn on_contact_added(
        &self,
        character: *const JPC_CharacterVirtual,
        body_id2: JPC_BodyID,
        sub_shape_id2: JPC_SubShapeID,
        contact_position: JPC_RVec3,
        contact_normal: JPC_Vec3,
        settings: &mut JPC_CharacterContactSettings,
    );

    fn on_contact_persisted(
        &self,
        character: *const JPC_CharacterVirtual,
        body_id2: JPC_BodyID,
        sub_shape_id2: JPC_SubShapeID,
        contact_position: JPC_RVec3,
        contact_normal: JPC_Vec3,
        settings: &mut JPC_CharacterContactSettings,
    );

    fn on_contact_solve(
        &self,
        character: *const JPC_CharacterVirtual,
        body_id2: JPC_BodyID,
        sub_shape_id2: JPC_SubShapeID,
        contact_position: JPC_RVec3,
        contact_normal: JPC_Vec3,
        contact_velocity: JPC_Vec3,
        contact_material: *const JPC_PhysicsMaterial,
        character_velocity: JPC_Vec3,
        new_character_velocity: &mut JPC_Vec3,
    );
}

define_impl_struct!(mut CharacterContactListener {
    OnAdjustBodyVelocity,
    OnContactValidate,
    OnContactAdded,
    OnContactPersisted,
    OnContactSolve,
});

struct CharacterContactListenerBridge<T> {
    _phantom: PhantomData<T>,
}

impl<T: CharacterContactListener> CharacterContactListenerBridge<T> {
    unsafe extern "C" fn OnAdjustBodyVelocity(
        this: *mut c_void,
        character: *const JPC_CharacterVirtual,
        body2: *const JPC_Body,
        linear_velocity: *mut JPC_Vec3,
        angular_velocity: *mut JPC_Vec3,
    ) {
        let this = this.cast::<T>().as_ref().unwrap();
        this.on_adjust_body_velocity(character, body2, &mut *linear_velocity, &mut *angular_velocity);
    }

    unsafe extern "C" fn OnContactValidate(
        this: *mut c_void,
        character: *const JPC_CharacterVirtual,
        body_id2: JPC_BodyID,
        sub_shape_id2: JPC_SubShapeID,
    ) -> bool {
        let this = this.cast::<T>().as_ref().unwrap();
        this.on_contact_validate(character, body_id2, sub_shape_id2)
    }

    unsafe extern "C" fn OnContactAdded(
        this: *mut c_void,
        character: *const JPC_CharacterVirtual,
        body_id2: JPC_BodyID,
        sub_shape_id2: JPC_SubShapeID,
        contact_position: JPC_RVec3,
        contact_normal: JPC_Vec3,
        settings: *mut JPC_CharacterContactSettings,
    ) {
        let this = this.cast::<T>().as_ref().unwrap();
        this.on_contact_added(character, body_id2, sub_shape_id2, contact_position, contact_normal, &mut *settings);
    }

    unsafe extern "C" fn OnContactPersisted(
        this: *mut c_void,
        character: *const JPC_CharacterVirtual,
        body_id2: JPC_BodyID,
        sub_shape_id2: JPC_SubShapeID,
        contact_position: JPC_RVec3,
        contact_normal: JPC_Vec3,
        settings: *mut JPC_CharacterContactSettings,
    ) {
        let this = this.cast::<T>().as_ref().unwrap();
        this.on_contact_persisted(character, body_id2, sub_shape_id2, contact_position, contact_normal, &mut *settings);
    }

    unsafe extern "C" fn OnContactSolve(
        this: *mut c_void,
        character: *const JPC_CharacterVirtual,
        body_id2: JPC_BodyID,
        sub_shape_id2: JPC_SubShapeID,
        contact_position: JPC_RVec3,
        contact_normal: JPC_Vec3,
        contact_velocity: JPC_Vec3,
        contact_material: *const JPC_PhysicsMaterial,
        character_velocity: JPC_Vec3,
        new_character_velocity: *mut JPC_Vec3,
    ) {
        let this = this.cast::<T>().as_ref().unwrap();
        this.on_contact_solve(
            character, body_id2, sub_shape_id2, contact_position, contact_normal,
            contact_velocity, contact_material, character_velocity, &mut *new_character_velocity,
        );
    }
}

////////////////////////////////////////////////////////////////////////////////
// SoftBodyContactListener

/// Settings that control how a soft body contact is resolved.
///
/// See also: Jolt's [`SoftBodyContactSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/struct_soft_body_contact_settings.html).
pub struct SoftBodyContactSettings(pub JPC_SoftBodyContactSettings);

impl SoftBodyContactSettings {
    pub fn inv_mass_scale1(&self) -> f32 { self.0.InvMassScale1 }
    pub fn set_inv_mass_scale1(&mut self, v: f32) { self.0.InvMassScale1 = v; }
    pub fn inv_mass_scale2(&self) -> f32 { self.0.InvMassScale2 }
    pub fn set_inv_mass_scale2(&mut self, v: f32) { self.0.InvMassScale2 = v; }
    pub fn inv_inertia_scale2(&self) -> f32 { self.0.InvInertiaScale2 }
    pub fn set_inv_inertia_scale2(&mut self, v: f32) { self.0.InvInertiaScale2 = v; }
    pub fn is_sensor(&self) -> bool { self.0.IsSensor }
    pub fn set_is_sensor(&mut self, v: bool) { self.0.IsSensor = v; }
}

/// Opaque handle to a soft body manifold — use accessors to inspect contacts.
///
/// See also: Jolt's [`SoftBodyManifold`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_soft_body_manifold.html).
pub struct SoftBodyManifold {
    raw: *const JPC_SoftBodyManifold,
}

impl SoftBodyManifold {
    pub fn body_id(&self) -> BodyId {
        BodyId::new(unsafe { JPC_SoftBodyManifold_GetBodyID(self.raw) })
    }

    pub fn num_vertices(&self) -> u32 {
        unsafe { JPC_SoftBodyManifold_GetNumVertices(self.raw) }
    }

    pub fn vertex_has_contact(&self, index: u32) -> bool {
        unsafe { JPC_SoftBodyManifold_VertexHasContact(self.raw, index) }
    }
}

/// Notified when a soft body contacts a rigid body.
///
/// See also: Jolt's [`SoftBodyContactListener`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_soft_body_contact_listener.html).
pub trait SoftBodyContactListener {
    /// called before the contact is resolved — return `JPC_ValidateResult_AcceptAllContactsForThisBodyPair`
    /// to accept or `JPC_ValidateResult_RejectContact` to reject.
    fn on_soft_body_contact_validate(
        &self,
        soft_body: &crate::Body<'_>,
        other_body: &crate::Body<'_>,
        settings: &mut SoftBodyContactSettings,
    ) -> JPC_ValidateResult;

    /// called after the contact manifold has been finalized.
    fn on_soft_body_contact_added(
        &self,
        soft_body: &crate::Body<'_>,
        manifold: &SoftBodyManifold,
    );
}

define_impl_struct!(mut SoftBodyContactListener {
    OnSoftBodyContactValidate,
    OnSoftBodyContactAdded,
});

struct SoftBodyContactListenerBridge<T> {
    _phantom: PhantomData<T>,
}

impl<T: SoftBodyContactListener> SoftBodyContactListenerBridge<T> {
    unsafe extern "C" fn OnSoftBodyContactValidate(
        this: *mut c_void,
        soft_body: *const JPC_Body,
        other_body: *const JPC_Body,
        settings: *mut JPC_SoftBodyContactSettings,
    ) -> u32 {
        let this = this.cast::<T>().as_ref().unwrap();
        let soft_body = crate::Body::new(soft_body.cast_mut());
        let other_body = crate::Body::new(other_body.cast_mut());
        let mut wrapped = SoftBodyContactSettings(*settings);
        let result = this.on_soft_body_contact_validate(&soft_body, &other_body, &mut wrapped);
        *settings = wrapped.0;
        result
    }

    unsafe extern "C" fn OnSoftBodyContactAdded(
        this: *mut c_void,
        soft_body: *const JPC_Body,
        manifold: *const JPC_SoftBodyManifold,
    ) {
        let this = this.cast::<T>().as_ref().unwrap();
        let soft_body = crate::Body::new(soft_body.cast_mut());
        let manifold = SoftBodyManifold { raw: manifold };
        this.on_soft_body_contact_added(&soft_body, &manifold);
    }
}

////////////////////////////////////////////////////////////////////////////////
// BodyActivationListener

/// Notified when bodies activate or deactivate.
///
/// See also: Jolt's [`BodyActivationListener`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_body_activation_listener.html).
pub trait BodyActivationListener {
    fn on_body_activated(&self, body_id: BodyId, body_user_data: u64);
    fn on_body_deactivated(&self, body_id: BodyId, body_user_data: u64);
}

define_impl_struct!(mut BodyActivationListener {
    OnBodyActivated,
    OnBodyDeactivated,
});

struct BodyActivationListenerBridge<T> {
    _phantom: PhantomData<T>,
}

impl<T: BodyActivationListener> BodyActivationListenerBridge<T> {
    unsafe extern "C" fn OnBodyActivated(this: *mut c_void, body_id: JPC_BodyID, body_user_data: u64) {
        let this = this.cast::<T>().as_ref().unwrap();
        this.on_body_activated(BodyId::new(body_id), body_user_data);
    }

    unsafe extern "C" fn OnBodyDeactivated(this: *mut c_void, body_id: JPC_BodyID, body_user_data: u64) {
        let this = this.cast::<T>().as_ref().unwrap();
        this.on_body_deactivated(BodyId::new(body_id), body_user_data);
    }
}
