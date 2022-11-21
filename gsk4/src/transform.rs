// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Transform;
use glib::translate::*;

impl Transform {
    #[doc(alias = "gsk_transform_parse")]
    pub fn parse(string: &str) -> Result<Self, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            let mut out_transform = std::ptr::null_mut();
            let ret = from_glib(ffi::gsk_transform_parse(
                string.to_glib_none().0,
                &mut out_transform,
            ));
            if ret {
                Ok(from_glib_full(out_transform))
            } else {
                Err(glib::bool_error!("Can't parse Transform"))
            }
        }
    }

    #[doc(alias = "gsk_transform_invert")]
    pub fn invert(&self) -> Result<Self, glib::BoolError> {
        unsafe {
            let matrix = self.to_matrix();
            if matrix == graphene::Matrix::new_identity() {
                return Ok(self.clone());
            }

            let res: Option<Self> = from_glib_full(ffi::gsk_transform_invert(self.to_glib_full()));
            res.ok_or_else(|| glib::bool_error!("Failed to invert the transform"))
        }
    }

    #[doc(alias = "gsk_transform_rotate")]
    #[must_use]
    pub fn rotate(&self, angle: f32) -> Self {
        unsafe {
            let res: Option<Self> =
                from_glib_full(ffi::gsk_transform_rotate(self.to_glib_full(), angle));
            res.unwrap_or_else(Self::new)
        }
    }

    #[doc(alias = "gsk_transform_rotate_3d")]
    #[must_use]
    pub fn rotate_3d(&self, angle: f32, axis: &graphene::Vec3) -> Self {
        unsafe {
            let res: Option<Self> = from_glib_full(ffi::gsk_transform_rotate_3d(
                self.to_glib_full(),
                angle,
                axis.to_glib_none().0,
            ));
            res.unwrap_or_else(Self::new)
        }
    }

    #[doc(alias = "gsk_transform_scale")]
    #[must_use]
    pub fn scale(&self, factor_x: f32, factor_y: f32) -> Self {
        unsafe {
            let res: Option<Self> = from_glib_full(ffi::gsk_transform_scale(
                self.to_glib_full(),
                factor_x,
                factor_y,
            ));
            res.unwrap_or_else(Self::new)
        }
    }

    #[doc(alias = "gsk_transform_scale_3d")]
    #[must_use]
    pub fn scale_3d(&self, factor_x: f32, factor_y: f32, factor_z: f32) -> Self {
        unsafe {
            let res: Option<Self> = from_glib_full(ffi::gsk_transform_scale_3d(
                self.to_glib_full(),
                factor_x,
                factor_y,
                factor_z,
            ));
            res.unwrap_or_else(Self::new)
        }
    }

    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    #[doc(alias = "gsk_transform_skew")]
    #[must_use]
    pub fn skew(&self, skew_x: f32, skew_y: f32) -> Self {
        unsafe {
            let res: Option<Self> =
                from_glib_full(ffi::gsk_transform_skew(self.to_glib_full(), skew_x, skew_y));
            res.unwrap_or_else(Self::new)
        }
    }

    #[doc(alias = "gsk_transform_transform")]
    #[must_use]
    pub fn transform(&self, other: Option<&Self>) -> Self {
        unsafe {
            let res: Option<Self> = from_glib_full(ffi::gsk_transform_transform(
                self.to_glib_full(),
                other.to_glib_none().0,
            ));
            res.unwrap_or_else(Self::new)
        }
    }

    #[doc(alias = "gsk_transform_translate")]
    #[must_use]
    pub fn translate(&self, point: &graphene::Point) -> Self {
        unsafe {
            let res: Option<Self> = from_glib_full(ffi::gsk_transform_translate(
                self.to_glib_full(),
                point.to_glib_none().0,
            ));
            res.unwrap_or_else(Self::new)
        }
    }

    #[doc(alias = "gsk_transform_translate_3d")]
    #[must_use]
    pub fn translate_3d(&self, point: &graphene::Point3D) -> Self {
        unsafe {
            let res: Option<Self> = from_glib_full(ffi::gsk_transform_translate_3d(
                self.to_glib_full(),
                point.to_glib_none().0,
            ));
            res.unwrap_or_else(Self::new)
        }
    }
}

impl std::str::FromStr for Transform {
    type Err = glib::BoolError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        skip_assert_initialized!();
        Self::parse(s)
    }
}

#[test]
fn invert_identity_is_identity() {
    let transform = Transform::new();
    let output = transform.invert();
    assert_eq!(output.unwrap(), transform);
}
