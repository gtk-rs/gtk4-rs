// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{TextBuffer, TextIter};
use glib::object::{Cast, IsA};
use glib::signal::{connect_raw, SignalHandlerId};
use glib::translate::*;
use libc::{c_char, c_int};
use std::boxed::Box as Box_;
use std::mem::transmute;
use std::{slice, str};

pub trait TextBufferExtManual: 'static {
    fn connect_insert_text<F: Fn(&Self, &mut TextIter, &str) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<TextBuffer>> TextBufferExtManual for O {
    fn connect_insert_text<F: Fn(&Self, &mut TextIter, &str) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.to_glib_none().0 as *mut _,
                b"insert-text\0".as_ptr() as *mut _,
                Some(transmute(insert_text_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe extern "C" fn insert_text_trampoline<T, F: Fn(&T, &mut TextIter, &str) + 'static>(
    this: *mut ffi::GtkTextBuffer,
    location: *mut ffi::GtkTextIter,
    text: *mut c_char,
    len: c_int,
    f: glib::ffi::gpointer,
) where
    T: IsA<TextBuffer>,
{
    let mut location_copy = from_glib_none(location);
    let f: &F = &*(f as *const F);
    f(
        TextBuffer::from_glib_borrow(this).unsafe_cast_ref(),
        &mut location_copy,
        str::from_utf8(slice::from_raw_parts(text as *const u8, len as usize)).unwrap(),
    )
}
