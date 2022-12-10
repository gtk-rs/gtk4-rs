// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, Editable};
use glib::{
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use libc::{c_char, c_int, c_uchar};
use std::{ffi::CStr, mem::transmute, slice, str};

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of [`Editable`](crate::Editable).
pub trait EditableExtManual: 'static {
    fn connect_insert_text<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self, &str, &mut i32) + 'static;
}

impl<T: IsA<Editable>> EditableExtManual for T {
    fn connect_insert_text<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self, &str, &mut i32) + 'static,
    {
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(
                self.to_glib_none().0 as *mut _,
                b"insert-text\0".as_ptr() as *mut _,
                Some(transmute(insert_text_trampoline::<Self, F> as usize)),
                Box::into_raw(f),
            )
        }
    }
}

unsafe extern "C" fn insert_text_trampoline<T, F: Fn(&T, &str, &mut i32) + 'static>(
    this: *mut ffi::GtkEditable,
    new_text: *mut c_char,
    new_text_length: c_int,
    position: *mut c_int,
    f: &F,
) where
    T: IsA<Editable>,
{
    let buf = if new_text_length == 0 {
        &[]
    } else if new_text_length != -1 {
        slice::from_raw_parts(new_text as *mut c_uchar, new_text_length as usize)
    } else {
        CStr::from_ptr(new_text).to_bytes()
    };
    let string = str::from_utf8(buf).unwrap();
    f(
        Editable::from_glib_borrow(this).unsafe_cast_ref(),
        string,
        &mut *position,
    );
}
