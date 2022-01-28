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
}
