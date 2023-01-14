// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`Adjustment`](crate::Adjustment).

use crate::{prelude::*, subclass::prelude::*, Adjustment};
use glib::translate::*;

pub trait AdjustmentImpl: AdjustmentImplExt + ObjectImpl {
    fn changed(&self) {
        self.parent_changed()
    }

    fn value_changed(&self) {
        self.parent_value_changed()
    }
}

pub trait AdjustmentImplExt: ObjectSubclass {
    fn parent_changed(&self);
    fn parent_value_changed(&self);
}

impl<T: AdjustmentImpl> AdjustmentImplExt for T {
    fn parent_changed(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkAdjustmentClass;
            if let Some(f) = (*parent_class).changed {
                f(self.obj().unsafe_cast_ref::<Adjustment>().to_glib_none().0)
            }
        }
    }

    fn parent_value_changed(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkAdjustmentClass;
            if let Some(f) = (*parent_class).value_changed {
                f(self.obj().unsafe_cast_ref::<Adjustment>().to_glib_none().0)
            }
        }
    }
}

unsafe impl<T: AdjustmentImpl> IsSubclassable<T> for Adjustment {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        assert_initialized_main_thread!();

        let klass = class.as_mut();
        klass.changed = Some(adjustment_changed::<T>);
        klass.value_changed = Some(adjustment_value_changed::<T>);
    }
}

unsafe extern "C" fn adjustment_changed<T: AdjustmentImpl>(ptr: *mut ffi::GtkAdjustment) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.changed()
}

unsafe extern "C" fn adjustment_value_changed<T: AdjustmentImpl>(ptr: *mut ffi::GtkAdjustment) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.value_changed()
}
