// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

#[derive(Debug)]
#[repr(transparent)]
pub struct ToplevelSize(ffi::GdkToplevelSize);

impl ToplevelSize {
    #[doc(alias = "gdk_toplevel_size_get_bounds")]
    #[doc(alias = "get_bounds")]
    pub fn bounds(&self) -> (i32, i32) {
        unsafe {
            let bounds_width = std::ptr::null_mut();
            let bounds_height = std::ptr::null_mut();

            ffi::gdk_toplevel_size_get_bounds(
                self.to_glib_none().0 as *mut _,
                bounds_width,
                bounds_height,
            );
            (bounds_width as i32, bounds_height as i32)
        }
    }

    #[doc(alias = "gdk_toplevel_size_set_min_size")]
    pub fn set_min_size(&mut self, min_width: i32, min_height: i32) {
        unsafe {
            ffi::gdk_toplevel_size_set_min_size(self.to_glib_none_mut().0, min_width, min_height);
        }
    }

    #[doc(alias = "gdk_toplevel_size_set_shadow_width")]
    pub fn set_shadow_width(&mut self, left: i32, right: i32, top: i32, bottom: i32) {
        unsafe {
            ffi::gdk_toplevel_size_set_shadow_width(
                self.to_glib_none_mut().0,
                left,
                right,
                top,
                bottom,
            );
        }
    }

    #[doc(alias = "gdk_toplevel_size_set_size")]
    pub fn set_size(&mut self, width: i32, height: i32) {
        unsafe {
            ffi::gdk_toplevel_size_set_size(self.to_glib_none_mut().0, width, height);
        }
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::GdkToplevelSize> for ToplevelSize {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::GdkToplevelSize, Self> {
        let ptr: *const ToplevelSize = &*self;
        Stash(ptr as *const ffi::GdkToplevelSize, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::GdkToplevelSize> for ToplevelSize {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::GdkToplevelSize, Self> {
        let ptr: *mut ToplevelSize = &mut *self;
        StashMut(ptr as *mut ffi::GdkToplevelSize, self)
    }
}
