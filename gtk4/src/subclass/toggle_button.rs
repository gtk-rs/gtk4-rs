// Take a look at the license at the top of the repository in the LICENSE file.

use crate::subclass::prelude::*;
use crate::ToggleButton;
use glib::translate::*;
use glib::Cast;

pub trait ToggleButtonImpl: ToggleButtonImplExt + ButtonImpl {
    fn toggled(&self, toggle_button: &Self::Type) {
        self.parent_toggled(toggle_button)
    }
}

pub trait ToggleButtonImplExt: ObjectSubclass {
    fn parent_toggled(&self, toggle_button: &Self::Type);
}

impl<T: ToggleButtonImpl> ToggleButtonImplExt for T {
    fn parent_toggled(&self, toggle_button: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkToggleButtonClass;
            if let Some(f) = (*parent_class).toggled {
                f(toggle_button
                    .unsafe_cast_ref::<ToggleButton>()
                    .to_glib_none()
                    .0)
            }
        }
    }
}

unsafe impl<T: ToggleButtonImpl> IsSubclassable<T> for ToggleButton {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.toggled = Some(toggle_button_toggled::<T>);
    }
}

unsafe extern "C" fn toggle_button_toggled<T: ToggleButtonImpl>(ptr: *mut ffi::GtkToggleButton) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<ToggleButton> = from_glib_borrow(ptr);

    imp.toggled(wrap.unsafe_cast_ref())
}
