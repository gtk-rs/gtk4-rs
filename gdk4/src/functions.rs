// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, ContentDeserializer, ContentSerializer};
use glib::translate::*;
use std::{future, pin::Pin, ptr};

#[repr(packed)]
pub struct GRange(pub i32, pub i32);

#[doc(alias = "gdk_pango_layout_get_clip_region")]
pub fn pango_layout_get_clip_region(
    layout: &pango::Layout,
    x_origin: i32,
    y_origin: i32,
    index_ranges: &[GRange],
) -> cairo::Region {
    assert_initialized_main_thread!();

    let ptr: *const i32 = index_ranges.as_ptr() as _;
    unsafe {
        from_glib_full(ffi::gdk_pango_layout_get_clip_region(
            layout.to_glib_none().0,
            x_origin,
            y_origin,
            ptr,
            (index_ranges.len() / 2) as i32,
        ))
    }
}

#[doc(alias = "gdk_content_deserialize_async")]
pub fn content_deserialize_async<R: FnOnce(Result<glib::Value, glib::Error>) + 'static>(
    stream: &impl IsA<gio::InputStream>,
    mime_type: &str,
    type_: glib::types::Type,
    io_priority: glib::Priority,
    cancellable: Option<&impl IsA<gio::Cancellable>>,
    callback: R,
) {
    assert_initialized_main_thread!();
    let main_context = glib::MainContext::ref_thread_default();
    let is_main_context_owner = main_context.is_owner();
    let has_acquired_main_context = (!is_main_context_owner)
        .then(|| main_context.acquire().ok())
        .flatten();
    assert!(
        is_main_context_owner || has_acquired_main_context.is_some(),
        "Async operations only allowed if the thread is owning the MainContext"
    );

    let user_data: Box<glib::thread_guard::ThreadGuard<R>> =
        Box::new(glib::thread_guard::ThreadGuard::new(callback));
    unsafe extern "C" fn content_deserialize_async_trampoline<
        R: FnOnce(Result<glib::Value, glib::Error>) + 'static,
    >(
        _source_object: *mut glib::gobject_ffi::GObject,
        res: *mut gio::ffi::GAsyncResult,
        user_data: glib::ffi::gpointer,
    ) {
        let mut error = ptr::null_mut();
        let mut value = glib::Value::uninitialized();
        let _ = ffi::gdk_content_deserialize_finish(res, value.to_glib_none_mut().0, &mut error);
        let result = if error.is_null() {
            Ok(value)
        } else {
            Err(from_glib_full(error))
        };
        let callback: Box<glib::thread_guard::ThreadGuard<R>> = Box::from_raw(user_data as *mut _);
        let callback = callback.into_inner();
        callback(result);
    }
    let callback = content_deserialize_async_trampoline::<R>;
    unsafe {
        ffi::gdk_content_deserialize_async(
            stream.as_ref().to_glib_none().0,
            mime_type.to_glib_none().0,
            type_.into_glib(),
            io_priority.into_glib(),
            cancellable.map(|p| p.as_ref()).to_glib_none().0,
            Some(callback),
            Box::into_raw(user_data) as *mut _,
        );
    }
}

pub fn content_deserialize_future(
    stream: &(impl IsA<gio::InputStream> + Clone + 'static),
    mime_type: &str,
    type_: glib::types::Type,
    io_priority: glib::Priority,
) -> Pin<Box<dyn future::Future<Output = Result<glib::Value, glib::Error>> + 'static>> {
    assert_initialized_main_thread!();

    let stream = stream.clone();
    let mime_type = String::from(mime_type);
    Box::pin(gio::GioFuture::new(&(), move |_obj, cancellable, send| {
        content_deserialize_async(
            &stream,
            &mime_type,
            type_,
            io_priority,
            Some(cancellable),
            move |res| {
                send.resolve(res);
            },
        );
    }))
}

