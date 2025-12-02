// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`Application`].

use glib::translate::*;

use crate::{ffi, prelude::*, subclass::prelude::*, Application, Window};

pub trait GtkApplicationImpl: ApplicationImpl + ObjectSubclass<Type: IsA<Application>> {
    fn window_added(&self, window: &Window) {
        self.parent_window_added(window)
    }

    fn window_removed(&self, window: &Window) {
        self.parent_window_removed(window)
    }

    #[cfg(feature = "v4_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_22")))]
    fn save_state(&self, state: &glib::VariantDict) -> bool {
        self.parent_save_state(state)
    }

    #[cfg(feature = "v4_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_22")))]
    fn restore_state(&self, reason: crate::RestoreReason, state: &glib::Variant) -> bool {
        self.parent_restore_state(reason, state)
    }

    #[cfg(feature = "v4_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_22")))]
    fn restore_window(&self, reason: crate::RestoreReason, state: &glib::Variant) {
        self.parent_restore_window(reason, state)
    }
}

pub trait GtkApplicationImplExt: GtkApplicationImpl {
    fn parent_window_added(&self, window: &Window) {
        unsafe {
            let data = Self::type_data();
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
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkApplicationClass;
            if let Some(f) = (*parent_class).window_removed {
                f(
                    self.obj().unsafe_cast_ref::<Application>().to_glib_none().0,
                    window.to_glib_none().0,
                )
            }
        }
    }

    #[cfg(feature = "v4_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_22")))]
    fn parent_save_state(&self, state: &glib::VariantDict) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkApplicationClass;
            if let Some(f) = (*parent_class).save_state {
                from_glib(f(
                    self.obj().unsafe_cast_ref::<Application>().to_glib_none().0,
                    state.to_glib_none().0,
                ))
            } else {
                false
            }
        }
    }

    #[cfg(feature = "v4_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_22")))]
    fn parent_restore_state(&self, reason: crate::RestoreReason, state: &glib::Variant) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkApplicationClass;
            if let Some(f) = (*parent_class).restore_state {
                from_glib(f(
                    self.obj().unsafe_cast_ref::<Application>().to_glib_none().0,
                    reason.into_glib(),
                    state.to_glib_none().0,
                ))
            } else {
                false
            }
        }
    }

    #[cfg(feature = "v4_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_22")))]
    fn parent_restore_window(&self, reason: crate::RestoreReason, state: &glib::Variant) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkApplicationClass;
            if let Some(f) = (*parent_class).restore_window {
                f(
                    self.obj().unsafe_cast_ref::<Application>().to_glib_none().0,
                    reason.into_glib(),
                    state.to_glib_none().0,
                )
            }
        }
    }
}

impl<T: GtkApplicationImpl> GtkApplicationImplExt for T {}

unsafe impl<T: GtkApplicationImpl> IsSubclassable<T> for Application {
    fn class_init(class: &mut ::glib::Class<Self>) {
        // Override the original `GtkApplication` startup implementation so that
        // we can set GTK as initialized right afterwards.
        {
            use std::sync;

            // Needed because the function pointer is not `Send+Sync` otherwise but has to
            // be to be stored in a `static mut`.
            struct WrapFn(unsafe extern "C" fn(*mut gio::ffi::GApplication));
            unsafe impl Send for WrapFn {}
            unsafe impl Sync for WrapFn {}

            static ONCE: sync::Once = sync::Once::new();
            static mut OLD_STARTUP: Option<WrapFn> = None;

            // One the very first call replace the original `GtkApplication` startup with a
            // function that first calls the original one and then marks gtk-rs as
            // initialized.
            ONCE.call_once(|| unsafe {
                let base_klass =
                    glib::gobject_ffi::g_type_class_ref(ffi::gtk_application_get_type());
                debug_assert!(!base_klass.is_null());

                let app_klass = &mut *(base_klass as *mut gio::ffi::GApplicationClass);
                OLD_STARTUP = app_klass.startup.map(WrapFn);

                unsafe extern "C" fn replace_startup(app: *mut gio::ffi::GApplication) { unsafe {
                    if let Some(WrapFn(old_startup)) = OLD_STARTUP {
                        old_startup(app);
                    }
                    crate::rt::set_initialized();
                }}

                app_klass.startup = Some(replace_startup);

                glib::gobject_ffi::g_type_class_unref(base_klass);
            });
        }

        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.window_added = Some(application_window_added::<T>);
        klass.window_removed = Some(application_window_removed::<T>);

        #[cfg(feature = "v4_22")]
        {
            klass.save_state = Some(application_save_state::<T>);
            klass.restore_state = Some(application_restore_state::<T>);
            klass.restore_window = Some(application_restore_window::<T>);
        }
    }
}

unsafe extern "C" fn application_window_added<T: GtkApplicationImpl>(
    ptr: *mut ffi::GtkApplication,
    wptr: *mut ffi::GtkWindow,
) { unsafe {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.window_added(&from_glib_borrow(wptr))
}}

unsafe extern "C" fn application_window_removed<T: GtkApplicationImpl>(
    ptr: *mut ffi::GtkApplication,
    wptr: *mut ffi::GtkWindow,
) { unsafe {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.window_removed(&from_glib_borrow(wptr))
}}

#[cfg(feature = "v4_22")]
unsafe extern "C" fn application_save_state<T: GtkApplicationImpl>(
    ptr: *mut ffi::GtkApplication,
    state: *mut glib::ffi::GVariantDict,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.save_state(&from_glib_borrow(state)).into_glib()
}

#[cfg(feature = "v4_22")]
unsafe extern "C" fn application_restore_state<T: GtkApplicationImpl>(
    ptr: *mut ffi::GtkApplication,
    reason: ffi::GtkRestoreReason,
    state: *mut glib::ffi::GVariant,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.restore_state(from_glib(reason), &from_glib_borrow(state))
        .into_glib()
}

#[cfg(feature = "v4_22")]
unsafe extern "C" fn application_restore_window<T: GtkApplicationImpl>(
    ptr: *mut ffi::GtkApplication,
    reason: ffi::GtkRestoreReason,
    state: *mut glib::ffi::GVariant,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.restore_window(from_glib(reason), &from_glib_borrow(state))
}
