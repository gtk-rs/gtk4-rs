// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::{ffi, Path, PathForeachFlags, PathOperation};

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
            let op = from_glib(op);
            let pts = from_glib_borrow(pts);
            let callback = user_data as *mut P;
            (*callback)(&op, &pts, n_pts, weight).into_glib()
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
}

impl std::str::FromStr for Path {
    type Err = glib::BoolError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_initialized_main_thread!();
        Path::parse(s)
    }
}
