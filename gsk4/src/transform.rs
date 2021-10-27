// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Transform;
use glib::translate::*;

impl Transform {
    #[doc(alias = "gsk_transform_parse")]
    pub fn parse(string: &str) -> Result<Transform, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            let mut out_transform = ptr::null_mut();
            let ret = from_glib(ffi::gsk_transform_parse(
                string.to_glib_none().0,
                &mut out_transform,
            ));
            if ret {
                Ok(from_glib_full(out_transform))
            } else {
                glib::bool_error!("Can't parse Transform")
            }
        }
    }
}

impl std::str::FromStr for Transform {
    type Err = glib::BoolError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        skip_assert_initialized!();
        Transform::parse(s)
    }
}
