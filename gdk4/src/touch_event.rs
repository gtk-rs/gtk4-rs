// Take a look at the license at the top of the repository in the LICENSE file.

use crate::EventType;
use glib::translate::*;
use std::fmt;

define_event! {
    TouchEvent,
    ffi::GdkTouchEvent,
    ffi::gdk_touch_event_get_type,
    &[EventType::TouchBegin, EventType::TouchUpdate, EventType::TouchEnd, EventType::TouchCancel]
}

impl TouchEvent {
    #[doc(alias = "gdk_touch_event_get_emulating_pointer")]
    #[doc(alias = "get_emulating_pointer")]
    pub fn emulates_pointer(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_touch_event_get_emulating_pointer(
                self.to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for TouchEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TouchEvent")
            .field("emulating_pointer", &self.emulates_pointer())
            .finish()
    }
}
