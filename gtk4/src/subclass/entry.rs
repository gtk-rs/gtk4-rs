// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`Entry`](crate::Entry).

use crate::{prelude::*, subclass::prelude::*, Entry};
use glib::translate::*;

pub trait EntryImpl: EntryImplExt + WidgetImpl {
    fn activate(&self) {
        self.parent_activate()
    }
}

pub trait EntryImplExt: ObjectSubclass {
    fn parent_activate(&self);
}

impl<T: EntryImpl> EntryImplExt for T {
    fn parent_activate(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkEntryClass;
            if let Some(f) = (*parent_class).activate {
                f(self.obj().unsafe_cast_ref::<Entry>().to_glib_none().0)
            }
        }
    }
}

unsafe impl<T: EntryImpl> IsSubclassable<T> for Entry {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.activate = Some(entry_activate::<T>);
    }
}

unsafe extern "C" fn entry_activate<T: EntryImpl>(ptr: *mut ffi::GtkEntry) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.activate()
}
