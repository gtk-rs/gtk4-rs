// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::object::ObjectImpl;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::{Cast, Object};

use crate::{NativeDialog, ResponseType};

pub trait NativeDialogImpl: NativeDialogImplExt + ObjectImpl {
    fn response(&self, dialog: &Self::Type, response: ResponseType) {
        self.parent_response(dialog, response)
    }

    fn show(&self, dialog: &Self::Type) {
        self.parent_show(dialog)
    }

    fn hide(&self, dialog: &Self::Type) {
        self.parent_hide(dialog)
    }
}

pub trait NativeDialogImplExt: ObjectSubclass {
    fn parent_response(&self, dialog: &Self::Type, response: ResponseType);
    fn parent_show(&self, dialog: &Self::Type);
    fn parent_hide(&self, dialog: &Self::Type);
}

impl<T: NativeDialogImpl> NativeDialogImplExt for T {
    fn parent_response(&self, dialog: &Self::Type, response: ResponseType) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkNativeDialogClass;
            if let Some(f) = (*parent_class).response {
                f(
                    dialog.unsafe_cast_ref::<NativeDialog>().to_glib_none().0,
                    response.to_glib(),
                )
            }
        }
    }

    fn parent_show(&self, dialog: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkNativeDialogClass;
            let f = (*parent_class)
                .show
                .expect("No parent class impl for \"show\"");
            f(dialog.unsafe_cast_ref::<NativeDialog>().to_glib_none().0)
        }
    }

    fn parent_hide(&self, dialog: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkNativeDialogClass;
            let f = (*parent_class)
                .hide
                .expect("No parent class impl for \"hide\"");
            f(dialog.unsafe_cast_ref::<NativeDialog>().to_glib_none().0)
        }
    }
}

unsafe impl<T: NativeDialogImpl> IsSubclassable<T> for NativeDialog {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <Object as IsSubclassable<T>>::override_vfuncs(class);

        let klass = class.as_mut();
        klass.response = Some(dialog_response::<T>);
        klass.show = Some(dialog_show::<T>);
        klass.hide = Some(dialog_hide::<T>);
    }
}

unsafe extern "C" fn dialog_response<T: NativeDialogImpl>(
    ptr: *mut ffi::GtkNativeDialog,
    responseptr: i32,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<NativeDialog> = from_glib_borrow(ptr);
    let res: ResponseType = from_glib(responseptr);

    imp.response(wrap.unsafe_cast_ref(), res)
}

unsafe extern "C" fn dialog_show<T: NativeDialogImpl>(ptr: *mut ffi::GtkNativeDialog) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<NativeDialog> = from_glib_borrow(ptr);

    imp.show(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn dialog_hide<T: NativeDialogImpl>(ptr: *mut ffi::GtkNativeDialog) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<NativeDialog> = from_glib_borrow(ptr);

    imp.hide(wrap.unsafe_cast_ref())
}
