// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use gtk_sys;
use gdk::ModifierType;
use AccelFlags;

#[repr(C)]
#[derive(Clone)]
pub struct AccelKey(gtk_sys::GtkAccelKey);

impl AccelKey {
    pub fn new(accel_key: u32, accel_mods: ModifierType, accel_flags: AccelFlags) -> AccelKey {
        assert_initialized_main_thread!();
        AccelKey(gtk_sys::GtkAccelKey {
            accel_key,
            accel_mods: accel_mods.to_glib(),
            accel_flags: accel_flags.to_glib(),
        })
    }

    pub fn get_accel_key(&self) -> u32 {
        self.0.accel_key
    }

    pub fn get_accel_mods(&self) -> ModifierType {
        from_glib(self.0.accel_mods)
    }

    pub fn get_accel_flags(&self) -> AccelFlags {
        from_glib(self.0.accel_flags)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut gtk_sys::GtkAccelKey> for AccelKey {
    unsafe fn from_glib_none(ptr: *mut gtk_sys::GtkAccelKey) -> Self {
        AccelKey((*ptr).clone())
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut gtk_sys::GtkAccelKey> for AccelKey {
    unsafe fn from_glib_borrow(ptr: *mut gtk_sys::GtkAccelKey) -> Self {
        AccelKey((*ptr).clone())
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut gtk_sys::GtkAccelKey> for AccelKey {
    type Storage = &'a mut Self;

    fn to_glib_none_mut(&'a mut self) -> StashMut<*mut gtk_sys::GtkAccelKey, Self> {
        StashMut(&mut self.0, self)
    }
}

#[doc(hidden)]
impl Uninitialized for AccelKey {
    unsafe fn uninitialized() -> Self {
        AccelKey::new(0, ModifierType::empty(), AccelFlags::empty())
    }
}
