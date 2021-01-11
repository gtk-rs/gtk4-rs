// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{keys::Key, KeyEvent};
use glib::translate::*;

impl KeyEvent {
    #[doc(alias = "gdk_key_event_get_keyval")]
    pub fn get_keyval(&self) -> Key {
        unsafe { ffi::gdk_key_event_get_keyval(self.to_glib_none().0).into() }
    }
}
