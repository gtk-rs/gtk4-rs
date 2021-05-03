// Take a look at the license at the top of the repository in the LICENSE file.

pub use ffi;
pub use gdk;
pub use gio;
pub use glib;
pub use wayland_client;

mod auto;

pub use auto::*;

mod wayland_device;
mod wayland_display;
mod wayland_monitor;
mod wayland_seat;
mod wayland_surface;
