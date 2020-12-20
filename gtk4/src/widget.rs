// Take a look at the license at the top of the repository in the LICENSE file.

use crate::prelude::WidgetExt;
use crate::Widget;

use glib::object::{Cast, IsA, WeakRef};
use glib::translate::*;
use glib::ObjectExt;

use glib::Continue;

pub trait WidgetExtManual: 'static {
    #[doc(alias = "gtk_widget_add_tick_callback")]
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
            widget: *mut ffi::GtkWidget,
            frame_clock: *mut gdk::ffi::GdkFrameClock,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let widget: Borrowed<Widget> = from_glib_borrow(widget);
            let frame_clock = from_glib_borrow(frame_clock);
            let callback: &P = &*(user_data as *mut _);
            let res = (*callback)(&widget.unsafe_cast_ref(), &frame_clock);
            res.to_glib()
        }
        let callback = Some(callback_func::<Self, P> as _);

        unsafe extern "C" fn notify_func<
            O: IsA<Widget>,
            P: Fn(&O, &gdk::FrameClock) -> Continue + 'static,
        >(
            data: glib::ffi::gpointer,
        ) {
            let _callback: Box<P> = Box::from_raw(data as *mut _);
        }
        let destroy_call = Some(notify_func::<Self, P> as _);

        let id = unsafe {
            ffi::gtk_widget_add_tick_callback(
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
            ffi::gtk_widget_set_name(self.as_ref().to_glib_none().0, name.to_glib_none().0);
        }
    }
}

pub struct TickCallbackId {
    id: u32,
    widget: WeakRef<Widget>,
}

impl TickCallbackId {
    #[doc(alias = "gtk_widget_remove_tick_callback")]
    pub fn remove(self) {
        if let Some(widget) = self.widget.upgrade() {
            unsafe {
                ffi::gtk_widget_remove_tick_callback(widget.to_glib_none().0, self.id);
            }
        }
    }
}

pub trait InitializingWidgetExt {
    fn init_template(&self);
}

impl<T: IsA<Widget>> InitializingWidgetExt for glib::subclass::InitializingObject<T> {
    fn init_template(&self) {
        unsafe {
            self.as_ref().init_template();
        }
    }
}
