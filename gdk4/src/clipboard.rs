// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Clipboard, Texture};
use glib::object::IsA;
use glib::translate::*;
use glib::{GString, ToValue};
use std::future;
use std::pin::Pin;
use std::ptr;

impl Clipboard {
    #[doc(alias = "gdk_clipboard_read_async")]
    pub fn read_async<
        P: IsA<gio::Cancellable>,
        Q: FnOnce(Result<(gio::InputStream, GString), glib::Error>) + 'static,
    >(
        &self,
        mime_types: &[&str],
        io_priority: glib::Priority,
        cancellable: Option<&P>,
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
            let ret = ffi::gdk_clipboard_read_finish(
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
            ffi::gdk_clipboard_read_async(
                self.to_glib_none().0,
                mime_types.to_glib_none().0,
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn read_async_future(
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
        Box::pin(gio::GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            let mime_types = mime_types.iter().map(|s| s.as_str()).collect::<Vec<_>>();
            obj.read_async(&mime_types, io_priority, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    #[doc(alias = "gdk_clipboard_set_value")]
    #[doc(alias = "gdk_clipboard_set_valist")]
    #[doc(alias = "gdk_clipboard_set")]
    pub fn set(&self, value: &dyn ToValue) {
        unsafe {
            ffi::gdk_clipboard_set_value(self.to_glib_none().0, value.to_value().to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_clipboard_store_async")]
    pub fn store_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn store_async_trampoline<
            Q: FnOnce(Result<(), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::gdk_clipboard_store_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = store_async_trampoline::<Q>;
        unsafe {
            ffi::gdk_clipboard_store_async(
                self.to_glib_none().0,
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn store_async_future(
        &self,
        io_priority: glib::Priority,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box::pin(gio::GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            obj.store_async(io_priority, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    #[doc(alias = "gdk_clipboard_read_value_async")]
    pub fn read_value_async<
        P: IsA<gio::Cancellable>,
        Q: FnOnce(Result<glib::Value, glib::Error>) + 'static,
    >(
        &self,
        type_: glib::types::Type,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
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
            let ret =
                ffi::gdk_clipboard_read_value_finish(_source_object as *mut _, res, &mut error);
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
            ffi::gdk_clipboard_read_value_async(
                self.to_glib_none().0,
                type_.into_glib(),
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn read_value_async_future(
        &self,
        type_: glib::types::Type,
        io_priority: glib::Priority,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<glib::Value, glib::Error>> + 'static>>
    {
        Box::pin(gio::GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            obj.read_value_async(type_, io_priority, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    #[doc(alias = "gdk_clipboard_read_text_async")]
    pub fn read_text_async<
        P: IsA<gio::Cancellable>,
        Q: FnOnce(Result<Option<glib::GString>, glib::Error>) + 'static,
    >(
        &self,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn read_text_async_trampoline<
            Q: FnOnce(Result<Option<glib::GString>, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                ffi::gdk_clipboard_read_text_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = read_text_async_trampoline::<Q>;
        unsafe {
            ffi::gdk_clipboard_read_text_async(
                self.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn read_text_async_future(
        &self,
    ) -> Pin<
        Box<dyn std::future::Future<Output = Result<Option<glib::GString>, glib::Error>> + 'static>,
    > {
        Box::pin(gio::GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            obj.read_text_async(Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    #[doc(alias = "gdk_clipboard_read_texture_async")]
    pub fn read_texture_async<
        P: IsA<gio::Cancellable>,
        Q: FnOnce(Result<Option<Texture>, glib::Error>) + 'static,
    >(
        &self,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn read_texture_async_trampoline<
            Q: FnOnce(Result<Option<Texture>, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                ffi::gdk_clipboard_read_texture_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = read_texture_async_trampoline::<Q>;
        unsafe {
            ffi::gdk_clipboard_read_texture_async(
                self.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn read_texture_async_future(
        &self,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<Option<Texture>, glib::Error>> + 'static>>
    {
        Box::pin(gio::GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            obj.read_texture_async(Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }
}
