use std::ffi::CStr;

use joltc_sys::*;

use crate::{IntoJolt, Quat, Ref, RefConst, Vec3};

// helper used by all shape creators (pub(crate) so shape_settings.rs can reuse it)
pub(crate) fn create_shape_inner(
    f: impl FnOnce(*mut *mut JPC_Shape, *mut *mut JPC_String) -> bool,
) -> Result<RefConst<JPC_Shape>, String> {
    let mut shape: *mut JPC_Shape = std::ptr::null_mut();
    let mut error: *mut JPC_String = std::ptr::null_mut();
    if f(&mut shape, &mut error) {
        // HandleShapeResult already called AddRef
        Ok(unsafe { RefConst::from_addrefed(shape) })
    } else {
        let msg = if error.is_null() {
            "unknown error".to_string()
        } else {
            let c_str = unsafe { CStr::from_ptr(JPC_String_c_str(error)) };
            let s = c_str.to_string_lossy().into_owned();
            unsafe { JPC_String_delete(error) };
            s
        };
        Err(msg)
    }
}

pub fn box_shape(half_extent: Vec3, convex_radius: f32) -> Result<RefConst<JPC_Shape>, String> {
    let settings = JPC_BoxShapeSettings { HalfExtent: half_extent.into_jolt(), ConvexRadius: convex_radius, ..Default::default() };
    create_shape_inner(|s, e| unsafe { JPC_BoxShapeSettings_Create(&settings, s, e) })
}

pub fn sphere_shape(radius: f32) -> Result<RefConst<JPC_Shape>, String> {
    let settings = JPC_SphereShapeSettings { Radius: radius, ..Default::default() };
    create_shape_inner(|s, e| unsafe { JPC_SphereShapeSettings_Create(&settings, s, e) })
}

pub fn capsule_shape(half_height_of_cylinder: f32, radius: f32) -> Result<RefConst<JPC_Shape>, String> {
    let settings = JPC_CapsuleShapeSettings { HalfHeightOfCylinder: half_height_of_cylinder, Radius: radius, ..Default::default() };
    create_shape_inner(|s, e| unsafe { JPC_CapsuleShapeSettings_Create(&settings, s, e) })
}

pub fn cylinder_shape(half_height: f32, radius: f32, convex_radius: f32) -> Result<RefConst<JPC_Shape>, String> {
    let settings = JPC_CylinderShapeSettings { HalfHeight: half_height, Radius: radius, ConvexRadius: convex_radius, ..Default::default() };
    create_shape_inner(|s, e| unsafe { JPC_CylinderShapeSettings_Create(&settings, s, e) })
}

/// Plane shape with a `normal` direction and a signed `constant` (distance from origin along normal).
pub fn plane_shape(normal: Vec3, constant: f32, half_extent: f32) -> Result<RefConst<JPC_Shape>, String> {
    let settings = JPC_PlaneShapeSettings { Normal: normal.into_jolt(), Constant: constant, HalfExtent: half_extent, ..Default::default() };
    create_shape_inner(|s, e| unsafe { JPC_PlaneShapeSettings_Create(&settings, s, e) })
}

pub fn triangle_shape(v1: Vec3, v2: Vec3, v3: Vec3, convex_radius: f32) -> Result<RefConst<JPC_Shape>, String> {
    let settings = JPC_TriangleShapeSettings { V1: v1.into_jolt(), V2: v2.into_jolt(), V3: v3.into_jolt(), ConvexRadius: convex_radius, ..Default::default() };
    create_shape_inner(|s, e| unsafe { JPC_TriangleShapeSettings_Create(&settings, s, e) })
}

pub fn convex_hull_shape(points: &[Vec3]) -> Result<RefConst<JPC_Shape>, String> {
    let raw_points: Vec<JPC_Vec3> = points.iter().copied().map(IntoJolt::into_jolt).collect();
    let settings = JPC_ConvexHullShapeSettings { Points: raw_points.as_ptr(), PointsLen: raw_points.len(), ..Default::default() };
    create_shape_inner(|s, e| unsafe { JPC_ConvexHullShapeSettings_Create(&settings, s, e) })
}

