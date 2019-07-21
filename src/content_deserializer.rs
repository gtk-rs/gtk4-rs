// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ContentDeserializer;
use gdk_sys;
use glib;
use glib::translate::*;

impl ContentDeserializer {
    pub fn set_value(&self, value: glib::Value) {
        assert!(value.type_() == self.get_gtype(), "Type mismatch");

        let src_value = value.to_glib_none();
        unsafe {
            let dest_value = gdk_sys::gdk_content_deserializer_get_value(self.to_glib_none().0);
            gobject_sys::g_value_copy(src_value.0, dest_value);
        }
    }
}
