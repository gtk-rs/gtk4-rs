// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`ListBoxRow`].

use glib::translate::*;

use crate::{ffi, prelude::*, subclass::prelude::*, Actionable, ListBoxRow};

pub trait ListBoxRowImpl:
    WidgetImpl + ObjectSubclass<Type: IsA<ListBoxRow> + IsA<Actionable>>
{
    fn activate(&self) {
        self.parent_activate()
    }
}

pub trait ListBoxRowImplExt: ListBoxRowImpl {
    fn parent_activate(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkListBoxRowClass;
            if let Some(f) = (*parent_class).activate {
                f(self.obj().unsafe_cast_ref::<ListBoxRow>().to_glib_none().0)
            }
        }
    }
}

impl<T: ListBoxRowImpl> ListBoxRowImplExt for T {}

unsafe impl<T: ListBoxRowImpl> IsSubclassable<T> for ListBoxRow {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.activate = Some(row_activate::<T>);
    }
}

unsafe extern "C" fn row_activate<T: ListBoxRowImpl>(ptr: *mut ffi::GtkListBoxRow) {
    unsafe {
        let instance = &*(ptr as *mut T::Instance);
        let imp = instance.imp();

        imp.activate()
    }
}
