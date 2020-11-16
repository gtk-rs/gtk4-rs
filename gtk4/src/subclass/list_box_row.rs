use gtk_sys;

use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

use super::widget::WidgetImpl;
use ListBoxRow;
use Widget;

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
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkListBoxRowClass;
            if let Some(f) = (*parent_class).activate {
                f(row.unsafe_cast_ref::<ListBoxRow>().to_glib_none().0)
            }
        }
    }
}

unsafe impl<T: ListBoxRowImpl> IsSubclassable<T> for ListBoxRow {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <Widget as IsSubclassable<T>>::override_vfuncs(class);

        let klass = class.as_mut();
        klass.activate = Some(row_activate::<T>);
    }
}

unsafe extern "C" fn row_activate<T: ListBoxRowImpl>(ptr: *mut gtk_sys::GtkListBoxRow) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<ListBoxRow> = from_glib_borrow(ptr);

    imp.activate(wrap.unsafe_cast_ref())
}
