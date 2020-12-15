// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct KeymapKey {
    group: i32,
    keycode: u32,
    level: i32,
}

impl KeymapKey {
    pub fn new(keycode: u32, group: i32, level: i32) -> KeymapKey {
        assert_initialized_main_thread!();
        KeymapKey {
            keycode,
            group,
            level,
        }
    }

    pub fn get_keycode(&self) -> u32 {
        self.keycode
    }

    pub fn get_group(&self) -> i32 {
        self.group
    }

    pub fn get_level(&self) -> i32 {
        self.level
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const crate::ffi::GdkKeymapKey> for KeymapKey {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const crate::ffi::GdkKeymapKey, Self> {
        let ptr: *const KeymapKey = &*self;
        Stash(ptr as *const crate::ffi::GdkKeymapKey, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut crate::ffi::GdkKeymapKey> for KeymapKey {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut crate::ffi::GdkKeymapKey, Self> {
        let ptr: *mut KeymapKey = &mut *self;
        StashMut(ptr as *mut crate::ffi::GdkKeymapKey, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const crate::ffi::GdkKeymapKey> for KeymapKey {
    unsafe fn from_glib_none(ptr: *const crate::ffi::GdkKeymapKey) -> Self {
        *(ptr as *const KeymapKey)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut crate::ffi::GdkKeymapKey> for KeymapKey {
    unsafe fn from_glib_none(ptr: *mut crate::ffi::GdkKeymapKey) -> Self {
        *(ptr as *mut KeymapKey)
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut crate::ffi::GdkKeymapKey> for KeymapKey {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut crate::ffi::GdkKeymapKey) -> Self {
        let geom = *(ptr as *mut KeymapKey);
        glib::ffi::g_free(ptr as *mut _);
        geom
    }
}
