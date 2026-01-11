// Take a look at the license at the top of the repository in the LICENSE file.

use std::{boxed::Box as Box_, mem::transmute};

use glib::{
    signal::{SignalHandlerId, connect_raw},
    translate::*,
};

use crate::{Toplevel, ToplevelSize, ffi, prelude::*};

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of
/// [`Toplevel`](crate::Toplevel).
pub trait ToplevelExtManual: IsA<Toplevel> {
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
            unsafe {
                let f: &F = &*(f as *const F);
                f(&from_glib_borrow(this), &mut *(size as *mut ToplevelSize))
            }
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"compute-size".as_ptr() as *const _,
                Some(transmute::<*const (), unsafe extern "C" fn()>(
                    compute_size_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Toplevel>> ToplevelExtManual for O {}
