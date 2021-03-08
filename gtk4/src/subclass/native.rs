// Take a look at the license at the top of the repository in the LICENSE file.

use crate::subclass::prelude::WidgetImpl;
use crate::Native;
use glib::subclass::prelude::*;

pub trait NativeImpl: WidgetImpl {}

unsafe impl<T: NativeImpl> IsImplementable<T> for Native {
    fn interface_init(_iface: &mut glib::Class<Self>) {}
}
