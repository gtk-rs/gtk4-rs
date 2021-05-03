// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{CrossingMode, EventType, NotifyType};
use glib::translate::*;
use std::fmt;

define_event! {
    CrossingEvent,
    ffi::GdkCrossingEvent,
    ffi::gdk_crossing_event_get_type,
    &[EventType::EnterNotify, EventType::LeaveNotify]
}

impl CrossingEvent {
    #[doc(alias = "gdk_crossing_event_get_detail")]
    #[doc(alias = "get_detail")]
    pub fn detail(&self) -> NotifyType {
        unsafe { from_glib(ffi::gdk_crossing_event_get_detail(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_crossing_event_get_focus")]
    #[doc(alias = "get_focus")]
    pub fn gets_focus(&self) -> bool {
        unsafe { from_glib(ffi::gdk_crossing_event_get_focus(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_crossing_event_get_mode")]
    #[doc(alias = "get_mode")]
    pub fn mode(&self) -> CrossingMode {
        unsafe { from_glib(ffi::gdk_crossing_event_get_mode(self.to_glib_none().0)) }
    }
}

impl fmt::Display for CrossingEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("PadEvent")
            .field("detail", &self.detail())
            .field("focus", &self.gets_focus())
            .field("mode", &self.mode())
            .finish()
    }
}
