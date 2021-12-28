// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`ScaleButton`](crate::ScaleButton).

use crate::subclass::prelude::*;
use crate::ScaleButton;
use glib::translate::*;
use glib::Cast;

pub trait ScaleButtonImpl: ScaleButtonImplExt + WidgetImpl {
    fn value_changed(&self, scale_button: &Self::Type, new_value: f64) {
        self.parent_value_changed(scale_button, new_value)
    }
}

pub trait ScaleButtonImplExt: ObjectSubclass {
    fn parent_value_changed(&self, scale_button: &Self::Type, new_value: f64);
}

impl<T: ScaleButtonImpl> ScaleButtonImplExt for T {
    fn parent_value_changed(&self, scale_button: &Self::Type, new_value: f64) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkScaleButtonClass;
            if let Some(f) = (*parent_class).value_changed {
                f(
                    scale_button
                        .unsafe_cast_ref::<ScaleButton>()
                        .to_glib_none()
                        .0,
                    new_value,
                )
            }
        }
    }
}

unsafe impl<T: ScaleButtonImpl> IsSubclassable<T> for ScaleButton {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.value_changed = Some(scale_button_value_changed::<T>);
    }
}

unsafe extern "C" fn scale_button_value_changed<T: ScaleButtonImpl>(
    ptr: *mut ffi::GtkScaleButton,
    new_value: f64,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<ScaleButton> = from_glib_borrow(ptr);

    imp.value_changed(wrap.unsafe_cast_ref(), new_value)
}
