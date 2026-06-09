use joltc_sys::{JPC_Color, JPC_DVec3, JPC_Mat44, JPC_Quat, JPC_Vec2, JPC_Vec3, JPC_Vec4};

#[allow(unused_imports)]
pub use joltc_sys::Real;

pub use glam::{DMat4, DVec3, Mat4, Quat, Vec2, Vec3, Vec4};

use crate::{FromJolt, IntoJolt};

/// Represents a world-space vector, which can use either `f32` or `f64`.
///
/// Because the `double-precision` feature is enabled, this uses `f64`.
#[cfg(feature = "double-precision")]
pub type RVec3 = DVec3;

/// Represents a world-space vector, which can use either `f32` or `f64`.
///
/// Because the `double-precision` feature is NOT enabled, this uses `f32`.
#[cfg(not(feature = "double-precision"))]
pub type RVec3 = Vec3;

impl IntoJolt for Vec3 {
    type Jolt = JPC_Vec3;

    fn into_jolt(self) -> Self::Jolt {
        JPC_Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
            _w: self.z,
        }
    }
}

impl FromJolt for Vec3 {
    type Jolt = JPC_Vec3;

    fn from_jolt(value: Self::Jolt) -> Self {
        Vec3::new(value.x, value.y, value.z)
    }
}

impl IntoJolt for Vec4 {
    type Jolt = JPC_Vec4;

    fn into_jolt(self) -> Self::Jolt {
        JPC_Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w,
        }
    }
}

impl FromJolt for Vec4 {
    type Jolt = JPC_Vec4;

    fn from_jolt(value: Self::Jolt) -> Self {
        Vec4::new(value.x, value.y, value.z, value.w)
    }
}

impl IntoJolt for DVec3 {
    type Jolt = JPC_DVec3;

    fn into_jolt(self) -> Self::Jolt {
        JPC_DVec3 {
            x: self.x,
            y: self.y,
            z: self.z,
            _w: self.z,
        }
    }
}

impl FromJolt for DVec3 {
    type Jolt = JPC_DVec3;

    fn from_jolt(value: Self::Jolt) -> Self {
        DVec3::new(value.x, value.y, value.z)
    }
}

impl IntoJolt for Quat {
    type Jolt = JPC_Quat;

    fn into_jolt(self) -> Self::Jolt {
        JPC_Quat {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w,
        }
    }
}

impl FromJolt for Quat {
    type Jolt = JPC_Quat;

    fn from_jolt(value: Self::Jolt) -> Self {
        Quat::from_xyzw(value.x, value.y, value.z, value.w)
    }
}

impl IntoJolt for Mat4 {
    type Jolt = JPC_Mat44;

    fn into_jolt(self) -> Self::Jolt {
        JPC_Mat44 {
            col: [
                self.x_axis.into_jolt(),
                self.y_axis.into_jolt(),
                self.z_axis.into_jolt(),
            ],
            col3: self.w_axis.truncate().into_jolt(),
        }
    }
}

impl FromJolt for Mat4 {
    type Jolt = JPC_Mat44;

    fn from_jolt(value: Self::Jolt) -> Self {
        Mat4::from_cols(
            Vec4::from_jolt(value.col[0]),
            Vec4::from_jolt(value.col[1]),
            Vec4::from_jolt(value.col[2]),
            Vec3::from_jolt(value.col3).extend(1.0),
        )
    }
}

impl IntoJolt for Vec2 {
    type Jolt = JPC_Vec2;

    fn into_jolt(self) -> Self::Jolt {
        JPC_Vec2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl FromJolt for Vec2 {
    type Jolt = JPC_Vec2;

    fn from_jolt(value: Self::Jolt) -> Self {
        Vec2::new(value.x, value.y)
    }
}

/// World-space 4x4 matrix — `Mat4` in single-precision, `DMat4` in double-precision.
#[cfg(feature = "double-precision")]
pub type RMat4 = DMat4;

/// World-space 4x4 matrix — `Mat4` in single-precision, `DMat4` in double-precision.
#[cfg(not(feature = "double-precision"))]
pub type RMat4 = Mat4;

#[cfg(feature = "double-precision")]
impl IntoJolt for DMat4 {
    type Jolt = joltc_sys::JPC_DMat44;

    fn into_jolt(self) -> Self::Jolt {
        joltc_sys::JPC_DMat44 {
            col: [
                Vec4::new(
                    self.x_axis.x as f32,
                    self.x_axis.y as f32,
                    self.x_axis.z as f32,
                    self.x_axis.w as f32,
                )
                .into_jolt(),
                Vec4::new(
                    self.y_axis.x as f32,
                    self.y_axis.y as f32,
                    self.y_axis.z as f32,
                    self.y_axis.w as f32,
                )
                .into_jolt(),
                Vec4::new(
                    self.z_axis.x as f32,
                    self.z_axis.y as f32,
                    self.z_axis.z as f32,
                    self.z_axis.w as f32,
                )
                .into_jolt(),
            ],
            col3: DVec3::new(self.w_axis.x, self.w_axis.y, self.w_axis.z).into_jolt(),
            ..Default::default()
        }
    }
}

#[cfg(feature = "double-precision")]
impl FromJolt for DMat4 {
    type Jolt = joltc_sys::JPC_DMat44;

    fn from_jolt(value: Self::Jolt) -> Self {
        DMat4::from_cols(
            glam::DVec4::new(
                value.col[0].x as f64,
                value.col[0].y as f64,
                value.col[0].z as f64,
                value.col[0].w as f64,
            ),
            glam::DVec4::new(
                value.col[1].x as f64,
                value.col[1].y as f64,
                value.col[1].z as f64,
                value.col[1].w as f64,
            ),
            glam::DVec4::new(
                value.col[2].x as f64,
                value.col[2].y as f64,
                value.col[2].z as f64,
                value.col[2].w as f64,
            ),
            glam::DVec4::new(value.col3.x, value.col3.y, value.col3.z, 1.0),
        )
    }
}

/// Represents an sRGB color with alpha.
#[repr(C)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl IntoJolt for Color {
    type Jolt = JPC_Color;

    fn into_jolt(self) -> Self::Jolt {
        JPC_Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}

impl FromJolt for Color {
    type Jolt = JPC_Color;

    fn from_jolt(value: Self::Jolt) -> Self {
        Self {
            r: value.r,
            g: value.g,
            b: value.b,
            a: value.a,
        }
    }
}
