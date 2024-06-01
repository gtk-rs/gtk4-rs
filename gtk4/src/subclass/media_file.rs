// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`MediaFile`](crate::MediaFile).

use glib::translate::*;

use crate::{ffi, prelude::*, subclass::prelude::*, MediaFile};

pub trait MediaFileImpl: MediaFileImplExt + MediaStreamImpl {
    fn close(&self) {
        self.parent_close()
    }
    fn open(&self) {
        self.parent_open()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::MediaFileImplExt> Sealed for T {}
}

pub trait MediaFileImplExt: sealed::Sealed + ObjectSubclass {
    fn parent_close(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkMediaFileClass;
            if let Some(f) = (*parent_class).close {
                f(self.obj().unsafe_cast_ref::<MediaFile>().to_glib_none().0)
            }
        }
    }

    fn parent_open(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkMediaFileClass;
            if let Some(f) = (*parent_class).open {
                f(self.obj().unsafe_cast_ref::<MediaFile>().to_glib_none().0)
            }
        }
    }
}

impl<T: MediaFileImpl> MediaFileImplExt for T {}

unsafe impl<T: MediaFileImpl> IsSubclassable<T> for MediaFile {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.close = Some(media_file_close::<T>);
        klass.open = Some(media_file_open::<T>);
    }
}

unsafe extern "C" fn media_file_close<T: MediaFileImpl>(ptr: *mut ffi::GtkMediaFile) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.close()
}

unsafe extern "C" fn media_file_open<T: MediaFileImpl>(ptr: *mut ffi::GtkMediaFile) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.open()
}
