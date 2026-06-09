use joltc_sys::*;

use crate::{shape::create_shape_inner, shape::SubShape, FromJolt, IntoJolt, Quat, Ref, RefConst, Vec3};

// --- simple shape settings (all fields directly accessible) ---

/// See also: Jolt's [`BoxShapeSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_box_shape_settings.html) class.
pub struct BoxShapeSettings { raw: JPC_BoxShapeSettings, material: Option<RefConst<JPC_PhysicsMaterial>> }
impl BoxShapeSettings {
    pub fn new(half_extent: Vec3, convex_radius: f32) -> Self {
        Self { raw: JPC_BoxShapeSettings { HalfExtent: half_extent.into_jolt(), ConvexRadius: convex_radius, ..Default::default() }, material: None }
    }
    pub fn half_extent(&self) -> Vec3 { Vec3::from_jolt(self.raw.HalfExtent) }
    pub fn set_half_extent(&mut self, v: Vec3) { self.raw.HalfExtent = v.into_jolt(); }
    pub fn convex_radius(&self) -> f32 { self.raw.ConvexRadius }
    pub fn set_convex_radius(&mut self, v: f32) { self.raw.ConvexRadius = v; }
    pub fn density(&self) -> f32 { self.raw.Density }
    pub fn set_density(&mut self, v: f32) { self.raw.Density = v; }
    pub fn user_data(&self) -> u64 { self.raw.UserData }
    pub fn set_user_data(&mut self, v: u64) { self.raw.UserData = v; }
    pub fn material(&self) -> Option<&RefConst<JPC_PhysicsMaterial>> { self.material.as_ref() }
    pub fn set_material(&mut self, material: Option<RefConst<JPC_PhysicsMaterial>>) {
        self.raw.Material = material.as_ref().map_or(std::ptr::null(), |m| m.get());
        self.material = material;
    }
    pub fn create(&self) -> Result<RefConst<JPC_Shape>, String> {
        create_shape_inner(|s, e| unsafe { JPC_BoxShapeSettings_Create(&self.raw, s, e) })
    }
}
impl Default for BoxShapeSettings {
    fn default() -> Self { Self { raw: Default::default(), material: None } }
}

/// See also: Jolt's [`SphereShapeSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_sphere_shape_settings.html) class.
pub struct SphereShapeSettings { raw: JPC_SphereShapeSettings, material: Option<RefConst<JPC_PhysicsMaterial>> }
impl SphereShapeSettings {
    pub fn new(radius: f32) -> Self {
        Self { raw: JPC_SphereShapeSettings { Radius: radius, ..Default::default() }, material: None }
    }
    pub fn radius(&self) -> f32 { self.raw.Radius }
    pub fn set_radius(&mut self, v: f32) { self.raw.Radius = v; }
    pub fn density(&self) -> f32 { self.raw.Density }
    pub fn set_density(&mut self, v: f32) { self.raw.Density = v; }
    pub fn user_data(&self) -> u64 { self.raw.UserData }
    pub fn set_user_data(&mut self, v: u64) { self.raw.UserData = v; }
    pub fn material(&self) -> Option<&RefConst<JPC_PhysicsMaterial>> { self.material.as_ref() }
    pub fn set_material(&mut self, material: Option<RefConst<JPC_PhysicsMaterial>>) {
        self.raw.Material = material.as_ref().map_or(std::ptr::null(), |m| m.get());
        self.material = material;
    }
    pub fn create(&self) -> Result<RefConst<JPC_Shape>, String> {
        create_shape_inner(|s, e| unsafe { JPC_SphereShapeSettings_Create(&self.raw, s, e) })
    }
}
impl Default for SphereShapeSettings {
    fn default() -> Self { Self { raw: Default::default(), material: None } }
}

