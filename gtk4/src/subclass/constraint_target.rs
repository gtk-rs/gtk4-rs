// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ConstraintTarget;
use glib::subclass::prelude::*;

pub trait ConstraintTargetImpl: ObjectImpl {}

unsafe impl<T: ConstraintTargetImpl> IsImplementable<T> for ConstraintTarget {
    fn interface_init(_iface: &mut glib::Class<Self>) {}

    fn instance_init(_instance: &mut glib::subclass::InitializingObject<T>) {}
}
