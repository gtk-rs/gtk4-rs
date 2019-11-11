use gdk::Rectangle;
use gdk_sys::GdkRectangle;
use glib::object::Cast;
use glib::signal::{connect_raw, SignalHandlerId};
use glib::translate::*;
use glib::IsA;
use glib_sys::{gboolean, gpointer};
use gtk_sys::{GtkOverlay, GtkWidget};
use std::mem::transmute;
use std::ptr;
use Overlay;
use Widget;

pub trait OverlayExtManual: 'static {
    fn connect_get_child_position<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self, &Widget) -> Option<Rectangle> + 'static;
}

impl<O: IsA<Overlay>> OverlayExtManual for O {
    fn connect_get_child_position<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self, &Widget) -> Option<Rectangle> + 'static,
    {
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(
                self.to_glib_none().0 as *mut _,
                b"get-child-position\0".as_ptr() as *mut _,
                Some(transmute(get_child_position_trampoline::<Self, F> as usize)),
                Box::into_raw(f),
            )
        }
    }
}

unsafe extern "C" fn get_child_position_trampoline<
    T,
    F: Fn(&T, &Widget) -> Option<Rectangle> + 'static,
>(
    this: *mut GtkOverlay,
    widget: *mut GtkWidget,
    allocation: *mut GdkRectangle,
    f: gpointer,
) -> gboolean
where
    T: IsA<Overlay>,
{
    let f: &F = &*(f as *const F);
    match f(
        &Overlay::from_glib_borrow(this).unsafe_cast(),
        &from_glib_borrow(widget),
    ) {
        Some(rect) => {
            ptr::write(allocation, ptr::read(rect.to_glib_none().0));
            true
        }
        None => false,
    }
    .to_glib()
}
