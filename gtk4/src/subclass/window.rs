// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`Window`](crate::Window).

use crate::{prelude::*, subclass::prelude::*, Window};
use glib::translate::*;

pub trait WindowImpl: WindowImplExt + WidgetImpl {
    fn activate_focus(&self) {
        self.parent_activate_focus()
    }

    fn activate_default(&self) {
        self.parent_activate_default()
    }

    fn keys_changed(&self) {
        self.parent_keys_changed()
    }

    fn enable_debugging(&self, toggle: bool) -> bool {
        self.parent_enable_debugging(toggle)
    }

    fn close_request(&self) -> glib::signal::Inhibit {
        self.parent_close_request()
    }
}

pub trait WindowImplExt: ObjectSubclass {
    fn parent_activate_focus(&self);
    fn parent_activate_default(&self);
    fn parent_keys_changed(&self);
    fn parent_enable_debugging(&self, toggle: bool) -> bool;
    fn parent_close_request(&self) -> glib::signal::Inhibit;
}

impl<T: WindowImpl> WindowImplExt for T {
    fn parent_activate_focus(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWindowClass;
            let f = (*parent_class)
                .activate_focus
                .expect("No parent class impl for \"activate_focus\"");
            f(self.obj().unsafe_cast_ref::<Window>().to_glib_none().0)
        }
    }

    fn parent_activate_default(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWindowClass;
            let f = (*parent_class)
                .activate_default
                .expect("No parent class impl for \"activate_default\"");
            f(self.obj().unsafe_cast_ref::<Window>().to_glib_none().0)
        }
    }

    fn parent_keys_changed(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWindowClass;
            let f = (*parent_class)
                .keys_changed
                .expect("No parent class impl for \"keys_changed\"");
            f(self.obj().unsafe_cast_ref::<Window>().to_glib_none().0)
        }
    }

    fn parent_enable_debugging(&self, toggle: bool) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWindowClass;
            let f = (*parent_class)
                .enable_debugging
                .expect("No parent class impl for \"enable_debugging\"");
            from_glib(f(
                self.obj().unsafe_cast_ref::<Window>().to_glib_none().0,
                toggle.into_glib(),
            ))
        }
    }

    fn parent_close_request(&self) -> glib::signal::Inhibit {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWindowClass;
            let f = (*parent_class)
                .close_request
                .expect("No parent class impl for \"close_request\"");
            glib::signal::Inhibit(from_glib(f(self
                .obj()
                .unsafe_cast_ref::<Window>()
                .to_glib_none()
                .0)))
        }
    }
}

unsafe impl<T: WindowImpl> IsSubclassable<T> for Window {
    fn class_init(class: &mut ::glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.activate_focus = Some(window_activate_focus::<T>);
        klass.activate_default = Some(window_activate_default::<T>);
        klass.keys_changed = Some(window_keys_changed::<T>);
        klass.enable_debugging = Some(window_enable_debugging::<T>);
        klass.close_request = Some(window_close_request::<T>);
    }
}

unsafe extern "C" fn window_activate_focus<T: WindowImpl>(ptr: *mut ffi::GtkWindow) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.activate_focus()
}

unsafe extern "C" fn window_activate_default<T: WindowImpl>(ptr: *mut ffi::GtkWindow) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.activate_default()
}

unsafe extern "C" fn window_keys_changed<T: WindowImpl>(ptr: *mut ffi::GtkWindow) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.keys_changed()
}

unsafe extern "C" fn window_enable_debugging<T: WindowImpl>(
    ptr: *mut ffi::GtkWindow,
    toggleptr: glib::ffi::gboolean,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let toggle: bool = from_glib(toggleptr);

    imp.enable_debugging(toggle).into_glib()
}

unsafe extern "C" fn window_close_request<T: WindowImpl>(
    ptr: *mut ffi::GtkWindow,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.close_request().into_glib()
}
