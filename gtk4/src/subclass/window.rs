// Take a look at the license at the top of the repository in the LICENSE file.

use crate::subclass::prelude::*;
use crate::{Widget, Window};
use glib::translate::*;
use glib::Cast;

pub trait WindowImpl: WindowImplExt + WidgetImpl {
    fn activate_focus(&self, window: &Self::Type) {
        self.parent_activate_focus(window)
    }

    fn activate_default(&self, window: &Self::Type) {
        self.parent_activate_default(window)
    }

    fn keys_changed(&self, window: &Self::Type) {
        self.parent_keys_changed(window)
    }

    fn enable_debugging(&self, window: &Self::Type, toggle: bool) -> bool {
        self.parent_enable_debugging(window, toggle)
    }

    fn close_request(&self, window: &Self::Type) -> glib::signal::Inhibit {
        self.parent_close_request(window)
    }
}

pub trait WindowImplExt: ObjectSubclass {
    fn parent_activate_focus(&self, window: &Self::Type);
    fn parent_activate_default(&self, window: &Self::Type);
    fn parent_keys_changed(&self, window: &Self::Type);
    fn parent_enable_debugging(&self, window: &Self::Type, toggle: bool) -> bool;
    fn parent_close_request(&self, window: &Self::Type) -> glib::signal::Inhibit;
}

impl<T: WindowImpl> WindowImplExt for T {
    fn parent_activate_focus(&self, window: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWindowClass;
            let f = (*parent_class)
                .activate_focus
                .expect("No parent class impl for \"activate_focus\"");
            f(window.unsafe_cast_ref::<Window>().to_glib_none().0)
        }
    }

    fn parent_activate_default(&self, window: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWindowClass;
            let f = (*parent_class)
                .activate_default
                .expect("No parent class impl for \"activate_default\"");
            f(window.unsafe_cast_ref::<Window>().to_glib_none().0)
        }
    }

    fn parent_keys_changed(&self, window: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWindowClass;
            let f = (*parent_class)
                .keys_changed
                .expect("No parent class impl for \"keys_changed\"");
            f(window.unsafe_cast_ref::<Window>().to_glib_none().0)
        }
    }

    fn parent_enable_debugging(&self, window: &Self::Type, toggle: bool) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWindowClass;
            let f = (*parent_class)
                .enable_debugging
                .expect("No parent class impl for \"enable_debugging\"");
            from_glib(f(
                window.unsafe_cast_ref::<Window>().to_glib_none().0,
                toggle.into_glib(),
            ))
        }
    }

    fn parent_close_request(&self, window: &Self::Type) -> glib::signal::Inhibit {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkWindowClass;
            let f = (*parent_class)
                .close_request
                .expect("No parent class impl for \"close_request\"");
            glib::signal::Inhibit(from_glib(f(window
                .unsafe_cast_ref::<Window>()
                .to_glib_none()
                .0)))
        }
    }
}

unsafe impl<T: WindowImpl> IsSubclassable<T> for Window {
    fn class_init(class: &mut ::glib::Class<Self>) {
        <Widget as IsSubclassable<T>>::class_init(class);

        let klass = class.as_mut();
        klass.activate_focus = Some(window_activate_focus::<T>);
        klass.activate_default = Some(window_activate_default::<T>);
        klass.keys_changed = Some(window_keys_changed::<T>);
        klass.enable_debugging = Some(window_enable_debugging::<T>);
        klass.close_request = Some(window_close_request::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <Widget as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn window_activate_focus<T: WindowImpl>(ptr: *mut ffi::GtkWindow) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Window> = from_glib_borrow(ptr);

    imp.activate_focus(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn window_activate_default<T: WindowImpl>(ptr: *mut ffi::GtkWindow) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Window> = from_glib_borrow(ptr);

    imp.activate_default(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn window_keys_changed<T: WindowImpl>(ptr: *mut ffi::GtkWindow) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Window> = from_glib_borrow(ptr);

    imp.keys_changed(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn window_enable_debugging<T: WindowImpl>(
    ptr: *mut ffi::GtkWindow,
    toggleptr: glib::ffi::gboolean,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Window> = from_glib_borrow(ptr);
    let toggle: bool = from_glib(toggleptr);

    imp.enable_debugging(wrap.unsafe_cast_ref(), toggle)
        .into_glib()
}

unsafe extern "C" fn window_close_request<T: WindowImpl>(
    ptr: *mut ffi::GtkWindow,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Window> = from_glib_borrow(ptr);

    imp.close_request(wrap.unsafe_cast_ref()).into_glib()
}