/// See also: Jolt's [`CapsuleShapeSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_capsule_shape_settings.html) class.
pub struct CapsuleShapeSettings { raw: JPC_CapsuleShapeSettings, material: Option<RefConst<JPC_PhysicsMaterial>> }
impl CapsuleShapeSettings {
    pub fn new(half_height_of_cylinder: f32, radius: f32) -> Self {
        Self { raw: JPC_CapsuleShapeSettings { HalfHeightOfCylinder: half_height_of_cylinder, Radius: radius, ..Default::default() }, material: None }
    }
    pub fn half_height_of_cylinder(&self) -> f32 { self.raw.HalfHeightOfCylinder }
    pub fn set_half_height_of_cylinder(&mut self, v: f32) { self.raw.HalfHeightOfCylinder = v; }
    pub fn radius(&self) -> f32 { self.raw.Radius }
    pub fn set_radius(&mut self, v: f32) { self.raw.Radius = v; }
    pub fn density(&self) -> f32 { self.raw.Density }
    pub fn set_density(&mut self, v: f32) { self.raw.Density = v; }
    pub fn user_data(&self) -> u64 { self.raw.UserData }
    pub fn set_user_data(&mut self, v: u64) { self.raw.UserData = v; }
    pub fn material(&self) -> Option<&RefConst<JPC_PhysicsMaterial>> { self.material.as_ref() }
    pub fn set_material(&mut self, material: Option<RefConst<JPC_PhysicsMaterial>>) {
        self.raw.Material = material.as_ref().map_or(std::ptr::null(), |m| m.get());
        self.material = material;
    }
    pub fn create(&self) -> Result<RefConst<JPC_Shape>, String> {
        create_shape_inner(|s, e| unsafe { JPC_CapsuleShapeSettings_Create(&self.raw, s, e) })
    }
}
impl Default for CapsuleShapeSettings {
    fn default() -> Self { Self { raw: Default::default(), material: None } }
}

/// See also: Jolt's [`CylinderShapeSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_cylinder_shape_settings.html) class.
pub struct CylinderShapeSettings { raw: JPC_CylinderShapeSettings, material: Option<RefConst<JPC_PhysicsMaterial>> }
impl CylinderShapeSettings {
    pub fn new(half_height: f32, radius: f32, convex_radius: f32) -> Self {
        Self { raw: JPC_CylinderShapeSettings { HalfHeight: half_height, Radius: radius, ConvexRadius: convex_radius, ..Default::default() }, material: None }
    }
    pub fn half_height(&self) -> f32 { self.raw.HalfHeight }
    pub fn set_half_height(&mut self, v: f32) { self.raw.HalfHeight = v; }
    pub fn radius(&self) -> f32 { self.raw.Radius }
    pub fn set_radius(&mut self, v: f32) { self.raw.Radius = v; }
    pub fn convex_radius(&self) -> f32 { self.raw.ConvexRadius }
    pub fn set_convex_radius(&mut self, v: f32) { self.raw.ConvexRadius = v; }
    pub fn density(&self) -> f32 { self.raw.Density }
    pub fn set_density(&mut self, v: f32) { self.raw.Density = v; }
    pub fn user_data(&self) -> u64 { self.raw.UserData }
    pub fn set_user_data(&mut self, v: u64) { self.raw.UserData = v; }
    pub fn material(&self) -> Option<&RefConst<JPC_PhysicsMaterial>> { self.material.as_ref() }
    pub fn set_material(&mut self, material: Option<RefConst<JPC_PhysicsMaterial>>) {
        self.raw.Material = material.as_ref().map_or(std::ptr::null(), |m| m.get());
        self.material = material;
    }
    pub fn create(&self) -> Result<RefConst<JPC_Shape>, String> {
        create_shape_inner(|s, e| unsafe { JPC_CylinderShapeSettings_Create(&self.raw, s, e) })
    }
}
impl Default for CylinderShapeSettings {
    fn default() -> Self { Self { raw: Default::default(), material: None } }
}

