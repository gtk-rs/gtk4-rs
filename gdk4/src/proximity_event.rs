// Take a look at the license at the top of the repository in the LICENSE file.

use crate::EventType;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct ProximityEvent(Shared<ffi::GdkProximityEvent>);

    match fn {
        ref => |ptr| ffi::gdk_event_ref(ptr as *mut ffi::GdkEvent),
        unref => |ptr| ffi::gdk_event_unref(ptr as *mut ffi::GdkEvent),
    }
}

define_event! {
    ProximityEvent,
    ffi::GdkProximityEvent,
    ffi::gdk_proximity_event_get_type,
    &[EventType::ProximityIn, EventType::ProximityOut]
}

impl fmt::Display for ProximityEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ProximityEvent")
    }
}
