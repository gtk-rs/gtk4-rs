// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::object::{Cast, IsA};
use glib::signal::{connect_raw, SignalHandlerId};
use glib::translate::*;
use glib_sys;
use libc::{c_char, c_int};
use std::boxed::Box as Box_;
use std::mem::transmute;
use std::{slice, str};
use gtk_sys;
use TextBuffer;
use TextIter;

pub trait TextBufferExtManual: 'static {
    fn connect_insert_text<F: Fn(&Self, &mut TextIter, &str) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TextBuffer>> TextBufferExtManual for O {
    fn connect_insert_text<F: Fn(&Self, &mut TextIter, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.to_glib_none().0 as *mut _, b"insert-text\0".as_ptr() as *mut _,
                Some(transmute(insert_text_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn insert_text_trampoline<T, F: Fn(&T, &mut TextIter, &str) + 'static>(
    this: *mut gtk_sys::GtkTextBuffer,
    location: *mut gtk_sys::GtkTextIter,
    text: *mut c_char,
    len: c_int,
    f: glib_sys::gpointer,
) where T: IsA<TextBuffer> {
    let f: &F = &*(f as *const F);
    f(&TextBuffer::from_glib_borrow(this).unsafe_cast(),
      &mut from_glib_borrow(location),
      str::from_utf8(slice::from_raw_parts(text as *const u8, len as usize)).unwrap())
}
