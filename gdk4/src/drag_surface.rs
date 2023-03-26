// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, DragSurface, DragSurfaceSize};
use glib::{
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, mem::transmute};

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of [`DragSurface`](crate::DragSurface).
pub trait DragSurfaceExtManual {
    #[cfg(any(feature = "v4_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_12")))]
    #[doc(alias = "compute-size")]
    fn connect_compute_size<F: Fn(&DragSurface, &mut DragSurfaceSize) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<DragSurface>> DragSurfaceExtManual for O {
    #[cfg(any(feature = "v4_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_12")))]
    fn connect_compute_size<F: Fn(&DragSurface, &mut DragSurfaceSize) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn compute_size_trampoline<
            F: Fn(&DragSurface, &mut DragSurfaceSize) + 'static,
        >(
            this: *mut ffi::GdkDragSurface,
            size: *mut ffi::GdkDragSurfaceSize,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &mut *(size as *mut DragSurfaceSize),
            )
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
