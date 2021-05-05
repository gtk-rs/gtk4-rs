// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use std::ptr;

#[repr(C)]
pub struct PageRange(ffi::GtkPageRange);

impl PageRange {
    pub fn new(start: i32, end: i32) -> Self {
        skip_assert_initialized!();
        Self(ffi::GtkPageRange { start, end })
    }

    pub fn start(&self) -> i32 {
        self.0.start
    }

    pub fn end(&self) -> i32 {
        self.0.end
    }
}

#[doc(hidden)]
impl IntoGlib for PageRange {
    type GlibType = ffi::GtkPageRange;

    fn into_glib(self) -> ffi::GtkPageRange {
        self.0
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::GtkPageRange> for PageRange {
    type Storage = Box<ffi::GtkPageRange>;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::GtkPageRange, Self> {
        let page_range = Box::new(self.0);
        Stash(&*page_range, page_range)
    }
}

#[doc(hidden)]
impl<'a> ToGlibContainerFromSlice<'a, *const ffi::GtkPageRange> for PageRange {
    type Storage = &'a [Self];

    fn to_glib_none_from_slice(t: &'a [Self]) -> (*const ffi::GtkPageRange, &'a [Self]) {
        assert_initialized_main_thread!();
        (t.as_ptr() as *const _, t)
    }

    fn to_glib_container_from_slice(t: &'a [Self]) -> (*const ffi::GtkPageRange, &'a [Self]) {
        assert_initialized_main_thread!();
        (ToGlibContainerFromSlice::to_glib_full_from_slice(t), t)
    }

    fn to_glib_full_from_slice(t: &[Self]) -> *const ffi::GtkPageRange {
        assert_initialized_main_thread!();

        if t.is_empty() {
            return ptr::null_mut();
        }

        unsafe {
            let res = glib::ffi::g_malloc(std::mem::size_of::<ffi::GtkPageRange>() * t.len())
                as *mut ffi::GtkPageRange;
            ptr::copy_nonoverlapping(t.as_ptr() as *const _, res, t.len());
            res
        }
    }
}

#[doc(hidden)]
impl FromGlibContainerAsVec<ffi::GtkPageRange, *mut ffi::GtkPageRange> for PageRange {
    unsafe fn from_glib_none_num_as_vec(ptr: *mut ffi::GtkPageRange, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }

        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            res.push(PageRange(ptr::read(ptr.add(i))));
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(_: *mut ffi::GtkPageRange, _: usize) -> Vec<Self> {
        unimplemented!();
    }

    unsafe fn from_glib_full_num_as_vec(_: *mut ffi::GtkPageRange, _: usize) -> Vec<Self> {
        unimplemented!();
    }
}
