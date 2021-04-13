// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

use super::widget::WidgetImpl;
use crate::{Entry, Widget};

pub trait EntryImpl: EntryImplExt + WidgetImpl {
    fn activate(&self, entry: &Self::Type) {
        self.parent_activate(entry)
    }
}

pub trait EntryImplExt: ObjectSubclass {
    fn parent_activate(&self, entry: &Self::Type);
}

impl<T: EntryImpl> EntryImplExt for T {
    fn parent_activate(&self, entry: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkEntryClass;
            if let Some(f) = (*parent_class).activate {
                f(entry.unsafe_cast_ref::<Entry>().to_glib_none().0)
            }
        }
    }
}

unsafe impl<T: EntryImpl> IsSubclassable<T> for Entry {
    fn class_init(class: &mut glib::Class<Self>) {
        <Widget as IsSubclassable<T>>::class_init(class);

        let klass = class.as_mut();
        klass.activate = Some(entry_activate::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <Widget as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn entry_activate<T: EntryImpl>(ptr: *mut ffi::GtkEntry) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Entry> = from_glib_borrow(ptr);

    imp.activate(wrap.unsafe_cast_ref())
}
