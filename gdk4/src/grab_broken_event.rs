// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

use crate::{EventType, GrabBrokenEvent};

define_event! {
    GrabBrokenEvent,
    crate::ffi::GdkGrabBrokenEvent,
    &[EventType::GrabBroken]
}

impl fmt::Debug for GrabBrokenEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("GrabBrokenEvent")
            .field("grab_surface", &self.grab_surface())
            .field("implicit", &self.is_implicit())
            .finish()
    }
}
