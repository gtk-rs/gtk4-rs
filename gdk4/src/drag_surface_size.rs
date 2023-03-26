// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GdkDragSurfaceSize")]
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DragSurfaceSize(Boxed<ffi::GdkDragSurfaceSize>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::gdk_drag_surface_size_get_type(), ptr as *mut _) as *mut ffi::GdkDragSurfaceSize,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::gdk_drag_surface_size_get_type(), ptr as *mut _),
        type_ => || ffi::gdk_drag_surface_size_get_type(),
    }
}

impl DragSurfaceSize {
    #[doc(alias = "gdk_drag_surface_size_set_size")]
    pub fn set_size(&self, width: i32, height: i32) {
        unsafe { ffi::gdk_drag_surface_size_set_size(self.to_glib_none().0, width, height) }
    }
}
