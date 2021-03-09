// Take a look at the license at the top of the repository in the LICENSE file.

use super::media_stream::MediaStreamImpl;
use crate::{MediaFile, MediaStream};
use glib::subclass::prelude::*;
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
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkMediaFileClass;
            if let Some(f) = (*parent_class).close {
                f(media_file.unsafe_cast_ref::<MediaFile>().to_glib_none().0)
            }
        }
    }

    fn parent_open(&self, media_file: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkMediaFileClass;
            if let Some(f) = (*parent_class).open {
                f(media_file.unsafe_cast_ref::<MediaFile>().to_glib_none().0)
            }
        }
    }
}

unsafe impl<T: MediaFileImpl> IsSubclassable<T> for MediaFile {
    fn class_init(class: &mut glib::Class<Self>) {
        <MediaStream as IsSubclassable<T>>::class_init(class);

        let klass = class.as_mut();
        klass.close = Some(media_file_close::<T>);
        klass.open = Some(media_file_open::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <MediaStream as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn media_file_close<T: MediaFileImpl>(ptr: *mut ffi::GtkMediaFile) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<MediaFile> = from_glib_borrow(ptr);

    imp.close(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn media_file_open<T: MediaFileImpl>(ptr: *mut ffi::GtkMediaFile) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<MediaFile> = from_glib_borrow(ptr);

    imp.open(wrap.unsafe_cast_ref())
}
