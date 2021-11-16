// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`MediaFile`](crate::MediaFile).

use crate::subclass::prelude::*;
use crate::MediaFile;
use glib::translate::*;
use glib::Cast;

pub trait MediaFileImpl: MediaFileImplExt + MediaStreamImpl {
    fn close(&self, media_file: &Self::Type) {
        self.parent_close(media_file)
    }
    fn open(&self, media_file: &Self::Type) {
        self.parent_open(media_file)
    }
}

pub trait MediaFileImplExt: ObjectSubclass {
    fn parent_close(&self, media_file: &Self::Type);
    fn parent_open(&self, media_file: &Self::Type);
}

impl<T: MediaFileImpl> MediaFileImplExt for T {
    fn parent_close(&self, media_file: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkMediaFileClass;
            if let Some(f) = (*parent_class).close {
                f(media_file.unsafe_cast_ref::<MediaFile>().to_glib_none().0)
            }
        }
    }

    fn parent_open(&self, media_file: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkMediaFileClass;
            if let Some(f) = (*parent_class).open {
                f(media_file.unsafe_cast_ref::<MediaFile>().to_glib_none().0)
            }
        }
    }
}

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
    let imp = instance.impl_();
    let wrap: Borrowed<MediaFile> = from_glib_borrow(ptr);

    imp.close(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn media_file_open<T: MediaFileImpl>(ptr: *mut ffi::GtkMediaFile) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<MediaFile> = from_glib_borrow(ptr);

    imp.open(wrap.unsafe_cast_ref())
}
