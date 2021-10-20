// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use std::fmt;

glib::wrapper! {

#[doc(alias = "GdkToplevelSize")]
    pub struct ToplevelSize(BoxedInline<ffi::GdkToplevelSize>);

    match fn {
        type_ => || ffi::gdk_toplevel_size_get_type(),
    }

}

impl ToplevelSize {
    #[doc(alias = "gdk_toplevel_size_get_bounds")]
    #[doc(alias = "get_bounds")]
    pub fn bounds(&self) -> (i32, i32) {
        unsafe {
            let mut bounds_width = std::mem::MaybeUninit::uninit();
            let mut bounds_height = std::mem::MaybeUninit::uninit();

            ffi::gdk_toplevel_size_get_bounds(
                self.to_glib_none().0 as *mut _,
                bounds_width.as_mut_ptr(),
                bounds_height.as_mut_ptr(),
            );
            (bounds_width.assume_init(), bounds_height.assume_init())
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

impl fmt::Debug for ToplevelSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ToplevelSize")
            .field("bounds", &self.bounds())
            .finish()
    }
}
