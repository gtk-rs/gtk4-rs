// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Path, PathPoint};
use glib::translate::*;

impl PathPoint {
    #[doc(alias = "gsk_path_point_get_curvature")]
    #[doc(alias = "get_curvature")]
    pub fn curvature(&self, path: &Path) -> (f32, Option<graphene::Point>) {
        unsafe {
            let mut center = graphene::Point::uninitialized();
            let ret = ffi::gsk_path_point_get_curvature(
                self.to_glib_none().0,
                path.to_glib_none().0,
                center.to_glib_none_mut().0,
            );

            if ret == 0.0 {
                (ret, None)
            } else {
                (ret, Some(center))
            }
        }
    }
}
