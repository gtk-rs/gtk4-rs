// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, SpinButton};
use glib::{
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use libc::{c_double, c_int};
use std::{boxed::Box as Box_, mem::transmute};

impl SpinButton {
    pub fn connect_input<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self) -> Option<Result<f64, ()>> + 'static,
    {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"input\0".as_ptr() as *mut _,
                Some(transmute(input_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe extern "C" fn input_trampoline<F: Fn(&SpinButton) -> Option<Result<f64, ()>> + 'static>(
    this: *mut ffi::GtkSpinButton,
    new_value: *mut c_double,
    f: &F,
) -> c_int {
    match f(SpinButton::from_glib_borrow(this).unsafe_cast_ref()) {
        Some(Ok(v)) => {
            *new_value = v;
            glib::ffi::GTRUE
        }
        Some(Err(_)) => ffi::GTK_INPUT_ERROR,
        None => glib::ffi::GFALSE,
    }
}
