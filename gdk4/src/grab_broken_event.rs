// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{EventType, GrabBrokenEvent};
use std::fmt;

define_event! {
    GrabBrokenEvent,
    ffi::GdkGrabBrokenEvent,
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
