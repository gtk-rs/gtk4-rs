// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Clipboard, ContentFormats, ContentProvider};

use crate::subclass::prelude::*;
use glib::object::IsA;
use glib::translate::*;
use glib::{Cast, Object, ToValue, Value};
use std::future::Future;
use std::pin::Pin;

pub trait ContentProviderImpl: ContentProviderImplExt + ObjectImpl {
    fn content_changed(&self, provider: &Self::Type) {
        self.parent_content_changed(provider)
    }

    fn attach_clipboard(&self, provider: &Self::Type, clipboard: &Clipboard) {
        self.parent_attach_clipboard(provider, clipboard)
    }

    fn detach_clipboard(&self, provider: &Self::Type, clipboard: &Clipboard) {
        self.parent_detach_clipboard(provider, clipboard)
    }

    fn formats(&self, provider: &Self::Type) -> ContentFormats {
        self.parent_formats(provider)
    }

    fn storable_formats(&self, provider: &Self::Type) -> ContentFormats {
        self.parent_storable_formats(provider)
    }

    fn write_mime_type_future(
        &self,
        provider: &Self::Type,
        mime_type: &str,
        stream: &gio::OutputStream,
        io_priority: glib::Priority,
    ) -> Pin<Box<dyn Future<Output = Result<(), glib::Error>> + 'static>> {
        self.parent_write_mime_type_future(provider, mime_type, stream, io_priority)
    }

    fn value(&self, provider: &Self::Type, type_: glib::Type) -> Result<Value, glib::Error> {
        self.parent_value(provider, type_)
    }
}

pub trait ContentProviderImplExt: ObjectSubclass {
    fn parent_content_changed(&self, provider: &Self::Type);

    fn parent_attach_clipboard(&self, provider: &Self::Type, clipboard: &Clipboard);

    fn parent_detach_clipboard(&self, provider: &Self::Type, clipboard: &Clipboard);

    fn parent_formats(&self, provider: &Self::Type) -> ContentFormats;

    fn parent_storable_formats(&self, provider: &Self::Type) -> ContentFormats;

    fn parent_write_mime_type_async<
        Q: IsA<gio::Cancellable>,
        R: FnOnce(Result<(), glib::Error>) + Send + 'static,
    >(
        &self,
        provider: &Self::Type,
        mime_type: &str,
        stream: &gio::OutputStream,
        io_priority: glib::Priority,
        cancellable: Option<&Q>,
        callback: R,
    );

    fn parent_write_mime_type_future(
        &self,
        provider: &Self::Type,
        mime_type: &str,
        stream: &gio::OutputStream,
        io_priority: glib::Priority,
    ) -> Pin<Box<dyn Future<Output = Result<(), glib::Error>> + 'static>>;

    fn parent_value(&self, provider: &Self::Type, type_: glib::Type) -> Result<Value, glib::Error>;
}

