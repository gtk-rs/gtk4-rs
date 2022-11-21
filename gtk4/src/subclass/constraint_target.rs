// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for implementing the [`ConstraintTarget`](crate::ConstraintTarget) interface.

use crate::subclass::prelude::*;
use crate::ConstraintTarget;

pub trait ConstraintTargetImpl: ObjectImpl {}

unsafe impl<T: ConstraintTargetImpl> IsImplementable<T> for ConstraintTarget {
    fn interface_init(_iface: &mut glib::Interface<Self>) {
        assert!(
            crate::rt::is_initialized(),
            "GTK has to be initialized first"
        );
    }
}
