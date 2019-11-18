use gtk_sys;

use glib::translate::*;

use glib::subclass::prelude::*;

use super::window::WindowImpl;
use Dialog;
use DialogClass;
use ResponseType;
use WindowClass;

pub trait DialogImpl: DialogImplExt + WindowImpl + 'static {
    fn response(&self, dialog: &Dialog, response: ResponseType) {
        self.parent_response(dialog, response)
    }

    fn close(&self, dialog: &Dialog) {
        self.parent_close(dialog)
    }
}

pub trait DialogImplExt {
    fn parent_response(&self, dialog: &Dialog, response: ResponseType);
    fn parent_close(&self, dialog: &Dialog);
}

impl<T: DialogImpl + ObjectImpl> DialogImplExt for T {
    fn parent_response(&self, dialog: &Dialog, response: ResponseType) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkDialogClass;
            let f = (*parent_class)
                .response
                .expect("No parent class impl for \"response\"");
            f(dialog.to_glib_none().0, response.to_glib())
        }
    }

    fn parent_close(&self, dialog: &Dialog) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkDialogClass;
            let f = (*parent_class)
                .close
                .expect("No parent class impl for \"close\"");
            f(dialog.to_glib_none().0)
        }
    }
}

unsafe impl<T: ObjectSubclass + DialogImpl> IsSubclassable<T> for DialogClass {
    fn override_vfuncs(&mut self) {
        <WindowClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gtk_sys::GtkDialogClass);
            klass.response = Some(dialog_response::<T>);
            klass.close = Some(dialog_close::<T>);
        }
    }
}

unsafe extern "C" fn dialog_response<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkDialog,
    responseptr: i32,
) where
    T: DialogImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Dialog = from_glib_borrow(ptr);
    let res: ResponseType = from_glib(responseptr);

    imp.response(&wrap, res)
}

unsafe extern "C" fn dialog_close<T: ObjectSubclass>(ptr: *mut gtk_sys::GtkDialog)
where
    T: DialogImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Dialog = from_glib_borrow(ptr);

    imp.close(&wrap)
}
