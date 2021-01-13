// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ConstraintTarget;
use glib::subclass::prelude::*;

pub trait ConstraintTargetImpl: ObjectImpl {}

unsafe impl<T: ConstraintTargetImpl> IsImplementable<T> for ConstraintTarget {
    unsafe extern "C" fn interface_init(
        _iface: glib::ffi::gpointer,
        _iface_data: glib::ffi::gpointer,
    ) {
    }
}
