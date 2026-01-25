// Take a look at the license at the top of the repository in the LICENSE file.

use std::{mem::transmute, ptr};

use glib::{
    signal::{SignalHandlerId, connect_raw},
    translate::*,
};

use crate::{Overlay, Widget, ffi, prelude::*};

impl Overlay {
    pub fn connect_get_child_position<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self, &Widget) -> Option<gdk::Rectangle> + 'static,
    {
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"get-child-position".as_ptr() as *mut _,
                Some(transmute::<*const (), unsafe extern "C" fn()>(
                    get_child_position_trampoline::<F> as *const (),
                )),
                Box::into_raw(f),
            )
        }
    }
}

unsafe extern "C" fn get_child_position_trampoline<
    F: Fn(&Overlay, &Widget) -> Option<gdk::Rectangle> + 'static,
>(
    this: *mut ffi::GtkOverlay,
    widget: *mut ffi::GtkWidget,
    allocation: *mut gdk::ffi::GdkRectangle,
    f: glib::ffi::gpointer,
) -> glib::ffi::gboolean {
    unsafe {
        let f: &F = &*(f as *const F);
        match f(
            Overlay::from_glib_borrow(this).unsafe_cast_ref(),
            &from_glib_borrow(widget),
        ) {
            Some(rect) => {
                ptr::write(allocation, ptr::read(rect.to_glib_none().0));
                true
            }
            None => false,
        }
        .into_glib()
    }
}
