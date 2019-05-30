use Widget;
use glib::object::{Cast, IsA, WeakRef};
use glib::translate::*;
use glib::ObjectExt;
use gdk;
use gtk_sys;

use Continue;

pub trait WidgetExtManual: 'static {
    fn add_tick_callback<P: Fn(&Self, &gdk::FrameClock) -> Continue + 'static>(
        &self,
        callback: P,
    ) -> TickCallbackId;

    fn set_name(&self, name: &str);
}

impl<O: IsA<Widget>> WidgetExtManual for O {
    fn add_tick_callback<P: Fn(&Self, &gdk::FrameClock) -> Continue + 'static>(
        &self,
        callback: P,
    ) -> TickCallbackId {
        let callback_data: Box<P> = Box::new(callback);

        unsafe extern "C" fn callback_func<
            O: IsA<Widget>,
            P: Fn(&O, &gdk::FrameClock) -> Continue + 'static,
        >(
            widget: *mut gtk_sys::GtkWidget,
            frame_clock: *mut gdk_sys::GdkFrameClock,
            user_data: glib_sys::gpointer,
        ) -> glib_sys::gboolean {
            let widget: Widget = from_glib_borrow(widget);
            let widget = widget.downcast().unwrap();
            let frame_clock = from_glib_borrow(frame_clock);
            let callback: &P = &*(user_data as *mut _);
            let res = (*callback)(&widget, &frame_clock);
            res.to_glib()
        }
        let callback = Some(callback_func::<Self, P> as _);

        unsafe extern "C" fn notify_func<
            O: IsA<Widget>,
            P: Fn(&O, &gdk::FrameClock) -> Continue + 'static,
        >(
            data: glib_sys::gpointer,
        ) {
            let _callback: Box<P> = Box::from_raw(data as *mut _);
        }
        let destroy_call = Some(notify_func::<Self, P> as _);

        let id = unsafe {
            gtk_sys::gtk_widget_add_tick_callback(
                self.as_ref().to_glib_none().0,
                callback,
                Box::into_raw(callback_data) as *mut _,
                destroy_call,
            )
        };
        TickCallbackId {
            id,
            widget: self.upcast_ref().downgrade(),
        }
    }

    fn set_name(&self, name: &str) {
        unsafe {
            gtk_sys::gtk_widget_set_name(self.as_ref().to_glib_none().0, name.to_glib_none().0);
        }
    }
}

pub struct TickCallbackId {
    id: u32,
    widget: WeakRef<Widget>,
}

impl TickCallbackId {
    pub fn remove(self) {
        if let Some(widget) = self.widget.upgrade() {
            unsafe {
                gtk_sys::gtk_widget_remove_tick_callback(widget.to_glib_none().0, self.id);
            }
        }
    }
}