#[doc(alias = "gdk_content_register_deserializer")]
pub fn content_register_deserializer<
    T: 'static,
    P: Fn(&ContentDeserializer, &mut Option<T>) + 'static,
>(
    mime_type: &str,
    type_: glib::types::Type,
    deserialize: P,
) {
    assert_initialized_main_thread!();
    let deserialize_data: Box<P> = Box::new(deserialize);
    unsafe extern "C" fn deserialize_func<
        T: 'static,
        P: Fn(&ContentDeserializer, &mut Option<T>) + 'static,
    >(
        deserializer: *mut ffi::GdkContentDeserializer,
    ) {
        let deserializer: ContentDeserializer = from_glib_full(deserializer);
        let callback: &P =
            &*(ffi::gdk_content_deserializer_get_user_data(deserializer.to_glib_none().0)
                as *mut _);

        let mut task_data: *mut Option<T> =
            ffi::gdk_content_deserializer_get_task_data(deserializer.to_glib_none().0) as *mut _;
        if task_data.is_null() {
            unsafe extern "C" fn notify_func<T: 'static>(data: glib::ffi::gpointer) {
                let _task_data: Box<Option<T>> = Box::from_raw(data as *mut _);
            }
            task_data = Box::into_raw(Box::new(None));
            ffi::gdk_content_deserializer_set_task_data(
                deserializer.to_glib_none().0,
                task_data as *mut _,
                Some(notify_func::<T>),
            );
        }

        (*callback)(&deserializer, &mut *task_data);
    }
    let deserialize = Some(deserialize_func::<T, P> as _);
    unsafe extern "C" fn notify_func<
        T: 'static,
        P: Fn(&ContentDeserializer, &mut Option<T>) + 'static,
    >(
        data: glib::ffi::gpointer,
    ) {
        let _callback: Box<P> = Box::from_raw(data as *mut _);
    }
    let destroy_call4 = Some(notify_func::<T, P> as _);
    let super_callback0: Box<P> = deserialize_data;
    unsafe {
        ffi::gdk_content_register_deserializer(
            mime_type.to_glib_none().0,
            type_.into_glib(),
            deserialize,
            Box::into_raw(super_callback0) as *mut _,
            destroy_call4,
        );
    }
}

#[doc(alias = "gdk_content_register_serializer")]
pub fn content_register_serializer<
    T: 'static,
    P: Fn(&ContentSerializer, &mut Option<T>) + 'static,
>(
    type_: glib::types::Type,
    mime_type: &str,
    serialize: P,
) {
    assert_initialized_main_thread!();
    let serialize_data: Box<P> = Box::new(serialize);
    unsafe extern "C" fn serialize_func<
        T: 'static,
        P: Fn(&ContentSerializer, &mut Option<T>) + 'static,
    >(
        serializer: *mut ffi::GdkContentSerializer,
    ) {
        let serializer: ContentSerializer = from_glib_full(serializer);
        let callback: &P =
            &*(ffi::gdk_content_serializer_get_user_data(serializer.to_glib_none().0) as *mut _);

        let mut task_data: *mut Option<T> =
            ffi::gdk_content_serializer_get_task_data(serializer.to_glib_none().0) as *mut _;
        if task_data.is_null() {
            unsafe extern "C" fn notify_func<T: 'static>(data: glib::ffi::gpointer) {
                let _task_data: Box<Option<T>> = Box::from_raw(data as *mut _);
            }
            task_data = Box::into_raw(Box::new(None));
            ffi::gdk_content_serializer_set_task_data(
                serializer.to_glib_none().0,
                task_data as *mut _,
                Some(notify_func::<T>),
            );
        }

        (*callback)(&serializer, &mut *task_data);
    }
    let serialize = Some(serialize_func::<T, P> as _);
    unsafe extern "C" fn notify_func<
        T: 'static,
        P: Fn(&ContentSerializer, &mut Option<T>) + 'static,
    >(
        data: glib::ffi::gpointer,
    ) {
        let _callback: Box<P> = Box::from_raw(data as *mut _);
    }
    let destroy_call4 = Some(notify_func::<T, P> as _);
    let super_callback0: Box<P> = serialize_data;
    unsafe {
        ffi::gdk_content_register_serializer(
            type_.into_glib(),
            mime_type.to_glib_none().0,
            serialize,
            Box::into_raw(super_callback0) as *mut _,
            destroy_call4,
        );
    }
}

