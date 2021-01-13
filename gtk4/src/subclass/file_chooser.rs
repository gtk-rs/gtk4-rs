// Take a look at the license at the top of the repository in the LICENSE file.

use crate::FileChooser;
use glib::subclass::prelude::*;

pub trait FileChooserImpl: ObjectImpl {}

unsafe impl<T: FileChooserImpl> IsImplementable<T> for FileChooser {
    unsafe extern "C" fn interface_init(
        _iface: glib::ffi::gpointer,
        _iface_data: glib::ffi::gpointer,
    ) {
    }
}
