// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, ContentFormats};
use glib::{
    object::ObjectType as _,
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, pin::Pin};

glib::wrapper! {
    #[doc(alias = "GdkContentProvider")]
    pub struct ContentProvider(Object<ffi::GdkContentProvider, ffi::GdkContentProviderClass>);

    match fn {
        type_ => || ffi::gdk_content_provider_get_type(),
    }
}

impl ContentProvider {
    pub const NONE: Option<&'static ContentProvider> = None;

    #[doc(alias = "gdk_content_provider_new_for_bytes")]
    #[doc(alias = "new_for_bytes")]
    pub fn for_bytes(mime_type: &str, bytes: &glib::Bytes) -> ContentProvider {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gdk_content_provider_new_for_bytes(
                mime_type.to_glib_none().0,
                bytes.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_content_provider_new_for_value")]
    #[doc(alias = "new_for_value")]
    pub fn for_value(value: &glib::Value) -> ContentProvider {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gdk_content_provider_new_for_value(
                value.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_content_provider_new_union")]
    pub fn new_union(providers: &[ContentProvider]) -> ContentProvider {
        assert_initialized_main_thread!();
        let n_providers = providers.len() as _;
        unsafe {
            from_glib_full(ffi::gdk_content_provider_new_union(
                providers.to_glib_full(),
                n_providers,
            ))
        }
    }
}

pub trait ContentProviderExt: IsA<ContentProvider> + 'static {
    #[doc(alias = "gdk_content_provider_content_changed")]
    fn content_changed(&self) {
        unsafe {
            ffi::gdk_content_provider_content_changed(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_content_provider_ref_formats")]
    #[doc(alias = "ref_formats")]
    fn formats(&self) -> ContentFormats {
        unsafe {
            from_glib_full(ffi::gdk_content_provider_ref_formats(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_content_provider_ref_storable_formats")]
    #[doc(alias = "ref_storable_formats")]
    #[doc(alias = "storable-formats")]
    fn storable_formats(&self) -> ContentFormats {
        unsafe {
            from_glib_full(ffi::gdk_content_provider_ref_storable_formats(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_content_provider_write_mime_type_async")]
    fn write_mime_type_async<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        mime_type: &str,
        stream: &impl IsA<gio::OutputStream>,
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
        unsafe extern "C" fn write_mime_type_async_trampoline<
            P: FnOnce(Result<(), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            ffi::gdk_content_provider_write_mime_type_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = write_mime_type_async_trampoline::<P>;
        unsafe {
            ffi::gdk_content_provider_write_mime_type_async(
                self.as_ref().to_glib_none().0,
                mime_type.to_glib_none().0,
                stream.as_ref().to_glib_none().0,
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn write_mime_type_future(
        &self,
        mime_type: &str,
        stream: &(impl IsA<gio::OutputStream> + Clone + 'static),
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let mime_type = String::from(mime_type);
        let stream = stream.clone();
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.write_mime_type_async(
                &mime_type,
                &stream,
                io_priority,
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    #[doc(alias = "content-changed")]
    fn connect_content_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn content_changed_trampoline<
            P: IsA<ContentProvider>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GdkContentProvider,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ContentProvider::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"content-changed".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    content_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "formats")]
    fn connect_formats_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_formats_trampoline<
            P: IsA<ContentProvider>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GdkContentProvider,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ContentProvider::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::formats".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_formats_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "storable-formats")]
    fn connect_storable_formats_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_storable_formats_trampoline<
            P: IsA<ContentProvider>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GdkContentProvider,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ContentProvider::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::storable-formats".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_storable_formats_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<ContentProvider>> ContentProviderExt for O {}
