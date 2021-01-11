// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{keys::Key, Display, ModifierType};
use glib::translate::*;
use std::mem;

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
                state.to_glib(),
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
}