/// Mesh shape from indexed triangles.
/// `vertices` are `Float3` (packed xyz), `triangles` index into them.
pub fn mesh_shape(vertices: &[JPC_Float3], triangles: &[JPC_IndexedTriangle]) -> Result<RefConst<JPC_Shape>, String> {
    let settings = JPC_MeshShapeSettings {
        TriangleVertices: vertices.as_ptr().cast_mut(),
        TriangleVerticesLen: vertices.len(),
        IndexedTriangles: triangles.as_ptr().cast_mut(),
        IndexedTrianglesLen: triangles.len(),
        ..Default::default()
    };
    create_shape_inner(|s, e| unsafe { JPC_MeshShapeSettings_Create(&settings, s, e) })
}

pub fn scaled_shape(inner: &RefConst<JPC_Shape>, scale: Vec3) -> Result<RefConst<JPC_Shape>, String> {
    let settings = JPC_ScaledShapeSettings { InnerShape: inner.get(), Scale: scale.into_jolt(), ..Default::default() };
    create_shape_inner(|s, e| unsafe { JPC_ScaledShapeSettings_Create(&settings, s, e) })
}

pub fn rotated_translated_shape(inner: &RefConst<JPC_Shape>, position: Vec3, rotation: Quat) -> Result<RefConst<JPC_Shape>, String> {
    let settings = JPC_RotatedTranslatedShapeSettings { InnerShape: inner.get(), Position: position.into_jolt(), Rotation: rotation.into_jolt(), ..Default::default() };
    create_shape_inner(|s, e| unsafe { JPC_RotatedTranslatedShapeSettings_Create(&settings, s, e) })
}

pub fn offset_center_of_mass_shape(inner: &RefConst<JPC_Shape>, offset: Vec3) -> Result<RefConst<JPC_Shape>, String> {
    let settings = JPC_OffsetCenterOfMassShapeSettings { InnerShape: inner.get(), Offset: offset.into_jolt(), ..Default::default() };
    create_shape_inner(|s, e| unsafe { JPC_OffsetCenterOfMassShapeSettings_Create(&settings, s, e) })
}

pub fn empty_shape(center_of_mass: Vec3) -> Result<RefConst<JPC_Shape>, String> {
    let settings = JPC_EmptyShapeSettings { CenterOfMass: center_of_mass.into_jolt(), ..Default::default() };
    create_shape_inner(|s, e| unsafe { JPC_EmptyShapeSettings_Create(&settings, s, e) })
}

pub fn tapered_capsule_shape(half_height: f32, top_radius: f32, bottom_radius: f32) -> Result<RefConst<JPC_Shape>, String> {
    let settings = JPC_TaperedCapsuleShapeSettings { HalfHeightOfTaperedCylinder: half_height, TopRadius: top_radius, BottomRadius: bottom_radius, ..Default::default() };
    create_shape_inner(|s, e| unsafe { JPC_TaperedCapsuleShapeSettings_Create(&settings, s, e) })
}

pub fn tapered_cylinder_shape(half_height: f32, top_radius: f32, bottom_radius: f32, convex_radius: f32) -> Result<RefConst<JPC_Shape>, String> {
    let settings = JPC_TaperedCylinderShapeSettings { HalfHeight: half_height, TopRadius: top_radius, BottomRadius: bottom_radius, ConvexRadius: convex_radius, ..Default::default() };
    create_shape_inner(|s, e| unsafe { JPC_TaperedCylinderShapeSettings_Create(&settings, s, e) })
}

/// Height field shape.  `height_samples` is a `side_length x side_length` row-major grid.
pub fn height_field_shape(height_samples: &[f32], side_length: u32, offset: Vec3, scale: Vec3) -> Result<RefConst<JPC_Shape>, String> {
    let settings = JPC_HeightFieldShapeSettings {
        HeightSamples: height_samples.as_ptr().cast_mut(),
        HeightSamplesLen: height_samples.len(),
        SampleCount: side_length,
        Offset: offset.into_jolt(),
        Scale: scale.into_jolt(),
        ..Default::default()
    };
    create_shape_inner(|s, e| unsafe { JPC_HeightFieldShapeSettings_Create(&settings, s, e) })
}

/// A sub-shape entry for compound shapes.
pub struct SubShape {
    pub shape: RefConst<JPC_Shape>,
    pub position: Vec3,
    pub rotation: Quat,
    pub user_data: u32,
}

