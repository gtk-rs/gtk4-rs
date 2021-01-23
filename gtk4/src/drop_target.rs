// Take a look at the license at the top of the repository in the LICENSE file.

use crate::DropTarget;
use glib::signal::connect_raw;
use glib::{translate::*, ObjectType, SignalHandlerId};
use std::boxed::Box as Box_;
use std::mem::transmute;

impl DropTarget {
    pub fn connect_drop<F: Fn(&DropTarget, &glib::Value, f64, f64) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn drop_trampoline<
            F: Fn(&DropTarget, &glib::Value, f64, f64) -> bool + 'static,
        >(
            this: *mut ffi::GtkDropTarget,
            value: *mut glib::gobject_ffi::GValue,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &*(value as *const glib::Value),
                x,
                y,
            )
            .to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"drop\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    drop_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
