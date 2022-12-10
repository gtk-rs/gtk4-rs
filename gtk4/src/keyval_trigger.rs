// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, ShortcutTrigger};
use gdk::Key;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct KeyvalTrigger(Object<ffi::GtkKeyvalTrigger, ffi::GtkKeyvalTriggerClass>) @extends ShortcutTrigger;

    match fn {
        type_ => || ffi::gtk_keyval_trigger_get_type(),
    }
}

impl KeyvalTrigger {
    #[doc(alias = "gtk_keyval_trigger_new")]
    pub fn new(keyval: Key, modifiers: gdk::ModifierType) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            ShortcutTrigger::from_glib_full(ffi::gtk_keyval_trigger_new(
                keyval.into_glib(),
                modifiers.into_glib(),
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_keyval_trigger_get_keyval")]
    #[doc(alias = "get_keyval")]
    pub fn keyval(&self) -> Key {
        unsafe { from_glib(ffi::gtk_keyval_trigger_get_keyval(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_keyval_trigger_get_modifiers")]
    #[doc(alias = "get_modifiers")]
    pub fn modifiers(&self) -> gdk::ModifierType {
        unsafe { from_glib(ffi::gtk_keyval_trigger_get_modifiers(self.to_glib_none().0)) }
    }
}

impl fmt::Display for KeyvalTrigger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("KeyvalTrigger")
    }
}
