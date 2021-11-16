// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for implementing the [`Native`](crate::Native) interface.

use crate::subclass::prelude::*;
use crate::Native;

pub trait NativeImpl: WidgetImpl {}

unsafe impl<T: NativeImpl> IsImplementable<T> for Native {
    fn interface_init(_iface: &mut glib::Interface<Self>) {
        assert!(
            crate::rt::is_initialized(),
            "GTK has to be initialized first"
        );
    }
}
