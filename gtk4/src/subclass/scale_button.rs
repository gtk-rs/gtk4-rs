// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

use super::widget::WidgetImpl;
use crate::{ScaleButton, Widget};

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
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkScaleButtonClass;
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
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <Widget as IsSubclassable<T>>::override_vfuncs(class);

        let klass = class.as_mut();
        klass.value_changed = Some(scale_button_value_changed::<T>);
    }
}

unsafe extern "C" fn scale_button_value_changed<T: ScaleButtonImpl>(
    ptr: *mut ffi::GtkScaleButton,
    new_value: f64,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<ScaleButton> = from_glib_borrow(ptr);

    imp.value_changed(wrap.unsafe_cast_ref(), new_value)
}
