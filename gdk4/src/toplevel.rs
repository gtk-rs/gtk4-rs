// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Event, Toplevel};
use glib::translate::*;
use glib::IsA;

pub trait ToplevelExtManual {
    #[doc(alias = "gdk_toplevel_inhibit_system_shortcuts")]
    fn inhibit_system_shortcuts<P: AsRef<Event>>(&self, event: Option<&P>);

    #[doc(alias = "gdk_toplevel_show_window_menu")]
    fn show_window_menu<P: AsRef<Event>>(&self, event: &P) -> bool;
}

impl<O: IsA<Toplevel>> ToplevelExtManual for O {
    fn inhibit_system_shortcuts<P: AsRef<Event>>(&self, event: Option<&P>) {
        unsafe {
            ffi::gdk_toplevel_inhibit_system_shortcuts(
                self.as_ref().to_glib_none().0,
                event.map(|e| e.as_ref()).to_glib_none().0,
            );
        }
    }

    fn show_window_menu<P: AsRef<Event>>(&self, event: &P) -> bool {
        unsafe {
            from_glib(ffi::gdk_toplevel_show_window_menu(
                self.as_ref().to_glib_none().0,
                event.as_ref().to_glib_none().0,
            ))
        }
    }
}
