// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

use crate::ffi;
use glib::translate::*;

#[repr(C)]
#[derive(Clone)]
pub struct CssLocation(ffi::GtkCssLocation);

impl CssLocation {
    pub fn new(
        bytes: usize,
        chars: usize,
        lines: usize,
        line_bytes: usize,
        line_chars: usize,
    ) -> CssLocation {
        assert_initialized_main_thread!();
        CssLocation(ffi::GtkCssLocation {
            bytes,
            chars,
            lines,
            line_bytes,
            line_chars,
        })
    }

    pub fn get_bytes(&self) -> usize {
        self.0.bytes
    }

    pub fn get_chars(&self) -> usize {
        self.0.chars
    }

    pub fn get_lines(&self) -> usize {
        self.0.lines
    }

    pub fn get_line_bytes(&self) -> usize {
        self.0.line_bytes
    }

    pub fn get_line_chars(&self) -> usize {
        self.0.line_chars
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::GtkCssLocation> for CssLocation {
    unsafe fn from_glib_none(ptr: *const ffi::GtkCssLocation) -> Self {
        CssLocation(*ptr)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::GtkCssLocation> for CssLocation {
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> Stash<*const ffi::GtkCssLocation, Self> {
        Stash(&self.0, self)
    }
}
