use gtk_sys;

use glib::translate::*;

use glib::subclass::prelude::*;

use Application;
use ApplicationClass;
use Window;

pub trait GtkApplicationImpl: GtkApplicationImplExt + gio::subclass::prelude::ApplicationImpl + 'static {
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

impl<T: GtkApplicationImpl + ObjectImpl> GtkApplicationImplExt for T {
    fn parent_window_added(&self, application: &Application, window: &Window) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkApplicationClass;
            let f = (*parent_class)
                .window_added
                .expect("No parent class implementation for \"window_added\"");
            f(application.to_glib_none().0, window.to_glib_none().0)
        }
    }

    fn parent_window_removed(&self, application: &Application, window: &Window) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gtk_sys::GtkApplicationClass;
            let f = (*parent_class)
                .window_removed
                .expect("No parent class implementation for \"window_added\"");
            f(application.to_glib_none().0, window.to_glib_none().0)
        }
    }
}

unsafe impl<T: ObjectSubclass + GtkApplicationImpl> IsSubclassable<T> for ApplicationClass {
    fn override_vfuncs(&mut self) {
        <gio::ApplicationClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut gtk_sys::GtkApplicationClass);
            klass.window_added = Some(application_window_added::<T>);
            klass.window_removed = Some(application_window_removed::<T>);
        }
    }
}

unsafe extern "C" fn application_window_added<T: ObjectSubclass>(ptr: *mut gtk_sys::GtkApplication, wptr: *mut gtk_sys::GtkWindow)
where
    T: GtkApplicationImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Application = from_glib_borrow(ptr);

    imp.window_added(&wrap, &from_glib_borrow(wptr))
}

unsafe extern "C" fn application_window_removed<T: ObjectSubclass>(ptr: *mut gtk_sys::GtkApplication, wptr: *mut gtk_sys::GtkWindow)
where
    T: GtkApplicationImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Application = from_glib_borrow(ptr);

    imp.window_removed(&wrap, &from_glib_borrow(wptr))
}
