// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{CellArea, CellAreaContext, CellRenderer, CellRendererState, Widget};
use gdk::Event;
use glib::translate::*;
use glib::{IsA, ToValue};

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

    #[doc(alias = "gtk_cell_area_cell_get_valist")]
    #[doc(alias = "gtk_cell_area_cell_get_property")]
    fn cell_get<P: IsA<CellRenderer>>(&self, renderer: &P, property_name: &str) -> glib::Value;

    #[doc(alias = "gtk_cell_area_cell_set_valist")]
    #[doc(alias = "gtk_cell_area_cell_set_property")]
    fn cell_set<P: IsA<CellRenderer>>(
        &self,
        renderer: &P,
        property_name: &str,
        value: &dyn ToValue,
    );
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

    fn cell_get<P: IsA<CellRenderer>>(&self, renderer: &P, property_name: &str) -> glib::Value {
        unsafe {
            let mut value = glib::Value::uninitialized();
            ffi::gtk_cell_area_cell_get_property(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
                value.to_glib_none_mut().0,
            );
            value
        }
    }

    fn cell_set<P: IsA<CellRenderer>>(
        &self,
        renderer: &P,
        property_name: &str,
        value: &dyn ToValue,
    ) {
        unsafe {
            ffi::gtk_cell_area_cell_set_property(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
                value.to_value().to_glib_none().0,
            );
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
