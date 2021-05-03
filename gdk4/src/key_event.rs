// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{keys::Key, EventType, KeyMatch, ModifierType};
use glib::translate::*;
use std::fmt;
use std::mem;

define_event! {
    KeyEvent,
    ffi::GdkKeyEvent,
    ffi::gdk_key_event_get_type,
    &[EventType::KeyPress, EventType::KeyRelease]
}

impl KeyEvent {
    #[doc(alias = "gdk_key_event_get_consumed_modifiers")]
    #[doc(alias = "get_consumed_modifiers")]
    pub fn consumed_modifiers(&self) -> ModifierType {
        unsafe {
            from_glib(ffi::gdk_key_event_get_consumed_modifiers(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_key_event_get_keycode")]
    #[doc(alias = "get_keycode")]
    pub fn keycode(&self) -> u32 {
        unsafe { ffi::gdk_key_event_get_keycode(self.to_glib_none().0) }
    }
    #[doc(alias = "gdk_key_event_get_keyval")]
    #[doc(alias = "get_keyval")]
    pub fn keyval(&self) -> Key {
        unsafe { ffi::gdk_key_event_get_keyval(self.to_glib_none().0).into() }
    }

    #[doc(alias = "gdk_key_event_get_layout")]
    #[doc(alias = "get_layout")]
    pub fn layout(&self) -> u32 {
        unsafe { ffi::gdk_key_event_get_layout(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_key_event_get_level")]
    #[doc(alias = "get_level")]
    pub fn level(&self) -> u32 {
        unsafe { ffi::gdk_key_event_get_level(self.to_glib_none().0) }
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
                let keyval: Key = keyval.assume_init().into();
                let modifiers = modifiers.assume_init();
                Some((keyval, from_glib(modifiers)))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gdk_key_event_is_modifier")]
    pub fn is_modifier(&self) -> bool {
        unsafe { from_glib(ffi::gdk_key_event_is_modifier(self.to_glib_none().0)) }
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

impl fmt::Display for KeyEvent {
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
