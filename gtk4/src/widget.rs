// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, Widget};

use glib::{translate::*, Continue, WeakRef};

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of [`Widget`](crate::Widget).
pub trait WidgetExtManual: 'static {
    #[doc(alias = "gtk_widget_add_tick_callback")]
    fn add_tick_callback<P: Fn(&Self, &gdk::FrameClock) -> Continue + 'static>(
        &self,
        callback: P,
    ) -> TickCallbackId;
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
            let res = (*callback)(widget.unsafe_cast_ref(), &frame_clock);
            res.into_glib()
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
}

#[derive(Debug)]
pub struct TickCallbackId {
    id: u32,
    widget: WeakRef<Widget>,
}

impl PartialEq for TickCallbackId {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl TickCallbackId {
    #[doc(alias = "gtk_widget_remove_tick_callback")]
    #[doc(alias = "remove_tick_callback")]
    pub fn remove(self) {
        if let Some(widget) = self.widget.upgrade() {
            unsafe {
                ffi::gtk_widget_remove_tick_callback(widget.to_glib_none().0, self.id);
            }
        }
    }
}