pub fn static_compound_shape(sub_shapes: &[SubShape]) -> Result<RefConst<JPC_Shape>, String> {
    let raw: Vec<JPC_SubShapeSettings> = sub_shapes
        .iter()
        .map(|s| JPC_SubShapeSettings {
            Shape: s.shape.get(),
            Position: s.position.into_jolt(),
            Rotation: s.rotation.into_jolt(),
            UserData: s.user_data,
            ..Default::default()
        })
        .collect();
    let settings = JPC_StaticCompoundShapeSettings {
        SubShapes: raw.as_ptr(),
        SubShapesLen: raw.len(),
        ..Default::default()
    };
    create_shape_inner(|s, e| unsafe { JPC_StaticCompoundShapeSettings_Create(&settings, s, e) })
}

/// Mutable compound shape, returned as a `JPC_MutableCompoundShape` ref.
pub fn mutable_compound_shape(sub_shapes: &[SubShape]) -> Result<crate::Ref<JPC_MutableCompoundShape>, String> {
    let raw: Vec<JPC_SubShapeSettings> = sub_shapes
        .iter()
        .map(|s| JPC_SubShapeSettings {
            Shape: s.shape.get(),
            Position: s.position.into_jolt(),
            Rotation: s.rotation.into_jolt(),
            UserData: s.user_data,
            ..Default::default()
        })
        .collect();
    let settings = JPC_MutableCompoundShapeSettings {
        SubShapes: raw.as_ptr(),
        SubShapesLen: raw.len(),
        ..Default::default()
    };
    let mut shape: *mut JPC_MutableCompoundShape = std::ptr::null_mut();
    let mut error: *mut JPC_String = std::ptr::null_mut();
    let ok = unsafe { JPC_MutableCompoundShapeSettings_Create(&settings, &mut shape, &mut error) };
    if ok {
        Ok(unsafe { crate::Ref::from_addrefed(shape) })
    } else {
        let msg = if error.is_null() {
            "unknown error".to_string()
        } else {
            let c_str = unsafe { CStr::from_ptr(JPC_String_c_str(error)) };
            let s = c_str.to_string_lossy().into_owned();
            unsafe { JPC_String_delete(error) };
            s
        };
        Err(msg)
    }
}

/// Returns the sub-shape at `index` in a compound shape.
///
/// Panics if `shape` is not a compound shape (debug builds only; UB in release without Jolt asserts).
pub fn compound_sub_shape(shape: &RefConst<JPC_Shape>, index: u32) -> RefConst<JPC_Shape> {
    let raw = unsafe {
        JPC_CompoundShape_GetSubShape_Shape(shape.get().cast::<JPC_CompoundShape>(), index)
    };
    unsafe { RefConst::from_active(raw) }
}

/// Resolve a `SubShapeID` to its index within a compound shape, returning the remainder for nested compounds.
pub fn compound_sub_shape_index_from_id(
    shape: &RefConst<JPC_Shape>,
    sub_shape_id: JPC_SubShapeID,
) -> (u32, JPC_SubShapeID) {
    let mut remainder: JPC_SubShapeID = 0;
    let index = unsafe {
        JPC_CompoundShape_GetSubShapeIndexFromID(
            shape.get().cast::<JPC_CompoundShape>(),
            sub_shape_id,
            &mut remainder,
        )
    };
    (index, remainder)
}

/// Wrapper around a mutable compound shape for convenient runtime modification.
pub struct MutableCompoundShape(pub crate::Ref<JPC_MutableCompoundShape>);

impl MutableCompoundShape {
    pub fn new(sub_shapes: &[SubShape]) -> Result<Self, String> {
        Ok(Self(mutable_compound_shape(sub_shapes)?))
    }

    pub fn add_shape(&mut self, position: Vec3, rotation: Quat, shape: &RefConst<JPC_Shape>, user_data: u32) -> u32 {
        unsafe {
            JPC_MutableCompoundShape_AddShape(*self.0, position.into_jolt(), rotation.into_jolt(), shape.get(), user_data)
        }
    }

    pub fn remove_shape(&mut self, index: u32) {
        unsafe { JPC_MutableCompoundShape_RemoveShape(*self.0, index) }
    }

    pub fn modify_shape(&mut self, index: u32, position: Vec3, rotation: Quat) {
        unsafe { JPC_MutableCompoundShape_ModifyShape(*self.0, index, position.into_jolt(), rotation.into_jolt()) }
    }

    pub fn adjust_center_of_mass(&mut self) {
        unsafe { JPC_MutableCompoundShape_AdjustCenterOfMass(*self.0) }
    }

    pub fn as_shape(&self) -> Ref<JPC_Shape> {
        self.0.clone().cast()
    }
}
