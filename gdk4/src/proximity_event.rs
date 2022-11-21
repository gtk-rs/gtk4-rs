// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{EventType, ProximityEvent};
use std::fmt;

define_event! {
    ProximityEvent,
    ffi::GdkProximityEvent,
    &[EventType::ProximityIn, EventType::ProximityOut]
}

impl fmt::Debug for ProximityEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ProximityEvent")
    }
}
