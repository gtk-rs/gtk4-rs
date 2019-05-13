// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::Object;
use glib::object::{Cast, IsA};
use glib::translate::*;
use ffi;
use std::path::Path;
use Builder;
use Error;


impl Builder {
    pub fn new_from_file<T: AsRef<Path>>(file_path: T) -> Builder {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_builder_new_from_file(file_path.as_ref().to_glib_none().0)) }
    }
}

pub trait BuilderExtManual: 'static {
    fn get_object<T: IsA<Object>>(&self, name: &str) -> Option<T>;
    fn add_from_file<T: AsRef<Path>>(&self, file_path: T) -> Result<(), Error>;
}

impl<O: IsA<Builder>> BuilderExtManual for O {
    fn get_object<T: IsA<Object>>(&self, name: &str) -> Option<T> {
        unsafe {
            Option::<Object>::from_glib_none(
                ffi::gtk_builder_get_object(self.upcast_ref().to_glib_none().0, name.to_glib_none().0))
                .and_then(|obj| obj.dynamic_cast::<T>().ok())
        }
    }

    fn add_from_file<T: AsRef<Path>>(&self, file_path: T) -> Result<(), Error> {
        unsafe {
            let mut error = ::std::ptr::null_mut();
            ffi::gtk_builder_add_from_file(self.upcast_ref().to_glib_none().0,
                                           file_path.as_ref().to_glib_none().0,
                                           &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }
}
