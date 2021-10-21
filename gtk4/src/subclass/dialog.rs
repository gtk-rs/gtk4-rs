// Take a look at the license at the top of the repository in the LICENSE file.

use crate::subclass::prelude::*;
use crate::{Dialog, ResponseType};
use glib::translate::*;
use glib::Cast;

pub trait DialogImpl: DialogImplExt + WindowImpl {
    fn response(&self, dialog: &Self::Type, response: ResponseType) {
        self.parent_response(dialog, response)
    }

    fn close(&self, dialog: &Self::Type) {
        self.parent_close(dialog)
    }
}

pub trait DialogImplExt: ObjectSubclass {
    fn parent_response(&self, dialog: &Self::Type, response: ResponseType);
    fn parent_close(&self, dialog: &Self::Type);
}

impl<T: DialogImpl> DialogImplExt for T {
    fn parent_response(&self, dialog: &Self::Type, response: ResponseType) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkDialogClass;
            if let Some(f) = (*parent_class).response {
                f(
                    dialog.unsafe_cast_ref::<Dialog>().to_glib_none().0,
                    response.into_glib(),
                )
            }
        }
    }

    fn parent_close(&self, dialog: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkDialogClass;
            if let Some(f) = (*parent_class).close {
                f(dialog.unsafe_cast_ref::<Dialog>().to_glib_none().0)
            }
        }
    }
}

unsafe impl<T: DialogImpl> IsSubclassable<T> for Dialog {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.response = Some(dialog_response::<T>);
        klass.close = Some(dialog_close::<T>);
    }
}

unsafe extern "C" fn dialog_response<T: DialogImpl>(ptr: *mut ffi::GtkDialog, responseptr: i32) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Dialog> = from_glib_borrow(ptr);
    let res: ResponseType = from_glib(responseptr);

    imp.response(wrap.unsafe_cast_ref(), res)
}

unsafe extern "C" fn dialog_close<T: DialogImpl>(ptr: *mut ffi::GtkDialog) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Dialog> = from_glib_borrow(ptr);

    imp.close(wrap.unsafe_cast_ref())
}
