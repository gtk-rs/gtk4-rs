// Take a look at the license at the top of the repository in the LICENSE file.

use crate::subclass::prelude::*;
use crate::Root;

pub trait RootImpl: NativeImpl {}

unsafe impl<T: RootImpl> IsImplementable<T> for Root {
    fn interface_init(_iface: &mut glib::Interface<Self>) {}

    fn instance_init(_instance: &mut glib::subclass::InitializingObject<T>) {}
}
