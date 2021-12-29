// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{EventType, ProximityEvent};

define_event! {
    ProximityEvent,
    ffi::GdkProximityEvent,
    &[EventType::ProximityIn, EventType::ProximityOut]
}
