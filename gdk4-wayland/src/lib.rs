pub use ffi;
pub use wayland_client;

#[allow(unused_imports)]
#[allow(clippy::let_and_return)]
mod auto;

pub use auto::*;

mod wayland_device;
pub use wayland_device::WaylandDeviceManualExt;
mod wayland_display;
pub use wayland_display::WaylandDisplayManualExt;
mod wayland_monitor;
pub use wayland_monitor::WaylandMonitorManualExt;
mod wayland_seat;
pub use wayland_seat::WaylandSeatManualExt;
mod wayland_surface;
pub use wayland_surface::WaylandSurfaceManualExt;
