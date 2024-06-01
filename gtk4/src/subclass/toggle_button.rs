// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`ToggleButton`](crate::ToggleButton).

use glib::translate::*;

use crate::{ffi, prelude::*, subclass::prelude::*, ToggleButton};

pub trait ToggleButtonImpl: ToggleButtonImplExt + ButtonImpl {
    fn toggled(&self) {
        self.parent_toggled()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::ToggleButtonImplExt> Sealed for T {}
}

pub trait ToggleButtonImplExt: sealed::Sealed + ObjectSubclass {
    fn parent_toggled(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkToggleButtonClass;
            if let Some(f) = (*parent_class).toggled {
                f(self
                    .obj()
                    .unsafe_cast_ref::<ToggleButton>()
                    .to_glib_none()
                    .0)
            }
        }
    }
}

impl<T: ToggleButtonImpl> ToggleButtonImplExt for T {}

unsafe impl<T: ToggleButtonImpl> IsSubclassable<T> for ToggleButton {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.toggled = Some(toggle_button_toggled::<T>);
    }
}

unsafe extern "C" fn toggle_button_toggled<T: ToggleButtonImpl>(ptr: *mut ffi::GtkToggleButton) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.toggled()
}
