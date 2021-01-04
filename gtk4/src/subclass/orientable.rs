// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Orientable;
use glib::subclass::prelude::*;

pub trait OrientableImpl: ObjectImpl {}

unsafe impl<T: OrientableImpl> IsImplementable<T> for Orientable {
    unsafe extern "C" fn interface_init(
        _iface: glib::ffi::gpointer,
        _iface_data: glib::ffi::gpointer,
    ) {
    }
}