/// See also: Jolt's [`TriangleShapeSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_triangle_shape_settings.html) class.
pub struct TriangleShapeSettings { raw: JPC_TriangleShapeSettings, material: Option<RefConst<JPC_PhysicsMaterial>> }
impl TriangleShapeSettings {
    pub fn new(v1: Vec3, v2: Vec3, v3: Vec3, convex_radius: f32) -> Self {
        Self { raw: JPC_TriangleShapeSettings { V1: v1.into_jolt(), V2: v2.into_jolt(), V3: v3.into_jolt(), ConvexRadius: convex_radius, ..Default::default() }, material: None }
    }
    pub fn v1(&self) -> Vec3 { Vec3::from_jolt(self.raw.V1) }
    pub fn set_v1(&mut self, v: Vec3) { self.raw.V1 = v.into_jolt(); }
    pub fn v2(&self) -> Vec3 { Vec3::from_jolt(self.raw.V2) }
    pub fn set_v2(&mut self, v: Vec3) { self.raw.V2 = v.into_jolt(); }
    pub fn v3(&self) -> Vec3 { Vec3::from_jolt(self.raw.V3) }
    pub fn set_v3(&mut self, v: Vec3) { self.raw.V3 = v.into_jolt(); }
    pub fn convex_radius(&self) -> f32 { self.raw.ConvexRadius }
    pub fn set_convex_radius(&mut self, v: f32) { self.raw.ConvexRadius = v; }
    pub fn density(&self) -> f32 { self.raw.Density }
    pub fn set_density(&mut self, v: f32) { self.raw.Density = v; }
    pub fn user_data(&self) -> u64 { self.raw.UserData }
    pub fn set_user_data(&mut self, v: u64) { self.raw.UserData = v; }
    pub fn material(&self) -> Option<&RefConst<JPC_PhysicsMaterial>> { self.material.as_ref() }
    pub fn set_material(&mut self, material: Option<RefConst<JPC_PhysicsMaterial>>) {
        self.raw.Material = material.as_ref().map_or(std::ptr::null(), |m| m.get());
        self.material = material;
    }
    pub fn create(&self) -> Result<RefConst<JPC_Shape>, String> {
        create_shape_inner(|s, e| unsafe { JPC_TriangleShapeSettings_Create(&self.raw, s, e) })
    }
}
impl Default for TriangleShapeSettings {
    fn default() -> Self { Self { raw: Default::default(), material: None } }
}

/// See also: Jolt's [`PlaneShapeSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_plane_shape_settings.html) class.
pub struct PlaneShapeSettings { raw: JPC_PlaneShapeSettings, material: Option<RefConst<JPC_PhysicsMaterial>> }
impl PlaneShapeSettings {
    pub fn new(normal: Vec3, constant: f32, half_extent: f32) -> Self {
        Self { raw: JPC_PlaneShapeSettings { Normal: normal.into_jolt(), Constant: constant, HalfExtent: half_extent, ..Default::default() }, material: None }
    }
    pub fn normal(&self) -> Vec3 { Vec3::from_jolt(self.raw.Normal) }
    pub fn set_normal(&mut self, v: Vec3) { self.raw.Normal = v.into_jolt(); }
    pub fn constant(&self) -> f32 { self.raw.Constant }
    pub fn set_constant(&mut self, v: f32) { self.raw.Constant = v; }
    pub fn half_extent(&self) -> f32 { self.raw.HalfExtent }
    pub fn set_half_extent(&mut self, v: f32) { self.raw.HalfExtent = v; }
    pub fn user_data(&self) -> u64 { self.raw.UserData }
    pub fn set_user_data(&mut self, v: u64) { self.raw.UserData = v; }
    pub fn material(&self) -> Option<&RefConst<JPC_PhysicsMaterial>> { self.material.as_ref() }
    pub fn set_material(&mut self, material: Option<RefConst<JPC_PhysicsMaterial>>) {
        self.raw.Material = material.as_ref().map_or(std::ptr::null(), |m| m.get());
        self.material = material;
    }
    pub fn create(&self) -> Result<RefConst<JPC_Shape>, String> {
        create_shape_inner(|s, e| unsafe { JPC_PlaneShapeSettings_Create(&self.raw, s, e) })
    }
}
impl Default for PlaneShapeSettings {
    fn default() -> Self { Self { raw: Default::default(), material: None } }
}

