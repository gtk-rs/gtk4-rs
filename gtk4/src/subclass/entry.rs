// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`Entry`].

use glib::translate::*;

use crate::{ffi, prelude::*, subclass::prelude::*, CellEditable, Entry};

pub trait EntryImpl: WidgetImpl + ObjectSubclass<Type: IsA<Entry> + IsA<CellEditable>> {
    fn activate(&self) {
        self.parent_activate()
    }
}

pub trait EntryImplExt: EntryImpl {
    fn parent_activate(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkEntryClass;
            if let Some(f) = (*parent_class).activate {
                f(self.obj().unsafe_cast_ref::<Entry>().to_glib_none().0)
            }
        }
    }
}

impl<T: EntryImpl> EntryImplExt for T {}

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
