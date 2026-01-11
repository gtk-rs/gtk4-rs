// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::{Path, PathForeachFlags, PathOperation, ffi};
#[cfg(feature = "v4_20")]
use crate::{PathIntersection, PathPoint};

impl Path {
    #[doc(alias = "gsk_path_foreach")]
    pub fn foreach<P: FnMut(&PathOperation, &graphene::Point, usize, f32) -> glib::ControlFlow>(
        &self,
        flags: PathForeachFlags,
        func: P,
    ) -> glib::ControlFlow {
        let mut func_data: P = func;
        unsafe extern "C" fn func_func<
            P: FnMut(&PathOperation, &graphene::Point, usize, f32) -> glib::ControlFlow,
        >(
            op: ffi::GskPathOperation,
            pts: *const graphene::ffi::graphene_point_t,
            n_pts: libc::size_t,
            weight: libc::c_float,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            unsafe {
                let op = from_glib(op);
                let pts = from_glib_borrow(pts);
                let callback = user_data as *mut P;
                (*callback)(&op, &pts, n_pts, weight).into_glib()
            }
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &mut P = &mut func_data;
        unsafe {
            from_glib(ffi::gsk_path_foreach(
                self.to_glib_none().0,
                flags.into_glib(),
                func,
                super_callback0 as *mut _ as *mut _,
            ))
        }
    }

    #[cfg(feature = "v4_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_20")))]
    #[doc(alias = "gsk_path_foreach_intersection")]
    pub fn foreach_intersection<
        P: FnMut(&Path, &PathPoint, &Path, &PathPoint, PathIntersection) -> bool,
    >(
        &self,
        path2: Option<&Path>,
        func: P,
    ) -> bool {
        let mut func_data: P = func;
        unsafe extern "C" fn func_func<
            P: FnMut(&Path, &PathPoint, &Path, &PathPoint, PathIntersection) -> bool,
        >(
            path1: *mut ffi::GskPath,
            point1: *const ffi::GskPathPoint,
            path2: *mut ffi::GskPath,
            point2: *const ffi::GskPathPoint,
            kind: ffi::GskPathIntersection,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            unsafe {
                let path1 = from_glib_borrow(path1);
                let point1 = from_glib_borrow(point1);
                let path2 = from_glib_borrow(path2);
                let point2 = from_glib_borrow(point2);
                let kind = from_glib(kind);
                let callback = user_data as *mut P;
                (*callback)(&path1, &point1, &path2, &point2, kind).into_glib()
            }
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &mut P = &mut func_data;
        unsafe {
            from_glib(ffi::gsk_path_foreach_intersection(
                self.to_glib_none().0,
                path2.to_glib_none().0,
                func,
                super_callback0 as *mut _ as *mut _,
            ))
        }
    }
}

impl std::str::FromStr for Path {
    type Err = glib::BoolError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_initialized_main_thread!();
        Path::parse(s)
    }
}