/// See also: Jolt's [`TaperedCapsuleShapeSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_tapered_capsule_shape_settings.html) class.
pub struct TaperedCapsuleShapeSettings { raw: JPC_TaperedCapsuleShapeSettings }
impl TaperedCapsuleShapeSettings {
    pub fn new(half_height: f32, top_radius: f32, bottom_radius: f32) -> Self {
        Self { raw: JPC_TaperedCapsuleShapeSettings { HalfHeightOfTaperedCylinder: half_height, TopRadius: top_radius, BottomRadius: bottom_radius, ..Default::default() } }
    }
    pub fn half_height_of_tapered_cylinder(&self) -> f32 { self.raw.HalfHeightOfTaperedCylinder }
    pub fn set_half_height_of_tapered_cylinder(&mut self, v: f32) { self.raw.HalfHeightOfTaperedCylinder = v; }
    pub fn top_radius(&self) -> f32 { self.raw.TopRadius }
    pub fn set_top_radius(&mut self, v: f32) { self.raw.TopRadius = v; }
    pub fn bottom_radius(&self) -> f32 { self.raw.BottomRadius }
    pub fn set_bottom_radius(&mut self, v: f32) { self.raw.BottomRadius = v; }
    pub fn density(&self) -> f32 { self.raw.Density }
    pub fn set_density(&mut self, v: f32) { self.raw.Density = v; }
    pub fn user_data(&self) -> u64 { self.raw.UserData }
    pub fn set_user_data(&mut self, v: u64) { self.raw.UserData = v; }
    pub fn create(&self) -> Result<RefConst<JPC_Shape>, String> {
        create_shape_inner(|s, e| unsafe { JPC_TaperedCapsuleShapeSettings_Create(&self.raw, s, e) })
    }
}
impl Default for TaperedCapsuleShapeSettings {
    fn default() -> Self { Self { raw: Default::default() } }
}

/// See also: Jolt's [`TaperedCylinderShapeSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_tapered_cylinder_shape_settings.html) class.
pub struct TaperedCylinderShapeSettings { raw: JPC_TaperedCylinderShapeSettings }
impl TaperedCylinderShapeSettings {
    pub fn new(half_height: f32, top_radius: f32, bottom_radius: f32, convex_radius: f32) -> Self {
        Self { raw: JPC_TaperedCylinderShapeSettings { HalfHeight: half_height, TopRadius: top_radius, BottomRadius: bottom_radius, ConvexRadius: convex_radius, ..Default::default() } }
    }
    pub fn half_height(&self) -> f32 { self.raw.HalfHeight }
    pub fn set_half_height(&mut self, v: f32) { self.raw.HalfHeight = v; }
    pub fn top_radius(&self) -> f32 { self.raw.TopRadius }
    pub fn set_top_radius(&mut self, v: f32) { self.raw.TopRadius = v; }
    pub fn bottom_radius(&self) -> f32 { self.raw.BottomRadius }
    pub fn set_bottom_radius(&mut self, v: f32) { self.raw.BottomRadius = v; }
    pub fn convex_radius(&self) -> f32 { self.raw.ConvexRadius }
    pub fn set_convex_radius(&mut self, v: f32) { self.raw.ConvexRadius = v; }
    pub fn density(&self) -> f32 { self.raw.Density }
    pub fn set_density(&mut self, v: f32) { self.raw.Density = v; }
    pub fn user_data(&self) -> u64 { self.raw.UserData }
    pub fn set_user_data(&mut self, v: u64) { self.raw.UserData = v; }
    pub fn create(&self) -> Result<RefConst<JPC_Shape>, String> {
        create_shape_inner(|s, e| unsafe { JPC_TaperedCylinderShapeSettings_Create(&self.raw, s, e) })
    }
}
impl Default for TaperedCylinderShapeSettings {
    fn default() -> Self { Self { raw: Default::default() } }
}

