// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{CellEditable, CellRenderer, CellRendererState, Widget};
use glib::translate::*;
use glib::IsA;

pub trait CellRendererExtManual {
    #[doc(alias = "gtk_cell_renderer_activate")]
    fn activate<Q: IsA<Widget>, R: AsRef<gdk::Event>>(
        &self,
        event: &R,
        widget: &Q,
        path: &str,
        background_area: &gdk::Rectangle,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) -> bool;

    #[doc(alias = "gtk_cell_renderer_start_editing")]
    fn start_editing<Q: IsA<Widget>, R: AsRef<gdk::Event>>(
        &self,
        event: Option<&R>,
        widget: &Q,
        path: &str,
        background_area: &gdk::Rectangle,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) -> Option<CellEditable>;
}

impl<O: IsA<CellRenderer>> CellRendererExtManual for O {
    fn activate<Q: IsA<Widget>, R: AsRef<gdk::Event>>(
        &self,
        event: &R,
        widget: &Q,
        path: &str,
        background_area: &gdk::Rectangle,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) -> bool {
        unsafe {
            from_glib(ffi::gtk_cell_renderer_activate(
                self.as_ref().to_glib_none().0,
                event.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                path.to_glib_none().0,
                background_area.to_glib_none().0,
                cell_area.to_glib_none().0,
                flags.to_glib(),
            ))
        }
    }

    fn start_editing<Q: IsA<Widget>, R: AsRef<gdk::Event>>(
        &self,
        event: Option<&R>,
        widget: &Q,
        path: &str,
        background_area: &gdk::Rectangle,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) -> Option<CellEditable> {
        unsafe {
            from_glib_none(ffi::gtk_cell_renderer_start_editing(
                self.as_ref().to_glib_none().0,
                event.map(|e| e.as_ref()).to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                path.to_glib_none().0,
                background_area.to_glib_none().0,
                cell_area.to_glib_none().0,
                flags.to_glib(),
            ))
        }
    }
}
