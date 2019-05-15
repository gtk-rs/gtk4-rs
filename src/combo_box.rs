// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::object::IsA;
use glib::translate::*;
use gtk_sys;
use ComboBox;

pub trait ComboBoxExtManual: 'static {
    fn set_active(&self, index_: Option<u32>);
    fn get_active(&self) -> Option<u32>;
}

impl<O: IsA<ComboBox>> ComboBoxExtManual for O {
    fn set_active(&self, index_: Option<u32>) {
        let index_ = match index_ {
            Some(i) => i as _,
            None => -1,
        };
        unsafe {
            gtk_sys::gtk_combo_box_set_active(self.as_ref().to_glib_none().0, index_);
        }
    }

    fn get_active(&self) -> Option<u32> {
        match unsafe {
            gtk_sys::gtk_combo_box_get_active(self.as_ref().to_glib_none().0)
        } {
            -1 => None,
            x => Some(x as _),
        }
    }
}