/// See also: Jolt's [`EmptyShapeSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_empty_shape_settings.html) class.
pub struct EmptyShapeSettings { raw: JPC_EmptyShapeSettings }
impl EmptyShapeSettings {
    pub fn new(center_of_mass: Vec3) -> Self {
        Self { raw: JPC_EmptyShapeSettings { CenterOfMass: center_of_mass.into_jolt(), ..Default::default() } }
    }
    pub fn center_of_mass(&self) -> Vec3 { Vec3::from_jolt(self.raw.CenterOfMass) }
    pub fn set_center_of_mass(&mut self, v: Vec3) { self.raw.CenterOfMass = v.into_jolt(); }
    pub fn user_data(&self) -> u64 { self.raw.UserData }
    pub fn set_user_data(&mut self, v: u64) { self.raw.UserData = v; }
    pub fn create(&self) -> Result<RefConst<JPC_Shape>, String> {
        create_shape_inner(|s, e| unsafe { JPC_EmptyShapeSettings_Create(&self.raw, s, e) })
    }
}
impl Default for EmptyShapeSettings {
    fn default() -> Self { Self { raw: Default::default() } }
}

// --- slice-owning shape settings ---

/// Owns the point data so the pointer passed to joltc at `create()` time is always valid.
///
/// See also: Jolt's [`ConvexHullShapeSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_convex_hull_shape_settings.html) class.
pub struct ConvexHullShapeSettings {
    raw: JPC_ConvexHullShapeSettings,
    points: Vec<JPC_Vec3>,
    material: Option<RefConst<JPC_PhysicsMaterial>>,
}
impl ConvexHullShapeSettings {
    pub fn new(points: &[Vec3]) -> Self {
        let mut this = Self { raw: Default::default(), points: points.iter().copied().map(IntoJolt::into_jolt).collect(), material: None };
        // raw.Points/PointsLen are set in create() to avoid dangling ptrs
        this.raw.MaxConvexRadius = 0.05; // jolt default
        this
    }
    pub fn max_convex_radius(&self) -> f32 { self.raw.MaxConvexRadius }
    pub fn set_max_convex_radius(&mut self, v: f32) { self.raw.MaxConvexRadius = v; }
    pub fn max_error_convex_radius(&self) -> f32 { self.raw.MaxErrorConvexRadius }
    pub fn set_max_error_convex_radius(&mut self, v: f32) { self.raw.MaxErrorConvexRadius = v; }
    pub fn hull_tolerance(&self) -> f32 { self.raw.HullTolerance }
    pub fn set_hull_tolerance(&mut self, v: f32) { self.raw.HullTolerance = v; }
    pub fn density(&self) -> f32 { self.raw.Density }
    pub fn set_density(&mut self, v: f32) { self.raw.Density = v; }
    pub fn user_data(&self) -> u64 { self.raw.UserData }
    pub fn set_user_data(&mut self, v: u64) { self.raw.UserData = v; }
    pub fn material(&self) -> Option<&RefConst<JPC_PhysicsMaterial>> { self.material.as_ref() }
    pub fn set_material(&mut self, material: Option<RefConst<JPC_PhysicsMaterial>>) {
        self.raw.Material = material.as_ref().map_or(std::ptr::null(), |m| m.get());
        self.material = material;
    }
    pub fn create(&self) -> Result<RefConst<JPC_Shape>, String> {
        let raw = JPC_ConvexHullShapeSettings { Points: self.points.as_ptr(), PointsLen: self.points.len(), ..self.raw };
        create_shape_inner(|s, e| unsafe { JPC_ConvexHullShapeSettings_Create(&raw, s, e) })
    }
}

