// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`Button`](crate::Button).

use crate::{prelude::*, subclass::prelude::*, Button};
use glib::translate::*;

pub trait ButtonImpl: ButtonImplExt + WidgetImpl {
    fn activate(&self) {
        self.parent_activate()
    }

    fn clicked(&self) {
        self.parent_clicked()
    }
}

pub trait ButtonImplExt: ObjectSubclass {
    fn parent_activate(&self);
    fn parent_clicked(&self);
}

impl<T: ButtonImpl> ButtonImplExt for T {
    fn parent_activate(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkButtonClass;
            if let Some(f) = (*parent_class).activate {
                f(self.obj().unsafe_cast_ref::<Button>().to_glib_none().0)
            }
        }
    }

    fn parent_clicked(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkButtonClass;
            if let Some(f) = (*parent_class).clicked {
                f(self.obj().unsafe_cast_ref::<Button>().to_glib_none().0)
            }
        }
    }
}

unsafe impl<T: ButtonImpl> IsSubclassable<T> for Button {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.activate = Some(button_activate::<T>);
        klass.clicked = Some(button_clicked::<T>);
    }
}

unsafe extern "C" fn button_activate<T: ButtonImpl>(ptr: *mut ffi::GtkButton) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.activate()
}

unsafe extern "C" fn button_clicked<T: ButtonImpl>(ptr: *mut ffi::GtkButton) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.clicked()
}
