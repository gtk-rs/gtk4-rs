// Take a look at the license at the top of the repository in the LICENSE file.

use gdk::RGBA;
use glib::translate::*;
use std::mem;
use std::ptr;

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct ColorStop(ffi::GskColorStop);

impl ColorStop {
    pub fn new(offset: f32, color: RGBA) -> Self {
        assert_initialized_main_thread!();
        Self(ffi::GskColorStop {
            offset,
            color: unsafe { *color.to_glib_none().0 },
        })
    }

    pub fn offset(&self) -> f32 {
        self.0.offset
    }

    pub fn color(&self) -> RGBA {
        unsafe { from_glib_none(&self.0.color as *const _) }
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::GskColorStop> for ColorStop {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::GskColorStop, Self> {
        let ptr: *const ColorStop = &*self;
        Stash(ptr as *const ffi::GskColorStop, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::GskColorStop> for ColorStop {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::GskColorStop, Self> {
        let ptr: *mut ColorStop = &mut *self;
        StashMut(ptr as *mut ffi::GskColorStop, self)
    }
}

#[doc(hidden)]
impl FromGlibContainerAsVec<ffi::GskColorStop, *const ffi::GskColorStop> for ColorStop {
    unsafe fn from_glib_none_num_as_vec(ptr: *const ffi::GskColorStop, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }

        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            res.push(ColorStop(ptr::read(ptr.add(i))));
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(_: *const ffi::GskColorStop, _: usize) -> Vec<Self> {
        // Can't really free a *const
        unimplemented!();
    }

    unsafe fn from_glib_full_num_as_vec(_: *const ffi::GskColorStop, _: usize) -> Vec<Self> {
        // Can't really free a *const
        unimplemented!();
    }
}

#[doc(hidden)]
impl<'a> ToGlibContainerFromSlice<'a, *const ffi::GskColorStop> for ColorStop {
    type Storage = &'a [Self];

    fn to_glib_none_from_slice(t: &'a [Self]) -> (*const ffi::GskColorStop, &'a [Self]) {
        assert_initialized_main_thread!();
        (t.as_ptr() as *const _, t)
    }

    fn to_glib_container_from_slice(t: &'a [Self]) -> (*const ffi::GskColorStop, &'a [Self]) {
        assert_initialized_main_thread!();
        (ToGlibContainerFromSlice::to_glib_full_from_slice(t), t)
    }

    fn to_glib_full_from_slice(t: &[Self]) -> *const ffi::GskColorStop {
        assert_initialized_main_thread!();

        if t.is_empty() {
            return ptr::null_mut();
        }

        unsafe {
            let res = glib::ffi::g_malloc(mem::size_of::<ffi::GskColorStop>() * t.len())
                as *mut ffi::GskColorStop;
            ptr::copy_nonoverlapping(t.as_ptr() as *const _, res, t.len());
            res
        }
    }
}
