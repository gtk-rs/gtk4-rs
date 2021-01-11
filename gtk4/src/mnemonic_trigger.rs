// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ShortcutTrigger;
use gdk::keys::Key;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct MnemonicTrigger(Object<ffi::GtkMnemonicTrigger, ffi::GtkMnemonicTriggerClass>) @extends ShortcutTrigger;

    match fn {
        get_type => || ffi::gtk_mnemonic_trigger_get_type(),
    }
}

impl MnemonicTrigger {
    #[doc(alias = "gtk_mnemonic_trigger_new")]
    pub fn new(keyval: Key) -> MnemonicTrigger {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_mnemonic_trigger_new(keyval.to_glib())) }
    }

    #[doc(alias = "gtk_mnemonic_trigger_get_keyval")]
    pub fn get_keyval(&self) -> Key {
        unsafe { ffi::gtk_mnemonic_trigger_get_keyval(self.to_glib_none().0).into() }
    }
}

impl fmt::Display for MnemonicTrigger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("MnemonicTrigger")
    }
}