#[doc(alias = "gdk_content_serialize_async")]
pub fn content_serialize_async<R: FnOnce(Result<(), glib::Error>) + 'static>(
    stream: &impl IsA<gio::OutputStream>,
    mime_type: &str,
    value: &glib::Value,
    io_priority: glib::Priority,
    cancellable: Option<&impl IsA<gio::Cancellable>>,
    callback: R,
) {
    assert_initialized_main_thread!();
    let main_context = glib::MainContext::ref_thread_default();
    let is_main_context_owner = main_context.is_owner();
    let has_acquired_main_context = (!is_main_context_owner)
        .then(|| main_context.acquire().ok())
        .flatten();
    assert!(
        is_main_context_owner || has_acquired_main_context.is_some(),
        "Async operations only allowed if the thread is owning the MainContext"
    );
    let user_data: Box<glib::thread_guard::ThreadGuard<R>> =
        Box::new(glib::thread_guard::ThreadGuard::new(callback));
    unsafe extern "C" fn content_serialize_async_trampoline<
        R: FnOnce(Result<(), glib::Error>) + 'static,
    >(
        _source_object: *mut glib::gobject_ffi::GObject,
        res: *mut gio::ffi::GAsyncResult,
        user_data: glib::ffi::gpointer,
    ) {
        let mut error = ptr::null_mut();
        let _ = ffi::gdk_content_serialize_finish(res, &mut error);
        let result = if error.is_null() {
            Ok(())
        } else {
            Err(from_glib_full(error))
        };
        let callback: Box<glib::thread_guard::ThreadGuard<R>> = Box::from_raw(user_data as *mut _);
        let callback = callback.into_inner();
        callback(result);
    }
    let callback = content_serialize_async_trampoline::<R>;
    unsafe {
        ffi::gdk_content_serialize_async(
            stream.as_ref().to_glib_none().0,
            mime_type.to_glib_none().0,
            value.to_glib_none().0,
            io_priority.into_glib(),
            cancellable.map(|p| p.as_ref()).to_glib_none().0,
            Some(callback),
            Box::into_raw(user_data) as *mut _,
        );
    }
}

pub fn content_serialize_future(
    stream: &(impl IsA<gio::OutputStream> + Clone + 'static),
    mime_type: &str,
    value: &glib::Value,
    io_priority: glib::Priority,
) -> Pin<Box<dyn future::Future<Output = Result<(), glib::Error>> + 'static>> {
    assert_initialized_main_thread!();

    let stream = stream.clone();
    let mime_type = String::from(mime_type);
    let value = value.clone();
    Box::pin(gio::GioFuture::new(&(), move |_obj, cancellable, send| {
        content_serialize_async(
            &stream,
            &mime_type,
            &value,
            io_priority,
            Some(cancellable),
            move |res| {
                send.resolve(res);
            },
        );
    }))
}

#[doc(alias = "gdk_pango_layout_line_get_clip_region")]
pub fn pango_layout_line_get_clip_region(
    line: &pango::LayoutLine,
    x_origin: i32,
    y_origin: i32,
    index_ranges: &[GRange],
) -> cairo::Region {
    assert_initialized_main_thread!();

    let ptr: *const i32 = index_ranges.as_ptr() as _;
    unsafe {
        from_glib_full(ffi::gdk_pango_layout_line_get_clip_region(
            line.to_glib_none().0,
            x_origin,
            y_origin,
            mut_override(ptr),
            (index_ranges.len() / 2) as i32,
        ))
    }
}
