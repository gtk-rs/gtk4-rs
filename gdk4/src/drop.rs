// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ffi;
use crate::Drop;
use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use std::future;
use std::pin::Pin;
use std::ptr;

impl Drop {
    #[doc(alias = "gdk_drop_read_async")]
    pub fn read_async<Q: FnOnce(Result<(gio::InputStream, GString), glib::Error>) + 'static>(
        &self,
        mime_types: &[&str],
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: Q,
    ) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn read_async_trampoline<
            Q: FnOnce(Result<(gio::InputStream, GString), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let mut out_mime_type = ptr::null();
            let ret = ffi::gdk_drop_read_finish(
                _source_object as *mut _,
                res,
                &mut out_mime_type,
                &mut error,
            );
            let result = if error.is_null() {
                Ok((from_glib_full(ret), from_glib_none(out_mime_type)))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = read_async_trampoline::<Q>;
        unsafe {
            ffi::gdk_drop_read_async(
                self.to_glib_none().0,
                mime_types.to_glib_none().0,
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn read_future(
        &self,
        mime_types: &[&str],
        io_priority: glib::Priority,
    ) -> Pin<
        Box<
            dyn future::Future<Output = Result<(gio::InputStream, GString), glib::Error>> + 'static,
        >,
    > {
        let mime_types = mime_types
            .iter()
            .copied()
            .map(String::from)
            .collect::<Vec<_>>();
        Box::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            let mime_types = mime_types.iter().map(|s| s.as_str()).collect::<Vec<_>>();
            obj.read_async(&mime_types, io_priority, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    #[doc(alias = "gdk_drop_read_value_async")]
    pub fn read_value_async<Q: FnOnce(Result<glib::Value, glib::Error>) + 'static>(
        &self,
        type_: glib::types::Type,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: Q,
    ) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn read_value_async_trampoline<
            Q: FnOnce(Result<glib::Value, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::gdk_drop_read_value_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = read_value_async_trampoline::<Q>;
        unsafe {
            ffi::gdk_drop_read_value_async(
                self.to_glib_none().0,
                type_.into_glib(),
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn read_value_future(
        &self,
        type_: glib::types::Type,
        io_priority: glib::Priority,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<glib::Value, glib::Error>> + 'static>>
    {
        Box::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.read_value_async(type_, io_priority, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }
}
