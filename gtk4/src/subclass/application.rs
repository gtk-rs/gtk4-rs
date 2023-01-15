// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`Application`](crate::Application).

use glib::translate::*;

use crate::{prelude::*, subclass::prelude::*, Application, Window};

pub trait GtkApplicationImpl: ObjectImpl + GtkApplicationImplExt + ApplicationImpl {
    fn window_added(&self, window: &Window) {
        self.parent_window_added(window)
    }

    fn window_removed(&self, window: &Window) {
        self.parent_window_removed(window)
    }
}

pub trait GtkApplicationImplExt: ObjectSubclass {
    fn parent_window_added(&self, window: &Window);
    fn parent_window_removed(&self, window: &Window);
}

impl<T: GtkApplicationImpl> GtkApplicationImplExt for T {
    fn parent_window_added(&self, window: &Window) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkApplicationClass;
            if let Some(f) = (*parent_class).window_added {
                f(
                    self.obj().unsafe_cast_ref::<Application>().to_glib_none().0,
                    window.to_glib_none().0,
                )
            }
        }
    }

    fn parent_window_removed(&self, window: &Window) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkApplicationClass;
            if let Some(f) = (*parent_class).window_removed {
                f(
                    self.obj().unsafe_cast_ref::<Application>().to_glib_none().0,
                    window.to_glib_none().0,
                )
            }
        }
    }
}

unsafe impl<T: GtkApplicationImpl> IsSubclassable<T> for Application {
    fn class_init(class: &mut ::glib::Class<Self>) {
        // Override the original `GtkApplication` startup implementation so that
        // we can set GTK as initialized right afterwards.
        {
            use std::sync;

            // Needed because the function pointer is not `Send+Sync` otherwise but has to be to be
            // stored in a `static mut`.
            struct WrapFn(unsafe extern "C" fn(*mut gio::ffi::GApplication));
            unsafe impl Send for WrapFn {}
            unsafe impl Sync for WrapFn {}

            static ONCE: sync::Once = sync::Once::new();
            static mut OLD_STARTUP: Option<WrapFn> = None;

            // One the very first call replace the original `GtkApplication` startup with a
            // function that first calls the original one and then marks gtk-rs as initialized.
            ONCE.call_once(|| unsafe {
                let base_klass =
                    glib::gobject_ffi::g_type_class_ref(ffi::gtk_application_get_type());
                debug_assert!(!base_klass.is_null());

                let app_klass = &mut *(base_klass as *mut gio::ffi::GApplicationClass);
                OLD_STARTUP = app_klass.startup.map(WrapFn);

                unsafe extern "C" fn replace_startup(app: *mut gio::ffi::GApplication) {
                    if let Some(WrapFn(old_startup)) = OLD_STARTUP {
                        old_startup(app);
                    }
                    crate::rt::set_initialized();
                }

                app_klass.startup = Some(replace_startup);

                glib::gobject_ffi::g_type_class_unref(base_klass);
            });
        }

        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.window_added = Some(application_window_added::<T>);
        klass.window_removed = Some(application_window_removed::<T>);
    }
}

unsafe extern "C" fn application_window_added<T: GtkApplicationImpl>(
    ptr: *mut ffi::GtkApplication,
    wptr: *mut ffi::GtkWindow,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.window_added(&from_glib_borrow(wptr))
}

unsafe extern "C" fn application_window_removed<T: GtkApplicationImpl>(
    ptr: *mut ffi::GtkApplication,
    wptr: *mut ffi::GtkWindow,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.window_removed(&from_glib_borrow(wptr))
}
