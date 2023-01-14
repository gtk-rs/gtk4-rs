// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`LayoutChild`](crate::LayoutChild).

use crate::{subclass::prelude::*, LayoutChild};

pub trait LayoutChildImpl: ObjectImpl {}

unsafe impl<T: LayoutChildImpl> IsSubclassable<T> for LayoutChild {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        assert_initialized_main_thread!();
    }
}
