// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ContentDeserializer;
use glib::translate::*;

impl ContentDeserializer {
    pub fn set_value(&self, value: glib::Value) {
        assert!(value.type_() == self.type_(), "Type mismatch");

        let src_value = value.to_glib_none();
        unsafe {
            let dest_value = ffi::gdk_content_deserializer_get_value(self.to_glib_none().0);
            glib::gobject_ffi::g_value_copy(src_value.0, dest_value);
        }
    }
}
