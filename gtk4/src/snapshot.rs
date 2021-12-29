// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Snapshot;
use glib::translate::*;

impl Snapshot {
    #[doc(alias = "gtk_snapshot_append_border")]
    pub fn append_border(
        &self,
        outline: &gsk::RoundedRect,
        border_width: &[f32; 4],
        border_color: &[gdk::RGBA; 4],
    ) {
        unsafe {
            let border_color_ptr: Vec<gdk::ffi::GdkRGBA> =
                border_color.iter().map(|c| *c.to_glib_none().0).collect();
            ffi::gtk_snapshot_append_border(
                self.to_glib_none().0,
                outline.to_glib_none().0,
                border_width,
                border_color_ptr.as_ptr() as *const _,
            )
        }
    }

    #[doc(alias = "gtk_snapshot_push_debug")]
    pub fn push_debug(&self, message: &str) {
        unsafe { ffi::gtk_snapshot_push_debug(self.to_glib_none().0, message.to_glib_none().0) }
    }
}
