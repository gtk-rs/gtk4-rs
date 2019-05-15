use Widget;
use glib::object::IsA;
use glib::translate::*;
use gtk_sys;

pub trait WidgetExtManual: 'static {
    fn set_name(&self, name: &str);
}

impl<O: IsA<Widget>> WidgetExtManual for O {
    fn set_name(&self, name: &str) {
        unsafe {
            gtk_sys::gtk_widget_set_name(self.as_ref().to_glib_none().0, name.to_glib_none().0);
        }
    }
}
