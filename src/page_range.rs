// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use std::ptr;
use gtk_sys;

#[repr(C)]
pub struct PageRange(gtk_sys::GtkPageRange);

impl PageRange {
    pub fn new(start: i32, end: i32) -> PageRange {
        skip_assert_initialized!();
        PageRange(gtk_sys::GtkPageRange {
            start,
            end,
        })
    }

    pub fn get_start(&self) -> i32 {
        self.0.start
    }

    pub fn get_end(&self) -> i32 {
        self.0.end
    }
}

#[doc(hidden)]
impl ToGlib for PageRange {
    type GlibType = gtk_sys::GtkPageRange;

    fn to_glib(&self) -> gtk_sys::GtkPageRange {
        self.0
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const gtk_sys::GtkPageRange> for PageRange {
    type Storage = Box<gtk_sys::GtkPageRange>;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const gtk_sys::GtkPageRange, Self> {
        let page_range = Box::new(self.0);
        Stash(&*page_range, page_range)
    }
}

impl FromGlibContainerAsVec<gtk_sys::GtkPageRange, *mut gtk_sys::GtkPageRange> for PageRange {
    unsafe fn from_glib_none_num_as_vec(ptr: *mut gtk_sys::GtkPageRange, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }

        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            res.push(PageRange(ptr::read(ptr.offset(i as isize))));
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(_: *mut gtk_sys::GtkPageRange, _: usize) -> Vec<Self> {
        unimplemented!();
    }

    unsafe fn from_glib_full_num_as_vec(_: *mut gtk_sys::GtkPageRange, _: usize) -> Vec<Self> {
        unimplemented!();
    }
}
