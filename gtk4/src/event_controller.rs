// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(feature = "v4_8")]
use glib::translate::*;

#[cfg(feature = "v4_8")]
use crate::ffi;
use crate::{prelude::*, EventController};

pub trait EventControllerExtManual: IsA<EventController> + 'static {
    #[cfg(feature = "v4_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_8")))]
    #[doc(alias = "gtk_event_controller_set_static_name")]
    fn set_static_name(&self, name: Option<&'static glib::GStr>) {
        unsafe {
            ffi::gtk_event_controller_set_static_name(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            );
        }
    }
}

impl<O: IsA<EventController>> EventControllerExtManual for O {}
