// Take a look at the license at the top of the repository in the LICENSE file.

use crate::EventType;
use glib::translate::*;
use std::fmt;

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
