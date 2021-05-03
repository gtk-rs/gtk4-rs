// Take a look at the license at the top of the repository in the LICENSE file.

use crate::EventType;
use glib::translate::*;
use std::fmt;

define_event! {
    ButtonEvent,
    ffi::GdkButtonEvent,
    ffi::gdk_button_event_get_type,
    &[EventType::ButtonPress, EventType::ButtonRelease]
}

impl ButtonEvent {
    #[doc(alias = "gdk_button_event_get_button")]
    #[doc(alias = "get_button")]
    pub fn button(&self) -> u32 {
        unsafe { ffi::gdk_button_event_get_button(self.to_glib_none().0) }
    }
}

impl fmt::Display for ButtonEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ButtonEvent")
            .field("button", &self.button())
            .finish()
    }
}
