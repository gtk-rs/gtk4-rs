// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`NativeDialog`](crate::NativeDialog).

use glib::translate::*;

use crate::{ffi, prelude::*, subclass::prelude::*, NativeDialog, ResponseType};

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub trait NativeDialogImpl: NativeDialogImplExt + ObjectImpl {
    fn response(&self, response: ResponseType) {
        self.parent_response(response)
    }

    fn show(&self) {
        self.parent_show()
    }

    fn hide(&self) {
        self.parent_hide()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::NativeDialogImplExt> Sealed for T {}
}

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub trait NativeDialogImplExt: sealed::Sealed + ObjectSubclass {
    fn parent_response(&self, response: ResponseType) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkNativeDialogClass;
            if let Some(f) = (*parent_class).response {
                f(
                    self.obj()
                        .unsafe_cast_ref::<NativeDialog>()
                        .to_glib_none()
                        .0,
                    response.into_glib(),
                )
            }
        }
    }

    fn parent_show(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkNativeDialogClass;
            let f = (*parent_class)
                .show
                .expect("No parent class impl for \"show\"");
            f(self
                .obj()
                .unsafe_cast_ref::<NativeDialog>()
                .to_glib_none()
                .0)
        }
    }

    fn parent_hide(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkNativeDialogClass;
            let f = (*parent_class)
                .hide
                .expect("No parent class impl for \"hide\"");
            f(self
                .obj()
                .unsafe_cast_ref::<NativeDialog>()
                .to_glib_none()
                .0)
        }
    }
}

impl<T: NativeDialogImpl> NativeDialogImplExt for T {}

unsafe impl<T: NativeDialogImpl> IsSubclassable<T> for NativeDialog {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        assert_initialized_main_thread!();

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
    let imp = instance.imp();
    let res: ResponseType = from_glib(responseptr);

    imp.response(res)
}

unsafe extern "C" fn dialog_show<T: NativeDialogImpl>(ptr: *mut ffi::GtkNativeDialog) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.show()
}

unsafe extern "C" fn dialog_hide<T: NativeDialogImpl>(ptr: *mut ffi::GtkNativeDialog) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.hide()
}
