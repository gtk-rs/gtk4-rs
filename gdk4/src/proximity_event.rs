// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

use crate::{EventType, ProximityEvent};

define_event! {
    ProximityEvent,
    crate::ffi::GdkProximityEvent,
    &[EventType::ProximityIn, EventType::ProximityOut]
}

impl fmt::Debug for ProximityEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ProximityEvent")
    }
}