/// Owns the vertex and triangle data so the pointers passed to joltc are always valid.
///
/// See also: Jolt's [`MeshShapeSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_mesh_shape_settings.html) class.
pub struct MeshShapeSettings {
    raw: JPC_MeshShapeSettings,
    vertices: Vec<JPC_Float3>,
    triangles: Vec<JPC_IndexedTriangle>,
}
impl MeshShapeSettings {
    pub fn new(vertices: &[JPC_Float3], triangles: &[JPC_IndexedTriangle]) -> Self {
        Self {
            raw: Default::default(),
            vertices: vertices.to_vec(),
            triangles: triangles.to_vec(),
        }
    }
    pub fn user_data(&self) -> u64 { self.raw.UserData }
    pub fn set_user_data(&mut self, v: u64) { self.raw.UserData = v; }
    pub fn create(&self) -> Result<RefConst<JPC_Shape>, String> {
        let raw = JPC_MeshShapeSettings {
            TriangleVertices: self.vertices.as_ptr().cast_mut(),
            TriangleVerticesLen: self.vertices.len(),
            IndexedTriangles: self.triangles.as_ptr().cast_mut(),
            IndexedTrianglesLen: self.triangles.len(),
            ..self.raw
        };
        create_shape_inner(|s, e| unsafe { JPC_MeshShapeSettings_Create(&raw, s, e) })
    }
}

/// Owns height sample and material index data.
///
/// See also: Jolt's [`HeightFieldShapeSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_height_field_shape_settings.html) class.
pub struct HeightFieldShapeSettings {
    raw: JPC_HeightFieldShapeSettings,
    height_samples: Vec<f32>,
    material_indices: Vec<u8>,
}
impl HeightFieldShapeSettings {
    /// `height_samples` is a `side_length × side_length` row-major grid.
    pub fn new(height_samples: &[f32], side_length: u32, offset: Vec3, scale: Vec3) -> Self {
        Self {
            raw: JPC_HeightFieldShapeSettings {
                SampleCount: side_length,
                Offset: offset.into_jolt(),
                Scale: scale.into_jolt(),
                ..Default::default()
            },
            height_samples: height_samples.to_vec(),
            material_indices: Vec::new(),
        }
    }
    pub fn offset(&self) -> Vec3 { Vec3::from_jolt(self.raw.Offset) }
    pub fn set_offset(&mut self, v: Vec3) { self.raw.Offset = v.into_jolt(); }
    pub fn scale(&self) -> Vec3 { Vec3::from_jolt(self.raw.Scale) }
    pub fn set_scale(&mut self, v: Vec3) { self.raw.Scale = v.into_jolt(); }
    pub fn sample_count(&self) -> u32 { self.raw.SampleCount }
    pub fn min_height_value(&self) -> f32 { self.raw.MinHeightValue }
    pub fn set_min_height_value(&mut self, v: f32) { self.raw.MinHeightValue = v; }
    pub fn max_height_value(&self) -> f32 { self.raw.MaxHeightValue }
    pub fn set_max_height_value(&mut self, v: f32) { self.raw.MaxHeightValue = v; }
    pub fn block_size(&self) -> u32 { self.raw.BlockSize }
    pub fn set_block_size(&mut self, v: u32) { self.raw.BlockSize = v; }
    pub fn bits_per_sample(&self) -> u32 { self.raw.BitsPerSample }
    pub fn set_bits_per_sample(&mut self, v: u32) { self.raw.BitsPerSample = v; }
    pub fn active_edge_cos_threshold_angle(&self) -> f32 { self.raw.ActiveEdgeCosThresholdAngle }
    pub fn set_active_edge_cos_threshold_angle(&mut self, v: f32) { self.raw.ActiveEdgeCosThresholdAngle = v; }
    pub fn set_material_indices(&mut self, indices: &[u8]) { self.material_indices = indices.to_vec(); }
    pub fn user_data(&self) -> u64 { self.raw.UserData }
    pub fn set_user_data(&mut self, v: u64) { self.raw.UserData = v; }
    pub fn create(&self) -> Result<RefConst<JPC_Shape>, String> {
        let raw = JPC_HeightFieldShapeSettings {
            HeightSamples: self.height_samples.as_ptr(),
            HeightSamplesLen: self.height_samples.len(),
            MaterialIndices: self.material_indices.as_ptr(),
            MaterialIndicesLen: self.material_indices.len(),
            ..self.raw
        };
        create_shape_inner(|s, e| unsafe { JPC_HeightFieldShapeSettings_Create(&raw, s, e) })
    }
}

