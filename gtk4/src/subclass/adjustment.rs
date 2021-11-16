// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`Adjustment`](crate::Adjustment).

use crate::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

use crate::Adjustment;

pub trait AdjustmentImpl: AdjustmentImplExt + ObjectImpl {
    fn changed(&self, adjustment: &Self::Type) {
        self.parent_changed(adjustment)
    }

    fn value_changed(&self, adjustment: &Self::Type) {
        self.parent_value_changed(adjustment)
    }
}

pub trait AdjustmentImplExt: ObjectSubclass {
    fn parent_changed(&self, adjustment: &Self::Type);
    fn parent_value_changed(&self, adjustment: &Self::Type);
}

impl<T: AdjustmentImpl> AdjustmentImplExt for T {
    fn parent_changed(&self, adjustment: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkAdjustmentClass;
            if let Some(f) = (*parent_class).changed {
                f(adjustment.unsafe_cast_ref::<Adjustment>().to_glib_none().0)
            }
        }
    }

    fn parent_value_changed(&self, adjustment: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkAdjustmentClass;
            if let Some(f) = (*parent_class).value_changed {
                f(adjustment.unsafe_cast_ref::<Adjustment>().to_glib_none().0)
            }
        }
    }
}

unsafe impl<T: AdjustmentImpl> IsSubclassable<T> for Adjustment {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        assert!(
            crate::rt::is_initialized(),
            "GTK has to be initialized first"
        );

        let klass = class.as_mut();
        klass.changed = Some(adjustment_changed::<T>);
        klass.value_changed = Some(adjustment_value_changed::<T>);
    }
}

unsafe extern "C" fn adjustment_changed<T: AdjustmentImpl>(ptr: *mut ffi::GtkAdjustment) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Adjustment> = from_glib_borrow(ptr);

    imp.changed(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn adjustment_value_changed<T: AdjustmentImpl>(ptr: *mut ffi::GtkAdjustment) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Adjustment> = from_glib_borrow(ptr);

    imp.value_changed(wrap.unsafe_cast_ref())
}
