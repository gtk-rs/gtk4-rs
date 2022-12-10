// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, Overlay, Widget};
use glib::{
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{mem::transmute, ptr};

impl Overlay {
    pub fn connect_get_child_position<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self, &Widget) -> Option<gdk::Rectangle> + 'static,
    {
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"get-child-position\0".as_ptr() as *mut _,
                Some(transmute(get_child_position_trampoline::<F> as usize)),
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
