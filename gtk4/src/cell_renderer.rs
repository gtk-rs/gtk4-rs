// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{CellEditable, CellRenderer, CellRendererState, Widget};
use glib::translate::*;
use glib::IsA;

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of [`CellRenderer`](crate::CellRenderer).
pub trait CellRendererExtManual {
    #[doc(alias = "gtk_cell_renderer_activate")]
    fn activate(
        &self,
        event: &impl AsRef<gdk::Event>,
        widget: &impl IsA<Widget>,
        path: &str,
        background_area: &gdk::Rectangle,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) -> bool;

    #[doc(alias = "gtk_cell_renderer_start_editing")]
    fn start_editing(
        &self,
        event: Option<&impl AsRef<gdk::Event>>,
        widget: &impl IsA<Widget>,
        path: &str,
        background_area: &gdk::Rectangle,
        cell_area: &gdk::Rectangle,
        flags: CellRendererState,
    ) -> Option<CellEditable>;
}

impl<O: IsA<CellRenderer>> CellRendererExtManual for O {
    fn activate(
        &self,
        event: &impl AsRef<gdk::Event>,
        widget: &impl IsA<Widget>,
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
                flags.into_glib(),
            ))
        }
    }

    fn start_editing(
        &self,
        event: Option<&impl AsRef<gdk::Event>>,
        widget: &impl IsA<Widget>,
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
                flags.into_glib(),
            ))
        }
    }
}
