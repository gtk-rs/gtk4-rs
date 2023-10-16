// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for implementing the
//! [`AccessibleRange`](crate::AccessibleRange) interface.

use glib::translate::*;

use crate::{prelude::*, subclass::prelude::*, AccessibleRange};

pub trait AccessibleRangeImpl: WidgetImpl {
    fn set_current_value(&self, accessible_range: &Self::Type, value: f64) -> bool {
        self.parent_set_current_value(accessible_range, value)
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::AccessibleRangeImplExt> Sealed for T {}
}

pub trait AccessibleRangeImplExt: sealed::Sealed + ObjectSubclass {
    // Returns true if the operation was performed, false otherwise
    fn parent_set_current_value(&self, accessible_range: &Self::Type, value: f64) -> bool {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<AccessibleRange>()
                as *const ffi::GtkAccessibleRangeInterface;

            let func = (*parent_iface)
                .set_current_value
                .expect("no parent \"set_current_value\" implementation");

            from_glib(func(
                accessible_range
                    .unsafe_cast_ref::<AccessibleRange>()
                    .to_glib_none()
                    .0,
                value,
            ))
        }
    }
}

impl<T: AccessibleRangeImpl> AccessibleRangeImplExt for T {}

unsafe impl<T: AccessibleRangeImpl> IsImplementable<T> for AccessibleRange {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        iface.set_current_value = Some(accessible_range_set_current_value::<T>);
    }
}

unsafe extern "C" fn accessible_range_set_current_value<T: AccessibleRangeImpl>(
    accessible_range: *mut ffi::GtkAccessibleRange,
    value: f64,
) -> glib::ffi::gboolean {
    let instance = &*(accessible_range as *mut T::Instance);
    let imp = instance.imp();

    imp.set_current_value(
        from_glib_borrow::<_, AccessibleRange>(accessible_range).unsafe_cast_ref(),
        value,
    )
    .into_glib()
}
