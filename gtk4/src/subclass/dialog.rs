// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`Dialog`](crate::Dialog).

use crate::{prelude::*, subclass::prelude::*, Dialog, ResponseType};
use glib::translate::*;

pub trait DialogImpl: DialogImplExt + WindowImpl {
    fn response(&self, response: ResponseType) {
        self.parent_response(response)
    }

    fn close(&self) {
        self.parent_close()
    }
}

pub trait DialogImplExt: ObjectSubclass {
    fn parent_response(&self, response: ResponseType);
    fn parent_close(&self);
}

impl<T: DialogImpl> DialogImplExt for T {
    fn parent_response(&self, response: ResponseType) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkDialogClass;
            if let Some(f) = (*parent_class).response {
                f(
                    self.obj().unsafe_cast_ref::<Dialog>().to_glib_none().0,
                    response.into_glib(),
                )
            }
        }
    }

    fn parent_close(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkDialogClass;
            if let Some(f) = (*parent_class).close {
                f(self.obj().unsafe_cast_ref::<Dialog>().to_glib_none().0)
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
    let imp = instance.imp();
    let res: ResponseType = from_glib(responseptr);

    imp.response(res)
}

unsafe extern "C" fn dialog_close<T: DialogImpl>(ptr: *mut ffi::GtkDialog) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.close()
}
