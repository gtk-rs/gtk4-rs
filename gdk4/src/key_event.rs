// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{keys::Key, KeyEvent, KeyMatch, ModifierType};
use glib::translate::*;
use std::mem;

impl KeyEvent {
    #[doc(alias = "gdk_key_event_get_keyval")]
    pub fn get_keyval(&self) -> Key {
        unsafe { ffi::gdk_key_event_get_keyval(self.to_glib_none().0).into() }
    }

    #[doc(alias = "gdk_key_event_get_match")]
    pub fn get_match(&self) -> Option<(Key, ModifierType)> {
        unsafe {
            let mut keyval = mem::MaybeUninit::uninit();
            let mut modifiers = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_key_event_get_match(
                self.to_glib_none().0,
                keyval.as_mut_ptr(),
                modifiers.as_mut_ptr(),
            ));
            if ret {
                let keyval: Key = keyval.assume_init().into();
                let modifiers = modifiers.assume_init();
                Some((keyval, from_glib(modifiers)))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gdk_key_event_matches")]
    pub fn matches(&self, keyval: Key, modifiers: ModifierType) -> KeyMatch {
        unsafe {
            from_glib(ffi::gdk_key_event_matches(
                self.to_glib_none().0,
                keyval.to_glib(),
                modifiers.to_glib(),
            ))
        }
    }
}
