// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Accessible;
use glib::subclass::prelude::*;

pub trait AccessibleImpl: ObjectImpl {}

unsafe impl<T: AccessibleImpl> IsImplementable<T> for Accessible {
    unsafe extern "C" fn interface_init(
        _iface: glib::ffi::gpointer,
        _iface_data: glib::ffi::gpointer,
    ) {
    }
}
