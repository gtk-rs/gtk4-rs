use gtk_sys;

use glib::subclass::prelude::*;
use glib::translate::*;

use Application;
use ApplicationClass;
use Window;

pub trait GtkApplicationImpl:
    GtkApplicationImplExt + gio::subclass::prelude::ApplicationImpl
{
    fn window_added(&self, application: &Application, window: &Window) {
        self.parent_window_added(application, window)
    }

    fn window_removed(&self, application: &Application, window: &Window) {
        self.parent_window_removed(application, window)
    }
}

pub trait GtkApplicationImplExt {
    fn parent_window_added(&self, application: &Application, window: &Window);
    fn parent_window_removed(&self, application: &Application, window: &Window);
}

impl<T: GtkApplicationImpl> GtkApplicationImplExt for T {
    fn parent_window_added(&self, application: &Application, window: &Window) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gtk_sys::GtkApplicationClass;
            if let Some(f) = (*parent_class).window_added {
                f(application.to_glib_none().0, window.to_glib_none().0)
            }
        }
    }

    fn parent_window_removed(&self, application: &Application, window: &Window) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gtk_sys::GtkApplicationClass;
            if let Some(f) = (*parent_class).window_removed {
                f(application.to_glib_none().0, window.to_glib_none().0)
            }
        }
    }
}

unsafe impl<T: GtkApplicationImpl> IsSubclassable<T> for ApplicationClass {
    fn override_vfuncs(&mut self) {
        unsafe extern "C" fn application_window_added<T: GtkApplicationImpl>(
            ptr: *mut gtk_sys::GtkApplication,
            wptr: *mut gtk_sys::GtkWindow,
        ) {
            let instance = &*(ptr as *mut T::Instance);
            let imp = instance.get_impl();
            let wrap: Borrowed<Application> = from_glib_borrow(ptr);

            imp.window_added(&wrap, &from_glib_borrow(wptr))
        }
        unsafe extern "C" fn application_window_removed<T: GtkApplicationImpl>(
            ptr: *mut gtk_sys::GtkApplication,
            wptr: *mut gtk_sys::GtkWindow,
        ) {
            let instance = &*(ptr as *mut T::Instance);
            let imp = instance.get_impl();
            let wrap: Borrowed<Application> = from_glib_borrow(ptr);

            imp.window_removed(&wrap, &from_glib_borrow(wptr))
        }

        unsafe extern "C" fn application_startup<T: GtkApplicationImpl>(
            ptr: *mut gio_sys::GApplication,
        ) {
            let instance = &*(ptr as *mut T::Instance);
            let imp = instance.get_impl();
            let wrap: Borrowed<gio::Application> = from_glib_borrow(ptr);
            crate::rt::set_initialized();
            imp.startup(&wrap)
        }

        <gio::ApplicationClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gtk_sys::GtkApplicationClass);
            klass.window_added = Some(application_window_added::<T>);
            klass.window_removed = Some(application_window_removed::<T>);
            // Chain our startup handler in here
            let klass = &mut *(self as *mut Self as *mut gio_sys::GApplicationClass);
            klass.startup = Some(application_startup::<T>);
        }
    }
}
