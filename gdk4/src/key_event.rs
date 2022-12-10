// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{EventType, Key, KeyEvent, KeyMatch, ModifierType};
use glib::translate::*;
use std::{fmt, mem};

define_event! {
    KeyEvent,
    ffi::GdkKeyEvent,
    &[EventType::KeyPress, EventType::KeyRelease]
}

impl KeyEvent {
    #[doc(alias = "gdk_key_event_get_keyval")]
    #[doc(alias = "get_keyval")]
    pub fn keyval(&self) -> Key {
        unsafe { from_glib(ffi::gdk_key_event_get_keyval(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_key_event_get_match")]
    #[doc(alias = "get_match")]
    pub fn match_(&self) -> Option<(Key, ModifierType)> {
        unsafe {
            let mut keyval = mem::MaybeUninit::uninit();
            let mut modifiers = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_key_event_get_match(
                self.to_glib_none().0,
                keyval.as_mut_ptr(),
                modifiers.as_mut_ptr(),
            ));
            if ret {
                let keyval = keyval.assume_init();
                let modifiers = modifiers.assume_init();
                Some((from_glib(keyval), from_glib(modifiers)))
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
                keyval.into_glib(),
                modifiers.into_glib(),
            ))
        }
    }
}

impl fmt::Debug for KeyEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("KeyEvent")
            .field("consumed_modifiers", &self.consumed_modifiers())
            .field("keycode", &self.keycode())
            .field("keyval", &self.keyval())
            .field("layout", &self.layout())
            .field("level", &self.level())
            .field("match", &self.match_())
            .field("is_modifier", &self.is_modifier())
            .finish()
    }
}
