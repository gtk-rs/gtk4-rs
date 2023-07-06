// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, Clipboard};
use glib::translate::*;
use glib::GString;
use std::{boxed::Box as Box_, future, pin::Pin, ptr};

impl Clipboard {
    #[doc(alias = "gdk_clipboard_read_async")]
    pub fn read_async<Q: FnOnce(Result<(gio::InputStream, GString), glib::Error>) + 'static>(
        &self,
        mime_types: &[&str],
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: Q,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box<glib::thread_guard::ThreadGuard<Q>> =
            Box::new(glib::thread_guard::ThreadGuard::new(callback));
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
            let callback: Box<glib::thread_guard::ThreadGuard<Q>> =
                Box::from_raw(user_data as *mut _);
            let callback = callback.into_inner();
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

    #[allow(clippy::type_complexity)]
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

    #[doc(alias = "gdk_clipboard_read_value_async")]
    pub fn read_value_async<T: for<'a> glib::value::FromValue<'a> + glib::StaticType>(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: impl FnOnce(Result<T, glib::Error>) + 'static,
    ) {
        let callback2 =
            |t: Result<glib::Value, glib::Error>| callback(t.map(|v| v.get::<T>().unwrap()));
        self.read_value_async_with_type(T::static_type(), io_priority, cancellable, callback2)
    }

    #[doc(alias = "gdk_clipboard_read_value_async")]
    pub fn read_value_async_with_type<P: FnOnce(Result<glib::Value, glib::Error>) + 'static>(
        &self,
        type_: glib::types::Type,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn read_value_async_trampoline<
            P: FnOnce(Result<glib::Value, glib::Error>) + 'static,
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
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = read_value_async_trampoline::<P>;
        unsafe {
            ffi::gdk_clipboard_read_value_async(
                self.to_glib_none().0,
                type_.into_glib(),
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub async fn read_value_future<T: for<'a> glib::value::FromValue<'a> + glib::StaticType>(
        &self,
        io_priority: glib::Priority,
    ) -> Result<T, glib::Error> {
        self.read_value_future_with_type(T::static_type(), io_priority)
            .await
            .map(|v| v.get::<T>().unwrap())
    }

    pub fn read_value_future_with_type(
        &self,
        type_: glib::types::Type,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<glib::Value, glib::Error>> + 'static>>
    {
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.read_value_async_with_type(type_, io_priority, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }
}
