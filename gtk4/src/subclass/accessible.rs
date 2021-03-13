// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Accessible;
use glib::subclass::prelude::*;

pub trait AccessibleImpl: ObjectImpl {}

unsafe impl<T: AccessibleImpl> IsImplementable<T> for Accessible {
    fn interface_init(_iface: &mut glib::Interface<Self>) {}

    fn instance_init(_instance: &mut glib::subclass::InitializingObject<T>) {}
}