// --- compound shape settings ---

/// Owns its sub-shapes (each sub-shape keeps its own `RefConst` alive).
///
/// See also: Jolt's [`StaticCompoundShapeSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_static_compound_shape_settings.html) class.
pub struct StaticCompoundShapeSettings {
    sub_shapes: Vec<SubShape>,
    user_data: u64,
}
impl StaticCompoundShapeSettings {
    pub fn new() -> Self { Self { sub_shapes: Vec::new(), user_data: 0 } }
    pub fn add_shape(&mut self, shape: &RefConst<JPC_Shape>, position: Vec3, rotation: Quat, user_data: u32) {
        self.sub_shapes.push(SubShape { shape: shape.clone(), position, rotation, user_data });
    }
    pub fn user_data(&self) -> u64 { self.user_data }
    pub fn set_user_data(&mut self, v: u64) { self.user_data = v; }
    pub fn create(&self) -> Result<RefConst<JPC_Shape>, String> {
        let raw_sub: Vec<JPC_SubShapeSettings> = self.sub_shapes.iter().map(|s| JPC_SubShapeSettings {
            Shape: s.shape.get(),
            Position: s.position.into_jolt(),
            Rotation: s.rotation.into_jolt(),
            UserData: s.user_data,
            ..Default::default()
        }).collect();
        let settings = JPC_StaticCompoundShapeSettings { SubShapes: raw_sub.as_ptr(), SubShapesLen: raw_sub.len(), UserData: self.user_data };
        create_shape_inner(|s, e| unsafe { JPC_StaticCompoundShapeSettings_Create(&settings, s, e) })
    }
}
impl Default for StaticCompoundShapeSettings {
    fn default() -> Self { Self::new() }
}

