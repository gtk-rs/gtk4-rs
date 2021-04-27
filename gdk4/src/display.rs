// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{keys::Key, Display, Event, KeymapKey, ModifierType};
use glib::translate::*;
use std::{mem, ptr};

impl Display {
    #[doc(alias = "gdk_display_translate_key")]
    pub fn translate_key(
        &self,
        keycode: u32,
        state: ModifierType,
        group: i32,
    ) -> Option<(Key, i32, i32, ModifierType)> {
        unsafe {
            let mut keyval = mem::MaybeUninit::uninit();
            let mut effective_group = mem::MaybeUninit::uninit();
            let mut level = mem::MaybeUninit::uninit();
            let mut consumed = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_display_translate_key(
                self.to_glib_none().0,
                keycode,
                state.into_glib(),
                group,
                keyval.as_mut_ptr(),
                effective_group.as_mut_ptr(),
                level.as_mut_ptr(),
                consumed.as_mut_ptr(),
            ));
            if ret {
                let keyval: Key = keyval.assume_init().into();
                let effective_group = effective_group.assume_init();
                let level = level.assume_init();
                let consumed = consumed.assume_init();
                Some((keyval, effective_group, level, from_glib(consumed)))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gdk_display_map_keyval")]
    pub fn map_keyval(&self, keyval: Key) -> Option<Vec<KeymapKey>> {
        unsafe {
            let mut keys = ptr::null_mut();
            let mut n_keys = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_display_map_keyval(
                self.to_glib_none().0,
                keyval.into_glib(),
                &mut keys,
                n_keys.as_mut_ptr(),
            ));
            if ret {
                Some(FromGlibContainer::from_glib_full_num(
                    keys,
                    n_keys.assume_init() as usize,
                ))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gdk_display_map_keycode")]
    pub fn map_keycode(&self, keycode: u32) -> Option<Vec<(KeymapKey, Key)>> {
        unsafe {
            let mut keys = ptr::null_mut();
            let mut keyvals = ptr::null_mut();
            let mut n_entries = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_display_map_keycode(
                self.to_glib_none().0,
                keycode,
                &mut keys,
                &mut keyvals,
                n_entries.as_mut_ptr(),
            ));
            if ret {
                let n_keys = n_entries.assume_init() as usize;
                let keyvals: Vec<u32> = FromGlibContainer::from_glib_full_num(keyvals, n_keys);
                let keyvals: Vec<Key> = keyvals.into_iter().map(|k| k.into()).collect();
                let keys: Vec<KeymapKey> = FromGlibContainer::from_glib_full_num(keys, n_keys);

                Some(keys.into_iter().zip(keyvals.into_iter()).collect())
            } else {
                None
            }
        }
    }

    #[doc(alias = "gdk_display_put_event")]
    pub fn put_event<P: AsRef<Event>>(&self, event: &P) {
        unsafe {
            ffi::gdk_display_put_event(self.to_glib_none().0, event.as_ref().to_glib_none().0);
        }
    }
}
