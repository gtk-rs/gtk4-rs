// Take a look at the license at the top of the repository in the LICENSE file.

use crate::subclass::prelude::NativeImpl;
use crate::Root;
use glib::subclass::prelude::*;

pub trait RootImpl: NativeImpl {}

unsafe impl<T: RootImpl> IsImplementable<T> for Root {
    unsafe extern "C" fn interface_init(
        _iface: glib::ffi::gpointer,
        _iface_data: glib::ffi::gpointer,
    ) {
    }
}
