// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::{ffi, Path, PathDirection, PathPoint};

impl PathPoint {
    #[doc(alias = "gsk_path_point_get_curvature")]
    #[doc(alias = "get_curvature")]
    pub fn curvature(
        &self,
        path: &Path,
        direction: PathDirection,
    ) -> (f32, Option<graphene::Point>) {
        unsafe {
            let mut center = graphene::Point::uninitialized();
            let ret = ffi::gsk_path_point_get_curvature(
                self.to_glib_none().0,
                path.to_glib_none().0,
                direction.into_glib(),
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
