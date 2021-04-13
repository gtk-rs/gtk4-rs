// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

use super::widget::WidgetImpl;
use crate::{ListBoxRow, Widget};

pub trait ListBoxRowImpl: ListBoxRowImplExt + WidgetImpl {
    fn activate(&self, row: &Self::Type) {
        self.parent_activate(row)
    }
}

pub trait ListBoxRowImplExt: ObjectSubclass {
    fn parent_activate(&self, row: &Self::Type);
}

impl<T: ListBoxRowImpl> ListBoxRowImplExt for T {
    fn parent_activate(&self, row: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkListBoxRowClass;
            if let Some(f) = (*parent_class).activate {
                f(row.unsafe_cast_ref::<ListBoxRow>().to_glib_none().0)
            }
        }
    }
}

unsafe impl<T: ListBoxRowImpl> IsSubclassable<T> for ListBoxRow {
    fn class_init(class: &mut glib::Class<Self>) {
        <Widget as IsSubclassable<T>>::class_init(class);

        let klass = class.as_mut();
        klass.activate = Some(row_activate::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <Widget as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn row_activate<T: ListBoxRowImpl>(ptr: *mut ffi::GtkListBoxRow) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<ListBoxRow> = from_glib_borrow(ptr);

    imp.activate(wrap.unsafe_cast_ref())
}
