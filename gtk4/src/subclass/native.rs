// Take a look at the license at the top of the repository in the LICENSE file.

use crate::subclass::prelude::WidgetImpl;
use crate::Native;
use glib::subclass::prelude::*;

pub trait NativeImpl: WidgetImpl {}

unsafe impl<T: NativeImpl> IsImplementable<T> for Native {
    unsafe extern "C" fn interface_init(
        _iface: glib::ffi::gpointer,
        _iface_data: glib::ffi::gpointer,
    ) {
    }
}
