// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::GString;
use glib::Object;
use glib::object::{Cast, IsA};
use glib::prelude::*;
use glib::translate::*;
use std::path::Path;
use Builder;
use Error;


impl Builder {
    pub fn new_from_file<T: AsRef<Path>>(file_path: T) -> Builder {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gtk_sys::gtk_builder_new_from_file(file_path.as_ref().to_glib_none().0)) }
    }
}

pub trait BuilderExtManual: 'static {
    fn get_object<T: IsA<Object>>(&self, name: &str) -> Option<T>;
    fn add_from_file<T: AsRef<Path>>(&self, file_path: T) -> Result<(), Error>;
    fn connect_signals_full<P: FnMut(&Builder, &str) -> Box<dyn Fn(&[glib::Value]) -> Option<glib::Value> + Send + Sync + 'static>>(&self, func: P);
}

impl<O: IsA<Builder>> BuilderExtManual for O {
    fn get_object<T: IsA<Object>>(&self, name: &str) -> Option<T> {
        unsafe {
            Option::<Object>::from_glib_none(
                gtk_sys::gtk_builder_get_object(self.upcast_ref().to_glib_none().0, name.to_glib_none().0))
                .and_then(|obj| obj.dynamic_cast::<T>().ok())
        }
    }

    fn add_from_file<T: AsRef<Path>>(&self, file_path: T) -> Result<(), Error> {
        unsafe {
            let mut error = ::std::ptr::null_mut();
            gtk_sys::gtk_builder_add_from_file(self.upcast_ref().to_glib_none().0,
                                           file_path.as_ref().to_glib_none().0,
                                           &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn connect_signals_full<P: FnMut(&Builder, &str) -> Box<dyn Fn(&[glib::Value]) -> Option<glib::Value> + Send + Sync + 'static>>(&self, func: P) {
        let func_data: P = func;
        unsafe extern "C" fn func_func<P: FnMut(&Builder, &str) -> Box<dyn Fn(&[glib::Value]) -> Option<glib::Value> + Send + Sync + 'static>>(builder: *mut gtk_sys::GtkBuilder, object: *mut gobject_sys::GObject, signal_name: *const libc::c_char, handler_name: *const libc::c_char, connect_object: *mut gobject_sys::GObject, flags: gobject_sys::GConnectFlags, user_data: glib_sys::gpointer) {
            let builder = from_glib_borrow(builder);
            let object: glib::Object = from_glib_borrow(object);
            let signal_name: GString = from_glib_borrow(signal_name);
            let handler_name: GString = from_glib_borrow(handler_name);
            let connect_object: Option<glib::Object> = from_glib_borrow(connect_object); // TODO
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            let func = (*callback)(&builder, handler_name.as_str());
            object.connect(signal_name.as_str(), flags & gobject_sys::G_CONNECT_AFTER != 0, func);
            // TODO: signal id
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &P = &func_data;
        unsafe {
            gtk_sys::gtk_builder_connect_signals_full(self.as_ref().to_glib_none().0, func, super_callback0 as *const _ as usize as *mut _);
        }
    }
}
