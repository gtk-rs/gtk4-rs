// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gdk_sys;
use glib::translate::*;
use std::ptr;

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct KeymapKey(gdk_sys::GdkKeymapKey);

impl KeymapKey {
    pub fn new(keycode: u32, group: i32, level: i32) -> KeymapKey {
        assert_initialized_main_thread!();
        KeymapKey(gdk_sys::GdkKeymapKey {
            keycode,
            group,
            level
        })
    }

    pub fn get_keycode(&self) -> u32 {
        self.0.keycode
    }

    pub fn get_group(&self) -> i32 {
        self.0.group
    }

    pub fn get_level(&self) -> i32 {
        self.0.level
    }
}

#[doc(hidden)]
impl FromGlibContainerAsVec<gdk_sys::GdkKeymapKey, *mut gdk_sys::GdkKeymapKey> for KeymapKey {
    unsafe fn from_glib_none_num_as_vec(ptr: *mut gdk_sys::GdkKeymapKey, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }

        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            res.push(KeymapKey(ptr::read(ptr.offset(i as isize))));
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(ptr: *mut gdk_sys::GdkKeymapKey, num: usize) -> Vec<Self> {
        let res = FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
        glib_sys::g_free(ptr as *mut _);
        res
    }

    unsafe fn from_glib_full_num_as_vec(ptr: *mut gdk_sys::GdkKeymapKey, num: usize) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, num)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const gdk_sys::GdkKeymapKey> for KeymapKey {
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> Stash<*const gdk_sys::GdkKeymapKey, Self> {
        Stash(&self.0, self)
    }
}
