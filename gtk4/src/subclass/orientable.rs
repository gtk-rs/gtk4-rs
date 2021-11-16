// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for implementing the [`Orientable`](crate::Orientable) interface.

use crate::subclass::prelude::*;
use crate::Orientable;

pub trait OrientableImpl: ObjectImpl {}

unsafe impl<T: OrientableImpl> IsImplementable<T> for Orientable {
    fn interface_init(_iface: &mut glib::Interface<Self>) {
        assert!(
            crate::rt::is_initialized(),
            "GTK has to be initialized first"
        );
    }
}
