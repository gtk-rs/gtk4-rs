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
    pub fn new(keycode: u32, group: i32, level: i32) -> Self {
        assert_initialized_main_thread!();
        Self {
            group,
            keycode,
            level,
        }
    }

    #[doc(alias = "get_keycode")]
    pub fn keycode(&self) -> u32 {
        self.keycode
    }

    #[doc(alias = "get_group")]
    pub fn group(&self) -> i32 {
        self.group
    }

    #[doc(alias = "get_level")]
    pub fn level(&self) -> i32 {
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

#[doc(hidden)]
impl FromGlibContainerAsVec<ffi::GdkKeymapKey, *mut ffi::GdkKeymapKey> for KeymapKey {
    unsafe fn from_glib_none_num_as_vec(ptr: *mut ffi::GdkKeymapKey, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }

        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            res.push(from_glib_none(ptr.add(i)));
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(ptr: *mut ffi::GdkKeymapKey, num: usize) -> Vec<Self> {
        let res = FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
        glib::ffi::g_free(ptr as *mut _);
        res
    }

    unsafe fn from_glib_full_num_as_vec(ptr: *mut ffi::GdkKeymapKey, num: usize) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, num)
    }
}
