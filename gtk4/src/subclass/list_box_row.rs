// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`ListBoxRow`](crate::ListBoxRow).

use crate::{prelude::*, subclass::prelude::*, ListBoxRow};
use glib::translate::*;

pub trait ListBoxRowImpl: ListBoxRowImplExt + WidgetImpl {
    fn activate(&self) {
        self.parent_activate()
    }
}

pub trait ListBoxRowImplExt: ObjectSubclass {
    fn parent_activate(&self);
}

impl<T: ListBoxRowImpl> ListBoxRowImplExt for T {
    fn parent_activate(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkListBoxRowClass;
            if let Some(f) = (*parent_class).activate {
                f(self.obj().unsafe_cast_ref::<ListBoxRow>().to_glib_none().0)
            }
        }
    }
}

unsafe impl<T: ListBoxRowImpl> IsSubclassable<T> for ListBoxRow {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.activate = Some(row_activate::<T>);
    }
}

unsafe extern "C" fn row_activate<T: ListBoxRowImpl>(ptr: *mut ffi::GtkListBoxRow) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.activate()
}
