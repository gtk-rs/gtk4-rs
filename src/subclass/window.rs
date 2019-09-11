use gtk_sys;

use glib::translate::*;

use glib::subclass::prelude::*;

use super::bin::BinImpl;
use Window;
use WindowClass;

pub trait WindowImpl: WindowImplExt + BinImpl + 'static {
    fn activate_focus(&self, window: &Window) {
        self.parent_activate_focus(window)
    }

    fn activate_default(&self, window: &Window) {
        self.parent_activate_default(window)
    }

    fn keys_changed(&self, window: &Window) {
        self.parent_keys_changed(window)
    }

    fn enable_debugging(&self, window: &Window, toggle: bool) -> bool {
        self.parent_enable_debugging(window, toggle)
    }

    fn close_request(&self, window: &Window) -> glib::signal::Inhibit {
        self.parent_close_request(window)
    }
}

pub trait WindowImplExt {
    fn parent_activate_focus(&self, window: &Window);
    fn parent_activate_default(&self, window: &Window);
    fn parent_keys_changed(&self, window: &Window);
    fn parent_enable_debugging(&self, window: &Window, toggle: bool) -> bool;
    fn parent_close_request(&self, window: &Window) -> glib::signal::Inhibit;
}

impl<T: WindowImpl + ObjectImpl> WindowImplExt for T {
    fn parent_activate_focus(&self, window: &Window) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWindowClass;
            let f = (*parent_class)
                .activate_focus
                .expect("No parent class impl for \"activate_focus\"");
            f(window.to_glib_none().0)
        }
    }

    fn parent_activate_default(&self, window: &Window) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWindowClass;
            let f = (*parent_class)
                .activate_default
                .expect("No parent class impl for \"activate_default\"");
            f(window.to_glib_none().0)
        }
    }

    fn parent_keys_changed(&self, window: &Window) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWindowClass;
            let f = (*parent_class)
                .keys_changed
                .expect("No parent class impl for \"keys_changed\"");
            f(window.to_glib_none().0)
        }
    }

    fn parent_enable_debugging(&self, window: &Window, toggle: bool) -> bool {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWindowClass;
            let f = (*parent_class)
                .enable_debugging
                .expect("No parent class impl for \"enable_debugging\"");
            from_glib(f(window.to_glib_none().0, toggle.to_glib()))
        }
    }

    fn parent_close_request(&self, window: &Window) -> glib::signal::Inhibit {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkWindowClass;
            let f = (*parent_class)
                .close_request
                .expect("No parent class impl for \"close_request\"");
            glib::signal::Inhibit(from_glib(f(window.to_glib_none().0)))
        }
    }
}

unsafe impl<T: ObjectSubclass + WindowImpl> IsSubclassable<T> for WindowClass {
    fn override_vfuncs(&mut self) {
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gtk_sys::GtkWindowClass);
            klass.activate_focus = Some(window_activate_focus::<T>);
            klass.activate_default = Some(window_activate_default::<T>);
            klass.keys_changed = Some(window_keys_changed::<T>);
            klass.enable_debugging = Some(window_enable_debugging::<T>);
            klass.close_request = Some(window_close_request::<T>);
        }
    }
}

unsafe extern "C" fn window_activate_focus<T: ObjectSubclass>(ptr: *mut gtk_sys::GtkWindow)
where
    T: WindowImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Window = from_glib_borrow(ptr);

    imp.activate_focus(&wrap)
}

unsafe extern "C" fn window_activate_default<T: ObjectSubclass>(ptr: *mut gtk_sys::GtkWindow)
where
    T: WindowImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Window = from_glib_borrow(ptr);

    imp.activate_default(&wrap)
}

unsafe extern "C" fn window_keys_changed<T: ObjectSubclass>(ptr: *mut gtk_sys::GtkWindow)
where
    T: WindowImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Window = from_glib_borrow(ptr);

    imp.keys_changed(&wrap)
}

unsafe extern "C" fn window_enable_debugging<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWindow,
    toggleptr: glib_sys::gboolean,
) -> glib_sys::gboolean
where
    T: WindowImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Window = from_glib_borrow(ptr);
    let toggle: bool = from_glib(toggleptr);

    imp.enable_debugging(&wrap, toggle).to_glib()
}

unsafe extern "C" fn window_close_request<T: ObjectSubclass>(
    ptr: *mut gtk_sys::GtkWindow,
) -> glib_sys::gboolean
where
    T: WindowImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Window = from_glib_borrow(ptr);

    imp.close_request(&wrap).to_glib()
}
