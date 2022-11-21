// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ShortcutTrigger;
use gdk::Key;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct MnemonicTrigger(Object<ffi::GtkMnemonicTrigger, ffi::GtkMnemonicTriggerClass>) @extends ShortcutTrigger;

    match fn {
        type_ => || ffi::gtk_mnemonic_trigger_get_type(),
    }
}

impl MnemonicTrigger {
    #[doc(alias = "gtk_mnemonic_trigger_new")]
    pub fn new(keyval: Key) -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_mnemonic_trigger_new(keyval.into_glib())) }
    }

    #[doc(alias = "gtk_mnemonic_trigger_get_keyval")]
    #[doc(alias = "get_keyval")]
    pub fn keyval(&self) -> Key {
        unsafe { from_glib(ffi::gtk_mnemonic_trigger_get_keyval(self.to_glib_none().0)) }
    }
}

impl fmt::Display for MnemonicTrigger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("MnemonicTrigger")
    }
}
