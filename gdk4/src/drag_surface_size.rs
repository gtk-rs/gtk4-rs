// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ffi, prelude::*};
use glib::translate::*;

#[repr(transparent)]
#[doc(alias = "GdkDragSurfaceSize")]
pub struct DragSurfaceSize(std::ptr::NonNull<ffi::GdkDragSurfaceSize>);

impl StaticType for DragSurfaceSize {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gdk_drag_surface_size_get_type()) }
    }
}

impl DragSurfaceSize {
    #[doc(alias = "gdk_drag_surface_size_set_size")]
    pub fn set_size(&self, width: i32, height: i32) {
        unsafe { ffi::gdk_drag_surface_size_set_size(self.0.as_ptr(), width, height) }
    }
}