impl<T: ContentProviderImpl> ContentProviderImplExt for T {
    fn parent_content_changed(&self, provider: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GdkContentProviderClass;
            if let Some(f) = (*parent_class).content_changed {
                f(provider
                    .unsafe_cast_ref::<ContentProvider>()
                    .to_glib_none()
                    .0)
            }
        }
    }

    fn parent_attach_clipboard(&self, provider: &Self::Type, clipboard: &Clipboard) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GdkContentProviderClass;
            if let Some(f) = (*parent_class).attach_clipboard {
                f(
                    provider
                        .unsafe_cast_ref::<ContentProvider>()
                        .to_glib_none()
                        .0,
                    clipboard.to_glib_none().0,
                )
            }
        }
    }

    fn parent_detach_clipboard(&self, provider: &Self::Type, clipboard: &Clipboard) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GdkContentProviderClass;
            if let Some(f) = (*parent_class).detach_clipboard {
                f(
                    provider
                        .unsafe_cast_ref::<ContentProvider>()
                        .to_glib_none()
                        .0,
                    clipboard.to_glib_none().0,
                )
            }
        }
    }

    fn parent_formats(&self, provider: &Self::Type) -> ContentFormats {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GdkContentProviderClass;
            let f = (*parent_class)
                .ref_formats
                .expect("no parent \"ref_formats\" implementation");
            let ret = f(provider
                .unsafe_cast_ref::<ContentProvider>()
                .to_glib_none()
                .0);

            from_glib_full(ret)
        }
    }

    fn parent_storable_formats(&self, provider: &Self::Type) -> ContentFormats {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GdkContentProviderClass;
            let f = (*parent_class)
                .ref_storable_formats
                .expect("no parent \"ref_storable_formats\" implementation");
            let ret = f(provider
                .unsafe_cast_ref::<ContentProvider>()
                .to_glib_none()
                .0);

            from_glib_full(ret)
        }
    }

    fn parent_write_mime_type_async<
        Q: IsA<gio::Cancellable>,
        R: FnOnce(Result<(), glib::Error>) + Send + 'static,
    >(
        &self,
        provider: &Self::Type,
        mime_type: &str,
        stream: &gio::OutputStream,
        io_priority: glib::Priority,
        cancellable: Option<&Q>,
        callback: R,
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GdkContentProviderClass;
            let f = (*parent_class)
                .write_mime_type_async
                .expect("no parent \"write_mime_type_async\" implementation");
            let finish = (*parent_class)
                .write_mime_type_finish
                .expect("no parent \"write_mime_type_finish\" implementation");

            let user_data: Box<(R, _)> = Box::new((callback, finish));

            unsafe extern "C" fn parent_write_mime_type_async_trampoline<
                R: FnOnce(Result<(), glib::Error>) + Send + 'static,
            >(
                source_object_ptr: *mut glib::gobject_ffi::GObject,
                res: *mut gio::ffi::GAsyncResult,
                user_data: glib::ffi::gpointer,
            ) {
                let mut error = std::ptr::null_mut();
                let cb: Box<(
                    R,
                    fn(
                        *mut ffi::GdkContentProvider,
                        *mut gio::ffi::GAsyncResult,
                        *mut *mut glib::ffi::GError,
                    ) -> glib::ffi::gboolean,
                )> = Box::from_raw(user_data as *mut _);
                cb.1(source_object_ptr as _, res, &mut error);
                let result = if error.is_null() {
                    Ok(())
                } else {
                    Err(from_glib_full(error))
                };
                cb.0(result);
            }

            let cancellable = cancellable.map(|p| p.as_ref());
            let callback = parent_write_mime_type_async_trampoline::<R>;
            f(
                provider
                    .unsafe_cast_ref::<ContentProvider>()
                    .to_glib_none()
                    .0,
                mime_type.to_glib_none().0,
                stream.to_glib_none().0,
                io_priority.into_glib(),
                cancellable.to_glib_none().0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    fn parent_write_mime_type_future(
        &self,
        provider: &Self::Type,
        mime_type: &str,
        stream: &gio::OutputStream,
        io_priority: glib::Priority,
    ) -> Pin<Box<dyn Future<Output = Result<(), glib::Error>> + 'static>> {
        let stream = stream.clone();
        let mime_type = String::from(mime_type);
        Box::pin(gio::GioFuture::new(provider, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            let self_ = Self::from_instance(&obj);

            self_.parent_write_mime_type_async(
                &self_.instance(),
                &mime_type,
                &stream,
                io_priority,
                Some(&cancellable),
                move |res| {
                    send.resolve(res);
                },
            );

            cancellable
        }))
    }

    fn parent_value(&self, provider: &Self::Type, type_: glib::Type) -> Result<Value, glib::Error> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GdkContentProviderClass;
            let f = (*parent_class)
                .get_value
                .expect("no parent \"get_value\" implementation");
            let mut value = Value::from_type(type_);

            let mut error = std::ptr::null_mut();
            f(
                provider
                    .unsafe_cast_ref::<ContentProvider>()
                    .to_glib_none()
                    .0,
                value.to_glib_none_mut().0,
                &mut error,
            );

            if error.is_null() {
                Ok(value)
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

unsafe impl<T: ContentProviderImpl> IsSubclassable<T> for ContentProvider {
    fn class_init(class: &mut glib::Class<Self>) {
        <Object as IsSubclassable<T>>::class_init(class);

        let klass = class.as_mut();
        klass.content_changed = Some(content_provider_content_changed::<T>);
        klass.attach_clipboard = Some(content_provider_attach_clipboard::<T>);
        klass.detach_clipboard = Some(content_provider_detach_clipboard::<T>);
        klass.ref_formats = Some(content_provider_formats::<T>);
        klass.ref_storable_formats = Some(content_provider_storable_formats::<T>);
        klass.write_mime_type_async = Some(content_provider_write_mime_type_async::<T>);
        klass.write_mime_type_finish = Some(content_provider_write_mime_type_finish::<T>);
        klass.get_value = Some(content_provider_get_value::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <Object as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn content_provider_content_changed<T: ContentProviderImpl>(
    ptr: *mut ffi::GdkContentProvider,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<ContentProvider> = from_glib_borrow(ptr);

    imp.content_changed(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn content_provider_attach_clipboard<T: ContentProviderImpl>(
    ptr: *mut ffi::GdkContentProvider,
    clipboard_ptr: *mut ffi::GdkClipboard,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<ContentProvider> = from_glib_borrow(ptr);
    let clipboard = from_glib_borrow(clipboard_ptr);

    imp.attach_clipboard(wrap.unsafe_cast_ref(), &clipboard)
}

unsafe extern "C" fn content_provider_detach_clipboard<T: ContentProviderImpl>(
    ptr: *mut ffi::GdkContentProvider,
    clipboard_ptr: *mut ffi::GdkClipboard,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<ContentProvider> = from_glib_borrow(ptr);
    let clipboard = from_glib_borrow(clipboard_ptr);

    imp.detach_clipboard(wrap.unsafe_cast_ref(), &clipboard)
}

unsafe extern "C" fn content_provider_formats<T: ContentProviderImpl>(
    ptr: *mut ffi::GdkContentProvider,
) -> *mut ffi::GdkContentFormats {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<ContentProvider> = from_glib_borrow(ptr);

    imp.formats(wrap.unsafe_cast_ref()).to_glib_full()
}

unsafe extern "C" fn content_provider_storable_formats<T: ContentProviderImpl>(
    ptr: *mut ffi::GdkContentProvider,
) -> *mut ffi::GdkContentFormats {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<ContentProvider> = from_glib_borrow(ptr);

    imp.storable_formats(wrap.unsafe_cast_ref()).to_glib_full()
}

unsafe extern "C" fn content_provider_write_mime_type_async<T: ContentProviderImpl>(
    ptr: *mut ffi::GdkContentProvider,
    mime_type_ptr: *const libc::c_char,
    stream_ptr: *mut gio::ffi::GOutputStream,
    priority: libc::c_int,
    cancellable_ptr: *mut gio::ffi::GCancellable,
    callback: gio::ffi::GAsyncReadyCallback,
    user_data: glib::ffi::gpointer,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: ContentProvider = from_glib_none(ptr);
    let cancellable: Option<gio::Cancellable> = from_glib_none(cancellable_ptr);
    let mime_type: glib::GString = from_glib_none(mime_type_ptr);
    let stream: gio::OutputStream = from_glib_none(stream_ptr);

    let closure = move |result: &gio::AsyncResult, source_object: Option<&glib::Object>| {
        let result: *mut gio::ffi::GAsyncResult = result.to_glib_none().0;
        let source_object: *mut glib::object::GObject = source_object.to_glib_none().0;
        callback.unwrap()(source_object, result, user_data)
    };

    let t = gio::Task::new(
        Some(&wrap.upcast_ref::<glib::Object>()),
        cancellable.as_ref(),
        closure,
    );

    glib::MainContext::default().spawn_local(async move {
        let res = imp
            .write_mime_type_future(
                wrap.unsafe_cast_ref(),
                mime_type.as_str(),
                stream.unsafe_cast_ref::<gio::OutputStream>(),
                from_glib(priority),
            )
            .await;
        match res {
            Ok(_) => t.return_value(&true.to_value()),
            Err(e) => t.return_error(e),
        }
    });
}

unsafe extern "C" fn content_provider_write_mime_type_finish<T: ContentProviderImpl>(
    _ptr: *mut ffi::GdkContentProvider,
    res_ptr: *mut gio::ffi::GAsyncResult,
    error_ptr: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let res: gio::AsyncResult = from_glib_none(res_ptr);
    let t = res.downcast_ref::<gio::Task>().unwrap();
    let ret = t.propagate_value();
    match ret {
        Ok(v) => {
            assert!(v.get::<bool>().unwrap());
            true.into_glib()
        }
        Err(e) => {
            *error_ptr = e.into_raw();
            false.into_glib()
        }
    }
}

unsafe extern "C" fn content_provider_get_value<T: ContentProviderImpl>(
    ptr: *mut ffi::GdkContentProvider,
    value_ptr: *mut glib::gobject_ffi::GValue,
    error_ptr: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<ContentProvider> = from_glib_borrow(ptr);
    let value: Value = from_glib_none(value_ptr);

    let ret = imp.value(wrap.unsafe_cast_ref(), value.type_());
    match ret {
        Ok(v) => {
            glib::gobject_ffi::g_value_copy(v.to_glib_none().0, value_ptr);
            true.into_glib()
        }
        Err(e) => {
            *error_ptr = e.into_raw();
            false.into_glib()
        }
    }
}
