// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{CellArea, CellAreaContext, CellRenderer, CellRendererState, Widget};
use gdk::Event;
use glib::translate::*;
use glib::IsA;

pub trait CellAreaExtManual {
    #[doc(alias = "gtk_cell_area_activate_cell")]
    fn activate_cell<P: IsA<Widget>, Q: IsA<CellRenderer>, R: AsRef<Event>>(
        &self,
        widget: &P,
        renderer: &Q,
        event: &R,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) -> bool;

    #[doc(alias = "gtk_cell_area_event")]
    fn event<P: IsA<CellAreaContext>, Q: IsA<Widget>, R: AsRef<Event>>(
        &self,
        context: &P,
        widget: &Q,
        event: &R,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) -> i32;
}

impl<O: IsA<CellArea>> CellAreaExtManual for O {
    fn activate_cell<P: IsA<Widget>, Q: IsA<CellRenderer>, R: AsRef<Event>>(
        &self,
        widget: &P,
        renderer: &Q,
        event: &R,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_area_activate_cell(
                self.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                event.as_ref().to_glib_none().0,
                cell_area.to_glib_none().0,
                flags.to_glib(),
            ))
        }
    }

    fn event<P: IsA<CellAreaContext>, Q: IsA<Widget>, R: AsRef<Event>>(
        &self,
        context: &P,
        widget: &Q,
        event: &R,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) -> i32 {
        unsafe {
            ffi::gtk_cell_area_event(
                self.as_ref().to_glib_none().0,
                context.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                event.as_ref().to_glib_none().0,
                cell_area.to_glib_none().0,
                flags.to_glib(),
            )
        }
    }
}