/// Owns its sub-shapes (each sub-shape keeps its own `RefConst` alive).
/// Returns a mutable compound shape ref which can be modified at runtime.
///
/// See also: Jolt's [`MutableCompoundShapeSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_mutable_compound_shape_settings.html) class.
pub struct MutableCompoundShapeSettings {
    sub_shapes: Vec<SubShape>,
    user_data: u64,
}
impl MutableCompoundShapeSettings {
    pub fn new() -> Self { Self { sub_shapes: Vec::new(), user_data: 0 } }
    pub fn add_shape(&mut self, shape: &RefConst<JPC_Shape>, position: Vec3, rotation: Quat, user_data: u32) {
        self.sub_shapes.push(SubShape { shape: shape.clone(), position, rotation, user_data });
    }
    pub fn user_data(&self) -> u64 { self.user_data }
    pub fn set_user_data(&mut self, v: u64) { self.user_data = v; }
    pub fn create(&self) -> Result<Ref<JPC_MutableCompoundShape>, String> {
        use std::ffi::CStr;
        let raw_sub: Vec<JPC_SubShapeSettings> = self.sub_shapes.iter().map(|s| JPC_SubShapeSettings {
            Shape: s.shape.get(),
            Position: s.position.into_jolt(),
            Rotation: s.rotation.into_jolt(),
            UserData: s.user_data,
            ..Default::default()
        }).collect();
        let settings = JPC_MutableCompoundShapeSettings { SubShapes: raw_sub.as_ptr(), SubShapesLen: raw_sub.len(), UserData: self.user_data };
        let mut shape: *mut JPC_MutableCompoundShape = std::ptr::null_mut();
        let mut error: *mut JPC_String = std::ptr::null_mut();
        let ok = unsafe { JPC_MutableCompoundShapeSettings_Create(&settings, &mut shape, &mut error) };
        if ok {
            Ok(unsafe { Ref::from_addrefed(shape) })
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
}
impl Default for MutableCompoundShapeSettings {
    fn default() -> Self { Self::new() }
}

// --- inner-shape-owning settings (keep the inner RefConst alive) ---

/// Scales an inner shape.  Owns the inner shape's refcount so the pointer is always valid.
///
/// See also: Jolt's [`ScaledShapeSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_scaled_shape_settings.html) class.
pub struct ScaledShapeSettings {
    raw: JPC_ScaledShapeSettings,
    _inner: RefConst<JPC_Shape>,
}
impl ScaledShapeSettings {
    pub fn new(inner: &RefConst<JPC_Shape>, scale: Vec3) -> Self {
        Self { raw: JPC_ScaledShapeSettings { InnerShape: inner.get(), Scale: scale.into_jolt(), ..Default::default() }, _inner: inner.clone() }
    }
    pub fn scale(&self) -> Vec3 { Vec3::from_jolt(self.raw.Scale) }
    pub fn set_scale(&mut self, v: Vec3) { self.raw.Scale = v.into_jolt(); }
    pub fn user_data(&self) -> u64 { self.raw.UserData }
    pub fn set_user_data(&mut self, v: u64) { self.raw.UserData = v; }
    pub fn create(&self) -> Result<RefConst<JPC_Shape>, String> {
        create_shape_inner(|s, e| unsafe { JPC_ScaledShapeSettings_Create(&self.raw, s, e) })
    }
}

/// Rotates and translates an inner shape.  Owns the inner shape's refcount.
///
/// See also: Jolt's [`RotatedTranslatedShapeSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_rotated_translated_shape_settings.html) class.
pub struct RotatedTranslatedShapeSettings {
    raw: JPC_RotatedTranslatedShapeSettings,
    _inner: RefConst<JPC_Shape>,
}
impl RotatedTranslatedShapeSettings {
    pub fn new(inner: &RefConst<JPC_Shape>, position: Vec3, rotation: Quat) -> Self {
        Self { raw: JPC_RotatedTranslatedShapeSettings { InnerShape: inner.get(), Position: position.into_jolt(), Rotation: rotation.into_jolt(), ..Default::default() }, _inner: inner.clone() }
    }
    pub fn position(&self) -> Vec3 { Vec3::from_jolt(self.raw.Position) }
    pub fn set_position(&mut self, v: Vec3) { self.raw.Position = v.into_jolt(); }
    pub fn rotation(&self) -> Quat { Quat::from_jolt(self.raw.Rotation) }
    pub fn set_rotation(&mut self, v: Quat) { self.raw.Rotation = v.into_jolt(); }
    pub fn user_data(&self) -> u64 { self.raw.UserData }
    pub fn set_user_data(&mut self, v: u64) { self.raw.UserData = v; }
    pub fn create(&self) -> Result<RefConst<JPC_Shape>, String> {
        create_shape_inner(|s, e| unsafe { JPC_RotatedTranslatedShapeSettings_Create(&self.raw, s, e) })
    }
}

/// Shifts the center of mass of an inner shape.  Owns the inner shape's refcount.
///
/// See also: Jolt's [`OffsetCenterOfMassShapeSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_offset_center_of_mass_shape_settings.html) class.
pub struct OffsetCenterOfMassShapeSettings {
    raw: JPC_OffsetCenterOfMassShapeSettings,
    _inner: RefConst<JPC_Shape>,
}
impl OffsetCenterOfMassShapeSettings {
    pub fn new(inner: &RefConst<JPC_Shape>, offset: Vec3) -> Self {
        Self { raw: JPC_OffsetCenterOfMassShapeSettings { InnerShape: inner.get(), Offset: offset.into_jolt(), ..Default::default() }, _inner: inner.clone() }
    }
    pub fn offset(&self) -> Vec3 { Vec3::from_jolt(self.raw.Offset) }
    pub fn set_offset(&mut self, v: Vec3) { self.raw.Offset = v.into_jolt(); }
    pub fn user_data(&self) -> u64 { self.raw.UserData }
    pub fn set_user_data(&mut self, v: u64) { self.raw.UserData = v; }
    pub fn create(&self) -> Result<RefConst<JPC_Shape>, String> {
        create_shape_inner(|s, e| unsafe { JPC_OffsetCenterOfMassShapeSettings_Create(&self.raw, s, e) })
    }
}
