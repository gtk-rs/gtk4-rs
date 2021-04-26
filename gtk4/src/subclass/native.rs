// Take a look at the license at the top of the repository in the LICENSE file.

use crate::subclass::prelude::*;
use crate::Native;

pub trait NativeImpl: WidgetImpl {}

unsafe impl<T: NativeImpl> IsImplementable<T> for Native {
    fn interface_init(_iface: &mut glib::Interface<Self>) {}

    fn instance_init(_instance: &mut glib::subclass::InitializingObject<T>) {}
}
