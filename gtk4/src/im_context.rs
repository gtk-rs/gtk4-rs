// Take a look at the license at the top of the repository in the LICENSE file.

use crate::IMContext;
use glib::translate::*;
use glib::IsA;

pub trait IMContextExtManual {
    #[doc(alias = "gtk_im_context_filter_keypress")]
    fn filter_keypress(&self, event: &gdk::Event) -> bool;
}

impl<O: IsA<IMContext>> IMContextExtManual for O {
    fn filter_keypress(&self, event: &gdk::Event) -> bool {
        unsafe {
            from_glib(ffi::gtk_im_context_filter_keypress(
                self.as_ref().to_glib_none().0,
                event.to_glib_none().0,
            ))
        }
    }
}
