// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gdk_sys;
use glib::translate::*;
use std::mem;
use std::ptr;
use Keymap;
use KeymapKey;

impl Keymap {
    pub fn get_entries_for_keycode(
        &self,
        hardware_keycode: u32,
    ) -> Option<(Vec<KeymapKey>, Vec<u32>)> {
        unsafe {
            let mut keys: *mut gdk_sys::GdkKeymapKey = ptr::null_mut();
            let mut keyvals = ptr::null_mut();
            let mut n_entries = mem::uninitialized();
            let ret = from_glib(gdk_sys::gdk_keymap_get_entries_for_keycode(
                self.to_glib_none().0,
                hardware_keycode,
                &mut keys,
                &mut keyvals,
                &mut n_entries,
            ));
            if ret {
                Some((
                    FromGlibContainer::from_glib_full_num(keys, n_entries as usize),
                    FromGlibContainer::from_glib_full_num(keyvals, n_entries as usize),
                ))
            } else {
                None
            }
        }
    }
}
