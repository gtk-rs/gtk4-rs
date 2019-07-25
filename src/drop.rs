// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#[cfg(feature = "futures")]
use futures::future;
use gdk_sys;
use gio;
use gio_sys;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use glib_sys;
use gobject_sys;
use std::ptr;
use Drop;
use Error;

impl Drop {
    pub fn read_async<
        P: IsA<gio::Cancellable>,
        Q: FnOnce(Result<(gio::InputStream, GString), Error>) + Send + 'static,
    >(
        &self,
        mime_types: &[&str],
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn read_async_trampoline<
            Q: FnOnce(Result<(gio::InputStream, GString), Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let mut out_mime_type = ptr::null();
            let ret = gdk_sys::gdk_drop_read_finish(
                _source_object as *mut _,
                res,
                &mut out_mime_type,
                &mut error,
            );
            let result = if error.is_null() {
                Ok((from_glib_full(ret), from_glib_full(out_mime_type)))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = read_async_trampoline::<Q>;
        unsafe {
            gdk_sys::gdk_drop_read_async(
                self.to_glib_none().0,
                mime_types.to_glib_none().0,
                io_priority.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(feature = "futures")]
    pub fn read_async_future(
        &self,
        mime_types: &[&str],
        io_priority: glib::Priority,
    ) -> Box<
        dyn future::Future<Output = Result<(gio::InputStream, GString), Error>>
            + std::marker::Unpin,
    > {
        use fragile::Fragile;
        use gio::GioFuture;

        let mime_types = mime_types.clone();
        GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            let send = Fragile::new(send);
            obj.read_async(&mime_types, io_priority, Some(&cancellable), move |res| {
                let _ = send.into_inner().send(res);
            });

            cancellable
        })
    }
}
