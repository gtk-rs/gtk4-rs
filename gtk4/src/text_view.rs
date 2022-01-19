// Take a look at the license at the top of the repository in the LICENSE file.

use crate::TextView;
use glib::translate::*;
use glib::IsA;

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of [`TextView`](crate::TextView).
pub trait TextViewExtManual {
    #[doc(alias = "gtk_text_view_im_context_filter_keypress")]
    fn im_context_filter_keypress(&self, event: &impl AsRef<gdk::Event>) -> bool;
}

impl<O: IsA<TextView>> TextViewExtManual for O {
    fn im_context_filter_keypress(&self, event: &impl AsRef<gdk::Event>) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_view_im_context_filter_keypress(
                self.as_ref().to_glib_none().0,
                event.as_ref().to_glib_none().0,
            ))
        }
    }
}
