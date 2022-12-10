// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, Toplevel, ToplevelSize};
use glib::{
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, mem::transmute};

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of [`Toplevel`](crate::Toplevel).
pub trait ToplevelExtManual {
    fn connect_compute_size<F: Fn(&Toplevel, &mut ToplevelSize) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<Toplevel>> ToplevelExtManual for O {
    fn connect_compute_size<F: Fn(&Toplevel, &mut ToplevelSize) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn compute_size_trampoline<
            F: Fn(&Toplevel, &mut ToplevelSize) + 'static,
        >(
            this: *mut ffi::GdkToplevel,
            size: *mut ffi::GdkToplevelSize,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &mut *(size as *mut ToplevelSize))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"compute-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    compute_size_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
