// Take a look at the license at the top of the repository in the LICENSE file.

use crate::subclass::prelude::*;
use crate::Orientable;

pub trait OrientableImpl: ObjectImpl {}

unsafe impl<T: OrientableImpl> IsImplementable<T> for Orientable {
    fn interface_init(_iface: &mut glib::Interface<Self>) {
        if !crate::rt::is_initialized() {
            panic!("GTK has to be initialized first");
        }
    }

    fn instance_init(_instance: &mut glib::subclass::InitializingObject<T>) {}
}
