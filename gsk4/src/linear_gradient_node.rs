use crate::{ColorStop, LinearGradientNode};
use glib::translate::*;
use graphene::{Point, Rect};

impl LinearGradientNode {
    pub fn new(
        bounds: &Rect,
        start: &Point,
        end: &Point,
        color_stops: &[ColorStop],
    ) -> LinearGradientNode {
        assert_initialized_main_thread!();
        let n_color_stops = color_stops.len() as usize;
        unsafe {
            from_glib_full(ffi::gsk_linear_gradient_node_new(
                bounds.to_glib_none().0,
                start.to_glib_none().0,
                end.to_glib_none().0,
                color_stops.to_glib_none().0,
                n_color_stops,
            ))
        }
    }
}
