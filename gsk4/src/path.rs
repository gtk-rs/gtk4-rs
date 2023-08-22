// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Path, PathForeachFlags, PathOperation};
use glib::translate::*;

impl Path {
    #[doc(alias = "gsk_path_foreach")]
    pub fn foreach<P: FnMut(&PathOperation, &graphene::Point, usize) -> bool>(
        &self,
        flags: PathForeachFlags,
        func: P,
    ) -> bool {
        let func_data: P = func;
        unsafe extern "C" fn func_func<
            P: FnMut(&PathOperation, &graphene::Point, usize) -> bool,
        >(
            op: ffi::GskPathOperation,
            pts: *const graphene::ffi::graphene_point_t,
            n_pts: libc::size_t,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let op = from_glib(op);
            let pts = from_glib_borrow(pts);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            (*callback)(&op, &pts, n_pts).into_glib()
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &P = &func_data;
        unsafe {
            from_glib(ffi::gsk_path_foreach(
                self.to_glib_none().0,
                flags.into_glib(),
                func,
                super_callback0 as *const _ as usize as *mut _,
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
