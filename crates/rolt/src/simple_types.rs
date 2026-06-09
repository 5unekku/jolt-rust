use joltc_sys::*;

#[allow(unused_imports)]
// re-export commonly needed joltc_sys types so users don't need to import joltc_sys directly
pub use joltc_sys::{
    JPC_Activation,
    JPC_ACTIVATION_ACTIVATE,
    JPC_ACTIVATION_DONT_ACTIVATE,
    JPC_MotionType,
    JPC_MOTION_TYPE_STATIC,
    JPC_MOTION_TYPE_KINEMATIC,
    JPC_MOTION_TYPE_DYNAMIC,
    JPC_MotionQuality,
    JPC_MOTION_QUALITY_DISCRETE,
    JPC_MOTION_QUALITY_LINEAR_CAST,
    JPC_AllowedDOFs,
    JPC_ALLOWED_DOFS_NONE,
    JPC_ALLOWED_DOFS_ALL,
    JPC_ALLOWED_DOFS_TRANSLATIONX,
    JPC_ALLOWED_DOFS_TRANSLATIONY,
    JPC_ALLOWED_DOFS_TRANSLATIONZ,
    JPC_ALLOWED_DOFS_ROTATIONX,
    JPC_ALLOWED_DOFS_ROTATIONY,
    JPC_ALLOWED_DOFS_ROTATIONZ,
    JPC_ALLOWED_DOFS_PLANE2D,
    JPC_OverrideMassProperties,
    JPC_OVERRIDE_MASS_PROPS_CALC_MASS_INERTIA,
    JPC_OVERRIDE_MASS_PROPS_CALC_INERTIA,
    JPC_OVERRIDE_MASS_PROPS_MASS_INERTIA_PROVIDED,
    JPC_ConstraintSpace,
    JPC_CONSTRAINT_SPACE_LOCAL_TO_BODY_COM,
    JPC_CONSTRAINT_SPACE_WORLD_SPACE,
    JPC_SpringMode,
    JPC_SPRING_MODE_FREQUENCY_AND_DAMPING,
    JPC_SPRING_MODE_STIFFNESS_AND_DAMPING,
    JPC_GroundState,
    JPC_CHARACTER_GROUND_STATE_ON_GROUND,
    JPC_CHARACTER_GROUND_STATE_ON_STEEP_GROUND,
    JPC_CHARACTER_GROUND_STATE_NOT_SUPPORTED,
    JPC_CHARACTER_GROUND_STATE_IN_AIR,
    JPC_SwingType,
    JPC_SWING_TYPE_CONE,
    JPC_SWING_TYPE_PYRAMID,
    JPC_BackFaceMode,
    JPC_BACK_FACE_MODE_IGNORE,
    JPC_BACK_FACE_MODE_COLLIDE,
    JPC_ActiveEdgeMode,
    JPC_ACTIVE_EDGE_MODE_COLLIDE_ONLY_WITH_ACTIVE,
    JPC_ACTIVE_EDGE_MODE_COLLIDE_WITH_ALL,
    JPC_CollectFacesMode,
    JPC_COLLECT_FACES_MODE_COLLECT_FACES,
    JPC_COLLECT_FACES_MODE_NO_FACES,
    JPC_VehicleTransmissionMode,
    JPC_VEHICLE_TRANSMISSION_MODE_AUTO,
    JPC_VEHICLE_TRANSMISSION_MODE_MANUAL,
    JPC_ShapeColor,
    JPC_SHAPE_COLOR_INSTANCE_COLOR,
    JPC_SHAPE_COLOR_SHAPE_TYPE_COLOR,
    JPC_SHAPE_COLOR_MOTION_TYPE_COLOR,
    JPC_SHAPE_COLOR_SLEEP_COLOR,
    JPC_SHAPE_COLOR_ISLAND_COLOR,
    JPC_SHAPE_COLOR_MATERIAL_COLOR,
    JPC_SoftBodyConstraintColor,
    JPC_SOFT_BODY_CONSTRAINT_COLOR_CONSTRAINT_TYPE,
    JPC_SOFT_BODY_CONSTRAINT_COLOR_CONSTRAINT_GROUP,
    JPC_CollisionGroup,
    JPC_SubShapeID,
    JPC_ShapeType,
    JPC_ShapeSubType,
    JPC_CharacterID,
};

/// Represents an object layer, which is internally either a u16 or a u32.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectLayer(JPC_ObjectLayer);

impl ObjectLayer {
    pub const fn new(value: JPC_ObjectLayer) -> Self {
        Self(value)
    }

    pub const fn raw(self) -> JPC_ObjectLayer {
        self.0
    }
}

/// Represents a broad phase layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BroadPhaseLayer(JPC_BroadPhaseLayer);

impl BroadPhaseLayer {
    pub const fn new(value: JPC_BroadPhaseLayer) -> Self {
        Self(value)
    }

    pub const fn raw(self) -> JPC_BroadPhaseLayer {
        self.0
    }
}

/// An ID that can be used to access a body using [`BodyInterface`][crate::BodyInterface].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd)]
pub struct BodyId(JPC_BodyID);

impl BodyId {
    /// The sentinel value Jolt uses for an unassigned body ID.
    pub const INVALID: Self = Self(0xffffffff);

    pub const fn new(value: JPC_BodyID) -> Self {
        Self(value)
    }

    pub const fn raw(self) -> JPC_BodyID {
        self.0
    }

    pub const fn is_invalid(self) -> bool {
        self.0 == 0xffffffff
    }
}
