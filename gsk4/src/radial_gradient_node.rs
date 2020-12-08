use crate::{ColorStop, RadialGradientNode};
use glib::translate::*;

impl RadialGradientNode {
    #[doc(alias = "gsk_radial_gradient_node_new")]
    pub fn new(
        bounds: &graphene::Rect,
        center: &graphene::Point,
        hradius: f32,
        vradius: f32,
        start: f32,
        end: f32,
        color_stops: &[ColorStop],
    ) -> RadialGradientNode {
        assert_initialized_main_thread!();
        let n_color_stops = color_stops.len() as usize;
        unsafe {
            from_glib_full(ffi::gsk_radial_gradient_node_new(
                bounds.to_glib_none().0,
                center.to_glib_none().0,
                hradius,
                vradius,
                start,
                end,
                color_stops.to_glib_none().0,
                n_color_stops,
            ))
        }
    }
}
