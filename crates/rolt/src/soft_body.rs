use joltc_sys::*;

use crate::Ref;

/// Shared (ref-counted) settings for a soft body — mesh, edges, faces, constraints.
///
/// See also: Jolt's [`SoftBodySharedSettings`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_soft_body_shared_settings.html) class.
pub struct SoftBodySharedSettings(pub Ref<JPC_SoftBodySharedSettings>);

impl SoftBodySharedSettings {
    pub fn new() -> Self {
        let raw = unsafe { JPC_SoftBodySharedSettings_new() };
        // new() gives refcount=0, so use from_active to AddRef
        Self(unsafe { Ref::from_active(raw) })
    }

    pub fn add_vertex(&mut self, vertex: &JPC_SoftBodyVertex) {
        unsafe { JPC_SoftBodySharedSettings_AddVertex(*self.0, vertex) }
    }

    pub fn add_edge_constraint(&mut self, edge: &JPC_SoftBodyEdgeConstraint) {
        unsafe { JPC_SoftBodySharedSettings_AddEdgeConstraint(*self.0, edge) }
    }

    pub fn add_face(&mut self, face: &JPC_SoftBodyFace) {
        unsafe { JPC_SoftBodySharedSettings_AddFace(*self.0, face) }
    }

    pub fn add_dihedral_bend(&mut self, bend: &JPC_SoftBodyDihedralBend) {
        unsafe { JPC_SoftBodySharedSettings_AddDihedralBend(*self.0, bend) }
    }

    pub fn add_volume_constraint(&mut self, volume: &JPC_SoftBodyVolume) {
        unsafe { JPC_SoftBodySharedSettings_AddVolumeConstraint(*self.0, volume) }
    }

    pub fn add_skinned(&mut self, skinned: &JPC_SoftBodySkinned) {
        unsafe { JPC_SoftBodySharedSettings_AddSkinned(*self.0, skinned) }
    }

    pub fn add_inv_bind(&mut self, inv_bind: &JPC_SoftBodyInvBind) {
        unsafe { JPC_SoftBodySharedSettings_AddInvBind(*self.0, inv_bind) }
    }

    pub fn add_lra(&mut self, lra: &JPC_SoftBodyLRA) {
        unsafe { JPC_SoftBodySharedSettings_AddLRA(*self.0, lra) }
    }

    /// Add a physics material slot.  The material pointer must outlive this settings object.
    pub unsafe fn add_material(&mut self, material: *const JPC_PhysicsMaterial) {
        JPC_SoftBodySharedSettings_AddMaterial(*self.0, material)
    }

    /// Auto-generate edge/bend constraints from the mesh.
    pub fn create_constraints(
        &mut self,
        vertex_attributes: &[JPC_SoftBodyVertexAttributes],
        bend_type: JPC_SoftBodyEBendType,
        angle_tolerance: f32,
    ) {
        unsafe {
            JPC_SoftBodySharedSettings_CreateConstraints(
                *self.0,
                vertex_attributes.as_ptr(),
                vertex_attributes.len() as u32,
                bend_type,
                angle_tolerance,
            )
        }
    }

    pub fn optimize(&mut self) {
        unsafe { JPC_SoftBodySharedSettings_Optimize(*self.0) }
    }

    pub fn calculate_edge_lengths(&mut self) {
        unsafe { JPC_SoftBodySharedSettings_CalculateEdgeLengths(*self.0) }
    }

    pub fn calculate_bend_constraint_constants(&mut self) {
        unsafe { JPC_SoftBodySharedSettings_CalculateBendConstraintConstants(*self.0) }
    }

    pub fn calculate_volume_constraint_volumes(&mut self) {
        unsafe { JPC_SoftBodySharedSettings_CalculateVolumeConstraintVolumes(*self.0) }
    }

    pub fn calculate_lra_lengths(&mut self, max_distance_multiplier: f32) {
        unsafe { JPC_SoftBodySharedSettings_CalculateLRALengths(*self.0, max_distance_multiplier) }
    }

    pub fn calculate_skinned_constraint_normals(&mut self) {
        unsafe { JPC_SoftBodySharedSettings_CalculateSkinnedConstraintNormals(*self.0) }
    }

    pub fn with_raw<R>(&self, f: impl FnOnce(*mut JPC_SoftBodySharedSettings) -> R) -> R {
        self.0.with_raw(f)
    }

    pub fn raw(&self) -> *mut JPC_SoftBodySharedSettings {
        *self.0
    }
}

impl Default for SoftBodySharedSettings {
    fn default() -> Self {
        Self::new()
    }
}
