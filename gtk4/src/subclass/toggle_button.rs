use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

use super::button::ButtonImpl;
use crate::{Button, ToggleButton};

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
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkToggleButtonClass;
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
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <Button as IsSubclassable<T>>::override_vfuncs(class);

        let klass = class.as_mut();
        klass.toggled = Some(toggle_button_toggled::<T>);
    }
}

unsafe extern "C" fn toggle_button_toggled<T: ToggleButtonImpl>(ptr: *mut ffi::GtkToggleButton) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<ToggleButton> = from_glib_borrow(ptr);

    imp.toggled(wrap.unsafe_cast_ref())
}
