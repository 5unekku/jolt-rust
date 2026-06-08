mod generated;

pub use generated::*;

#[cfg(feature = "double-precision")]
pub type Real = f64;

#[cfg(not(feature = "double-precision"))]
pub type Real = f32;

macro_rules! ffi_default {
    ($($c_struct:ident -> $default_fn:ident,)*) => {
        $(
            impl Default for $c_struct {
                fn default() -> Self {
                    unsafe {
                        let mut settings = std::mem::MaybeUninit::<$c_struct>::zeroed();
                        $default_fn(settings.as_mut_ptr());
                        settings.assume_init()
                    }
                }
            }
        )*
    };
}

ffi_default! {
    JPC_BodyCreationSettings -> JPC_BodyCreationSettings_default,
    JPC_ShapeCastSettings -> JPC_ShapeCastSettings_default,
    JPC_CollideShapeSettings -> JPC_CollideShapeSettings_default,

    // All of the ShapeSettings types
    JPC_TriangleShapeSettings -> JPC_TriangleShapeSettings_default,
    JPC_MeshShapeSettings -> JPC_MeshShapeSettings_default,
    JPC_BoxShapeSettings -> JPC_BoxShapeSettings_default,
    JPC_SphereShapeSettings -> JPC_SphereShapeSettings_default,
    JPC_CapsuleShapeSettings -> JPC_CapsuleShapeSettings_default,
    JPC_CylinderShapeSettings -> JPC_CylinderShapeSettings_default,
    JPC_PlaneShapeSettings -> JPC_PlaneShapeSettings_default,
    JPC_ConvexHullShapeSettings -> JPC_ConvexHullShapeSettings_default,
    JPC_SubShapeSettings -> JPC_SubShapeSettings_default,
    JPC_StaticCompoundShapeSettings -> JPC_StaticCompoundShapeSettings_default,
    JPC_MutableCompoundShapeSettings -> JPC_MutableCompoundShapeSettings_default,

    // Character settings
    JPC_CharacterSettings -> JPC_CharacterSettings_default,
    JPC_CharacterVirtualSettings -> JPC_CharacterVirtualSettings_default,
    JPC_ExtendedUpdateSettings -> JPC_ExtendedUpdateSettings_default,

    // new shape settings
    JPC_HeightFieldShapeSettings -> JPC_HeightFieldShapeSettings_default,
    JPC_ScaledShapeSettings -> JPC_ScaledShapeSettings_default,
    JPC_RotatedTranslatedShapeSettings -> JPC_RotatedTranslatedShapeSettings_default,
    JPC_OffsetCenterOfMassShapeSettings -> JPC_OffsetCenterOfMassShapeSettings_default,
    JPC_TaperedCapsuleShapeSettings -> JPC_TaperedCapsuleShapeSettings_default,
    JPC_TaperedCylinderShapeSettings -> JPC_TaperedCylinderShapeSettings_default,
    JPC_EmptyShapeSettings -> JPC_EmptyShapeSettings_default,

    // new constraint settings
    JPC_PointConstraintSettings -> JPC_PointConstraintSettings_default,
    JPC_ConeConstraintSettings -> JPC_ConeConstraintSettings_default,
    JPC_PulleyConstraintSettings -> JPC_PulleyConstraintSettings_default,
    JPC_GearConstraintSettings -> JPC_GearConstraintSettings_default,
    JPC_RackAndPinionConstraintSettings -> JPC_RackAndPinionConstraintSettings_default,
    JPC_SwingTwistConstraintSettings -> JPC_SwingTwistConstraintSettings_default,
    JPC_PathConstraintSettings -> JPC_PathConstraintSettings_default,

    // vehicle
    JPC_VehicleEngineSettings -> JPC_VehicleEngineSettings_default,
    JPC_VehicleTransmissionSettings -> JPC_VehicleTransmissionSettings_default,
    JPC_VehicleDifferentialSettings -> JPC_VehicleDifferentialSettings_default,

    // soft body
    JPC_SoftBodyCreationSettings -> JPC_SoftBodyCreationSettings_default,
}
