// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Label;
use glib::translate::*;

impl Label {
    #[doc(alias = "gtk_label_get_mnemonic_keyval")]
    #[doc(alias = "get_mnemonic_keyval")]
    pub fn mnemonic_keyval(&self) -> gdk::Key {
        unsafe { from_glib(ffi::gtk_label_get_mnemonic_keyval(self.to_glib_none().0)) }
    }
}
