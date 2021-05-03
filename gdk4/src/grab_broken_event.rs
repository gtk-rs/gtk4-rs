// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{EventType, Surface};
use glib::translate::*;
use std::fmt;

define_event! {
    GrabBrokenEvent,
    ffi::GdkGrabBrokenEvent,
    ffi::gdk_grab_broken_event_get_type,
    &[EventType::GrabBroken]
}

impl GrabBrokenEvent {
    #[doc(alias = "gdk_grab_broken_event_get_grab_surface")]
    #[doc(alias = "get_grab_surface")]
    pub fn grab_surface(&self) -> Option<Surface> {
        unsafe {
            from_glib_none(ffi::gdk_grab_broken_event_get_grab_surface(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_grab_broken_event_get_implicit")]
    #[doc(alias = "get_implicit")]
    pub fn is_implicit(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_grab_broken_event_get_implicit(
                self.to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for GrabBrokenEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("GrabBrokenEvent")
            .field("grab_surface", &self.grab_surface())
            .field("implicit", &self.is_implicit())
            .finish()
    }
}
